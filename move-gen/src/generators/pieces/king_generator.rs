use sdk::{
    bitboard::Bitboard,
    position::{Piece, Position},
};

use crate::{
    generators::movegen::MoveGen,
    r#move::r#move::{Move, MoveKind},
};

use super::simple_move_generator::SimpleMoveGenerator;

pub trait KingMoveGenerator {
    fn generate_king_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_>;
}

impl KingMoveGenerator for MoveGen {
    fn generate_king_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        _enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> Box<dyn Iterator<Item = Move> + '_> {
        let color = pos.turn;
        let bb = pos.pieces[color as usize][Piece::King as usize] & !pinned_pieces;

        let iter = bb.into_iter().flat_map(move |from_square| {
            let attacks = self.king_attacks(from_square) & !friendly_occ;

            attacks.into_iter().filter_map(move |target_square| {
                // Move is illegal if it leaves the king in check
                if !self.attacks_to_sq(pos, target_square).is_empty() {
                    return None;
                }

                let captured_piece = pos.piece_at(&target_square).map(|piece| piece.0);
                let kind = if captured_piece.is_some() {
                    MoveKind::Capture
                } else {
                    MoveKind::Quiet
                };

                Some(Move::new(from_square, target_square, None, &kind))
            })
        });

        Box::new(iter)
    }
}
