use image::DynamicImage;
use lazy_static::lazy_static;
use mosquitto_client::{MosqMessage, TopicMatcher};
use reqwest::{Client, RequestBuilder};
use tokio::{sync::{RwLock, mpsc::Sender}, runtime::Handle};
use std::{error::Error, sync::Arc};

type Lock = Arc<RwLock<bool>>;

static HTTP_CGI_HOST: &'static str = dotenv!("HTTP_CGI_HOST");
static HTTP_CGI_USER: &'static str = dotenv!("HTTP_CGI_USER");
static HTTP_CGI_PASS: &'static str = dotenv!("HTTP_CGI_PASS");
pub static ABSENT: bool = false;
pub static PRESENT: bool = !ABSENT;

lazy_static! {
    static ref REQ_CAM: RequestBuilder = {
        Client::new()
            .get(HTTP_CGI_HOST)
            .basic_auth(HTTP_CGI_USER, Some(HTTP_CGI_PASS))
    };
}

async fn send_image(sender_cam: &Sender<DynamicImage>) -> Result<(), Box<dyn Error>> {
    let req = REQ_CAM.try_clone().unwrap();
    let buf = req.send().await?
                 .bytes().await?;
    let img = image::load_from_memory(buf.as_ref())?;

    sender_cam.send(img).await?;
    Ok(())
}

pub async fn if_absent_send_image(lock: &Lock, sender_cam: &Sender<DynamicImage>) -> Result<(), Box<dyn Error>> {
    if let Ok(is_locked) = lock.try_read() {
        if *is_locked == ABSENT {
            send_image(&sender_cam).await?;
        }
    }
    Ok(())
}

async fn update_presence(lock: &Lock, message: &str) -> Result<(), Box<dyn Error>> {
    let mut lock = lock.write().await;
    if "present" == message {
        *lock = PRESENT;
    } else if "absent" == message {
        *lock = ABSENT;
    }
    Ok(())
}

pub fn callback_update_presence(lock: &Lock, event: &TopicMatcher, message: MosqMessage) {
    if ! message.retained() && event.matches(&message) {
        tokio::task::block_in_place(move || {
            Handle::current().block_on(async move {
                update_presence(lock, message.text()).await.unwrap();
            })
        });
    }
}
