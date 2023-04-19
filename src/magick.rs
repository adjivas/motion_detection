use magick_rust::{MagickWand, bindings};

pub fn compare(past: &[u8], present: &[u8]) -> Result<f64, Box<dyn std::error::Error>> {
    let wand = MagickWand::new();
    wand.read_image_blob(past)?;
    wand.fit(10, 10);

    let wand2 = MagickWand::new();
    wand2.read_image_blob(present)?;
    wand2.fit(10, 10);

    let (distortion, _diff) =
        wand.compare_images(&wand2, bindings::MetricType_RootMeanSquaredErrorMetric);
    Ok(distortion)
}
