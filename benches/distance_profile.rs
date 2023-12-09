#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_profile::distance_profile;

fn criterion_benchmark(c: &mut Criterion) {
    // A year of minute candles.
    const HISTORY_LEN: usize = 525600;
    const WINDOW_SIZE: usize = 64;

    let history = Vec::from_iter((0..HISTORY_LEN).map(|v| v as f32));
    let window = &history[history.len() - WINDOW_SIZE..];
    c.bench_function("distance_profile", |bencher| {
        bencher.iter(|| {
            let profile = distance_profile(black_box(window), black_box(&history));
            let _ = black_box(profile);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
