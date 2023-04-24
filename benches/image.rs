extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

mod data;

use std::time::Duration;
use image_compare::prelude::*;

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(3));
    targets = criterion_benchmark
}
criterion_main!(benches);


fn load_from_memory() {
    assert!(image::load_from_memory(data::RAW_DATA).is_ok());
}

fn open() {
    let path = std::path::Path::new(data::PATH_DATA);

    assert!(image::open(path).is_ok());
}

fn reader() {
    let reader = image::io::Reader::new(std::io::Cursor::new(data::RAW_DATA))
                                   .with_guessed_format().expect("REASON");

    assert!(reader.decode().is_ok());
}

fn compare_rgb8_hybrid(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::rgb_hybrid_compare(
        &a.to_rgb8(),
        &b.to_rgb8()
    );

    assert!(compare.is_ok());
}

fn compare_luma8_rootmeansquared(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::gray_similarity_structure(
        &Algorithm::RootMeanSquared,
        &a.to_luma8(),
        &b.to_luma8()
    );

    assert!(compare.is_ok());
}

fn compare_luma8_mssimsimple(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::gray_similarity_structure(
        &Algorithm::MSSIMSimple,
        &a.to_luma8(),
        &b.to_luma8()
    );

    assert!(compare.is_ok());
}

pub fn criterion_benchmark(bench: &mut Criterion) {
    bench.bench_function("load_from_memory", |bench| bench.iter(|| load_from_memory()));
    bench.bench_function("reader", |bench| bench.iter(|| reader()));
    bench.bench_function("open", |bench| bench.iter(|| open()));

    let image = image::load_from_memory(data::RAW_DATA).unwrap();
    bench.bench_function("compare rgb8 hybrid", |bench| bench.iter(|| compare_rgb8(&image, &image)));
    bench.bench_function("compare luma8 RootMeanSquared", |bench| bench.iter(|| compare_luma8_rootmeansquared(&image, &image)));
    bench.bench_function("compare luma8 MSSIMSimple", |bench| bench.iter(|| compare_luma8_mssimsimple(&image, &image)));
}
