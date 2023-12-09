#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_profile::{squared_euclidean_distance_simd, squared_euclidean_distance_slice};
use simd_euclidean::Vectorized;
use std::simd::f32x16;

fn criterion_benchmark(c: &mut Criterion) {
    let a = f32x16::splat(2.0);
    let b = f32x16::splat(3.0);

    c.bench_function("squared_euclidean_distance_16", |bencher| {
        bencher.iter(|| {
            let dist = squared_euclidean_distance_simd(black_box(a), black_box(b));
            let _ = black_box(dist);
        });
    });
    let a = vec![2.0; 16];
    let b = vec![3.0; 16];
    c.bench_function("simd-euclidean-distance_16", |bencher| {
        bencher.iter(|| {
            let dist = Vectorized::distance(black_box(&a), black_box(&b));
            let _ = black_box(dist);
        });
    });

    let a = vec![2.0; 16];
    let b = vec![3.0; 16];
    c.bench_function("squared_euclidean_distance_slice", |bencher| {
        bencher.iter(|| {
            let dist = squared_euclidean_distance_slice(black_box(&a), black_box(&b));
            let _ = black_box(dist);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
