use rand::Rng;
use sdk::{
    bitboard::{Bitboard, Direction},
    lookup::{in_between, sliders::Slider},
    position::{Color, Piece, Position},
    square::{Rank, Square},
};

use crate::{
    lookup::{load_lookup_tables, LookupTables, MagicEntry},
    r#move::r#move::{MakeMove, Move, MoveKind},
    xray::XRayGenerator,
};

use super::pieces::{
    king_generator::KingMoveGenerator, knight_generator::KnightMoveGenerator,
    pawn_generator::PawnMoveGenerator, simple_move_generator::SimpleMoveGenerator,
    slider_generator::SliderMoveGenerator,
};

pub struct MoveGen {
    pub lookups: LookupTables,
}

impl MoveGen {
    pub fn new() -> Self {
        let lookup_tables = load_lookup_tables().expect("Couldn't load lookup tables");
        Self {
            lookups: lookup_tables,
        }
    }

    pub fn pinned_pieces(&self, pos: &Position) -> Bitboard {
        let king_square = pos.pieces[pos.turn as usize][Piece::King as usize].msb();

        let own_pieces = pos.occupation(&pos.turn);

        let op_rq = pos.pieces[pos.enemy() as usize][Piece::Rook as usize]
            | pos.pieces[pos.enemy() as usize][Piece::Queen as usize];

        let op_bq = pos.pieces[pos.enemy() as usize][Piece::Bishop as usize]
            | pos.pieces[pos.enemy() as usize][Piece::Queen as usize];

        let mut pinned_pieces = Bitboard(0);

        for sq in self.xray_rook_attacks(pos, king_square) & op_rq {
            pinned_pieces |=
                self.lookups.in_between[sq as usize][king_square as usize] & own_pieces;
        }

        for sq in self.xray_bishop_attacks(pos, king_square) & op_bq {
            pinned_pieces |=
                self.lookups.in_between[sq as usize][king_square as usize] & own_pieces;
        }

        pinned_pieces
    }

    pub fn attacks_to_sq(&self, position: &Position, sq: Square) -> Bitboard {
        info!("Attacks to square: {}", sq);
        let enemy_color = position.enemy();
        let our_color = position.turn;
        let enemy_pieces = position.pieces[enemy_color as usize];

        let knight_attacks = self.knight_attacks(sq) & enemy_pieces[Piece::Knight as usize];
        let pawn_attacks = self.pawn_attacks(our_color, sq) & enemy_pieces[Piece::Pawn as usize];
        dbg!(pawn_attacks);
        let rook_attacks = self.rook_moves(sq, position.occupied)
            & (enemy_pieces[Piece::Rook as usize] | enemy_pieces[Piece::Queen as usize]);
        let bishop_attacks = self.bishop_moves(sq, position.occupied)
            & (enemy_pieces[Piece::Bishop as usize] | enemy_pieces[Piece::Queen as usize]);

        knight_attacks | pawn_attacks | rook_attacks | bishop_attacks
    }

    pub fn is_check(&self, position: &Position) -> bool {
        !self
            .attacks_to_sq(
                position,
                position.pieces[position.turn as usize][Piece::King as usize].msb(),
            )
            .is_empty()
    }

    pub fn generate_legal_moves<'a>(
        &'a self,
        pos: &'a Position,
    ) -> Box<dyn Iterator<Item = Move> + 'a> {
        let friendly_occ = pos.occupation(&pos.turn);
        let enemy_occ = pos.occupation(&pos.enemy());
        let pinned_pieces = self.pinned_pieces(pos);

        let pawn_quiet_moves =
            self.generate_pawn_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let pawn_capturing_moves =
            self.generate_pawn_attacks(pos, friendly_occ, enemy_occ, pinned_pieces);
        let knight_moves = self.generate_knight_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let slider_moves = self.generate_slider_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let king_moves = self.generate_king_moves(pos, friendly_occ, enemy_occ, pinned_pieces);

        Box::new(
            pawn_quiet_moves
                .chain(pawn_capturing_moves)
                .chain(knight_moves)
                .chain(slider_moves)
                .chain(king_moves),
        )
    }
}

pub enum PositionState {
    Checkmate,
    Stalemate,
    InProgress,
}

impl Default for MoveGen {
    fn default() -> Self {
        Self::new()
    }
}
