use crate::d7::{d7p1_v2, d7p2_v1};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d7;

// d7p1_v1 n'est pas benchmarké : sur les vraies données la version sans cache
// part en explosion combinatoire, on l'a remplacée par v2 avec un HashSet.
pub fn d7p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d7p1");
    group.bench_function("d7p1_v2", |b| {
        b.iter(|| d7p1_v2(black_box(include_str!("d7/d7.txt"))))
    });
}

pub fn d7p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d7p2");
    group.bench_function("d7p2_v1", |b| {
        b.iter(|| d7p2_v1(black_box(include_str!("d7/d7.txt"))))
    });
}

criterion_group!(benches, d7p1, d7p2);
criterion_main!(benches);
