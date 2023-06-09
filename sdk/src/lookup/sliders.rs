use crate::{bitboard::Bitboard, square::Square};

const BISHOP_OFFSETS: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
const ROOK_OFFSETS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Clone, Copy)]
pub enum Slider {
    Bishop,
    Rook,
    Queen,
}

impl Slider {
    #[must_use]
    pub fn relevant_blockers(&self, sq: Square) -> Bitboard {
        let offsets = match self {
            Slider::Bishop => BISHOP_OFFSETS,
            Slider::Rook => ROOK_OFFSETS,
            Slider::Queen => unreachable!(),
        };

        let mut blockers = Bitboard::empty();

        for (file_offset, rank_offset) in offsets {
            let mut current = sq;
            while let Some(next_sq) = current.offset(rank_offset, file_offset) {
                current = next_sq;
                if next_sq.offset(rank_offset, file_offset).is_some() {
                    blockers |= next_sq.bitboard();
                }
            }
        }

        blockers
    }

    #[must_use]
    pub fn moves(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        mask_slider_attacks_occ(self, blockers, sq)
    }

    #[must_use]
    pub const fn index_bits(&self) -> usize {
        match self {
            Slider::Bishop => 11,
            Slider::Rook => 13,
            Slider::Queen => unreachable!(),
        }
    }
}

fn mask_slider_attacks_occ(slider: &Slider, occ: Bitboard, sq: Square) -> Bitboard {
    let mut attacks = Bitboard::empty();

    let offsets = match slider {
        Slider::Bishop => BISHOP_OFFSETS,
        Slider::Rook => ROOK_OFFSETS,
        Slider::Queen => unreachable!(),
    };

    for (file_offset, rank_offset) in offsets {
        let mut current = sq;
        while let Some(next_sq) = current.offset(rank_offset, file_offset) {
            attacks |= next_sq.bitboard();
            current = next_sq;

            if occ.has(current) {
                break;
            }
        }
    }

    attacks
}
