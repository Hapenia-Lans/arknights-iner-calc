use iner_calc::*;

fn main() {
    // let simulation = Simulation {
    //     available_techniques: vec![
    //         // 萃雕
    //         Technique::ExtractI,
    //         Technique::ExtractII,
    //         Technique::ExtractIII,
    //         Technique::ExtractIV,
    //         // 滤纯
    //         Technique::FilterI,
    //         Technique::FilterII,
    //         Technique::FilterIII,
    //         // 交糅
    //         Technique::MingleI,
    //         Technique::MingleII,
    //         Technique::MingleIII,
    //         // 落晶
    //         Technique::CrystalI,
    //         Technique::CrystalII,
    //         Technique::CrystalIII,
    //     ],
    //     board_size: 2,
    //     initial_nerse: 19,
    //     initial_pet: 32,
    //     initial_gabe: 20,
    //     initial_shay: 29,
    // };
    // let game = simulation.find_best_v1();
    // println!("{:?}", game);

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
        initial_nerse: 31,
        initial_pet: 29,
        initial_gabe: 22,
        initial_shay: 18,
    };
    let game = simulation.find_best_v2();
    let (s, _) = game.clone().run();
    let Game {board, technique, ..} = game;
    println!("Score: {}", s);
    println!("Board: {:?}", board);
    println!("Technique: {:?}", technique);
    // println!("Results: {:#?}", d);
}
