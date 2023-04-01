use criterion::{BenchmarkId, Criterion};
use kolakoskilib;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = 100000000;

    c.bench_with_input(BenchmarkId::new("test", "my cool test"), &input, |b, i| {
        b.iter(|| kolakoskilib::kolakoski_ratio_nilsson(*i));
    });
}

fn main() {
    let mut criterion = Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(3))
        .measurement_time(std::time::Duration::from_secs(80))
        .nresamples(20000)
        .significance_level(0.01)
        .noise_threshold(0.02)
        .sample_size(100);

    criterion_benchmark(&mut criterion);
}