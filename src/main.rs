mod magick;

use magick::compare;

use std::sync::Once;
use std::sync::mpsc::channel;
use lazy_static::lazy_static;
use tokio::time::{self, Duration};
use reqwest::{Client, RequestBuilder};
use mosquitto_client::Mosquitto;
use magick_rust::magick_wand_genesis;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

lazy_static! {
    static ref MQTT: Mosquitto = {
        let mqtt = Mosquitto::new("test");

        mqtt.connect("localhost" ,1883).unwrap();
        mqtt
    };
    static ref REQ_CAM: RequestBuilder = {
        let user_name = "user".to_string();
        let password: Option<String> = Some(String::from("password"));
    
        Client::new()
            .get("http://192.168.0.21:8080/cgi-bin/snap.cgi")
            .basic_auth(user_name, password)
    };
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let _: () = START.call_once(|| magick_wand_genesis());

    let (sender, receiver) = channel();
    let sender2 = sender.clone();
    let mut interval = time::interval(Duration::from_secs(1));
    tokio::spawn(async move {
        loop {
            interval.tick().await;
        
            let buf = REQ_CAM.try_clone().unwrap()
                     .send().await.unwrap()
                     .bytes().await.unwrap();
            sender.send(buf).unwrap();
        }
    });

    while let (Ok(past), Ok(ref present)) = (receiver.recv(), receiver.recv()) {
        sender2.send(present.clone()).unwrap();

        let distortion = compare(&past, present)?;
        MQTT.publish("home/doorbell/motion", distortion.to_string().as_bytes(), 1, false).unwrap();
    }
    MQTT.disconnect()?;
    Ok(())
}
