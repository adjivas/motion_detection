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
fn load_from_memory() {
    assert!(image::load_from_memory(data::RAW_DATA).is_ok());
}

#[cfg(feature = "nomagick")]
fn compare_rgb8_hybrid(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::rgb_hybrid_compare(
        &a.to_rgb8(),
        &b.to_rgb8()
    );

    assert!(compare.is_ok());
}

#[cfg(feature = "nomagick")]
fn compare_luma8_rootmeansquared(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::gray_similarity_structure(
        &image_compare::Algorithm::RootMeanSquared,
        &a.to_luma8(),
        &b.to_luma8()
    );

    assert!(compare.is_ok());
}

#[cfg(feature = "nomagick")]
fn compare_luma8_mssimsimple(a: &image::DynamicImage, b: &image::DynamicImage) {
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

fn compare_wand(a: &magick_rust::MagickWand, b: &magick_rust::MagickWand) {
    let (_, _) = a.compare_images(&b, magick_rust::bindings::MetricType_RootMeanSquaredErrorMetric);
}

pub fn criterion_benchmark_perf(bench: &mut Criterion<Perf>) {
    let image = image::load_from_memory(data::RAW_DATA).unwrap();
    bench.bench_function("load_from_memory", |bench| bench.iter(|| load_from_memory()));
    bench.bench_function("compare rgb8 hybrid", |bench| bench.iter(|| compare_rgb8_hybrid(&image, &image)));
    bench.bench_function("compare luma8 RootMeanSquared", |bench| bench.iter(|| compare_luma8_rootmeansquared(&image, &image)));
    bench.bench_function("compare luma8 MSSIMSimple", |bench| bench.iter(|| compare_luma8_mssimsimple(&image, &image)));

    let image = magick_rust::MagickWand::new();

    image.read_image_blob(data::RAW_DATA).unwrap();
    image.fit(100, 100);

    bench.bench_function("read_image_blob", |bench| bench.iter(|| read_image_blob()));
    bench.bench_function("compare wand", |bench| bench.iter(|| compare_wand(&image, &image)));
}

#[cfg(target_arch = "x86_64")]
pub fn criterion_benchmark_time(bench: &mut Criterion<WallTime>) {
    let image = image::load_from_memory(data::RAW_DATA).unwrap();
    bench.bench_function("load_from_memory", |bench| bench.iter(|| load_from_memory()));
    bench.bench_function("compare rgb8 hybrid", |bench| bench.iter(|| compare_rgb8_hybrid(&image, &image)));
    bench.bench_function("compare luma8 RootMeanSquared", |bench| bench.iter(|| compare_luma8_rootmeansquared(&image, &image)));
    bench.bench_function("compare luma8 MSSIMSimple", |bench| bench.iter(|| compare_luma8_mssimsimple(&image, &image)));

    let image = magick_rust::MagickWand::new();

    image.read_image_blob(data::RAW_DATA).unwrap();
    image.fit(100, 100);

    bench.bench_function("read_image_blob", |bench| bench.iter(|| read_image_blob()));
    bench.bench_function("compare wand", |bench| bench.iter(|| compare_wand(&image, &image)));
}
