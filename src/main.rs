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

fn main() {
    let cli = Cli::parse();

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
        board_size: cli.board_size,
        initial_nesre: cli.nesre,
        initial_pet: cli.pet,
        initial_gabe: cli.gabe,
        initial_shay: cli.shay,
    };
    let game = simulation.find_best_v2();
    let (s, _) = game.clone().run();
    let Game {board, technique, ..} = game;
    println!("======== CALCULATION FINISHED ========");
    println!("Score: {}", s);
    println!("Board: {:?}", board);
    println!("Technique: {:?}", technique);
    // println!("Results: {:#?}", d);
}
