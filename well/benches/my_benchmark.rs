use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use welllib;

pub fn criterion_benchmark(c: &mut Criterion) {
    let inputs: Vec<&str> = vec![ // too lazy to do user input lmao
        "3 3 1 9 6 2 8 5 3 7 4 4",
        "3 3 8 3 2 7 1 5 4 9 6 4",
        "7 7 38 33 11 48 19 45 22 47 30 24 15 46 28 3 14 13 2 34 8 21 17 10 9 5 16 27 36 39 18 32 20 1 35 49 12 43 29 4 41 26 31 37 25 6 23 44 7 42 40 35",
        "7 7 15 16 46 1 38 43 44 25 10 7 6 34 42 14 8 19 9 21 13 23 22 32 11 29 36 3 5 47 31 33 45 24 12 18 28 40 41 20 26 39 48 2 49 35 27 4 37 30 17 26",
    ];

    c.bench_with_input(BenchmarkId::new("test", "my cool test"), &inputs, |b, i| {
        b.iter(|| welllib::calculate_both(i.to_vec()));
    });
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);