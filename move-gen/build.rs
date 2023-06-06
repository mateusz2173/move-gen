use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use sdk::lookup::sliders::Slider;

fn main() {
    generate_lookup_tables().unwrap();
    println!("cargo:rerun-if-changed=tables.rs");
}

fn generate_lookup_tables() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let dest_path = std::path::Path::new(&out_dir).join("tables.rs");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest_path)?;

    let (pawn_moves, pawn_attacks) = sdk::lookup::pawns::gen_pawn_lookups();
    let knight_moves = sdk::lookup::knights::gen_knight_lookups();
    let king_attacks = sdk::lookup::king::gen_king_lookups();
    let rook_magics = load_magics(Slider::Rook, "../magic/rook_magics.bin".into())?;
    let bishop_magics = load_magics(Slider::Bishop, "../magic/bishop_magics.bin".into())?;
    let magic_entry_struct = gen_magic_entry_struct();
    file.write_all(b"use sdk::bitboard::Bitboard;\n")?;
    file.write_all(format!("{pawn_moves}\n{pawn_attacks}\n").as_bytes())?;
    file.write_all(format!("{knight_moves}\n").as_bytes())?;
    file.write_all(format!("{king_attacks}\n").as_bytes())?;
    file.write_all(format!("{rook_magics}\n").as_bytes())?;
    file.write_all(format!("{bishop_magics}\n").as_bytes())?;
    file.write_all(format!("{magic_entry_struct}\n").as_bytes())?;

    Ok(())
}

fn load_magics(slider: Slider, path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Failed to open file");
    let mut magics: [(u64, u64, u8); 64] = [(0, 0, 0); 64];

    for magic_entry in magics.iter_mut() {
        let mut mask_bytes = [0; 8];
        file.read_exact(&mut mask_bytes).unwrap();
        let mask = u64::from_be_bytes(mask_bytes);

        let mut magic_bytes = [0; 8];
        file.read_exact(&mut magic_bytes).unwrap();
        let magic = u64::from_be_bytes(magic_bytes);

        let mut index_bits_bytes = [0; 1];
        file.read_exact(&mut index_bits_bytes).unwrap();
        let index_bits = u8::from_be_bytes(index_bits_bytes);

        *magic_entry = (mask, magic, index_bits);
    }

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    let moves = buffer
        .chunks(8)
        .map(|chunk| {
            let mut bytes = [0; 8];
            bytes.copy_from_slice(chunk);
            u64::from_be_bytes(bytes)
        })
        .collect::<Vec<u64>>();

    let magics_name = match slider {
        Slider::Bishop => "BISHOP_MAGICS",
        Slider::Rook => "ROOK_MAGICS",
        Slider::Queen => unreachable!(),
    };

    let slider_moves = match slider {
        Slider::Bishop => "BISHOP_MOVES",
        Slider::Rook => "ROOK_MOVES",
        Slider::Queen => unreachable!(),
    };

    let mut result = String::new();
    result.push_str(&format!("pub const {magics_name}: [MagicEntry; 64] = [\n"));
    for magic in magics {
        let (mask, magic, index_bits) = magic;
        result.push_str(&format!(
            "    MagicEntry {{ mask: Bitboard({mask}), magic: {magic}, index_bits: {index_bits} }},\n",
        ));
    }
    result.push_str("];\n");

    let chunk_size = 1 << slider.index_bits();
    result.push_str(&format!("pub const {slider_moves}: [[Bitboard; {chunk_size}]; 64] = [\n"));
    for move_chunk in moves.chunks(chunk_size) {

        result.push('[');

        for move_bb in move_chunk {
            result.push_str(&format!("    Bitboard({}),\n", move_bb));
        }
        
        result.push_str("],\n");
    }
    result.push_str("];\n");
    Ok(result)
}

pub fn gen_magic_entry_struct() -> String {
    let mut result = String::new();
    result.push_str("pub struct MagicEntry {\n");
    result.push_str("    pub mask: Bitboard,\n");
    result.push_str("    pub magic: u64,\n");
    result.push_str("    pub index_bits: u8,\n");
    result.push_str("}\n");
    result
}
