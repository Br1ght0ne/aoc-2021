use std::io::BufReader;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use aoc_2021::day1::{part2, part2_updated, process_input};

const INPUT: &str = include_str!("../../inputs/1.txt");

fn bench_part2(c: &mut Criterion) {
    let mut group = c.benchmark_group("day1/part2");
    let input = process_input(BufReader::new(INPUT.as_bytes())).collect::<Vec<_>>();
    group.bench_with_input(BenchmarkId::new("original", "1.txt"), &input, |b, i| {
        b.iter(|| part2(black_box(i.iter().cloned())))
    });
    group.bench_with_input(BenchmarkId::new("updated", "1.txt"), &input, |b, i| {
        b.iter(|| part2_updated(black_box(i.iter().cloned())))
    });
    group.finish();
}

criterion_group!(bench_day1, bench_part2);
criterion_main!(bench_day1);
