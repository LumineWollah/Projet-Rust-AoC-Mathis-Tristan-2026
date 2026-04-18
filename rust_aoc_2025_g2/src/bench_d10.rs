use crate::d10::{d10p1_v1,d10p1_v2,d10p2_v1,d10p2_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d10;

pub fn d10p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d10p1");
    group.bench_function("d10p1_v1", |b| {
        b.iter(|| d10p1_v1(black_box(include_str!("d10/d10.txt"))))
    });
    group.bench_function("d10p1_v2", |b| {
         b.iter(|| d10p1_v2(black_box(include_str!("d10/d10.txt"))))
     });
}

pub fn d10p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d10p2");
    group.bench_function("d10p2_v1", |b| {
        b.iter(|| d10p2_v1(black_box(include_str!("d10/d10.txt"))))
    });
    group.bench_function("d10p2_v2", |b| {
         b.iter(|| d10p2_v2(black_box(include_str!("d10/d10.txt"))))
    });

}

criterion_group!(benches, d10p1, d10p2);
criterion_main!(benches);