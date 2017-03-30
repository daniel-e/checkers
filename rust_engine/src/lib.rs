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
pub mod pool;

use ai::minimax::MiniMax;
use board::board::Board;
use dame::Dame;

fn decode_board(b: String) -> Board {
    let mut b: Board = json::decode(&b).unwrap();
    b.clear_last_moves();
    b
}

// exported python functions

fn new_game(_: Python) -> PyResult<String> {
    Ok(Board::new().to_json())
}

fn moves_for(_: Python, b: String, x: i32, y: i32) -> PyResult<Vec<(i32, i32)>> {
    let b = decode_board(b);
    match b.mv(x, y) {
        Some(v) => Ok(v.iter().map(|ref p| (p.x, p.y)).collect()),
        _ => Ok(vec![])
    }
}

fn move_it(_: Python, b: String, x: i32, y: i32, dx: i32, dy: i32) -> PyResult<String> {
    let mut b = decode_board(b);
    b.move_it(x, y, dx, dy);
    Ok(b.to_json())
}

fn ai_random(_: Python, b: String) -> PyResult<String> {
    let mut b = decode_board(b);
    ai::random(&mut b);
    Ok(b.to_json())
}

fn ai_minimax(_: Python, b: String, depth: usize) -> PyResult<String> {
    let mut d = Dame::new(decode_board(b));

    while !d.b.finished() && d.b.player() == d.ai {
        let mut x = MiniMax::new(depth);
        let m = x.minimax(d.clone());
        println!("configured depth: {}", depth);
        println!("scores computed : {}", x.scores());
        println!("winning score   : {}", x.score());
        println!("time in ms      : {}", x.duration_ms());
        println!("path            : {}", x.path().iter().map(|&x| format!("{}", x)).collect::<Vec<_>>().join(", "));
        d.b.move_it(m.src_x, m.src_y, m.dst_x, m.dst_y);
    }
    Ok(d.b.to_json())
}

// initialize python functions

py_module_initializer!(engine, initengine, PyInit_engine, |py, m| {
    try!(m.add(py, "new_game",   py_fn!(py, new_game())));
    try!(m.add(py, "moves_for",  py_fn!(py, moves_for(b: String, x: i32, y: i32))));
    try!(m.add(py, "move_it",    py_fn!(py, move_it(b: String, x: i32, y: i32, dx: i32, dy: i32))));
    try!(m.add(py, "ai_random",  py_fn!(py, ai_random(b: String))));
    try!(m.add(py, "ai_minimax", py_fn!(py, ai_minimax(b: String, depth: usize))));
    Ok(())
});
