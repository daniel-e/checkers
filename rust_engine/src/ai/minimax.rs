use rand::{Rng, thread_rng};
use time::PreciseTime;
use std::fmt::Debug;
use std::marker::Send;

use futures::Future;
use futures::future::{FutureResult, err, ok, lazy};
use futures_cpupool::CpuPool;

#[derive(Debug, Clone, Copy)]
pub enum Player {
    AI,
    HUMAN
}

pub trait Game<Move>
    where Move: Clone + Copy + Debug
{
    fn valid_moves(&self)    -> Vec<Move>;
    fn score(&self)          -> f64;
    fn set(&self, m: Move)   -> Self;
    fn finished(&self)       -> bool;
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


// -------------------------------------------------------------------------------------------------


#[derive(Debug, Clone)]
struct Score<Move: Clone + Copy + Debug> {
    path: Path<Move>,
    score: f64,
    score_cnt: usize
}

impl<Move: Clone + Copy + Debug> Score<Move> {

    pub fn new(score: f64, p: Path<Move>) -> Score<Move> {
        Score {
            path: p,
            score: score,
            score_cnt: 1,
        }
    }

    pub fn set_n_scores(&self, n: usize) -> Score<Move> {
        let mut s = self.clone();
        s.score_cnt = n;
        s
    }

    pub fn scores_count(&self) -> usize {
        self.score_cnt
    }
}


// -------------------------------------------------------------------------------------------------


impl<Move> MiniMax<Move>
    where Move: Clone + Copy + Debug + Send + Sync + 'static
{

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

    pub fn minimax<T>(&mut self, game: T) -> Move
        where T: Game<Move> + Clone + Send + Sync + 'static
    {
        self.start = PreciseTime::now();
        let x = self.max_recursion;
        let m = MiniMax::_select_by(&game, 0, Path::new(), x);  // first move is done by AI
        self.score_winner = m.score;
        self.duration = self.start.to(PreciseTime::now()).num_milliseconds();
        self.score_cnt = m.scores_count();
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

    fn _ai_minimax<T>(game: &T, m: Move, rec: usize, path: Path<Move>, maxrec: usize) -> Score<Move>
        where T: Game<Move> + Clone + Send + Sync + 'static
    {

        let g: T = game.set(m);
        let p = path.push(m);

        if g.finished() || rec >= maxrec {
            Score::new(g.score(), p)
        } else {
            MiniMax::_select_by(&g, rec + 1, p, maxrec)
        }
    }

    fn _select_by<T>(game: &T, rec: usize, path: Path<Move>, maxrec: usize) -> Score<Move>
        where T: Game<Move> + Clone + Send + Sync + 'static
    {
        // Compute the score for each valid move.
        let scores = if rec == 0 {
            let jobs = game.valid_moves().iter().map(|&mv| {
                Job {
                    game: game.clone(),
                    rec: rec,
                    path: path.clone(),
                    maxrec: maxrec,
                    mv: mv
                }
            }).collect::<Vec<_>>();
            parallel(jobs)
        } else {
            game.valid_moves()
                .iter().map(|&mv| MiniMax::_ai_minimax(game, mv, rec, path.clone(), maxrec)).collect::<Vec<_>>()
        };

        let n: usize = scores.iter().map(|s| s.scores_count()).sum();

        // Search the maximum/minimum score depending on the player.
        let x = match game.current_player() {
            Player::AI    => scores.iter().max_by(|x, y| x.score.partial_cmp(&y.score).unwrap()).unwrap(),
            Player::HUMAN => scores.iter().min_by(|x, y| x.score.partial_cmp(&y.score).unwrap()).unwrap()
        };

        // TODO what happens if there's no valid move anymore; can this happen or will there be finished() == true

        // Select a move at random among the maximums/minimums.
        (**thread_rng()
            .choose(&scores.iter().filter(|s| s.score == x.score).collect::<Vec<_>>()).unwrap()
        ).clone().set_n_scores(n)
    }
}

struct Job<T, Move>
    where T   : Game<Move> + Clone + Send + Sync + 'static,
          Move: Clone + Copy + Debug + Send + Sync + 'static
{
    game: T,
    rec: usize,
    path: Path<Move>,
    maxrec: usize,
    mv: Move
}

fn parallel<T, Move>(v: Vec<Job<T, Move>>) -> Vec<Score<Move>>
    where T   : Game<Move> + Clone + Send + Sync + 'static,
          Move: Clone + Copy + Debug + Send + Sync + 'static
{
    let pool = CpuPool::new(4);

    let f = v.iter().map(|j| pool.spawn_fn(move || {
        let r = MiniMax::_ai_minimax(&j.game, j.mv, j.rec, j.path.clone(), j.maxrec);
        let res: FutureResult<Score<Move>, ()> = ok(r);
        res
    })).collect::<Vec<_>>();

    f.into_iter().map(|x| x.wait().unwrap().clone()).collect::<Vec<_>>()
}

