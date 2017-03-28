pub mod minimax;

use rand;
use rand::Rng;
use board::board::Board;

pub fn random(b: &mut Board) {
    // Choose a piece at random.
    let v = b.movable_pieces();
    let e = rand::thread_rng().choose(&v).unwrap();
    // Choose a valid move for that piece at random.
    let m = b.mv(e.0, e.1).unwrap();
    let n = rand::thread_rng().choose(&m).unwrap();
    b.move_it(e.0, e.1, n.x, n.y);
}