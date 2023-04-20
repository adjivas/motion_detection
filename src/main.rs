#[macro_use]
extern crate dotenv_codegen;

mod home;
mod magick;

use home::{if_absent_send_image, callback_update_presence, ABSENT};
use lazy_static::lazy_static;
use magick::compare;
use magick_rust::magick_wand_genesis;
use mosquitto_client::{Mosquitto, TopicMatcher};
use std::{error::Error, thread, time, sync::{Once, Arc}};
use tokio::time::{self as TokioTime, Duration as TokioDuration};
use tokio::sync::{RwLock, mpsc::channel};

static MQTT_NAME: &'static str = dotenv!("MQTT_NAME");
static MQTT_HOST: &'static str = dotenv!("MQTT_HOST");
static MQTT_PORT: &'static str = dotenv!("MQTT_PORT");
static MQTT_SUBSCRIBE: &'static str = dotenv!("MQTT_TOPIC_DOOR");
static MQTT_PUBLISH: &'static str = dotenv!("MQTT_TOPIC_MOTION");
static MQTT_INTERVAL: time::Duration = time::Duration::from_secs(1);
static HTTP_CGI_INTERVAL: TokioDuration = TokioDuration::from_secs(1);

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

lazy_static! {
    static ref MQTT: Mosquitto = {
        let mqtt = Mosquitto::new(MQTT_NAME);
        let port = MQTT_PORT.parse::<u32>().unwrap();

        mqtt.connect(MQTT_HOST, port).unwrap();
        mqtt
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    START.call_once(|| magick_wand_genesis());
    
    let lock = Arc::new(RwLock::new(ABSENT));
    let lock_cam = lock.clone();

    let (sender, mut receiver) = channel(2);
    let sender_cam = sender.clone();
    tokio::spawn(async move {
        let mut interval = TokioTime::interval(HTTP_CGI_INTERVAL);

        loop {
            if_absent_send_image(&lock_cam, &sender_cam).await.unwrap();
            interval.tick().await;
        }
    });

    tokio::spawn(async move {
        let event_leave: TopicMatcher = MQTT.subscribe(MQTT_SUBSCRIBE, 1).unwrap();
        let mut call = MQTT.callbacks(());

        call.on_message(|_, message| callback_update_presence(&lock, &event_leave, message));
        loop {
            thread::sleep(MQTT_INTERVAL);

            MQTT.do_loop(-1).unwrap();
        }
    });

    while let (Some(past), Some(ref present)) = (receiver.recv().await, receiver.recv().await) {
        sender.send(present.clone()).await?;

        let distortion = compare(&past, present)?;
        MQTT.publish(&MQTT_PUBLISH, distortion.to_string().as_bytes(), 1, false)?;
    }
    MQTT.disconnect()?;
    Ok(())
}
