use crate::d1::{d1p1_v1,d1p1_v2,d1p2_v1,d1p2_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d1;

pub fn d1p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d1p1");
    group.bench_function("d1p1_v1", |b| {
        b.iter(|| d1p1_v1(black_box(include_str!("d1/d1.txt"))))
    });
    group.bench_function("d1p1_v2", |b| {
         b.iter(|| d1p1_v2(black_box(include_str!("d1/d1.txt"))))
     });
}

pub fn d1p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d1p2");
    group.bench_function("d1p2_v1", |b| {
        b.iter(|| d1p2_v1(black_box(include_str!("d1/d1.txt"))))
    });
    group.bench_function("d1p2_v2", |b| {
         b.iter(|| d1p2_v2(black_box(include_str!("d1/d1.txt"))))
    });

}

criterion_group!(benches, d1p1, d1p2);
criterion_main!(benches);