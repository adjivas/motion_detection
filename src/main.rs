mod request;
mod compare;

use std::error::Error;
use std::{time::Duration, thread::sleep};
use std::iter;
use compare::{load, compare};
use rumqttc::{Client, MqttOptions, QoS, Outgoing, Event};
use request::get_image;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "magick")]
    std::sync::Once::new().call_once(|| magick_rust::magick_wand_genesis());

    let mqtt_name = Box::leak(std::env::var("MQTT_NAME").unwrap().into_boxed_str()) as &'static str;
    let mqtt_host = Box::leak(std::env::var("MQTT_HOST").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = Box::leak(std::env::var("MQTT_PORT").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = mqtt_port.parse::<u16>().unwrap();
    let mqtt_publish = Box::leak(std::env::var("MQTT_PUBLISH").unwrap().into_boxed_str()) as &'static str;
    let mqtt_interval = Duration::from_secs(5);
    let http_cgi_host = Box::leak(std::env::var("HTTP_CGI_HOST").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_user = Box::leak(std::env::var("HTTP_CGI_USER").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_pass = Box::leak(std::env::var("HTTP_CGI_PASS").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_interval = Duration::from_secs(1);
    let motion_sensibility = Box::leak(std::env::var("MOTION_SENSIBILITY").unwrap().into_boxed_str()) as &'static str;
    let motion_sensibility = motion_sensibility.parse::<f64>().unwrap();


    let mut mqtt_options = MqttOptions::new(mqtt_name, mqtt_host, mqtt_port);
    mqtt_options.set_keep_alive(mqtt_interval);
    let (mut client, mut eventloop) = Client::new(mqtt_options, 10);

    let mut it_images = iter::repeat_with(|| {
        sleep(http_cgi_interval);
        let buf = get_image(http_cgi_host, http_cgi_user, http_cgi_pass).unwrap();

        load(buf).unwrap()
    }).peekable();

    loop {
        let before = it_images.next().unwrap();
        let after = it_images.peek().unwrap();
        let distortion = compare(&before, &after)?;

        if motion_sensibility > distortion {
            client.publish(mqtt_publish, QoS::AtMostOnce, false, distortion.to_string().as_bytes()).unwrap();
            eventloop.iter().take_while(|x| match dbg!(x) {
                Ok(Event::Outgoing(Outgoing::Publish(_))) => false,
                _ => true,
            }).for_each(drop);
        }
    }
}
