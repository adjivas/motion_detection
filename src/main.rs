#[macro_use]
extern crate dotenv_codegen;

mod home;
mod compare;

use home::{if_absent_send_image, callback_update_presence, ABSENT};
use lazy_static::lazy_static;
use compare::compare;
use mosquitto_client::{Mosquitto, TopicMatcher};
use std::{error::Error, thread, time, sync::Arc};
use tokio::time::{self as TokioTime, Duration as TokioDuration};
use tokio::sync::{RwLock, mpsc::channel};
use tokio::runtime::Runtime;

static MQTT_NAME: &'static str = dotenv!("MQTT_NAME");
static MQTT_HOST: &'static str = dotenv!("MQTT_HOST");
static MQTT_PORT: &'static str = dotenv!("MQTT_PORT");
static MQTT_SUBSCRIBE: &'static str = dotenv!("MQTT_TOPIC_DOOR");
static MQTT_PUBLISH: &'static str = dotenv!("MQTT_TOPIC_MOTION");
static MQTT_INTERVAL: time::Duration = time::Duration::from_secs(1);
static HTTP_CGI_INTERVAL: TokioDuration = TokioDuration::from_secs(1);

lazy_static! {
    static ref MQTT: Mosquitto = {
        let mqtt = Mosquitto::new(MQTT_NAME);
        let port = MQTT_PORT.parse::<u32>().unwrap();

        mqtt.connect(MQTT_HOST, port).unwrap();
        mqtt
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let rt  = Runtime::new()?;

    let lock = Arc::new(RwLock::new(ABSENT));
    let lock_cam = lock.clone();

    let (sender, mut receiver) = channel(2);
    let sender_cam = sender.clone();
    rt.spawn(async move {
        let mut interval = TokioTime::interval(HTTP_CGI_INTERVAL);

        loop {
            if_absent_send_image(&lock_cam, &sender_cam).await.unwrap();
            interval.tick().await;
        }
    });

    rt.spawn(async move {
        let event_leave: TopicMatcher = MQTT.subscribe(MQTT_SUBSCRIBE, 1).unwrap();
        let mut call = MQTT.callbacks(());

        call.on_message(|_, message| callback_update_presence(&lock, &event_leave, message));
        loop {
            thread::sleep(MQTT_INTERVAL);

            MQTT.do_loop(-1).unwrap();
        }
    });

    let ref before = receiver.blocking_recv().unwrap();
    let mut before = image::load_from_memory(before)?;
    while let Some(ref after)  = receiver.blocking_recv() {
        let after = image::load_from_memory(after)?;

        let distortion = compare(&before, &after)?;
        MQTT.publish(&MQTT_PUBLISH, distortion.to_string().as_bytes(), 1, false)?;
        before = after;
    }
    MQTT.disconnect()?;
    Ok(())
}
