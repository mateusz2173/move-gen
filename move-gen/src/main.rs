pub mod r#move;

include!(concat!(env!("OUT_DIR"), "/tables.rs"));

use sdk::{lookup::sliders::Slider, square::Square};

fn main() {
    let sq = Square::B2;
    let occ = Square::B5.bitboard() | Square::C6.bitboard() | Square::A6.bitboard();

    dbg!(slider_attacks(Slider::Queen, sq, occ));
}

fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
    let blockers = blockers & entry.mask;
    let hash = blockers.0.wrapping_mul(entry.magic);
    (hash >> (64 - entry.index_bits)) as usize
}

pub fn slider_attacks(slider: Slider, sq: Square, occ: Bitboard) -> Bitboard {
    match slider {
        Slider::Rook => {
            let entry = &ROOK_MAGICS[sq as usize];
            let index = magic_index(entry, occ);
            ROOK_MOVES[sq as usize][index]
        }
        Slider::Bishop => {
            let entry = &BISHOP_MAGICS[sq as usize];
            let index = magic_index(entry, occ);
            BISHOP_MOVES[sq as usize][index]
        }
        _ => {
            dbg!("Queen");
            let rook = slider_attacks(Slider::Rook, sq, occ);
            let bishop = slider_attacks(Slider::Bishop, sq, occ);
            dbg!(&rook);
            dbg!(&bishop);
            rook | bishop
        }
    }
}
