use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use matrix_profile::naive::distance_profile;

fn bench_with_history<const HISTORY_LEN: usize>(c: &mut Criterion, normalize: bool) {
    let history = Vec::from_iter((0..HISTORY_LEN).map(|v| v as f32));

    let mut group = c.benchmark_group(format!(
        "distance_profile_{}k_norm_{normalize}",
        HISTORY_LEN / 1000
    ));
    for window_size in [16, 32, 64, 128, 256, 512, 1024].iter() {
        let window = &history[history.len() - window_size..];

        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::from_parameter(window_size),
            window_size,
            |b, _| {
                b.iter(|| {
                    let dist = distance_profile(&history, window, 1, normalize);
                    let _ = black_box(dist);
                });
            },
        );
    }
    group.finish()
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_with_history::<100_000>(c, false);
    bench_with_history::<250_000>(c, false);
    bench_with_history::<500_000>(c, false);

    bench_with_history::<100_000>(c, true);
    bench_with_history::<250_000>(c, true);
    bench_with_history::<500_000>(c, true);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
