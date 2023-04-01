use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use subfactoriallib;

pub fn criterion_benchmark(c: &mut Criterion) {
    let inputs = vec![6, 9, 14, 20];

    c.bench_with_input(BenchmarkId::new("test", "my cool test"), &inputs, |b, i| {
        b.iter(|| subfactoriallib::calc_subfactorials(i));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);