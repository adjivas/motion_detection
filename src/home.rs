use bytes::Bytes;
use reqwest::RequestBuilder;
use std::error::Error;
use tokio::sync::mpsc::Sender;

pub async fn send_image(
    request: RequestBuilder,
    sender_cam: &Sender<Bytes>,
) -> Result<(), Box<dyn Error>> {
    let buf = request.send().await?.bytes().await?;
    sender_cam.send(buf).await?;
    Ok(())
}
