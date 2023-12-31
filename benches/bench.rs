use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! benchmarks {
    (
        bench_day: $day:ident
    ) => {
        // Inputs need to be in this format to work with `cargo aoc input`.
        const DATA: &str = include_str!(concat!(
            "../input/",
            stringify!($day),
            ".txt"
        ));

        fn benchmark_function(c: &mut Criterion) {
            use aoc2022::$day;

            let input = $day::generator(&DATA);
            c.bench_function(concat!(stringify!($day), " gen"), |b| {
                b.iter(|| $day::generator(&DATA))
            });
            c.bench_function(concat!(stringify!($day), " part 1"), |b| {
                b.iter(|| $day::part1(&input))
            });
            c.bench_function(concat!(stringify!($day), " part 2"), |b| {
                b.iter(|| $day::part2(&input))
            });
        }
    };
}

benchmarks! {
    bench_day: day1
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
