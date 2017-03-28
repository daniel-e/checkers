#[macro_use]
extern crate cpython;
extern crate rustc_serialize;
extern crate rand;
extern crate time;

use rustc_serialize::json;
use cpython::{Python, PyResult};

pub mod board;
pub mod ai;
pub mod dame;

use ai::minimax::MiniMax;
use board::board::Board;
use dame::Dame;

fn new_game(_: Python) -> PyResult<String> {
    Ok(Board::new().to_json())
}

fn moves_for(_: Python, b: String, x: i32, y: i32) -> PyResult<Vec<(i32, i32)>> {
    let board: Board = json::decode(&b).unwrap();
    match board.mv(x, y) {
        Some(v) => Ok(v.iter().map(|ref p| (p.x, p.y)).collect()),
        _ => Ok(vec![])
    }
}

fn move_it(_: Python, b: String, x: i32, y: i32, dx: i32, dy: i32) -> PyResult<String> {
    let mut board: Board = json::decode(&b).unwrap();
    board.clear_last_moves();
    board.move_it(x, y, dx, dy);
    Ok(board.to_json())
}

fn ai_random(_: Python, b: String) -> PyResult<String> {
    let mut board: Board = json::decode(&b).unwrap();
    ai::random(&mut board);
    Ok(board.to_json())
}

fn ai_minimax(_: Python, b: String) -> PyResult<String> {
    let mut b: Board = json::decode(&b).unwrap();
    b.clear_last_moves();
    let p = b.player();
    let mut d = Dame {
        ai: b.player(), // current player is AI
        b: b
    };
    while !d.b.finished() && d.b.player() == p {
        let mut x = MiniMax::new(5); // XXX: 5
        let m = x.minimax(&d);
        println!("scores computed: {}", x.scores());
        println!("winning score: {}", x.score());
        println!("time in ms: {}", x.duration_ms());
        for i in x.path() {
            println!("{}", i);
        }
        d.b.move_it(m.src_x, m.src_y, m.dst_x, m.dst_y);
    }
    Ok(d.b.to_json())
}

py_module_initializer!(engine, initengine, PyInit_engine, |py, m| {
    try!(m.add(py, "new_game", py_fn!(py, new_game())));
    try!(m.add(py, "moves_for", py_fn!(py, moves_for(b: String, x: i32, y: i32))));
    try!(m.add(py, "move_it", py_fn!(py, move_it(b: String, x: i32, y: i32, dx: i32, dy: i32))));
    try!(m.add(py, "ai_random", py_fn!(py, ai_random(b: String))));
    try!(m.add(py, "ai_minimax", py_fn!(py, ai_minimax(b: String))));
    Ok(())
});

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
