use crate::d2::{d2p1_v1,d2p1_v2,d2p2_v1,d2p2_v2};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d2;

pub fn d2p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d2p1");
    group.bench_function("d2p1_v1", |b| {
        b.iter(|| d2p1_v1(black_box(include_str!("d2/d2.txt"))))
    });
    group.bench_function("d2p1_v2", |b| {
         b.iter(|| d2p1_v2(black_box(include_str!("d2/d2.txt"))))
     });
}

pub fn d2p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d2p2");
    group.bench_function("d2p2_v1", |b| {
        b.iter(|| d2p2_v1(black_box(include_str!("d2/d2.txt"))))
    });
    group.bench_function("d2p2_v2", |b| {
         b.iter(|| d2p2_v2(black_box(include_str!("d2/d2.txt"))))
    });

}

criterion_group!(benches, d2p1, d2p2);
criterion_main!(benches);