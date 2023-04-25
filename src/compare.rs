#[cfg(all(feature = "nomagick", not(feature = "magick")))]
pub fn load(buf: bytes::Bytes) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let image = image::load_from_memory(buf.as_ref())?;

    Ok(image)
}

#[cfg(all(feature = "magick", not(feature = "nomagick")))]
pub fn load(buf: bytes::Bytes) -> Result<magick_rust::MagickWand, Box<dyn std::error::Error>> {
    let wand = magick_rust::MagickWand::new();
    wand.read_image_blob(buf)?;
    wand.fit(100, 100);

    Ok(wand)
}

#[cfg(all(feature = "nomagick", not(feature = "magick")))]
pub fn compare(past: &image::DynamicImage, present: &image::DynamicImage) -> Result<f64, Box<dyn std::error::Error>> {
    let distortion = image_compare::gray_similarity_structure(
        &image_compare::Algorithm::RootMeanSquared,
        &past.to_luma8(),
        &present.to_luma8()
    )?;

    dbg!(distortion.score);

    Ok(distortion.score)
}

#[cfg(all(feature = "magick", not(feature = "nomagick")))]
pub fn compare(past: &magick_rust::MagickWand, present: &magick_rust::MagickWand) -> Result<f64, Box<dyn std::error::Error>> {
    let (distortion, _diff) =
        past.compare_images(&present, magick_rust::bindings::MetricType_RootMeanSquaredErrorMetric);

    dbg!(distortion);

    Ok(distortion)
}

#[cfg(all(feature = "magick", feature = "nomagick"))]
pub fn load(_: bytes::Bytes) -> Result<magick_rust::MagickWand, Box<dyn std::error::Error>> {
    unimplemented!()
}

#[cfg(all(feature = "magick", feature = "nomagick"))]
pub fn compare(_: &magick_rust::MagickWand, _: &magick_rust::MagickWand) -> Result<f64, Box<dyn std::error::Error>> {
    unimplemented!()
}
