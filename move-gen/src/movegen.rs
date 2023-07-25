use sdk::{
    bitboard::Bitboard,
    position::{Color, Piece, Position},
    square::Square,
};

use crate::lookup::{load_lookup_tables, LookupTables, MagicEntry};

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

    pub fn pawn_moves(&self, color: Color, square: Square) -> Bitboard {
        self.lookups.pawn_moves[color as usize][square as usize]
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

    pub fn pinned_pieces(&self, _color: Color) -> Bitboard {
        todo!()
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
