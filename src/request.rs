use bytes::Bytes;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use std::error::Error;

pub async fn req_send_image(sender: &Sender<Bytes>) -> Result<(), Box<dyn Error>> {
    let http_cgi_host = Box::leak(std::env::var("HTTP_CGI_HOST").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_user = Box::leak(std::env::var("HTTP_CGI_USER").unwrap().into_boxed_str()) as &'static str;
    let http_cgi_pass = Box::leak(std::env::var("HTTP_CGI_PASS").unwrap().into_boxed_str()) as &'static str;

    let buf = Client::new()
        .get(http_cgi_host)
        .basic_auth(http_cgi_user, Some(http_cgi_pass)).send().await?
        .bytes().await?;

    Ok(sender.send(buf).await?)
}
