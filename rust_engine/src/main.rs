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
    let b = Board::from(f);
    let mut x = MiniMax::new(4);
    let d = Dame {
        ai: b.player(), // current player is AI
        b: b
    };
    let _ = x.minimax(&d);
    println!("* scores computed: {}\n* winning score: {}\n* time {}", x.scores(), x.score(), x.duration_ms());
}

fn main() {
    println!("hello");
    perf();
}

