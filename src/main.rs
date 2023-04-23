#[macro_use]
extern crate dotenv_codegen;

mod home;
mod compare;

use std::{error::Error, thread, time, sync::Arc};
use home::{send_image, update_presence, ABSENT};
use lazy_static::lazy_static;
use compare::compare;
use rumqttc::{self, Client, AsyncClient, MqttOptions, QoS, Event, Incoming};
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
    static ref MQTT_OPTIONS: MqttOptions = {
        let port = MQTT_PORT.parse::<u16>().unwrap();
        
        MqttOptions::new(MQTT_NAME, MQTT_HOST, port)
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let rt  = Runtime::new()?;

    let lock = Arc::new(RwLock::new(ABSENT));
    let lock_cam = lock.clone();

    let (sender, mut receiver) = channel(2);
    rt.spawn(async move {
        let mut interval = TokioTime::interval(HTTP_CGI_INTERVAL);

        loop {
            let is_locked = lock_cam.try_read().unwrap();
            if *is_locked == ABSENT {
                send_image(&sender).await.unwrap();
            }
            interval.tick().await;
        }
    });

    let (client, mut eventloop) = AsyncClient::new(MQTT_OPTIONS.clone(), 10);
    rt.spawn(async move {
        client
            .subscribe(MQTT_SUBSCRIBE, QoS::AtMostOnce)    
            .await    
            .unwrap();

        while let Ok(notification) = eventloop.poll().await {
            if let Event::Incoming(Incoming::Publish(publish)) = notification {
                update_presence(&lock, &publish.payload).await.unwrap();
            }
            thread::sleep(MQTT_INTERVAL);
        }
    });

    let (mut client, mut eventloop) = Client::new(MQTT_OPTIONS.clone(), 10);

    let ref before = receiver.blocking_recv().unwrap();
    let mut before = image::load_from_memory(before)?;
    while let Some(ref after) = receiver.blocking_recv() {
        let after = image::load_from_memory(after)?;

        let distortion = compare(&before, &after)?;

        client.publish(MQTT_PUBLISH, QoS::AtLeastOnce, true, distortion.to_string().as_bytes()).unwrap();
        eventloop.recv().unwrap()?;
        before = after;
    }
    Ok(())
}
