#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iner_calc::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("calc", |b| {
        b.iter(|| {
            let simulation = Simulation {
                available_techniques: vec![
                    // 萃雕
                    // Technique::ExtractI,
                    // Technique::ExtractII,
                    // Technique::ExtractIII,
                    Technique::ExtractIV,
                    // 滤纯
                    Technique::FilterI,
                    Technique::FilterII,
                    Technique::FilterIII,
                    // 交糅
                    Technique::MingleI,
                    Technique::MingleII,
                    Technique::MingleIII,
                    // 落晶
                    Technique::CrystalI,
                    Technique::CrystalII,
                    Technique::CrystalIII,
                ],
                board_size: 6,
                initial_nerse: 19,
                initial_pet: 32,
                initial_gabe: 20,
                initial_shay: 29,
            };
            let game = simulation.find_best_v2();
            let (s, d) = game.clone().run();
        })
    });
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
