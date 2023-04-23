use bytes::Bytes;
use reqwest::Client;
use tokio::sync::{RwLock, mpsc::Sender};
use std::{error::Error, sync::Arc};

type Lock = Arc<RwLock<bool>>;

static HTTP_CGI_HOST: &'static str = dotenv!("HTTP_CGI_HOST");
static HTTP_CGI_USER: &'static str = dotenv!("HTTP_CGI_USER");
static HTTP_CGI_PASS: &'static str = dotenv!("HTTP_CGI_PASS");
pub static ABSENT: bool = false;
pub static PRESENT: bool = !ABSENT;

pub async fn send_image(sender: &Sender<Bytes>) -> Result<(), Box<dyn Error>> {
    let buf = Client::new()
        .get(HTTP_CGI_HOST)
        .basic_auth(HTTP_CGI_USER, Some(HTTP_CGI_PASS)).send().await?
        .bytes().await?;

    Ok(sender.send(buf).await?)
}

pub async fn update_presence(lock: &Lock, message: &Bytes) -> Result<(), Box<dyn Error>> {
    let mut lock = lock.write().await;
    if "present" == message {
        *lock = PRESENT;
    } else if "absent" == message {
        *lock = ABSENT;
    }
    Ok(())
}
