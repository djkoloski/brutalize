use anima::State;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn solve_free_radical(c: &mut Criterion) {
    const FREE_RADICAL: &str =
        " ....\n..r..\n.r.r.\n..r..\n.... \n\nR 1 3\nR 1 1\nB 2 2\nR 3 1\nR 3 3";

    let (initial_state, data) = <State as brutalize_cli::State>::parse(FREE_RADICAL).unwrap();

    c.bench_function("solve_free_radical", |b| {
        b.iter(|| brutalize::solve(black_box(&initial_state).clone(), &data))
    });

    const FRACTAL: &str =
        "  r  \n ... \n. b .\n.....\n. r .\n ... \n  b  \n\nR 1 3\nR 3 3\nB 2 2\nB 2 4";

    let (initial_state, data) = <State as brutalize_cli::State>::parse(FRACTAL).unwrap();

    c.bench_function("solve_fractal", |b| {
        b.iter(|| brutalize::solve(black_box(&initial_state).clone(), &data))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    solve_free_radical(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
