use crate::d4::{d4p1_v1,d4p1_v2,d4p2_v1,d4p2_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d4;

pub fn d4p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d4p1");
    group.bench_function("d4p1_v1", |b| {
        b.iter(|| d4p1_v1(black_box(include_str!("d4/d4.txt"))))
    });
    group.bench_function("d4p1_v2", |b| {
         b.iter(|| d4p1_v2(black_box(include_str!("d4/d4.txt"))))
     });
}

pub fn d4p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d4p2");
    group.bench_function("d4p2_v1", |b| {
        b.iter(|| d4p2_v1(black_box(include_str!("d4/d4.txt"))))
    });
    group.bench_function("d4p2_v2", |b| {
         b.iter(|| d4p2_v2(black_box(include_str!("d4/d4.txt"))))
    });

}

criterion_group!(benches, d4p1, d4p2);
criterion_main!(benches);