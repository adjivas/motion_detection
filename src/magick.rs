use magick_rust::{MagickWand, bindings};
use bindings::MetricType_RootMeanSquaredErrorMetric as MetricType;

static FIT: usize = 100;

pub fn compare(past: &[u8], present: &[u8]) -> Result<f64, Box<dyn std::error::Error>> {
    let wand_past = MagickWand::new();
    wand_past.read_image_blob(past)?;
    wand_past.fit(FIT, FIT);

    let wand_present = MagickWand::new();
    wand_present.read_image_blob(present)?;
    wand_present.fit(FIT, FIT);

    let (distortion, _diff) =
        wand_past.compare_images(&wand_present, MetricType);
    Ok(distortion)
}
