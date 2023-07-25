use crate::bitboard::Bitboard;
use crate::bitboard::Direction;
use crate::bitboard::EMPTY;
use crate::position::Color;
use crate::square::Rank;
use crate::square::Square;

#[must_use]
pub fn gen_pawn_moves() -> [[Bitboard; 64]; 2] {
    let mut pawn_moves = [[EMPTY; 64]; 2];
    for color in Color::iter() {
        let direction = match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        };

        let double_push_rank = match color {
            Color::White => Rank::R2,
            Color::Black => Rank::R7,
        };

        for sq in Square::iter() {
            pawn_moves[color as usize][sq as usize] |= sq.bitboard().shift(&direction);
            if sq.rank() == double_push_rank {
                pawn_moves[color as usize][sq as usize] |=
                    sq.bitboard().shift(&direction).shift(&direction);
            }
        }
    }
    pawn_moves
}

#[must_use]
pub fn gen_pawn_attacks() -> [[Bitboard; 64]; 2] {
    let mut pawn_attacks = [[EMPTY; 64]; 2];
    for color in Color::iter() {
        let (first_dir, second_dir) = match color {
            Color::White => (Direction::NorthEast, Direction::NorthWest),
            Color::Black => (Direction::SouthEast, Direction::SouthWest),
        };

        for sq in Square::iter() {
            pawn_attacks[color as usize][sq as usize] |= sq.bitboard().shift(&first_dir);
            pawn_attacks[color as usize][sq as usize] |= sq.bitboard().shift(&second_dir);
        }
    }

    pawn_attacks
}
