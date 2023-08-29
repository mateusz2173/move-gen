use sdk::{
    position::{Piece, Position},
    square::Square,
};

use crate::{generators::movegen::MoveGen, r#move::r#move::Move};

pub trait ChessNotation {
    fn find_ambiguous_piece(&self, pos: &Position, mv: &Move) -> Option<Square>;
    fn to_algebraic_notation(&self, pos: &Position, mv: &Move) -> String;
}

impl ChessNotation for MoveGen {
    fn find_ambiguous_piece(&self, pos: &Position, mv: &Move) -> Option<Square> {
        let from_square = mv.from();
        let from_piece = pos.piece_at(&from_square);

        self.attacks_to_sq(pos, mv.to())
            .into_iter()
            .find(|sq| (sq != &from_square) && pos.piece_at(sq) == from_piece)
    }

    fn to_algebraic_notation(&self, pos: &Position, mv: &Move) -> String {
        let (piece, _) = pos.piece_at(&mv.from()).expect("No piece at from square.");

        let piece_char = piece.to_string().to_uppercase();

        let is_pawn = piece == Piece::Pawn;

        let ambiguous = self.find_ambiguous_piece(pos, mv);

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
