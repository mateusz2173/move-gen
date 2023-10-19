use std::collections::VecDeque;

use move_gen::r#move::{MakeMove, Move};
use sdk::{
    position::{Color, Position}, square::Square,
};

use crate::{Engine, SearchStrategy};

pub struct DummySearch {
    path: VecDeque<Move>,
    trace: String,
    best_value: f64,
    pub best_move: Option<Move>,
    maximizing: bool,
}

fn path_to_string(path: &VecDeque<Move>) -> String {
    let mut s = String::new();
    for m in path {
        s.push_str(&format!("{} ", m));
    }
    s
}

impl DummySearch {
    pub fn new(maximizing: bool) -> Self {
        Self {
            path: VecDeque::new(),
            trace: String::new(),
            best_value: if maximizing { f64::MIN } else { f64::MAX },
            maximizing,
            best_move: None,
        }
    }

    fn search_impl(&self) {}
}

const PIECE_VALUES: [f64; 6] = [1.0, 3.0, 3.0, 5.0, 9.0, 100.0];

impl DummySearch {
    fn minmax(
        &mut self,
        engine: &mut Engine,
        depth: u8,
        pos: &Position,
        maximizing_player: bool,
    ) -> f64 {
        if depth == 0 {
            let val = self.evaluate(engine, pos);

            if self.maximizing {
                if val > self.best_value {
                    self.best_value = val;
                    self.trace = path_to_string(&self.path);
                    self.best_move = self.path.front().cloned();
                }
            } else if val < self.best_value {
                self.best_value = val;
                self.trace = path_to_string(&self.path);
                self.best_move = self.path.front().cloned();
            }

            return self.evaluate(engine, pos);
        }

        let moves: Vec<Move> = engine.gen.generate_legal_moves(pos).collect();

        if maximizing_player {
            let mut value = f64::MIN;

            if !moves.is_empty() {
                for mv in moves {
                    let mut new_pos = pos.clone();
                    new_pos.make_move(&mv).unwrap();
                    self.path.push_back(mv);
                    value = f64::max(value, self.minmax(engine, depth - 1, &new_pos, false));
                    self.path.pop_back();
                }
            }

            value
        } else {
            let mut value = f64::MAX;

            if !moves.is_empty() {
                for mv in moves {
                    let mut new_pos = pos.clone();
                    new_pos.make_move(&mv).unwrap();
                    self.path.push_back(mv);
                    value = f64::min(value, self.minmax(engine, depth - 1, &new_pos, true));
                    self.path.pop_back();
                }
            }

            value
        }
    }

    fn evaluate(&mut self, engine: &mut Engine, pos: &Position) -> f64 {
        engine.cnt += 1;
        let mut val = 0.0;
        for sq in 0..64 {
            match pos.piece_at(&Square::try_from(sq).unwrap()) {
                Some((piece, color)) => val += PIECE_VALUES[piece as usize] * (if color == Color::White { 1.0 } else { -1.0 }),
                None => {}
            }
        }

        val
    }
}

impl SearchStrategy for DummySearch {
    fn evaluate(&mut self, engine: &mut Engine, pos: &Position) -> f64 {
        DummySearch::evaluate(self, engine, pos)
    }

    fn search(&mut self, engine: &mut Engine, depth: u8, pos: &Position) -> (Move, f64) {
        let maximizing_player = pos.turn == Color::White;
        let val = DummySearch::minmax(self, engine, depth, pos, maximizing_player);
        println!("Trace: {}", self.trace);
        (self.best_move.clone().unwrap(), val)
    }
}
