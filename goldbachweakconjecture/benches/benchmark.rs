use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use goldbachweaklib;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = 111;

    c.bench_with_input(BenchmarkId::new("test", "my cool test"), &input, |b, i| {
        b.iter(|| goldbachweaklib::find_primes(*i as i64, 10));
    });
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);