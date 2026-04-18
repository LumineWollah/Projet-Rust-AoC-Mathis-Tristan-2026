use crate::d9::{d9p1_v1, d9p1_v2, d9p1_v3, d9p2_v3};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d9;

// d9p2_v1 et d9p2_v2 ne sont pas benchmarkés : sur les vraies données ils sont
// trop lents (boucle imbriquée sur tout l'intérieur du rectangle pour chaque
// paire de points). on garde le code dans le module pour l'évolution.
pub fn d9p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d9p1");
    group.bench_function("d9p1_v1", |b| {
        b.iter(|| d9p1_v1(black_box(include_str!("d9/d9.txt"))))
    });
    group.bench_function("d9p1_v2", |b| {
        b.iter(|| d9p1_v2(black_box(include_str!("d9/d9.txt"))))
    });
    group.bench_function("d9p1_v3", |b| {
        b.iter(|| d9p1_v3(black_box(include_str!("d9/d9.txt"))))
    });
}

pub fn d9p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d9p2");
    group.bench_function("d9p2_v3", |b| {
        b.iter(|| d9p2_v3(black_box(include_str!("d9/d9.txt"))))
    });
}

criterion_group!(benches, d9p1, d9p2);
criterion_main!(benches);
