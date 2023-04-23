use bytes::Bytes;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use std::error::Error;

static HTTP_CGI_HOST: &'static str = dotenv!("HTTP_CGI_HOST");
static HTTP_CGI_USER: &'static str = dotenv!("HTTP_CGI_USER");
static HTTP_CGI_PASS: &'static str = dotenv!("HTTP_CGI_PASS");

pub async fn req_send_image(sender: &Sender<Bytes>) -> Result<(), Box<dyn Error>> {
    let buf = Client::new()
        .get(HTTP_CGI_HOST)
        .basic_auth(HTTP_CGI_USER, Some(HTTP_CGI_PASS)).send().await?
        .bytes().await?;

    Ok(sender.send(buf).await?)
}
