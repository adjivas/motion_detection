use bytes::Bytes;
use reqwest::RequestBuilder;
use tokio::sync::mpsc::Sender;
use std::error::Error;

pub async fn req_send_image(client: RequestBuilder, sender: &Sender<Bytes>) -> Result<(), Box<dyn Error>> {
    let buf = client.send().await?
        .bytes().await?;

    Ok(sender.send(buf).await?)
}
