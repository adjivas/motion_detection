pub fn load(buf: bytes::Bytes) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let image = image::load_from_memory(buf.as_ref())?;

    Ok(image)
}

pub fn compare(past: &image::DynamicImage, present: &image::DynamicImage) -> Result<f64, Box<dyn std::error::Error>> {
    let distortion = image_compare::gray_similarity_structure(
        &image_compare::Algorithm::RootMeanSquared,
        &past.to_luma8(),
        &present.to_luma8()
    )?;

    Ok(distortion.score)
}
