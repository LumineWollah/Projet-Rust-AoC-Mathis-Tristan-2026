use crate::d3::{d3p1_v1, d3p1_v2, d3p2_v1, d3p2_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d3;

pub fn d3p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d3p1");
    group.bench_function("d3p1_v1", |b| {
        b.iter(|| d3p1_v1(black_box(include_str!("d3/d3.txt"))));
    });
    group.bench_function("d3p1_v2", |b| {
        b.iter(|| d3p1_v2(black_box(include_str!("d3/d3.txt"))));
    });
}

pub fn d3p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d3p2");
    group.bench_function("d3p2_v1", |b| {
        b.iter(|| d3p2_v1(black_box(include_str!("d3/d3.txt"))));
    });
    group.bench_function("d3p2_v2", |b| {
        b.iter(|| d3p2_v2(black_box(include_str!("d3/d3.txt"))));
    });
}

criterion_group!(benches, d3p1, d3p2);
criterion_main!(benches);
