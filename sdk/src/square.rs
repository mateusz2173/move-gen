use enum_index::{EnumIndex, IndexEnum};

use crate::bitboard::Bitboard;

#[derive(EnumIndex, IndexEnum, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    #[must_use]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(0x0101_0101_0101_0101 << self as u8)
    }
}

impl Rank {
    #[must_use]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(0xFF << (self as u8 * 8))
    }
}

#[derive(EnumIndex, IndexEnum, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

#[derive(EnumIndex, IndexEnum, Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    #[must_use]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(1 << self as usize)
    }

    #[must_use]
    pub fn rank(&self) -> Rank {
        Rank::index_enum(self.enum_index() >> 3).unwrap()
    }

    #[must_use]
    pub fn file(&self) -> File {
        File::index_enum(self.enum_index() & 0b00_0111).unwrap()
    }

    #[must_use]
    pub fn offset(&self, rank_offset: i8, file_offset: i8) -> Option<Square> {
        let (file, rank): (File, Rank) = self.into();

        let file = File::index_enum((file as i8 + file_offset).try_into().ok()?)?;
        let rank = Rank::index_enum((rank as i8 + rank_offset).try_into().ok()?)?;

        Some((file, rank).into())
    }

    #[must_use]
    pub fn iter() -> SqIter {
        SqIter { idx: 0 }
    }

    #[must_use]
    pub fn coords_str(&self) -> String {
        #[allow(clippy::cast_possible_truncation)]
        let square = self.enum_index() as u8;
        let file = (square % 8) + b'a';
        let rank = (square / 8) + b'1';
        let file_str = char::from(file);
        let rank_str = char::from(rank);
        format!("{file_str}{rank_str}")
    }
}

impl From<(File, Rank)> for Square {
    fn from((file, rank): (File, Rank)) -> Self {
        let file = file as u8;
        let rank = rank as u8;

        Square::index_enum((rank * 8 + file).into()).unwrap()
    }
}

impl From<Square> for (File, Rank) {
    fn from(value: Square) -> Self {
        let file = value.file();
        let rank = value.rank();

        (file, rank)
    }
}

impl From<&Square> for (File, Rank) {
    fn from(value: &Square) -> Self {
        let file = value.file();
        let rank = value.rank();

        (file, rank)
    }
}

pub struct SqIter {
    idx: u8,
}

impl Iterator for SqIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 64 {
            return None;
        }

        let sq = Square::index_enum(self.idx.into()).unwrap();
        self.idx += 1;
        Some(sq)
    }
}
