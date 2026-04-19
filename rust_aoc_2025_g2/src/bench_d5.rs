use crate::d5::{d5p1_v2, d5p2_v1};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod d5;

// d5p1_v1 matérialise les ids si l'union des plages reste raisonnable ; sinon elle
// délègue à v2. sur le puzzle complet elle suit le chemin v2, donc on ne bench que v2 ici.
pub fn d5p1(c: &mut Criterion) {
    let mut group = c.benchmark_group("d5p1");
    group.bench_function("d5p1_v2", |b| {
        b.iter(|| d5p1_v2(black_box(include_str!("d5/d5.txt"))));
    });
}

pub fn d5p2(c: &mut Criterion) {
    let mut group = c.benchmark_group("d5p2");
    group.bench_function("d5p2_v1", |b| {
        b.iter(|| d5p2_v1(black_box(include_str!("d5/d5.txt"))));
    });
}

criterion_group!(benches, d5p1, d5p2);
criterion_main!(benches);
