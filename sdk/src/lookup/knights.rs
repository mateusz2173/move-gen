use crate::{bitboard::Bitboard, square::{Square, File}};

const FILE_A: Bitboard = File::A.bitboard();
const FILE_B: Bitboard = File::B.bitboard();
const FILE_G: Bitboard = File::G.bitboard();
const FILE_H: Bitboard = File::H.bitboard();
const FILE_AB: Bitboard = Bitboard(FILE_A.0 | FILE_B.0);
const FILE_GH: Bitboard = Bitboard(FILE_G.0 | FILE_H.0);

#[must_use]
pub fn gen_knight_attacks() -> [Bitboard; 64] {
    let mut knight_attacks = [Bitboard(0); 64];
    for sq in Square::iter() {
        knight_attacks[sq as usize] = mask_knight_attacks(sq);
    }
    knight_attacks
}

#[must_use]
pub fn gen_knight_lookups() -> String {
    let knight_attacks = gen_knight_attacks();

    let mut knight_attacks_str = String::from("pub const KNIGHT_ATTACKS: [Bitboard; 64] = [\n");

    for sq in Square::iter() {
        knight_attacks_str.push_str(&format!("    Bitboard({}),\n", knight_attacks[sq as usize].0));
    }
    knight_attacks_str.push_str("];\n");

    knight_attacks_str
}

fn mask_knight_attacks(square: Square) -> Bitboard {
    let knight_square: Bitboard = square.into();
    let mut knight_bb: Bitboard = Bitboard(0);

    knight_bb |= (knight_square << 17) & !FILE_A;
    knight_bb |= (knight_square << 15) & !FILE_H;
    knight_bb |= (knight_square << 10) & !FILE_AB;
    knight_bb |= (knight_square << 6) & !FILE_GH;
    knight_bb |= (knight_square >> 17) & !FILE_H;
    knight_bb |= (knight_square >> 15) & !FILE_A;
    knight_bb |= (knight_square >> 10) & !FILE_GH;
    knight_bb |= (knight_square >> 6) & !FILE_AB;

    knight_bb & !Into::<Bitboard>::into(square)
}
