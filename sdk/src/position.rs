use std::fmt::Display;

use anyhow::anyhow;
use enum_index::IndexEnum;

use crate::{bitboard::Bitboard, fen::Fen, square::Square};

#[derive(Debug, Clone)]
pub struct Position {
    pub pieces: [[Bitboard; 6]; 2],
    pub occupied: Bitboard,
    pub turn: Color,
    pub castling: u8,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Castling {
    WhiteKingside = 1,
    WhiteQueenside = 2,
    BlackKingside = 4,
    BlackQueenside = 8,
}

impl Castling {
    #[must_use]
    pub fn from_u8(value: u8) -> Vec<Castling> {
        let mut castling = Vec::new();
        if value & 1 != 0 {
            castling.push(Castling::WhiteKingside);
        }
        if value & 2 != 0 {
            castling.push(Castling::WhiteQueenside);
        }
        if value & 4 != 0 {
            castling.push(Castling::BlackKingside);
        }
        if value & 8 != 0 {
            castling.push(Castling::BlackQueenside);
        }
        castling
    }

    #[must_use]
    pub fn to_u8(values: Vec<Castling>) -> u8 {
        let mut value = 0;
        for castling in values {
            value |= match castling {
                Castling::WhiteKingside => 1,
                Castling::WhiteQueenside => 2,
                Castling::BlackKingside => 4,
                Castling::BlackQueenside => 8,
            };
        }
        value
    }
}

impl Position {
    #[must_use]
    pub fn occupation(&self, color: &Color) -> Bitboard {
        self.pieces[*color as usize]
            .iter()
            .fold(Bitboard(0), |acc, x| acc | *x)
    }

    #[must_use]
    pub fn enemy(&self) -> Color {
        match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn remove_piece_at(&mut self, square: &Square) -> Result<(), anyhow::Error> {
        let (piece, color) = self.piece_at(square).ok_or(anyhow!(
            "No piece at {}, \nfen: {}",
            square.coords_str(),
            self.to_fen()
        ))?;

        self.pieces[color as usize][piece as usize] ^= square.bitboard();

        Ok(())
    }

    pub fn add_piece_at(&mut self, square: Square, piece: Piece, color: Color) {
        self.pieces[color as usize][piece as usize] |= Into::<Bitboard>::into(square);
    }

    #[must_use]
    pub fn piece_at(&self, square: &Square) -> Option<(Piece, Color)> {
        let square = square.bitboard();
        for (i, piece_bb) in self.pieces[0].iter().enumerate() {
            if !(piece_bb & square).is_empty() {
                return Some((Piece::from(i), Color::White));
            }
        }

        for (i, piece_bb) in self.pieces[1].iter().enumerate() {
            if !(piece_bb & square).is_empty() {
                return Some((Piece::from(i), Color::Black));
            }
        }

        None
    }
}

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("Invalid piece index"),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
            .expect("Invalid starting FEN.")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "w"),
            Color::Black => write!(f, "b"),
        }
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Castling::WhiteKingside => write!(f, "K"),
            Castling::WhiteQueenside => write!(f, "Q"),
            Castling::BlackKingside => write!(f, "k"),
            Castling::BlackQueenside => write!(f, "q"),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Pawn => write!(f, "p"),
            Piece::Knight => write!(f, "n"),
            Piece::Bishop => write!(f, "b"),
            Piece::Rook => write!(f, "r"),
            Piece::Queen => write!(f, "q"),
            Piece::King => write!(f, "k"),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8u8 {
                let square = Square::index_enum(rank * 8 + file as usize).unwrap();
                if let Some((piece, color)) = self.piece_at(&square) {
                    write!(f, "{} ", piece.to_utf8_symbol(color))?;
                } else {
                    write!(f, "x ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Piece {
    #[must_use]
    pub fn to_utf8_symbol(&self, color: Color) -> &'static str {
        match (self, color) {
            (Piece::Pawn, Color::Black) => "♙",
            (Piece::Knight, Color::Black) => "♘",
            (Piece::Bishop, Color::Black) => "♗",
            (Piece::Rook, Color::Black) => "♖",
            (Piece::Queen, Color::Black) => "♕",
            (Piece::King, Color::Black) => "♔",
            (Piece::Pawn, Color::White) => "♟",
            (Piece::Knight, Color::White) => "♞",
            (Piece::Bishop, Color::White) => "♝",
            (Piece::Rook, Color::White) => "♜",
            (Piece::Queen, Color::White) => "♛",
            (Piece::King, Color::White) => "♚",
        }
    }
}

impl From<String> for Piece {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "p" => Piece::Pawn,
            "n" => Piece::Knight,
            "b" => Piece::Bishop,
            "r" => Piece::Rook,
            "q" => Piece::Queen,
            "k" => Piece::King,
            _ => panic!("Invalid piece symbol"),
        }
    }
}

pub struct ColorIterator {
    idx: i8,
}

impl Iterator for ColorIterator {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        match self.idx - 1 {
            0 => Some(Color::White),
            1 => Some(Color::Black),
            _ => None, 
        }
    }
}

impl Color {
    #[must_use]
    pub fn iter() -> ColorIterator {
        ColorIterator {
            idx: 0
        }
    }
}
