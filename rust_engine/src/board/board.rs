use rustc_serialize::json::encode;
use std::iter::repeat;

use board::point::Point;

struct MoveFor {
    pub v: [(i32, i32); 4],
    pub n: usize
}

impl MoveFor {
    pub fn new() -> MoveFor {
        MoveFor {
            v: [(0, 0); 4],
            n: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable, PartialEq, Eq)]
pub enum Color {
    Empty = 0,
    WhiteNormal = 1,
    WhiteDame = 2,
    BlackNormal = 3,
    BlackDame = 4
}

#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
    None
}

#[derive(RustcEncodable, RustcDecodable, Clone, Debug)]
pub struct Board {
    board_bitmask: u64,
    positions: Vec<(i32, i32)>,
    board: Vec<Color>,
    next_move: Player,
    valid_pieces_to_move: Vec<(i32, i32)>,
    winner: Player,
    last_moves: Vec<(i32, i32, i32, i32)>,
    move_no: i32,
}

impl Board {
    pub fn new() -> Board {
        let w = vec![
            (0, 0), (2, 0), (4, 0), (6, 0), (1, 1), (3, 1), (5, 1), (7, 1),
            (0, 2), (2, 2), (4, 2), (6, 2)
        ];
        let b = vec![
            (1, 5), (3, 5), (5, 5), (7, 5), (0, 6), (2, 6), (4, 6), (6, 6),
            (1, 7), (3, 7), (5, 7), (7, 7)
        ];
        let mut brd: Vec<Color> = repeat(Color::Empty).take(8 * 8).collect();
        for (x, y) in w {
            brd[y * 8 + x] = Color::WhiteNormal;
        }
        for (x, y) in b {
            brd[y * 8 + x] = Color::BlackNormal;
        }
        Board::from(brd)
    }

    fn create_positions(v: &Vec<Color>) -> Vec<(i32, i32)> {
        v.iter().enumerate()
            .filter(|&(_, c)| *c != Color::Empty)
            .map(|(idx, _)| (idx as i32 % 8, idx as i32 / 8)).collect::<Vec<_>>()
    }

    fn remove_position(&mut self, x: i32, y: i32) {
        let p = self.positions.iter().position(|&i| i == (x, y)).unwrap();
        self.positions.swap_remove(p);
    }

    fn create_bitmask(&mut self) {
        self.board_bitmask = 0;
        for x in 0..8 {
            for y in 0..8 {
                let p = self.index(x, y).unwrap();
                if self.board[p] != Color::Empty {
                    self.set_bit(p);
                }
            }
        }
    }

    fn set_bit(&mut self, p: usize) {
        let mut i: u64 = 1;
        i <<= p;
        self.board_bitmask |= i;
    }

    fn clear_bit(&mut self, p: usize) {
        let mut i: u64 = 1;
        i <<= p;
        i = !i;
        self.board_bitmask &= i;
    }

    pub fn from(v: Vec<Color>) -> Board {
        let mut r = Board {
            board_bitmask: 0,
            positions: Board::create_positions(&v),
            board: v,
            next_move: Player::Black,
            valid_pieces_to_move: vec![],
            winner: Player::None,
            last_moves: vec![],
            move_no: 0,
        };
        r.create_bitmask();
        r.update_valid_pieces_to_move();
        r
    }

    pub fn get_last_moves(&self) -> Vec<(i32, i32, i32, i32)> {
        self.last_moves.clone()
    }

    pub fn player(&self) -> Player {
        self.next_move
    }

    pub fn winner(&self) -> Player {
        self.winner
    }

    pub fn movable_pieces(&self) -> Vec<(i32, i32)> {
        self.valid_pieces_to_move.clone()
    }

    pub fn valid_moves(&self) -> Vec<(i32, i32, i32, i32)> {
        // TODO refactoring
        let mut v: Vec<(i32, i32, i32, i32)> = Vec::new();
        for i in self.movable_pieces() {
            let mf = self.moves_for(i.0, i.1);
            for k in 0..mf.n {
                v.push((i.0, i.1, mf.v[k].0, mf.v[k].1));
            }
        }
        v
    }

    pub fn finished(&self) -> bool {
        self.winner != Player::None
    }

    pub fn to_json(&self) -> String {
        encode(self).unwrap()
    }

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            Some((y * 8 + x) as usize)
        } else {
            None
        }
    }

    fn color(&self, x: i32, y: i32) -> Option<Color> {
        match self.index(x, y) {
            Some(p) => Some(self.board[p]),
            _ => None
        }
    }

    fn is_black(&self, c: Color) -> bool {
        (c == Color::BlackNormal || c == Color::BlackDame)
    }

    fn is_white(&self, c: Color) -> bool {
        (c == Color::WhiteNormal || c == Color::WhiteDame)
    }

    fn matching(&self, c: Color) -> bool {
        (self.next_move == Player::Black && self.is_black(c)) ||
        (self.next_move == Player::White && self.is_white(c))
    }

    fn is_empty(&self, x: i32, y: i32) -> bool {

        match self.index(x, y) {
            Some(p) => {
                let mut i: u64 = 1;
                i <<= p;
                (self.board_bitmask & i) == 0
            },
            _ => false
        }
    }

    fn is_player(&self, x: i32, y: i32, p: Player) -> bool {
        let c = self.color(x, y).unwrap();
        match p {
            Player::Black => self.is_black(c),
            Player::White => self.is_white(c),
            _ => false
        }
    }

    pub fn other_player(&self, p: Player) -> Player {
        match p {
            Player::Black => Player::White,
            Player::White => Player::Black,
            _ => { panic!("invlaid player"); }
        }
    }

    fn is_normal(&self, x: i32, y: i32) -> bool {
        let p = self.index(x, y).unwrap();
        self.board[p] == Color::BlackNormal || self.board[p] == Color::WhiteNormal
    }

    fn is_dame(&self, x: i32, y: i32) -> bool {
        let p = self.index(x, y).unwrap();
        self.board[p] == Color::BlackDame || self.board[p] == Color::WhiteDame
    }

    pub fn count_normal(&self, p: Player) -> i32 {
        self.positions.iter()
            .filter(|&&(x, y)| self.is_player(x, y, p) && self.is_normal(x, y)).count() as i32
    }

    pub fn count_dame(&self, p: Player) -> i32 {
        self.positions.iter()
            .filter(|&&(x, y)| self.is_player(x, y, p) && self.is_dame(x, y)).count() as i32
    }

    pub fn positions(&self, p: Player) -> Vec<(i32, i32)> {
        self.positions.iter()
            .filter(|&&(x, y)| self.is_player(x, y, p)).cloned().collect()
    }

    fn move_piece(&self, x: i32, y: i32, dy: i32, p: Player, v: &mut [(i32, i32)], pos: usize) -> usize {

        let mut i = pos;
        let mut u = false;

        if pos > 0 && (v[0].0 - x).abs() == 2 {
            u = true;
        }

        if self.is_empty(x - 2, y + dy * 2) && self.is_player(x - 1, y + dy, p) {
            if !u {
                i = 0;
            }
            v[i] = (x - 2, y + dy * 2);
            i += 1;
            u = true;
        }
        if self.is_empty(x + 2, y + dy * 2) && self.is_player(x + 1, y + dy, p) {
            if !u {
                i = 0;
            }
            v[i] = (x + 2, y + dy * 2);
            i += 1;
            u = true;
        }

        // If we have found a jump over a piece of the opponent we don't have to search for other
        // moves as the jump is mandatory.
        if !u {
            if self.is_empty(x - 1, y + dy) {
                v[i] = (x - 1, y + dy);
                i += 1;
            }
            if self.is_empty(x + 1, y + dy) {
                v[i] = (x + 1, y + dy);
                i += 1;
            }
        }

        i
    }

    // Checks if the piece at (x, y) of the current player can jump over a piece of the opponent.
    fn can_remove_piece(&self, x: i32, y: i32) -> bool {
        let mut mf = MoveFor::new();
        self.get_moves_for(x, y, &mut mf);
        mf.v.iter().take(mf.n).any(|ref p| (p.0 - x).abs() == 2)
    }

    // Checks if moving the piece at (x, y) is allowed.
    fn moving_piece_is_allowed(&self, x: i32, y: i32) -> bool {
        self.valid_pieces_to_move.iter()
            .any(|&(px, py)| px == x && py == y)
    }

    fn update_valid_pieces_to_move(&mut self) { // XXX

        self.valid_pieces_to_move.clear();

        for i in self.positions.iter() {
            if self.can_remove_piece(i.0, i.1) {
                self.valid_pieces_to_move.push((i.0, i.1));
            }
        }

        if self.valid_pieces_to_move.len() == 0 {
            for i in self.positions.iter() {
                if self.moves_for(i.0, i.1).n > 0 {
                    self.valid_pieces_to_move.push((i.0, i.1));
                }
            }
        }
    }

    pub fn mv(&self, x: i32, y: i32) -> Option<Vec<Point>> {
        // Check if piece is allowed to be moved.
        if self.moving_piece_is_allowed(x, y) {
            let mf = self.moves_for(x, y);
            Some(mf.v.iter().take(mf.n).map(|&(x, y)| Point::new(x, y)).collect())
        } else {
            None
        }
    }

    fn get_moves_for(&self, x: i32, y: i32, r: &mut MoveFor) {

        r.n = match self.color(x, y) {
            Some(c) => {
                if self.matching(c) {
                    match c {
                        Color::WhiteNormal => {
                            self.move_piece(x, y,  1, Player::Black, &mut r.v, 0)
                        },
                        Color::WhiteDame => {
                            let p = self.move_piece(x, y,  1, Player::Black, &mut r.v, 0);
                            self.move_piece(x, y, -1, Player::Black, &mut r.v, p)
                        },
                        Color::BlackNormal => {
                            self.move_piece(x, y, -1, Player::White, &mut r.v, 0)
                        },
                        Color::BlackDame => {
                            let p = self.move_piece(x, y,  1, Player::White, &mut r.v, 0);
                            self.move_piece(x, y, -1, Player::White, &mut r.v, p)
                        },
                        _ => 0
                    }
                } else {
                    0
                }
            },
            _ => 0
        }
    }

    // Returns points to which the piece at (x, y) can move to.
    fn moves_for(&self, x: i32, y: i32) -> MoveFor {
        let mut mf = MoveFor::new();
        self.get_moves_for(x, y, &mut mf);
        mf
    }

    pub fn clear_last_moves(&mut self) {
        self.last_moves.clear()
    }
    pub fn move_it(&mut self, x: i32, y: i32, dx: i32, dy: i32) {

        // Return if piece is not allowed to be moved or if game is finished.
        if !self.moving_piece_is_allowed(x, y) || self.winner != Player::None {
            return;
        }

        // Get all valid moves for this piece.
        let mf = self.moves_for(x, y);

        // If (dx, dy) is not in the valid moves return.
        if !mf.v.iter().take(mf.n).any(|ref p| p.0 == dx && p.1 == dy) {
            return;
        }

        let p = self.index(x, y).unwrap();        // position of source
        let q = self.index(dx, dy).unwrap();      // position of destination

        self.last_moves.push((x, y, dx, dy));
        self.move_no += 1;

        // Jump to new position.
        self.positions.push((q as i32 % 8, q as i32 / 8));
        self.remove_position(p as i32 % 8, p as i32 / 8);
        self.board[q] = self.board[p];
        self.board[p] = Color::Empty;

        self.set_bit(q);
        self.clear_bit(p);

        // If we jumped over an opponent's piece remove that.
        let mut removed = false;
        if (dx - x).abs() == 2 {
            let pp = self.index(x + (dx - x) / 2, y + (dy - y) / 2).unwrap();
            self.remove_position(pp as i32 % 8, pp as i32 / 8);
            self.board[pp] = Color::Empty;
            self.clear_bit(pp);
            removed = true;
        }

        let player = self.next_move.clone();

        // If this piece removed an opponent's piece and can this piece remove another piece?
        if removed && self.can_remove_piece(dx, dy) {
            // Update status.
            self.valid_pieces_to_move = vec![(dx, dy)];
            // Do not update next player.
        } else {
            // Otherwise, update next player.
            self.next_move = self.other_player(self.next_move);
            // Update next valid pieces to move for next player.
            self.update_valid_pieces_to_move();
            // Check end.
            if self.valid_pieces_to_move.len() == 0 {
                self.winner = player;
            }
        }

        // Check if piece needs to be converted to dame.
        if dy == 0 && player == Player::Black {
            self.board[q] = Color::BlackDame;
        }
        if dy == 7 && player == Player::White {
            self.board[q] = Color::WhiteDame;
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate std;
    use board::board::{Board, Color, Player};

    #[test]
    fn index() {
        let g = Board::new();
        assert_eq!(g.index(4, 5).unwrap(), 44);
        assert!(g.index(8, 1).is_none());
        assert!(g.index(1, 8).is_none());
        assert!(g.index(-1, 3).is_none());
        assert!(g.index(3, -1).is_none());
    }

    #[test]
    fn color() {
        let g = Board::new();
        assert_eq!(g.color(0, 0).unwrap(), Color::WhiteNormal);
        assert_eq!(g.color(1, 0).unwrap(), Color::Empty);
        assert_eq!(g.color(0, 6).unwrap(), Color::BlackNormal);
        assert!(g.color(0, 8).is_none());
        assert!(g.color(8, 0).is_none());
    }

    #[test]
    fn matching() {
        let mut g = Board::new();
        assert!(g.matching(Color::BlackNormal));
        assert!(g.matching(Color::BlackDame));
        assert!(!g.matching(Color::WhiteNormal));
        assert!(!g.matching(Color::WhiteDame));
        assert!(!g.matching(Color::Empty));
        g.next_move = Player::White;
        assert!(!g.matching(Color::BlackNormal));
        assert!(!g.matching(Color::BlackDame));
        assert!(g.matching(Color::WhiteNormal));
        assert!(g.matching(Color::WhiteDame));
        assert!(!g.matching(Color::Empty));
    }

    #[test]
    fn is_color() {
        let g = Board::new();
        assert!(g.is_color(0, 0, Color::WhiteNormal));
        assert!(g.is_color(1, 0, Color::Empty));
        assert!(g.is_color(0, 6, Color::BlackNormal));
    }

//    #[test]
//    fn moves_for() {
//        let g = Board::new();
//        assert!(g.moves_for(1, 5).n == 2);
//    }

//    #[test]
//    fn constructor() {
//        let g = Board::new();
//        assert_eq!(g.next_move, Player::Black);
//        assert_eq!(g.winner, Player::None);
//        assert_eq!(g.valid_pieces_to_move, vec![(1, 5), (3, 5), (5, 5), (7, 5)]);
//    }

    fn empty_board() -> Board {
        let mut b = Board::new();
        b.board = std::iter::repeat(Color::Empty).take(8 * 8).collect();
        b
    }

    // TODO: reimplement test
//    #[test]
//    fn play1() {
//        let mut g = empty_board();
//        g.board[2 * 8 + 2] = Color::WhiteNormal;
//        g.board[4 * 8 + 4] = Color::BlackNormal;
//        g.next_move = Player::White;
//        g.valid_pieces_to_move = vec![(2, 2)];
//
//        g.move_it(2, 2, 3, 3);
//        assert_eq!(g.valid_pieces_to_move, vec![(4, 4)]);
//        let v: Vec<(i32, i32)> = g.mv(4, 4).unwrap().iter().map(|ref p| (p.x, p.y)).collect();
//        assert_eq!(v, vec![(2, 2)]);
//
//        g.move_it(4, 4, 2, 2);
//        assert_eq!(g.winner, Player::Black);
//    }

    #[test]
    fn positions() {
        let mut v: Vec<Color> = std::iter::repeat(Color::Empty).take(8 * 8).collect();
        v[0] = Color::WhiteNormal;
        v[9] = Color::WhiteNormal;
        v[11] = Color::WhiteNormal;
        v[63] = Color::BlackNormal;
        let mut g = Board::from(v);
        assert_eq!(g.positions[0], (0, 0));
        assert_eq!(g.positions[1], (1, 1));
        assert_eq!(g.positions[2], (3, 1));
        assert_eq!(g.positions[3], (7, 7));
        g.remove_position(1, 1);
        assert_eq!(g.positions[0], (0, 0));
        assert_eq!(g.positions[1], (7, 7));
        assert_eq!(g.positions[2], (3, 1));
        g.remove_position(3, 1);
        assert_eq!(g.positions[0], (0, 0));
        assert_eq!(g.positions[1], (7, 7));
    }
        // TODO: test for moves_for

    #[test]
    fn bits() {
        let mut v: Vec<Color> = std::iter::repeat(Color::Empty).take(8 * 8).collect();
        v[0] = Color::WhiteNormal;
        v[9] = Color::WhiteNormal;
        v[11] = Color::WhiteNormal;
        v[63] = Color::BlackNormal;
        let mut g = Board::from(v);

        assert!(g.board_bitmask & (1 << 0) > 0);
        assert!(g.board_bitmask & (1 << 9) > 0);
        assert!(g.board_bitmask & (1 << 11) > 0);
        assert!(g.board_bitmask & (1 << 63) > 0);
        assert!(g.is_empty(0, 0) == false);

        g.clear_bit(0);
        assert!(g.board_bitmask & (1 << 0) == 0);
        assert!(g.board_bitmask & (1 << 9) > 0);
        assert!(g.board_bitmask & (1 << 11) > 0);
        assert!(g.board_bitmask & (1 << 63) > 0);
        g.clear_bit(9);
        assert!(g.board_bitmask & (1 << 0) == 0);
        assert!(g.board_bitmask & (1 << 9) == 0);
        assert!(g.board_bitmask & (1 << 11) > 0);
        assert!(g.board_bitmask & (1 << 63) > 0);
        g.clear_bit(63);
        assert!(g.board_bitmask & (1 << 0) == 0);
        assert!(g.board_bitmask & (1 << 9) == 0);
        assert!(g.board_bitmask & (1 << 11) > 0);
        assert!(g.board_bitmask & (1 << 63) == 0);
        g.clear_bit(2);
        assert!(g.board_bitmask & (1 << 2) == 0);
        g.clear_bit(11);
        assert!(g.board_bitmask == 0);
    }

}