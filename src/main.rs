#[macro_use]
extern crate dotenv_codegen;

mod request;
mod compare;

use std::error::Error;
use request::req_send_image;
use lazy_static::lazy_static;
use compare::compare;
use rumqttc::{self, Client, MqttOptions, QoS};
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::channel;
use tokio::runtime::Runtime;

static MQTT_NAME: &'static str = dotenv!("MQTT_NAME");
static MQTT_HOST: &'static str = dotenv!("MQTT_HOST");
static MQTT_PORT: &'static str = dotenv!("MQTT_PORT");
static MQTT_PUBLISH: &'static str = dotenv!("MQTT_TOPIC_MOTION");
static HTTP_CGI_INTERVAL: Duration = Duration::from_secs(1);

lazy_static! {
    static ref MQTT_OPTIONS: MqttOptions = {
        let port = MQTT_PORT.parse::<u16>().unwrap();
        
        MqttOptions::new(MQTT_NAME, MQTT_HOST, port)
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let rt  = Runtime::new()?;

    let (sender, mut receiver) = channel(2);
    rt.spawn(async move {
        loop {
            req_send_image(&sender).await.unwrap();
            sleep(HTTP_CGI_INTERVAL).await;
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
