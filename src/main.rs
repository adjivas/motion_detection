use reqwest::blocking::Client;
use magick_rust::{MagickWand, magick_wand_genesis, bindings};
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    
    let client = Client::new();

    let user_name = "user".to_string();
    let password: Option<String> = Some(String::from("password"));

    let mut buf: Vec<u8> = vec![];

    let _resp = client
        .get("http://192.168.0.21:8080/cgi-bin/snap.cgi")
        .basic_auth(user_name, password)
        .send()?
        .copy_to(&mut buf)?;

    let wand1 = MagickWand::new();
    wand1.read_image_blob(buf.as_slice())?;

    let wand2 = MagickWand::new();
    wand2.read_image_blob(buf.as_slice())?;

    let (distortion, _diff) =
        wand1.compare_images(&wand2, bindings::MetricType_RootMeanSquaredErrorMetric);

    println!("R! {:?}", distortion);
    Ok(())
}
