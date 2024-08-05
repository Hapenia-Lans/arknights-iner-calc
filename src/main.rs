use factorial::Factorial;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std::{cell::RefCell, collections::HashMap, vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Iner {
    /// 火焰伊纳
    Nerse,
    /// 火焰伊纳 I
    NerseI,
    /// 火焰伊纳 II
    NerseII,
    // 火焰伊纳 III
    NerseIII,
    /// 火焰伊纳 IV
    NerseIV,
    /// 天空伊纳
    Pet,
    /// 天空伊纳 I
    PetI,
    /// 天空伊纳 II
    PetII,
    /// 天空伊纳 III
    PetIII,
    /// 草叶伊纳
    Gabe,
    /// 草叶伊纳 I
    GabeI,
    /// 草叶伊纳 II
    GabeII,
    /// 草叶伊纳 III
    GabeIII,
    /// 沙伊纳
    Shay,
}

impl Iner {
    /// 伊纳基础分数
    const fn base_score(&self) -> usize {
        match self {
            Iner::Nerse => 1,
            Iner::NerseI => 2,
            Iner::NerseII => 10,
            Iner::NerseIII => 35,
            Iner::NerseIV => 85,
            Iner::Pet => 1,
            Iner::PetI => 3,
            Iner::PetII => 22,
            Iner::PetIII => 105,
            Iner::Gabe => 1,
            Iner::GabeI => 5,
            Iner::GabeII => 50,
            Iner::GabeIII => 500,
            Iner::Shay => 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Technique {
    // 萃雕
    ExtractI,
    ExtractII,
    ExtractIII,
    ExtractIV,
    // 滤纯
    FilterI,
    FilterII,
    FilterIII,
    // 交糅
    MingleI,
    MingleII,
    MingleIII,
    // 落晶
    CrystalI,
    CrystalII,
    CrystalIII,
}

/// 天空系列宝石额外加分
fn add_pet_iner_score(game: &mut GameData, score: usize) {
    *game.iner_score_mut(Iner::Pet) += score;
    *game.iner_score_mut(Iner::PetI) += score;
    *game.iner_score_mut(Iner::PetII) += score;
    *game.iner_score_mut(Iner::PetIII) += score;
}

impl Technique {
    /// 在操作台上的操作
    fn operate_on_board(&self, data: &mut GameData) {
        match self {
            // 将一份 <火焰伊纳> 刻印为 2 份<火焰伊纳 I>
            Technique::ExtractI => {
                let input = data.iner(Iner::Nerse);
                *data.iner_mut(Iner::Nerse) = 0.0;
                *data.iner_mut(Iner::NerseI) += input * 2.0;
            }
            // 将一份 <火焰伊纳 I> 刻印为 2 份<火焰伊纳 II>
            Technique::ExtractII => {
                let input = data.iner(Iner::NerseI);
                *data.iner_mut(Iner::NerseI) = 0.0;
                *data.iner_mut(Iner::NerseII) += input * 2.0;
            }
            // 将一份 <火焰伊纳 II> 刻印为 2.4 份<火焰伊纳 III>
            // 向上取整
            Technique::ExtractIII => {
                let input = data.iner(Iner::NerseII);
                *data.iner_mut(Iner::NerseII) = 0.0;
                *data.iner_mut(Iner::NerseIII) += input * 2.4;
            }
            // 将一份 <火焰伊纳 III> 刻印为 1 份<火焰伊纳 IV>
            // 每有一个空置操作台，获得5000额外评价分数
            Technique::ExtractIV => {
                let input = data.iner(Iner::NerseIII);
                *data.iner_mut(Iner::NerseIII) = 0.0;
                *data.iner_mut(Iner::NerseIV) += input;
                data.extra_score += 5000 * data.empty_boards;
            }
            // 将 <草叶伊纳> 全部刻印为 <草叶伊纳 I>
            // 额外产出 <沙伊纳>
            Technique::FilterI => {
                let input = data.iner(Iner::Gabe);
                *data.iner_mut(Iner::Gabe) = 0.0;
                *data.iner_mut(Iner::GabeI) += input;
                *data.iner_mut(Iner::Shay) += input;
            }
            // 将 <草叶伊纳 I> 按 4:1 刻印为 <草叶伊纳 II> 和 <沙伊纳>
            // 额外产出 <沙伊纳>
            Technique::FilterII => {
                let input = data.iner(Iner::GabeI);
                *data.iner_mut(Iner::GabeI) = 0.0;
                *data.iner_mut(Iner::GabeII) += input * 0.8;
                // = input + input * 0.2
                *data.iner_mut(Iner::Shay) += input * 1.2;
            }
            // 将 <草叶伊纳 II> 按 7:3 刻印为 <草叶伊纳 III> 和 <沙伊纳>
            // 额外产出 <沙伊纳>
            Technique::FilterIII => {
                let input = data.iner(Iner::GabeII);
                *data.iner_mut(Iner::GabeII) = 0.0;
                *data.iner_mut(Iner::GabeIII) += input * 0.7;
                *data.iner_mut(Iner::Shay) += input * 1.3;
            }
            // 将 1 份 <天空伊纳> 和 1 份 <沙伊纳> 刻印为 1 份 <天空伊纳 I>
            // 本工艺生效前，将已有 <天空伊纳> 和 <沙伊纳> 数量均分
            // <天空伊纳> 系列宝石单个评价分数增加 5 分
            Technique::MingleI => {
                // 数量均分，故最终会完全消耗，不会有剩余部分
                let sum = data.iner(Iner::Pet) + data.iner(Iner::Shay);
                *data.iner_mut(Iner::Pet) = 0.0;
                *data.iner_mut(Iner::Shay) = 0.0;
                *data.iner_mut(Iner::PetI) += sum / 2.0;

                add_pet_iner_score(data, 5);
            }
            // 将 1 份 <天空伊纳 I> 和 1 份 <草叶伊纳 I> 刻印为 1 份 <天空伊纳 II>
            // 本工艺生效前，将已有 <天空伊纳 I> 和 <草叶伊纳 I> 数量均分
            // <天空伊纳> 系列宝石单个评价分数增加 15 分
            Technique::MingleII => {
                // 数量均分，故最终会完全消耗，不会有剩余部分
                let sum = data.iner(Iner::PetI) + data.iner(Iner::GabeI);
                *data.iner_mut(Iner::PetI) = 0.0;
                *data.iner_mut(Iner::GabeI) = 0.0;
                *data.iner_mut(Iner::PetII) += sum / 2.0;

                add_pet_iner_score(data, 15)
            }
            // 将 1 份 <天空伊纳 II> 和 1 份 <火焰伊纳 III> 刻印为 1 份 <天空伊纳 III>
            // 本工艺生效前，将已有 <天空伊纳 II> 和 <火焰伊纳 III> 数量均分
            // 若刻印产出只有1种宝石，额外获得 <天空伊纳 III> 数量 100 倍的评价分数
            Technique::MingleIII => {
                // 数量均分，故最终会完全消耗，不会有剩余部分
                let sum = data.iner(Iner::PetII) + data.iner(Iner::NerseIII);
                *data.iner_mut(Iner::PetII) = 0.0;
                *data.iner_mut(Iner::NerseIII) = 0.0;
                *data.iner_mut(Iner::PetIII) += sum / 2.0;
                data.extra_score += 100 * data.iner(Iner::PetIII).floor() as usize;
            }
            // 将 1 份 <沙伊纳> 刻印为 5 份 <沙伊纳>
            Technique::CrystalI => {
                *data.iner_mut(Iner::Shay) *= 5.0;
            }
            // 将 1 份 <沙伊纳> 刻印为 8 份 <沙伊纳>
            Technique::CrystalII => {
                *data.iner_mut(Iner::Shay) *= 8.0;
            }
            // 将 1 份 <沙伊纳> 刻印为 9 份 <沙伊纳>
            // <沙伊纳> 单个评价分数增加 1 分
            Technique::CrystalIII => {
                *data.iner_mut(Iner::Shay) *= 9.0;
                *data.iner_score_mut(Iner::Shay) += 1;
            }
        }
    }

    /// 在工艺区的操作
    /// 只有萃雕 I II III 可以在工艺区生效
    fn operate_in_technique_area(&self, data: &mut GameData) {
        match self {
            Technique::ExtractI | Technique::ExtractII | Technique::ExtractIII => {
                self.operate_on_board(data)
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone)]
struct InerInfo {
    amount: f64,
    score: usize,
}

impl InerInfo {
    fn new(iner: Iner, amount: f64) -> Self {
        Self {
            amount,
            score: iner.base_score(),
        }
    }
}

#[derive(Debug, Clone)]
struct GameData {
    extra_score: usize,
    iner: HashMap<Iner, InerInfo>,
    empty_boards: usize,
}

impl GameData {
    fn new(nerse: usize, pet: usize, gabe: usize, shay: usize, empty_boards: usize) -> Self {
        Self {
            extra_score: 0,
            iner: [
                (Iner::Nerse, nerse),
                (Iner::Pet, pet),
                (Iner::Gabe, gabe),
                (Iner::Shay, shay),
            ]
            .into_iter()
            .map(|(iner, amount)| (iner, InerInfo::new(iner, amount as f64)))
            .collect(),
            empty_boards,
        }
    }

    fn iner_mut(&mut self, iner: Iner) -> &mut f64 {
        &mut self
            .iner
            .entry(iner)
            .or_insert(InerInfo::new(iner, 0.0))
            .amount
    }

    fn iner(&self, iner: Iner) -> f64 {
        if let Some(info) = self.iner.get(&iner) {
            info.amount
        } else {
            0.0
        }
    }

    fn iner_score_mut(&mut self, iner: Iner) -> &mut usize {
        &mut self
            .iner
            .entry(iner)
            .or_insert(InerInfo::new(iner, 0.0))
            .score
    }
}

#[derive(Debug, Clone)]
struct Game {
    // 数据
    data: RefCell<GameData>,
    // 操作台
    board: [Option<Technique>; 6],
    // 工艺区
    technique: Vec<Technique>,
}

impl Game {
    fn run(&self) -> usize {
        for tech in &self.technique {
            tech.operate_in_technique_area(&mut self.data.borrow_mut());
        }
        for tech in &self.board {
            if let Some(tech) = tech {
                tech.operate_on_board(&mut self.data.borrow_mut());
            }
        }
        let mut score = 0;
        for (
            _,
            &InerInfo {
                amount: a,
                score: s,
            },
        ) in self.data.borrow().iner.iter()
        {
            score += a.floor() as usize * s;
        }
        score += self.data.borrow().extra_score;
        score
    }
}

struct Simulation {
    available_techniques: Vec<Technique>,
    board_size: usize,
    initial_nerse: usize,
    initial_pet: usize,
    initial_gabe: usize,
    initial_shay: usize,
}

impl Simulation {
    fn find_best(&self) -> Game {
        use itertools::*;
        let tech_amount = self.available_techniques.len();
        let best_game: Mutex<Option<Game>> = Mutex::new(None);
        let best_score: Mutex<Option<usize>> = Mutex::new(None);
        let total_iterations: u128 = (tech_amount as u128).factorial();
        let iters = Arc::new(RwLock::new(0u128));
        let iters1 = Arc::clone(&iters);
        let handle = thread::spawn(move || {
            let mut old = 0;
            loop {
                let iters = iters1.read().unwrap().to_owned();
                println!(
                    "[{:.2}%]: {} / {}, {}M iterations per sec",
                    iters as f64 / total_iterations as f64,
                    iters,
                    total_iterations,
                    (iters - old) as f64 / 1_000_000.0
                );
                thread::sleep(Duration::from_secs(1));
                if iters >= total_iterations {
                    break;
                }
                old = iters;
            }
        });
        self.available_techniques
            .iter()
            .permutations(tech_amount)
            .par_bridge()
            .for_each(|permutation| {
                let s = usize::min(tech_amount, self.board_size);
                for i in 0..=s {
                    let mut board: [Option<Technique>; 6] = [None; 6];
                    board.iter_mut().enumerate().for_each(|(k, v)| {
                        if k < i {
                            *v = Some(*permutation[k]);
                        }
                    });
                    let technique = permutation[i..].iter().map(|&&t| t).collect_vec();
                    let data = GameData::new(
                        self.initial_nerse,
                        self.initial_pet,
                        self.initial_gabe,
                        self.initial_shay,
                        self.board_size - i,
                    );
                    let game = Game {
                        data: RefCell::new(data),
                        board,
                        technique,
                    };
                    let score = game.run();
                    if score > best_score.lock().unwrap().unwrap_or(0) {
                        *best_game.lock().unwrap() = Some(game);
                        *best_score.lock().unwrap() = Some(score);
                    }
                }
                *iters.write().unwrap() += s as u128;
            });
        handle.join().unwrap();
        let game = best_game.lock().unwrap().clone();
        game.unwrap()
    }
}

fn main() {
    let simulation = Simulation {
        available_techniques: vec![
            // 萃雕
            Technique::ExtractI,
            Technique::ExtractII,
            Technique::ExtractIII,
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
        board_size: 2,
        initial_nerse: 19,
        initial_pet: 32,
        initial_gabe: 20,
        initial_shay: 29,
    };
    let game = simulation.find_best();
    println!("{:?}", game);
}
