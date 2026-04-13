use crate::d8::{d8p1_v1,d8p1_v2,d8p2_v1,d8p2_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d8;

pub fn d8p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d8p1");
    group.bench_function("d8p1_v1", |b| {
        b.iter(|| d8p1_v1(black_box(include_str!("d8/d8.txt")),1000))
    });
    group.bench_function("d8p1_v2", |b| {
         b.iter(|| d8p1_v2(black_box(include_str!("d8/d8.txt")),1000))
     });
}

pub fn d8p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d8p2");
    group.bench_function("d8p2_v1", |b| {
        b.iter(|| d8p2_v1(black_box(include_str!("d8/d8.txt"))))
    });
    group.bench_function("d8p2_v2", |b| {
         b.iter(|| d8p2_v2(black_box(include_str!("d8/d8.txt"))))
    });

}

criterion_group!(benches, d8p1, d8p2);
criterion_main!(benches);