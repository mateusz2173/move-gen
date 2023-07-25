use std::io::Read;

use sdk::{
    bitboard::Bitboard,
    lookup::{
        king::gen_king_attacks,
        knights::gen_knight_attacks,
        pawns::{gen_pawn_attacks, gen_pawn_moves},
    },
};

use anyhow::Result;

pub struct LookupTables {
    pub rook_magics: [MagicEntry; 64],
    pub rook_moves: [[Bitboard; 1 << 12]; 64],
    pub bishop_magics: [MagicEntry; 64],
    pub bishop_moves: [[Bitboard; 1 << 9]; 64],
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
    pub pawn_attacks: [[Bitboard; 64]; 2],
    pub pawn_moves: [[Bitboard; 64]; 2],
}

#[derive(Clone, Copy)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: u64,
    pub index_bits: u8,
}

pub fn load_lookup_tables() -> Result<LookupTables> {
    let (rook_magics, rook_moves) = load_rook_magics()
        .map_err(|err| anyhow::format_err!("Couldn't load rook magics: {err:?}"))?;
    let (bishop_magics, bishop_moves) = load_bishop_magics()
        .map_err(|err| anyhow::format_err!("Couldn't load bishop magics: {err:?}"))?;
    let pawn_moves = gen_pawn_moves();
    let pawn_attacks = gen_pawn_attacks();
    let knight_attacks = gen_knight_attacks();
    let king_attacks = gen_king_attacks();

    Ok(LookupTables {
        rook_magics,
        rook_moves,
        bishop_magics,
        bishop_moves,
        knight_attacks,
        king_attacks,
        pawn_attacks,
        pawn_moves,
    })
}

pub fn load_rook_magics() -> Result<([MagicEntry; 64], [[Bitboard; 1 << 12]; 64])> {
    let mut magics = [MagicEntry {
        mask: Bitboard(0),
        magic: 0,
        index_bits: 0,
    }; 64];

    let mut file = std::fs::File::open("rook_magics.bin")?;

    for magic in magics.iter_mut() {
        let mut mask_bytes = [0u8; 8];
        file.read_exact(&mut mask_bytes)?;
        let mask = Bitboard(u64::from_be_bytes(mask_bytes));

        let mut magic_bytes = [0u8; 8];
        file.read_exact(&mut magic_bytes)?;
        let magic_number = u64::from_be_bytes(magic_bytes);

        let mut index_bits_bytes = [0u8; 1];
        file.read_exact(&mut index_bits_bytes)?;
        let index_bits = u8::from_be_bytes(index_bits_bytes);

        magic.mask = mask;
        magic.magic = magic_number;
        magic.index_bits = index_bits;
    }

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut moves = [[Bitboard(0); 1 << 12]; 64];

    let size = 1 << 12;

    for (idx, elem) in buffer.chunks_exact(8).enumerate() {
        moves[idx / size][idx % size] = Bitboard(u64::from_be_bytes(elem.try_into().unwrap()));
    }

    Ok((magics, moves))
}

pub fn load_bishop_magics() -> Result<([MagicEntry; 64], [[Bitboard; 1 << 9]; 64])> {
    let mut magics = [MagicEntry {
        mask: Bitboard(0),
        magic: 0,
        index_bits: 0,
    }; 64];

    let mut file = std::fs::File::open("bishop_magics.bin")?;

    for magic in magics.iter_mut() {
        let mut mask_bytes = [0u8; 8];
        file.read_exact(&mut mask_bytes)?;
        let mask = Bitboard(u64::from_be_bytes(mask_bytes));

        let mut magic_bytes = [0u8; 8];
        file.read_exact(&mut magic_bytes)?;
        let magic_number = u64::from_be_bytes(magic_bytes);

        let mut index_bits_bytes = [0u8; 1];
        file.read_exact(&mut index_bits_bytes)?;
        let index_bits = u8::from_be_bytes(index_bits_bytes);

        magic.mask = mask;
        magic.magic = magic_number;
        magic.index_bits = index_bits;
    }

    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    let size = 1 << 9;

    let mut moves = [[Bitboard(0); 1 << 9]; 64];

    for (idx, elem) in buffer.chunks_exact(8).enumerate() {
        moves[idx / size][idx % size] = Bitboard(u64::from_be_bytes(elem.try_into().unwrap()));
    }

    Ok((magics, moves))
}
