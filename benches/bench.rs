use criterion::{criterion_group, criterion_main, Criterion};
use day6_aoc_tester::{old, new, newer, amos::marker_pos};

static INPUT: &str = include_str!("input.txt");

fn bench_4 (c: &mut Criterion) {
    // c.bench_function("old 4", |b| b.iter(|| old::<4>(INPUT)));
    c.bench_function("new 4", |b| b.iter(|| new::<4>(INPUT)));
    c.bench_function("newer 4", |b| b.iter(|| newer::<4>(INPUT)));
    // c.bench_function("amos 4", |b| b.iter(|| marker_pos::<3>(INPUT)));
}
fn bench_14 (c: &mut Criterion) {
    // c.bench_function("old 14", |b| b.iter(|| old::<14>(INPUT)));
    c.bench_function("new 14", |b| b.iter(|| new::<14>(INPUT)));
    c.bench_function("newer 14", |b| b.iter(|| newer::<14>(INPUT)));
    // c.bench_function("amos 14", |b| b.iter(|| marker_pos::<13>(INPUT)));
}

criterion_group!(benches, bench_4, bench_14);
criterion_main!(benches);