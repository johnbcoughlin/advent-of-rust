use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_rust::day15;

fn day15_benchmark(c: &mut Criterion) {
    c.bench_function("Day 15 Part 2", |b| {
        let input = include_str!("../resources/day15_input.txt");
        let cave = day15::parse_cave(input);
        let part2_cave = day15::part2(&cave);

        b.iter(|| {
            day15::best_path_through(black_box(&part2_cave));
        });
    });
}

criterion_group!(benches, day15_benchmark);
criterion_main!(benches);
