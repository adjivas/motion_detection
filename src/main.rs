mod request;
mod compare;

use std::error::Error;
use std::{time::Duration, thread::sleep};
use std::iter;
use compare::{load, compare};
use rumqttc::v5::{Client, MqttOptions, Event};
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::mqttbytes::v5::Packet;
use rumqttc::Outgoing;
use request::get_image;

fn main() -> Result<(), Box<dyn Error>> {
    let mqtt_name = Box::leak(std::env::var("MQTT_NAME").unwrap().into_boxed_str()) as &'static str;
    let mqtt_host = Box::leak(std::env::var("MQTT_HOST").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = Box::leak(std::env::var("MQTT_PORT").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = mqtt_port.parse::<u16>().unwrap();
    let mqtt_publish = Box::leak(std::env::var("MQTT_PUBLISH").unwrap().into_boxed_str()) as &'static str;
    let mqtt_interval_keep_alive = Duration::from_secs(5);
    let mqtt_interval_recv = Duration::from_millis(100);
    let http_cgi_host = Box::leak(std::env::var("HTTP_CGI_HOST").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_user = Box::leak(std::env::var("HTTP_CGI_USER").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_pass = Box::leak(std::env::var("HTTP_CGI_PASS").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_interval = Duration::from_millis(100);
    let motion_sensibility = Box::leak(std::env::var("MOTION_SENSIBILITY").unwrap().into_boxed_str()) as &'static str;
    let motion_sensibility = motion_sensibility.parse::<f64>().unwrap();


    let mut mqtt_options = MqttOptions::new(mqtt_name, mqtt_host, mqtt_port);
    mqtt_options.set_keep_alive(mqtt_interval_keep_alive);
    mqtt_options.set_clean_start(false);
    let (client, mut eventloop) = Client::new(mqtt_options, 10);

    let mut it_images = iter::repeat_with(|| {
        sleep(http_cgi_interval);
        let buf = get_image(http_cgi_host, http_cgi_user, http_cgi_pass).unwrap();

        load(buf).unwrap()
    }).peekable();

    loop {
        if let (Some(before), Some(after)) = (it_images.next(), it_images.peek()) {
            let distortion = compare(&before, &after)?;
            while let Ok(event) = eventloop.recv_timeout(mqtt_interval_recv) {
                match event {
                    Ok(Event::Outgoing(Outgoing::Publish(_))) => {},
                    Ok(Event::Outgoing(Outgoing::PingReq)) => {},
                    Ok(Event::Incoming(Packet::ConnAck(_))) => {
                    },
                    Ok(Event::Incoming(Packet::PingReq(_))) => {},
                    Ok(Event::Incoming(Packet::PubAck(_))) => {},
                    Ok(Event::Incoming(Packet::PingResp(_))) => {},
                    why => println!("Err {:?}", why),
                }
            }
            if distortion <= motion_sensibility {
                println!("{}:{}", chrono::Utc::now(), distortion);
                if let Err(why) = client.try_publish(mqtt_publish, QoS::AtMostOnce, false, distortion.to_string()) {
                    println!("Err {:?}", why)
                }
            }
        }
    }
}
