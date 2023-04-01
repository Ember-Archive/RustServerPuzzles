use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use kolakoskilib;

pub fn criterion_benchmark(c: &mut Criterion) {
    let inputs = vec![10, 100, 1000, 10000, 1000000];

    c.bench_with_input(BenchmarkId::new("test", "my cool test"), &inputs, |b, i| {
        b.iter(|| kolakoskilib::calc_kolakoski(i.to_vec()));
    });
}

fn main() {
    let mut criterion = Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(3))
        .measurement_time(std::time::Duration::from_secs(11))
        .nresamples(20000)
        .significance_level(0.01)
        .noise_threshold(0.02)
        .sample_size(100);

    criterion_benchmark(&mut criterion);
}