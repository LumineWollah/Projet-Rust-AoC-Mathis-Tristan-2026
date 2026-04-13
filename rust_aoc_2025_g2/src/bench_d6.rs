use crate::d6::{d6p1_v1,d6p1_v2,d6p2_v1,d6p2_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d6;

pub fn d6p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d6p1");
    group.bench_function("d6p1_v1", |b| {
        b.iter(|| d6p1_v1(black_box(include_str!("d6/d6.txt"))))
    });
    group.bench_function("d6p1_v2", |b| {
         b.iter(|| d6p1_v2(black_box(include_str!("d6/d6.txt"))))
     });
}

pub fn d6p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d6p2");
    group.bench_function("d6p2_v1", |b| {
        b.iter(|| d6p2_v1(black_box(include_str!("d6/d6.txt"))))
    });
    group.bench_function("d6p2_v2", |b| {
         b.iter(|| d6p2_v2(black_box(include_str!("d6/d6.txt"))))
    });

}

criterion_group!(benches, d6p1, d6p2);
criterion_main!(benches);