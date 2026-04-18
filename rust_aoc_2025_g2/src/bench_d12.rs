use crate::d12::{d12p1_v1,d12p1_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d12;

pub fn d12p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d12p1");
    group.bench_function("d12p1_v1", |b| {
        b.iter(|| d12p1_v1(black_box(include_str!("d12/d12.txt"))))
    })
    .sample_size(10);
    group.bench_function("d12p1_v2", |b| {
        b.iter(|| d12p1_v2(black_box(include_str!("d12/d12.txt"))))
    })
    .sample_size(10);
}

criterion_group!(benches, d12p1);
criterion_main!(benches);