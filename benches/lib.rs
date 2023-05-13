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

criterion_group