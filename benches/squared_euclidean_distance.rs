#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_profile::squared_euclidean_distance;
use std::simd::f32x16;

fn criterion_benchmark(c: &mut Criterion) {
    let a = f32x16::splat(2.0);
    let b = f32x16::splat(3.0);

    c.bench_function("squared_euclidean_distance", |bencher| {
        bencher.iter(|| {
            let dist = squared_euclidean_distance(black_box(a), black_box(b));
            let _ = black_box(dist);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
