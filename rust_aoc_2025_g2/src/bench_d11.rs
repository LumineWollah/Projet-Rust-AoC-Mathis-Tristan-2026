use crate::d11::{d11p1_v1, d11p2_v1, d11p2_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d11;

pub fn d11p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d11p1");
    group.bench_function("d11p1_v1", |b| {
        b.iter(|| d11p1_v1(black_box(include_str!("d11/d11.txt"))))
    });
}

pub fn d11p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d11p2");
    // d11p2_v1 (sans cache) est très lent sur les vraies données mais on le
    // benchmarke quand même pour montrer la différence avec v2 (avec cache).
    group.bench_function("d11p2_v1", |b| {
        b.iter(|| d11p2_v1(black_box(include_str!("d11/d11.txt"))))
    });
    group.bench_function("d11p2_v2", |b| {
        b.iter(|| d11p2_v2(black_box(include_str!("d11/d11.txt"))))
    });
}

criterion_group!(benches, d11p1, d11p2);
criterion_main!(benches);
