use criterion::{black_box, criterion_group, criterion_main, Criterion};
use montyhalllib::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("monty halling", |b| b.iter(|| print_monty_sim(black_box(1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);