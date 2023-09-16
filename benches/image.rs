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

fn load_from_memory() {
    assert!(image::load_from_memory(data::RAW_DATA).is_ok());
}

fn load_from_memory_with_format() {
    assert!(image::load_from_memory_with_format(data::RAW_DATA, image::ImageFormat::Jpeg).is_ok());
}

fn hybrid(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::rgb_hybrid_compare(
        &a.to_rgb8(),
        &b.to_rgb8()
    );

    assert!(compare.is_ok());
}

fn rmse(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::gray_similarity_structure(
        &image_compare::Algorithm::RootMeanSquared,
        &a.to_luma8(),
        &b.to_luma8()
    );

    assert!(compare.is_ok());
}

fn mssims(a: &image::DynamicImage, b: &image::DynamicImage) {
    let compare = image_compare::gray_similarity_structure(
        &image_compare::Algorithm::MSSIMSimple,
        &a.to_luma8(),
        &b.to_luma8()
    );

    assert!(compare.is_ok());
}

pub fn criterion_benchmark_perf(bench: &mut Criterion<Perf>) {
    let image = image::load_from_memory(data::RAW_DATA).unwrap();
    bench.bench_function("image-compare load", |bench| bench.iter(|| load_from_memory()));
    bench.bench_function("image-compare load_format", |bench| bench.iter(|| load_from_memory_with_format()));
    bench.bench_function("image-compare HYBRID", |bench| bench.iter(|| hybrid(&image, &image)));
    bench.bench_function("image-compare RMSE", |bench| bench.iter(|| rmse(&image, &image)));
    bench.bench_function("image-compare MSSIM", |bench| bench.iter(|| mssims(&image, &image)));
}

#[cfg(target_arch = "x86_64")]
pub fn criterion_benchmark_time(bench: &mut Criterion<WallTime>) {
    let image = image::load_from_memory(data::RAW_DATA).unwrap();
    bench.bench_function("image-compare load", |bench| bench.iter(|| load_from_memory()));
    bench.bench_function("image-compare load_format", |bench| bench.iter(|| load_from_memory_with_format()));
    bench.bench_function("image-compare HYBRID", |bench| bench.iter(|| hybrid(&image, &image)));
    bench.bench_function("image-compare RMSE", |bench| bench.iter(|| rmse(&image, &image)));
    bench.bench_function("image-compare MSSIM", |bench| bench.iter(|| mssims(&image, &image)));
}
