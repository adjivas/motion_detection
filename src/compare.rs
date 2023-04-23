use image::DynamicImage;
use image_compare::prelude::*;

pub fn compare(
    past: &DynamicImage,
    present: &DynamicImage,
) -> Result<f64, Box<dyn std::error::Error>> {
    let distortion = image_compare::gray_similarity_structure(
        &Algorithm::RootMeanSquared,
        &past.to_luma8(),
        &present.to_luma8(),
    )?;

    println!("D: {}", distortion.score);

    Ok(distortion.score)
}
