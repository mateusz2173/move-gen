use rand::Rng;
use sdk::{
    bitboard::{Bitboard, Direction},
    lookup::sliders::Slider,
    position::{Color, Piece, Position},
    square::{Rank, Square},
};

use crate::{
    lookup::{load_lookup_tables, LookupTables, MagicEntry},
    r#move::{MakeMove, Move},
};

pub struct MoveGen {
    pub lookups: LookupTables,
    pub position: Position,
}

impl MoveGen {
    pub fn new() -> Self {
        let lookup_tables = load_lookup_tables().expect("Couldn't load lookup tables");
        Self {
            lookups: lookup_tables,
            position: Position::default(),
        }
    }

    pub fn knight_attacks(&self, square: Square) -> Bitboard {
        self.lookups.knight_attacks[square as usize]
    }

    pub fn king_attacks(&self, square: Square) -> Bitboard {
        self.lookups.king_attacks[square as usize]
    }

    pub fn pawn_attacks(&self, color: Color, square: Square) -> Bitboard {
        self.lookups.pawn_attacks[color as usize][square as usize]
    }

    pub fn pawn_single_moves(&self, color: Color, square: Square) -> Bitboard {
        self.lookups.pawn_single_moves[color as usize][square as usize]
    }

    pub fn pawn_double_moves(&self, color: Color, square: Square) -> Bitboard {
        self.lookups.pawn_double_moves[color as usize][square as usize]
    }

    pub fn rook_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let entry = self.lookups.rook_magics[square as usize];
        self.lookups.rook_moves[square as usize][magic_index(&entry, blockers)]
    }

    pub fn bishop_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        let entry = self.lookups.bishop_magics[square as usize];
        self.lookups.bishop_moves[square as usize][magic_index(&entry, blockers)]
    }

    pub fn queen_moves(&self, square: Square, blockers: Bitboard) -> Bitboard {
        self.rook_moves(square, blockers) | self.bishop_moves(square, blockers)
    }

    pub fn slider_moves(&self, slider: Slider, square: Square, blockers: Bitboard) -> Bitboard {
        match slider {
            Slider::Rook => self.rook_moves(square, blockers),
            Slider::Bishop => self.bishop_moves(square, blockers),
            Slider::Queen => self.queen_moves(square, blockers),
        }
    }

    pub fn pinned_pieces(&self, _color: Color) -> Bitboard {
        Bitboard::empty()
    }

    pub fn is_check(&self, position: &Position) -> bool {
        let king_square = position.pieces[position.turn as usize][Piece::King as usize].msb();
        let enemy_color = position.enemy();
        let enemy_pieces = position.pieces[enemy_color as usize];

        let knight_attacks =
            self.knight_attacks(king_square) & enemy_pieces[Piece::Knight as usize];
        let pawn_attacks =
            self.pawn_attacks(enemy_color, king_square) & enemy_pieces[Piece::Pawn as usize];
        let rook_attacks = self.rook_moves(king_square, position.occupation(&enemy_color))
            & (enemy_pieces[Piece::Rook as usize] | enemy_pieces[Piece::Queen as usize]);
        let bishop_attacks = self.bishop_moves(king_square, position.occupation(&enemy_color))
            & (enemy_pieces[Piece::Bishop as usize] | enemy_pieces[Piece::Queen as usize]);

        !(knight_attacks | pawn_attacks | rook_attacks | bishop_attacks).is_empty()
    }

    pub fn generate_pawn_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> impl Iterator<Item = Move> + '_ {
        let color = pos.turn;
        let bb = pos.pieces[color as usize][Piece::Pawn as usize] & !pinned_pieces;
        let forward = match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        };
        let blockers = friendly_occ | enemy_occ;
        let double_push_blockers = blockers | blockers.shift(&forward);

        bb.into_iter().flat_map(move |from_square| {
            let single_moves = self.pawn_single_moves(color, from_square) & !blockers;
            let double_moves = self.pawn_double_moves(color, from_square) & !double_push_blockers;

            single_moves
                .into_iter()
                .chain(double_moves.into_iter())
                .flat_map(move |target_square| {
                    let promotion_rank = match color {
                        Color::White => Rank::R8,
                        Color::Black => Rank::R1,
                    };

                    if target_square.rank() == promotion_rank {
                        vec![
                            Move::new(from_square, target_square, Some(Piece::Queen)),
                            Move::new(from_square, target_square, Some(Piece::Rook)),
                            Move::new(from_square, target_square, Some(Piece::Bishop)),
                            Move::new(from_square, target_square, Some(Piece::Knight)),
                        ]
                    } else {
                        vec![Move::new(from_square, target_square, None)]
                    }
                    .into_iter()
                })
        })
    }

    pub fn generate_pawn_attacks<'a>(
        &'a self,
        pos: &'a Position,
        _friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> impl Iterator<Item = Move> + '_ {
        let color = pos.turn;
        let bb = pos.pieces[color as usize][Piece::Pawn as usize] & !pinned_pieces;

        bb.into_iter().flat_map(move |from_square| {
            let attacks = self.pawn_attacks(color, from_square) & enemy_occ;

            attacks.into_iter().flat_map(move |target_square| {
                let captured_piece = pos.piece_at(&target_square).map(|piece| piece.0);
                let promotion_rank = match color {
                    Color::White => Rank::R8,
                    Color::Black => Rank::R1,
                };

                if target_square.rank() == promotion_rank {
                    vec![
                        Move::new(from_square, target_square, Some(Piece::Queen)),
                        Move::new(from_square, target_square, Some(Piece::Rook)),
                        Move::new(from_square, target_square, Some(Piece::Bishop)),
                        Move::new(from_square, target_square, Some(Piece::Knight)),
                    ]
                } else {
                    vec![Move::new(from_square, target_square, None)]
                }
                .into_iter()
            })
        })
    }

    pub fn generate_knight_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        _enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> impl Iterator<Item = Move> + '_ {
        let color = pos.turn;
        let bb = pos.pieces[color as usize][Piece::Knight as usize] & !pinned_pieces;

        bb.into_iter().flat_map(move |from_square| {
            let attacks = self.knight_attacks(from_square) & !friendly_occ;

            attacks.into_iter().map(move |target_square| {
                let captured_piece = pos.piece_at(&target_square).map(|piece| piece.0);

                Move::new(from_square, target_square, None)
            })
        })
    }

    pub fn generate_slider_moves<'a>(
        &'a self,
        pos: &'a Position,
        friendly_occ: Bitboard,
        enemy_occ: Bitboard,
        pinned_pieces: Bitboard,
    ) -> impl Iterator<Item = Move> + '_ {
        [Slider::Bishop, Slider::Rook, Slider::Queen].into_iter().flat_map(move |slider| {
            let piece: Piece = slider.into();

            let bb = pos.pieces[pos.turn as usize][piece as usize] & !pinned_pieces;
            let blockers = friendly_occ | enemy_occ;

            bb.into_iter().flat_map(move |from_square| {
                let attacks = self.slider_moves(slider, from_square, blockers) & !friendly_occ;

                attacks.into_iter().map(move |target_square| {
                    let captured_piece = pos.piece_at(&target_square).map(|piece| piece.0);

                    Move::new(from_square, target_square, None)
                })
            })
        })
    }

    pub fn generate_legal_moves<'a>(
        &'a self,
        pos: &'a Position,
    ) -> impl Iterator<Item = Move> + '_ {
        let friendly_occ = pos.occupation(&pos.turn);
        let enemy_occ = pos.occupation(&pos.enemy());
        let pinned_pieces = self.pinned_pieces(pos.turn);

        let pawn_quiet_moves =
            self.generate_pawn_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let pawn_capturing_moves =
            self.generate_pawn_attacks(pos, friendly_occ, enemy_occ, pinned_pieces);
        let knight_moves = self.generate_knight_moves(pos, friendly_occ, enemy_occ, pinned_pieces);
        let slider_moves = self.generate_slider_moves(pos, friendly_occ, enemy_occ, pinned_pieces);

        pawn_quiet_moves
            .chain(pawn_capturing_moves)
            .chain(knight_moves)
            .chain(slider_moves)
    }

    pub(crate) fn make_random_move(&self, pos: &mut Position) -> Move {
        let moves = self.generate_legal_moves(pos).collect::<Vec<_>>();
        let index = rand::thread_rng().gen_range(0..moves.len());
        let mv = &moves[index];

        let _ = pos.make_move(mv);

        mv.clone()
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

fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
    let blockers = blockers & entry.mask;
    let hash = blockers.0.wrapping_mul(entry.magic);
    (hash >> (64 - entry.index_bits)) as usize
}
