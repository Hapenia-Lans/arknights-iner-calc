use std::collections::btree_map::Range;

use iner_calc::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// 解锁的操作台数量
    board_size: usize,
    /// 待加工火焰伊纳数量
    nesre: usize,
    /// 待加工天空伊纳数量
    pet: usize,
    /// 待加工草伊纳数量
    gabe: usize,
    /// 待加工沙伊纳数量
    shay: usize,
}

// fn main() {
//     let cli = Cli::parse();

//     let simulation = Simulation {
//         available_techniques: vec![
//             Technique::ExtractIV,
//             // 滤纯
//             Technique::FilterI,
//             Technique::FilterII,
//             Technique::FilterIII,
//             // 交糅
//             Technique::MingleI,
//             Technique::MingleII,
//             Technique::MingleIII,
//             // 落晶
//             Technique::CrystalI,
//             Technique::CrystalII,
//             Technique::CrystalIII,
//         ],
//         board_size: cli.board_size,
//         initial_nesre: cli.nesre,
//         initial_pet: cli.pet,
//         initial_gabe: cli.gabe,
//         initial_shay: cli.shay,
//     };
//     let game = simulation.find_best_v2();
//     let (s, _) = game.clone().run();
//     let Game {board, technique, ..} = game;
//     println!("======== CALCULATION FINISHED ========");
//     println!("Score: {}", s);
//     println!("Board: {:?}", board);
//     println!("Technique: {:?}", technique);
//     // println!("Results: {:#?}", d);
// }

#[derive(Default)]
struct Result {
    nesre_init: usize,
    pet_init: usize,
    gabe_init: usize,
    shay_init: usize,
    score: usize,
    technique: Vec<Technique>,
    board: [Option<Technique>; 6],
}

fn main() {
    let range = 10..40;
    let mut count = 0;
    let mut results: Vec<Result> = vec![];
    let total = 17892;
    let start_time = std::time::Instant::now();

    results.resize_with(1_000_000, Default::default);

    for nesre in range {
        for pet in 10..usize::min(100 - nesre, 40) {
            for gabe in 10..usize::min(100 - nesre - pet, 40) {
                let shay = 100 - nesre - pet - gabe;
                if !(10..40).contains(&shay) {
                    continue;
                }
                let now = std::time::Instant::now();
                println!("BATCH {}/{}, elapsed = {}s", count, total, (now - start_time).as_secs_f32());
                let simulation = Simulation {
                    available_techniques: vec![
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
                    initial_nesre: nesre,
                    initial_pet: pet,
                    initial_gabe: gabe,
                    initial_shay: shay,
                };
                let game = simulation.find_best_v2_scilenced();
                let (s, _) = game.clone().run();
                results.push(Result {
                    nesre_init: nesre,
                    pet_init: pet,
                    gabe_init: gabe,
                    shay_init: shay,
                    score: s,
                    technique: game.technique,
                    board: game.board,
                });
                count += 1
            }
        }
    }
}

fn some_shit(start: u16, end: u16) {
    for i in start..end {
        for j in start..end {
            for k in start..end {
                let l = 65535 - i - j - k;
                if l >= end || l < start { continue }
                println!("{}, {}, {}, {}", i, j, k, l);
            }
        }
    }
}