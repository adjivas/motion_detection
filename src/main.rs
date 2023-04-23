mod compare;
mod home;

use compare::compare;
use home::send_image;
use mosquitto_client::{Mosquitto, TopicMatcher};
use reqwest::Client;
use std::fmt::Display;
use std::{error::Error, thread, time};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::time::{self as TokioTime, Duration as TokioDuration};

static MQTT_INTERVAL: time::Duration = time::Duration::from_secs(1);
static HTTP_CGI_INTERVAL: TokioDuration = TokioDuration::from_secs(1);

#[derive(Debug)]
enum CameraCommand {
    Send,
}
#[derive(Debug)]
struct PresenceStateError;
impl Display for PresenceStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for PresenceStateError {}
#[derive(Debug, Clone, Copy)]
enum PresenceState {
    Present,
    Absent,
}

impl Display for PresenceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&str>::from(*self))
    }
}

impl TryFrom<&str> for PresenceState {
    type Error = PresenceStateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let state = match value {
            "present" => Self::Present,
            "absent" => Self::Absent,
            _ => return Err(PresenceStateError),
        };
        Ok(state)
    }
}

impl From<PresenceState> for &str {
    fn from(value: PresenceState) -> Self {
        match value {
            PresenceState::Present => "present",
            PresenceState::Absent => "absent",
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Leaking is bad. But we want immutable ref for the whole program lifetime.
    // Also it's data from ENV so it already the program lifetime.
    let mqtt_name = Box::leak(std::env::var("MQTT_NAME").unwrap().into_boxed_str()) as &'static str;
    let mqtt_host = Box::leak(std::env::var("MQTT_HOST").unwrap().into_boxed_str()) as &'static str;
    let mqtt_port = Box::leak(std::env::var("MQTT_PORT").unwrap().into_boxed_str()) as &'static str;
    let mqtt_subscribe =
        Box::leak(std::env::var("MQTT_TOPIC_DOOR").unwrap().into_boxed_str()) as &'static str;
    let mqtt_publish =
        Box::leak(std::env::var("MQTT_TOPIC_MOTION").unwrap().into_boxed_str()) as &'static str;

    let http_cgi_host =
        Box::leak(std::env::var("HTTP_CGI_HOST").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_user =
        Box::leak(std::env::var("HTTP_CGI_USER").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_pass =
        Box::leak(std::env::var("HTTP_CGI_PASS").unwrap().into_boxed_str()) as &'static str;
    let rt = Runtime::new()?;

    let (sender_cam, mut receiver) = channel(2);

    let mosquitto = {
        let mqtt = Mosquitto::new(&mqtt_name);
        let port = mqtt_port.parse::<u32>().unwrap();
        mqtt.connect(&mqtt_host, port).unwrap();
        mqtt
    };

    let (camera_cmd_tx, mut camera_cmd_rx) = channel::<CameraCommand>(1);

    let mosquitto_event_leave = mosquitto.clone();
    // Moskitto sync thread sending on_message blocking message to Async HTTP image send only
    // if absent message is found.
    let moskitto_thread = std::thread::spawn(move || {
        let event_leave: TopicMatcher =
            mosquitto_event_leave.subscribe(&mqtt_subscribe, 1).unwrap();
        let mut call = mosquitto_event_leave.callbacks(());

        call.on_message(|_, message| {
            if !message.retained() && event_leave.matches(&message) {
                let state = PresenceState::try_from(message.text());
                match state {
                    Ok(PresenceState::Absent) => {
                        camera_cmd_tx.blocking_send(CameraCommand::Send).unwrap()
                    }
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        });
        loop {
            thread::sleep(MQTT_INTERVAL);
            mosquitto_event_leave.do_loop(-1).unwrap();
        }
    });

    // Tokio task awaiting orders from moskitto sync thread
    // If a command arrive then issue a HTTP call for the image then send the result to
    // the main thread
    let handle_camera_send: tokio::task::JoinHandle<Result<(), ()>> = rt.spawn(async move {
        let mut interval = TokioTime::interval(HTTP_CGI_INTERVAL);

        while let Some(cmd) = camera_cmd_rx.recv().await {
            match cmd {
                CameraCommand::Send => {
                    let request = Client::new()
                        .get(http_cgi_host)
                        .basic_auth(http_cgi_user, Some(http_cgi_pass));
                    send_image(request, &sender_cam).await.map_err(|_| ())?
                }
            }
            interval.tick().await;
        }
        Ok(())
    });

    // Withing the main thread to the comparison of the image sended by the HTTP tokio task.
    let ref before = receiver.blocking_recv().unwrap();
    let mut before = image::load_from_memory(before)?;
    while let Some(ref after) = receiver.blocking_recv() {
        let after = image::load_from_memory(after)?;
        let distortion = compare(&before, &after)?;
        mosquitto.publish(&mqtt_publish, distortion.to_string().as_bytes(), 1, false)?;
        before = after;
    }

    // Lets finish them all.
    moskitto_thread.join().unwrap();
    mosquitto.disconnect()?;
    handle_camera_send.abort();
    Ok(())
}
