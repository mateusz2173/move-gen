use std::collections::HashSet;

use sdk::{
    position::{Piece, Position},
    square::Square,
};

use super::r#move::Move;

pub struct MoveSet<'a> {
    position: &'a Position,
    moves: HashSet<Move>,
}

impl<'a> MoveSet<'a> {
    pub fn new(pos: &'a Position, moves: impl Iterator<Item = Move> + 'a) -> Self {
        Self {
            position: pos,
            moves: moves.collect(),
        }
    }

    pub fn moves(self) -> HashSet<Move> {
        self.moves
    }

    pub fn chess_notation_moves(self) -> HashSet<String> {
        self.moves
            .iter()
            .map(|mv| self.to_algebraic_notation(mv))
            .collect()
    }

    pub fn position(&self) -> &Position {
        self.position
    }

    pub fn find_ambiguous_piece(&self, mv: &Move) -> Option<Square> {
        self.moves
            .iter()
            .find(|other| {
                other.from() != mv.from()
                    && other.to() == mv.to()
                    && self.position.piece_at(&other.from()) == self.position.piece_at(&mv.from())
            })
            .map(|other| other.from())
    }

    pub fn to_algebraic_notation(&self, mv: &Move) -> String {
        let (piece, _) = self
            .position
            .piece_at(&mv.from())
            .expect("No piece at from square.");

        let piece_char = piece.to_string().to_uppercase();

        let is_pawn = piece == Piece::Pawn;

        let ambiguous = self.find_ambiguous_piece(mv);

        let from_file = if ambiguous.is_some_and(|sq| sq.file() != mv.from().file()) {
            mv.from().file().to_string()
        } else {
            "".to_string()
        };

        let from_rank =
            if ambiguous.is_some_and(|sq| sq.rank() != mv.from().rank()) && from_file.is_empty() {
                mv.from().rank().to_string()
            } else {
                "".to_string()
            };

        let from_square = if ambiguous.is_some_and(|sq| sq != mv.from())
            && from_rank.is_empty()
            && from_file.is_empty()
        {
            mv.from().to_string()
        } else {
            "".to_string()
        };

        let to_square = mv.to().to_string();
        let promoted_to = mv
            .promotion()
            .map(|piece| piece.to_string())
            .unwrap_or("".to_string());

        let capture_indicator = if mv.is_capture() { "x" } else { "" };

        if is_pawn && mv.is_capture() {
            format!("{from_file}{from_rank}x{to_square}{promoted_to}")
        } else if is_pawn {
            format!("{to_square}{promoted_to}")
        } else {
            format!("{piece_char}{from_file}{from_rank}{from_square}{capture_indicator}{to_square}")
        }
    }
}
