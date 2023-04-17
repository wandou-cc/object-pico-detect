#[path = "./common/macros.rs"]
mod macros;

use std::time::Duration;

use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use rand::prelude::*;

use pico_detect::{clusterize::Clusterizer, perturbate::Perturbator, Detection, Square};
use rand_xoshiro::Xoroshiro128PlusPlus;

pub fn bench_clusterize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Clusterizer::clusterize");

    group.warm_up_time(Duration::from_