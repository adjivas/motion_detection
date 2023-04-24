use image::DynamicImage;

pub fn compare(past: &DynamicImage, present: &DynamicImage) -> Result<f64, Box<dyn std::error::Error>> {
    let distortion = image_compare::rgb_hybrid_compare(
        &past.to_rgb8(),
        &present.to_rgb8()
    )?;


    dbg!(distortion.score);

    Ok(distortion.score)
}
