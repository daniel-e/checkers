extern crate engine;

use std::iter::repeat;

use engine::board::board::{Board, Color};
use engine::ai::minimax::MiniMax;
use engine::dame::Dame;

fn perf() {
    let mut f: Vec<Color> = repeat(Color::Empty).take(8 * 8).collect();
    f[2 * 8 + 2] = Color::WhiteDame;
    f[2 * 8 + 6] = Color::WhiteDame;
    f[6 * 8 + 2] = Color::BlackDame;
    f[6 * 8 + 6] = Color::BlackDame;
    let mut x = MiniMax::new(5);
    x.minimax(Dame::new(Board::from(f)));
    println!("* scores computed: {}", x.scores());
    println!("* winning score  : {}", x.score());
    println!("* time           : {}", x.duration_ms());
}

fn main() {
    println!("measuring performance ...");
    perf();
}

