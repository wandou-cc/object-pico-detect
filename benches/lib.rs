use std::time::Duration;

#[macro_use]
extern crate criterion;

use criterion::Criterion;

mod clusterizer;
mod detector;
mod localizer;
mod multiscaler;
mod perturbator;
mod shaper;

criterion_group!(
    name = loading;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(15))
        .sample_size(25)
        .noise_threshold(0.05)
        .measurement_time(Duration::from_secs(10));
    targets =
        detector::bench_load,
        localizer::bench_load,
        shaper::bench_load,
);

criterion_group!(
    name = detection;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(5))
        .sample_size(100)
        .measurement_time(Duration::from_secs(20));
    targets =
        d