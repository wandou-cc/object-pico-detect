
#[path = "./common/macros.rs"]
mod macros;

use std::fmt::Display;

use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use imageproc::rect::Rect;
use pico_detect::multiscale::Multiscaler;

#[derive(Clone, Copy, Debug)]
struct Size {
    width: u32,
    height: u32,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

pub fn bench_run(c: &mut Criterion) {
    static SIZES: &[(u32, u32)] = &[
        (320, 240),
        (480, 360),
        (640, 480),
        (1280, 720),
        (1920, 1280),
    ];

    let mut group = c.benchmark_group("Multiscaler::run");

    group.sample_size(10000);

    for size in SIZES.iter().map(|s| Size::from(*s)) {
        let ms = Multiscaler::new(100, size.height, 0.1, 1.1).unwrap();

        let id = BenchmarkId::from_parameter(size);

        group.throughput(Throughput::Elements(
            ms.count(Rect::at(0, 0).of_size(size.width, size.height)) as u64,
        ));

        group.bench_with_input(id, &size, |b, &s| {
            b.iter(|| {
                ms.run(Rect::at(0, 0).of_size(s.width, s.height), |s| {
                    black_box(s);
                })
            })
        });
    }

    group.finish();
}