use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2021::{days::day1, shared::read_file};

pub fn day1_part1(c: &mut Criterion) {
    let contents = read_file::<usize>("src/input/day_1.txt", '\n').unwrap();
    c.bench_function("day 1 part 1", |b| {
        b.iter(|| day1::part_1(black_box(&contents)))
    });
}

pub fn day1_part2(c: &mut Criterion) {
    let contents = read_file::<usize>("src/input/day_1.txt", '\n').unwrap();
    c.bench_function("day 1 part 2", |b| {
        b.iter(|| day1::part_2(black_box(&contents)))
    });
}

criterion_group!(benches, day1_part1, day1_part2);
criterion_main!(benches);
