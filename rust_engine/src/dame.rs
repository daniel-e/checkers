use std;

use ai::minimax;
use board::board;
use board::board::Board;
use ai::minimax::Game;

#[derive(Debug, Clone, Copy)]
pub struct DameMove {
    pub src_x: i32,
    pub src_y: i32,
    pub dst_x: i32,
    pub dst_y: i32
}

impl DameMove {
    pub fn new(x: (i32, i32, i32, i32)) -> DameMove {
        DameMove {
            src_x: x.0,
            src_y: x.1,
            dst_x: x.2,
            dst_y: x.3
        }
    }
}

impl std::fmt::Display for DameMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let v = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        write!(f, "{}{}-{}{}", v[self.src_x as usize], self.src_y + 1, v[self.dst_x as usize], self.dst_y + 1)
    }
}

#[derive(Debug, Clone)]
pub struct Dame {
    pub b: Board,
    pub ai: board::Player
}

impl Dame {

    pub fn new(b: Board) -> Dame {
        let p = b.player();
        Dame {
            b: b,
            ai: p,
        }
    }

    fn avg_distance(a: Vec<(i32, i32)>, b: Vec<(i32, i32)>) -> f64 {

        let mut d = 0.0;
        if a.len() > 0 && b.len() > 0 {
            let s: f64 = a.iter().map(|&(x, y)|
                b.iter().map(|&(bx, by)| ((bx - x).pow(2) as f64 + (by - y).pow(2) as f64).sqrt()).collect::<Vec<_>>()
            ).flat_map(|x| x).sum();
            d = s / (a.len() as f64 * b.len() as f64);
        }
        d
    }
}

impl Game<DameMove> for Dame {

    fn current_player(&self) -> minimax::Player {
        match self.ai == self.b.player() {
            true => minimax::Player::AI,
            _ => minimax::Player::HUMAN
        }
    }

    fn valid_moves(&self) -> Vec<DameMove> {
        self.b.valid_moves().iter().map(|&x| DameMove::new(x)).collect()
    }

    fn finished(&self) -> bool {
        self.b.finished()
    }

    // returns large values (e.g. +1) if AI has an advantage
    // returns small values (e.g. -1) if HUMAN has an advantage
    fn score(&self) -> f64 {

        // check for win/loose
        let mut s0: f64 = 0.0;
        if self.b.winner() == self.ai { // AI wins
            s0 = 1.0;
        }
        if self.b.finished() && self.b.winner() != self.ai { // HUMAN wins
            s0 = -1.0;
        }

        // count number of pieces; bzw. the advantage
        let n_ai = self.b.count_normal(self.ai);
        let n_hm = self.b.count_normal(self.b.other_player(self.ai));
        let s1: f64 = (n_ai - n_hm) as f64 / 12.0;

        // advantage in Damen
        let d_ai = self.b.count_dame(self.ai);
        let d_hm = self.b.count_dame(self.b.other_player(self.ai));
        let s2: f64 = (d_ai - d_hm) as f64 / 12.0;

        let s3: f64 = d_ai as f64;

        // if AI has more pieces play more aggressive
//        let a = self.b.positions(self.ai);
//        let b = self.b.positions(self.b.other_player(self.ai));
//        let d = Dame::avg_distance(a, b);
        //println!("d = {}", d);

        let r = s0 * 20.0 + s1 * 1.0 + s2 * 3.0 + s3; // + d * 0.1;

        r
    }

    fn set(&self, m: DameMove) -> Dame {
        let mut b = self.b.clone();
        b.move_it(m.src_x, m.src_y, m.dst_x, m.dst_y);
        Dame {
            b: b,
            ai: self.ai
        }
    }
}


#[cfg(test)]
mod tests {
    use Dame;

    #[test]
    fn dame_avg_distance() {
        let mut a = vec![(0, 0)];
        let mut b = vec![(7, 7)];
        assert!((Dame::avg_distance(a, b) - 9.899).abs() < 0.001);

        a = vec![(0, 0), (1, 1)];
        b = vec![(7, 7), (6, 6)];
        assert!((Dame::avg_distance(a, b) - 8.4853).abs() < 0.001);

        a = vec![(0, 0), (1, 1)];
        b = vec![(2, 2), (3, 4)];
        assert!((Dame::avg_distance(a, b) - 3.212).abs() < 0.001);
    }
}