extern crate engine;

use engine::board::board::Board;
use engine::ai::minimax::MiniMax;
use engine::dame::Dame;

fn main() {
    println!("running ...");

    let b = Board::new();
    let p = b.player();

    let mut d = Dame {
        b: b,
        ai: p  // TODO: why do we need this  - I think for computing scores
    };

    while !d.b.finished() {

        let p = d.b.player();
        d.ai = d.b.player();

        while !d.b.finished() && d.b.player() == p {
            let mut x = MiniMax::new(5);
            let m = x.minimax(&d);
            println!("scores computed: {}", x.scores());
            println!("winning score: {}", x.score());
            println!("time in ms: {}", x.duration_ms());
            d.b.move_it(m.src_x, m.src_y, m.dst_x, m.dst_y);
        }

        println!("moves: {:?}", d.b.get_last_moves());
        d.b.clear_last_moves();
    }

    println!("done");
}