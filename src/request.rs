use bytes::Bytes;
use std::error::Error;

pub fn get_image(http_cgi_host: &str, http_cgi_user: &str, http_cgi_pass: &str) -> Result<Bytes, Box<dyn Error>> {
    let request = reqwest::blocking::Client::new()
        .get(http_cgi_host)
        .basic_auth(http_cgi_user, Some(http_cgi_pass));

    Ok(request.send()?.bytes()?)
}
