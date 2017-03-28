use rand::{Rng, thread_rng};
use time::PreciseTime;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum Player {
    AI,
    HUMAN
}

pub trait Game<Move: Clone + Copy + Debug> {
    fn valid_moves(&self) -> Vec<Move>;
    fn score(&self) -> f64;
    fn set(&self, m: Move) -> Self;
    fn finished(&self) -> bool;
    fn current_player(&self) -> Player;
}

pub struct MiniMax<Move> {
    score_cnt: usize,
    max_recursion: usize,
    score_winner: f64,
    start: PreciseTime,
    duration: i64,
    path: Vec<Move>,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Path<Move: Clone + Copy + Debug> {
    p: Vec<Move>
}

impl<Move: Clone + Copy + Debug> Path<Move> {
    pub fn new() -> Path<Move> {
        Path {
            p: vec![]
        }
    }

    pub fn push(&self, m: Move) -> Path<Move> {
        let mut v = self.p.clone();
        v.push(m);
        Path {
            p: v
        }
    }
}

#[derive(Debug, Clone)]
struct Score<Move: Clone + Copy + Debug> {
    path: Path<Move>,
    score: f64,
}

impl<Move: Clone + Copy + Debug> Score<Move> {
    pub fn new(score: f64, p: Path<Move>) -> Score<Move> {
        Score {
            path: p,
            score: score
        }
    }
}

impl<Move: Clone + Copy + Debug> MiniMax<Move> {

    pub fn new(max_recurions: usize) -> MiniMax<Move> {
        MiniMax {
            score_cnt: 0,
            max_recursion: max_recurions,
            score_winner: 0.0,
            start: PreciseTime::now(),
            duration: 0,
            path: vec![]
        }
    }

    pub fn minimax<T: Game<Move>>(&mut self, game: T) -> Move {
        self.start = PreciseTime::now();
        let m = self._select_by(&game, 0, Path::new());  // first move is done by AI
        self.score_winner = m.score;
        self.duration = self.start.to(PreciseTime::now()).num_milliseconds();
        self.path = m.path.p;
        self.path.first().unwrap().clone()
    }

    pub fn path(&self) -> Vec<Move> {
        self.path.clone()
    }

    pub fn duration_ms(&self) -> i64 {
        self.duration
    }

    // Returns the number of scores computed.
    pub fn scores(&self) -> usize {
        self.score_cnt
    }

    // Returns the score of the chosen move.
    pub fn score(&self) -> f64 {
        self.score_winner
    }

    fn _ai_minimax<T: Game<Move>>(&mut self, game: &T, m: Move, rec: usize, path: Path<Move>) -> Score<Move> {

        let g: T = game.set(m);
        let p = path.push(m);

        if g.finished() || rec >= 1000 {
            self.score_cnt += 1;
            Score::new(g.score(), p)
        } else if rec >= self.max_recursion { //&& self.start.to(PreciseTime::now()).num_milliseconds() > 1000 { // TODO
            self.score_cnt += 1;
            Score::new(g.score(), p)
        } else {
            self._select_by(&g, rec + 1, p)
        }
    }

    fn _select_by<T: Game<Move>>(&mut self, game: &T, rec: usize, path: Path<Move>) -> Score<Move> {

        // Compute the score for each valid move.
        let scores = game.valid_moves()
            .iter().map(|&mv| self._ai_minimax(game, mv, rec, path.clone())).collect::<Vec<_>>();

        // Search the maximum/minimum score depending on the player.
        let x = match game.current_player() {
            Player::AI    => scores.iter().max_by(|x, y| x.score.partial_cmp(&y.score).unwrap()).unwrap(),
            Player::HUMAN => scores.iter().min_by(|x, y| x.score.partial_cmp(&y.score).unwrap()).unwrap()
        };

        // TODO what happens if there's no valid move anymore; can this happen or will there be finished() == true

        // Select a move at random among the maximums/minimums.
        (**thread_rng().choose(&
            scores.iter().filter(|s| s.score == x.score).collect::<Vec<_>>()).unwrap()).clone()
    }
}

