#[path = "./common/macros.rs"]
mod macros;

use std::fs;

use criterion::{black_box, Criterion};

use image;
use pico_detect::{Shaper, Square};

pub fn bench_load(c: &mut Criterion) {
    let model_data = fs::read(model_path!(shaper)).unwrap();

    c.bench_function("Shaper::load", |b| {
        b.iter(|| Shaper::load(black_box(model_data.as_slice())).unwrap())
    });
}

pub fn bench_inference(c: &mut Crit