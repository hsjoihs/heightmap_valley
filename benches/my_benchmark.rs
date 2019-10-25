extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};
use heightmap_valley::valley::convert;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test f", |b| {
        b.iter(|| heightmap_valley::valley::convert("f.png", "output_f.png"))
    });
    c.bench_function("test g", |b| {
        b.iter(|| heightmap_valley::valley::convert("g.png", "output_g.png"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
