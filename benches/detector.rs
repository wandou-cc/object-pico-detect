#[path = "./common/macros.rs"]
mod macros;

use std::fs;

use criterion::{black_box, Criterion};

use image;
use pico_detect::{Detector, Square};

pub fn bench_load(c: &mut Criterion) {
    let model_data = fs::read(model_path!(facefinder)).unwrap();

    c.bench_function("Detector::load", |b| {
        b.iter(|| Detector::load(black_box(m