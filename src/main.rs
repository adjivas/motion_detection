mod request;
mod compare;

use std::error::Error;
use request::req_send_image;
use compare::compare;
use rumqttc::{self, Client, MqttOptions, QoS};
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::channel;
use tokio::runtime::Runtime;


fn main() -> Result<(), Box<dyn Error>> {
    let mqtt_name = Box::leak(std::env::var("MQTT_NAME").unwrap().into_boxed_str()) as &'static str;
    let mqtt_host = Box::leak(std::env::var("MQTT_HOST").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = Box::leak(std::env::var("MQTT_PORT").unwrap().into_boxed_str()) as &'static str;
    let mqtt_publish = Box::leak(std::env::var("MQTT_PUBLISH").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_host = Box::leak(std::env::var("HTTP_CGI_HOST").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_user = Box::leak(std::env::var("HTTP_CGI_USER").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_pass = Box::leak(std::env::var("HTTP_CGI_PASS").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_interval = Duration::from_secs(1);

    let rt  = Runtime::new()?;

    let (sender, mut receiver) = channel(1);
    rt.spawn(async move {
        let build = reqwest::Client::new()
            .get(http_cgi_host)
            .basic_auth(http_cgi_user, Some(http_cgi_pass));
        while let Some(build) = build.try_clone() {
            req_send_image(build, &sender).await.unwrap();
            sleep(http_cgi_interval).await;
        }
    });

    let port = mqtt_port.parse::<u16>().unwrap();

    let options = MqttOptions::new(mqtt_name, mqtt_host, port);
    let (mut client, mut eventloop) = Client::new(options, 2);

    let ref before = receiver.blocking_recv().unwrap();
    let mut before = image::load_from_memory(before)?;
    while let Some(ref after) = receiver.blocking_recv() {
        let after = image::load_from_memory(after)?;

        let distortion = compare(&before, &after)?;

        client.publish(mqtt_publish, QoS::AtLeastOnce, true, distortion.to_string().as_bytes()).unwrap();
        eventloop.recv().unwrap()?;
        before = after;
    }
    Ok(())
}
