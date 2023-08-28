use sdk::{bitboard::Bitboard, position::Position, square::Square};

use crate::generators::{movegen::MoveGen, pieces::simple_move_generator::SimpleMoveGenerator};

pub trait XRayGenerator {
    fn xray_rook_attacks(&self, pos: &Position, square: Square) -> Bitboard;
    fn xray_bishop_attacks(&self, pos: &Position, square: Square) -> Bitboard;
}

impl XRayGenerator for MoveGen {
    fn xray_rook_attacks(&self, pos: &Position, square: Square) -> Bitboard {
        let occ = pos.occupied;
        let attacks = self.rook_moves(square, occ);
        let blockers = occ & attacks;

        attacks ^ self.rook_moves(square, occ ^ blockers)
    }

    fn xray_bishop_attacks(&self, pos: &Position, square: Square) -> Bitboard {
        let occ = pos.occupied;
        let attacks = self.bishop_moves(square, occ);
        let blockers = occ & attacks;

        attacks ^ self.bishop_moves(square, occ ^ blockers)
    }
}
