extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};
use criterion::measurement::WallTime;
#[cfg(target_arch = "x86_64")]
use criterion_perf_events::Perf;
#[cfg(target_arch = "x86_64")]
use perfcnt::linux::HardwareEventType as Hardware;
#[cfg(target_arch = "x86_64")]
use perfcnt::linux::PerfCounterBuilderLinux as Builder;
#[cfg(target_arch = "x86_64")]
use pprof::criterion::{PProfProfiler, Output};

use std::time::Duration;

mod data;

criterion_group! {
    name = benches_time;
    config = Criterion::default().sample_size(100)
                       .measurement_time(Duration::from_secs(10))
                       .warm_up_time(Duration::from_secs(3));
    targets = criterion_benchmark_time
}

#[cfg(target_arch = "x86_64")]
criterion_group! {
    name = benches_perf;
    config = Criterion::default().sample_size(100)
                       .with_measurement(Perf::new(Builder::from_hardware_event(Hardware::Instructions)))
                       .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark_perf
}

#[cfg(target_arch = "x86_64")]
criterion_main!(benches_perf, benches_time);
#[cfg(not(target_arch = "x86_64"))]
criterion_main!(benches_time);

#[cfg(feature = "nomagick")]
fn nomagick_load_from_memory() {
    assert!(image::load_from_memory(data::RAW_DATA).is_ok());
}

#[cfg(feature = "nomagick")]
fn nomagick_load_from_memory_with_format() {
    assert!(image::load_from_memory_with_format(data::RAW_DATA, image::ImageFormat::Jpeg).is_ok());
}

#[cfg(feature = "nomagick")]
fn nomagick_hybrid(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::rgb_hybrid_compare(
        &a.to_rgb8(),
        &b.to_rgb8()
    );

    assert!(compare.is_ok());
}

#[cfg(feature = "nomagick")]
fn nomagick_rmse(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::gray_similarity_structure(
        &image_compare::Algorithm::RootMeanSquared,
        &a.to_luma8(),
        &b.to_luma8()
    );

    assert!(compare.is_ok());
}

#[cfg(feature = "nomagick")]
fn nomagick_mssims(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::gray_similarity_structure(
        &image_compare::Algorithm::MSSIMSimple,
        &a.to_luma8(),
        &b.to_luma8()
    );

    assert!(compare.is_ok());
}

#[cfg(feature = "magick")]
fn read_image_blob() {
    let wand = magick_rust::MagickWand::new();

    wand.read_image_blob(data::RAW_DATA).unwrap();
    wand.fit(100, 100);
}

#[cfg(feature = "magick")]
fn magick_cmp(a: &magick_rust::MagickWand, b: &magick_rust::MagickWand, metric: magick_rust::bindings::MetricType) {
    let (_, _) = a.compare_images(&b, metric);
}

pub fn criterion_benchmark_perf(bench: &mut Criterion<Perf>) {
    let image = image::load_from_memory(data::RAW_DATA).unwrap();
    bench.bench_function("image-compare load", |bench| bench.iter(|| nomagick_load_from_memory()));
    bench.bench_function("image-compare load_format", |bench| bench.iter(|| nomagick_load_from_memory_with_format()));
    bench.bench_function("image-compare HYBRID", |bench| bench.iter(|| nomagick_hybrid(&image, &image)));
    bench.bench_function("image-compare RMSE", |bench| bench.iter(|| nomagick_rmse(&image, &image)));
    bench.bench_function("image-compare MSSIM", |bench| bench.iter(|| nomagick_mssims(&image, &image)));

    let image = magick_rust::MagickWand::new();

    image.read_image_blob(data::RAW_DATA).unwrap();
    image.fit(100, 100);

    bench.bench_function("magick load", |bench| bench.iter(|| read_image_blob()));
    bench.bench_function("magick AE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_AbsoluteErrorMetric)));
    bench.bench_function("magick FE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_FuzzErrorMetric)));
    bench.bench_function("magick AE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_MeanAbsoluteErrorMetric)));
    bench.bench_function("magick EPPE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_MeanErrorPerPixelErrorMetric)));
    bench.bench_function("magick SE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_MeanSquaredErrorMetric)));
    bench.bench_function("magick CCE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_NormalizedCrossCorrelationErrorMetric)));
    bench.bench_function("magick PAE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_PeakAbsoluteErrorMetric)));
    bench.bench_function("magick PtNR", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_PeakSignalToNoiseRatioErrorMetric)));
    bench.bench_function("magick PHE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_PerceptualHashErrorMetric)));
    bench.bench_function("magick RMSE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_RootMeanSquaredErrorMetric)));
    bench.bench_function("magick SS", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_StructuralSimilarityErrorMetric)));
    bench.bench_function("magick SD", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_StructuralDissimilarityErrorMetric)));
}

#[cfg(target_arch = "x86_64")]
pub fn criterion_benchmark_time(bench: &mut Criterion<WallTime>) {
    let image = image::load_from_memory(data::RAW_DATA).unwrap();
    bench.bench_function("image-compare load", |bench| bench.iter(|| nomagick_load_from_memory()));
    bench.bench_function("image-compare load_format", |bench| bench.iter(|| nomagick_load_from_memory_with_format()));
    bench.bench_function("image-compare HYBRID", |bench| bench.iter(|| nomagick_hybrid(&image, &image)));
    bench.bench_function("image-compare RMSE", |bench| bench.iter(|| nomagick_rmse(&image, &image)));
    bench.bench_function("image-compare MSSIM", |bench| bench.iter(|| nomagick_mssims(&image, &image)));

    let image = magick_rust::MagickWand::new();

    image.read_image_blob(data::RAW_DATA).unwrap();
    image.fit(100, 100);

    bench.bench_function("magick load", |bench| bench.iter(|| read_image_blob()));
    bench.bench_function("magick AE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_AbsoluteErrorMetric)));
    bench.bench_function("magick FE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_FuzzErrorMetric)));
    bench.bench_function("magick AE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_MeanAbsoluteErrorMetric)));
    bench.bench_function("magick EPPE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_MeanErrorPerPixelErrorMetric)));
    bench.bench_function("magick SE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_MeanSquaredErrorMetric)));
    bench.bench_function("magick CCE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_NormalizedCrossCorrelationErrorMetric)));
    bench.bench_function("magick PAE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_PeakAbsoluteErrorMetric)));
    bench.bench_function("magick PtNR", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_PeakSignalToNoiseRatioErrorMetric)));
    bench.bench_function("magick PHE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_PerceptualHashErrorMetric)));
    bench.bench_function("magick RMSE", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_RootMeanSquaredErrorMetric)));
    bench.bench_function("magick SS", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_StructuralSimilarityErrorMetric)));
    bench.bench_function("magick SD", |bench| bench.iter(|| magick_cmp(&image, &image, magick_rust::bindings::MetricType_StructuralDissimilarityErrorMetric)));
}
