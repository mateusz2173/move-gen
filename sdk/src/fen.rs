use anyhow::anyhow;
use enum_index::IndexEnum;

use crate::{
    bitboard::Bitboard,
    position::{Castling, Color, Piece, Position},
    square::Square,
};

pub trait Fen {
    fn from_fen(fen: String) -> anyhow::Result<Position>;
    fn to_fen(&self) -> String;
}

impl Fen for Position {
    #[allow(clippy::too_many_lines)]
    fn from_fen(fen: String) -> anyhow::Result<Position> {
        let mut position = Position {
            pieces: [[Bitboard(0); 6]; 2],
            occupied: Bitboard(0),
            turn: Color::White,
            castling: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };
        let mut fen = fen.split_whitespace();
        let ranks = fen.next().unwrap().split('/');
        let size = ranks.clone().count();
        if size != 8 {
            return Err(anyhow::anyhow!(
                "Invalid FEN: Invalid number of ranks, got {}, expected 8",
                size
            ));
        }

        for (rank, rank_str) in ranks.enumerate() {
            let mut file = 0;
            for c in rank_str.chars() {
                if let Some(digit) = c.to_digit(10) {
                    file += digit as usize;
                } else {
                    let color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

                    let piece = match c.to_ascii_lowercase() {
                        'p' => Piece::Pawn,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'r' => Piece::Rook,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => {
                            return Err(anyhow::anyhow!(
                                "Invalid FEN: Invalid piece character {}",
                                c
                            ))
                        }
                    };

                    let square = Square::index_enum((7 - rank) * 8 + file).unwrap();
                    file += 1;
                    let idx = piece as usize;

                    position.pieces[color as usize][idx] |= square.bitboard();
                }
            }
        }

        position.occupied = position.occupation(&Color::White) | position.occupation(&Color::Black);

        let turn = fen.next().unwrap();
        position.turn = match turn {
            "w" => Color::White,
            "b" => Color::Black,
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid FEN: Invalid turn character {}",
                    turn
                ))
            }
        };

        let castling = fen.next().unwrap();

        if castling != "-" {
            for c in castling.chars() {
                let castling = match c {
                    'K' => Castling::WhiteKingside,
                    'Q' => Castling::WhiteQueenside,
                    'k' => Castling::BlackKingside,
                    'q' => Castling::BlackQueenside,
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Invalid FEN: Invalid castling character {}",
                            c
                        ))
                    }
                };

                position.castling |= castling as u8;
            }
        }

        let en_pass = fen.next().unwrap();
        position.en_passant = if en_pass == "-" {
            None
        } else {
            let mut chars = en_pass.chars();
            let file_char = chars
                .next()
                .ok_or(anyhow!("Invalid FEN: Invalid en passant square: {en_pass}"))?;
            let rank_char = chars
                .next()
                .ok_or(anyhow!("Invalid FEN: Invalid en passant square: {en_pass}"))?;
            if (file_char as u8) < b'a' || (file_char as u8) > b'h' {
                return Err(anyhow!("Invalid FEN: Invalid en passant file: {file_char}, expected a-h. En passant square: {en_pass}"));
            }
            if (rank_char as u8) < b'1' || (rank_char as u8) > b'8' {
                return Err(anyhow!("Invalid FEN: Invalid en passant rank: {rank_char}, expected 1-8. En passant square: {en_pass}"));
            }
            let file = file_char as u8 - b'a';
            let rank = rank_char as u8 - b'1';

            Square::index_enum((rank * 8 + file) as usize)
        };

        Ok(position)
    }

    fn to_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty = 0;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square::index_enum(rank * 8 + file).unwrap();
                let piece = self.piece_at(&square);
                if let Some((piece, color)) = piece {
                    if empty != 0 {
                        fen.push_str(&format!("{empty}"));
                        empty = 0;
                    }
                    match color {
                        Color::White => fen.push_str(&format!("{piece}").to_uppercase()),
                        Color::Black => fen.push_str(&format!("{piece}")),
                    }
                } else {
                    empty += 1;
                }
            }
            if empty != 0 {
                fen.push_str(&format!("{empty}"));
                empty = 0;
            }
            if rank != 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push_str(&format!("{}", self.turn));
        fen.push(' ');
        if self.castling == 0 {
            fen.push('-');
        } else {
            if self.castling & Castling::WhiteKingside as u8 != 0 {
                fen.push('K');
            }
            if self.castling & Castling::WhiteQueenside as u8 != 0 {
                fen.push('Q');
            }
            if self.castling & Castling::BlackKingside as u8 != 0 {
                fen.push('k');
            }
            if self.castling & Castling::BlackQueenside as u8 != 0 {
                fen.push('q');
            }
        }
        fen.push(' ');
        if let Some(square) = &self.en_passant {
            fen.push_str(&square.coords_str());
        } else {
            fen.push('-');
        }
        fen.push(' ');
        fen.push_str(&format!("{}", self.halfmove_clock));
        fen.push(' ');
        fen.push_str(&format!("{}", self.fullmove_number));

        fen
    }
}

#[cfg(test)]
mod tests {

    use crate::bitboard::Bitboard;
    use crate::fen::Fen;
    use crate::position::Color;
    use crate::position::Position;

    #[test]
    fn test_starting_fen() {
        let starting_pos = Position {
            pieces: [
                [
                    Bitboard(0x0000_0000_0000_FF00),
                    Bitboard(0x0000_0000_0000_0042),
                    Bitboard(0x0000_0000_0000_0024),
                    Bitboard(0x0000_0000_0000_0081),
                    Bitboard(0x0000_0000_0000_0008),
                    Bitboard(0x0000_0000_0000_0010),
                ],
                [
                    Bitboard(0x00FF_0000_0000_0000),
                    Bitboard(0x4200_0000_0000_0000),
                    Bitboard(0x2400_0000_0000_0000),
                    Bitboard(0x8100_0000_0000_0000),
                    Bitboard(0x0800_0000_0000_0000),
                    Bitboard(0x1000_0000_0000_0000),
                ],
            ],
            occupied: Bitboard(0x00FF_FF00_0000_00FF),
            turn: Color::White,
            castling: 0b1111,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        assert_eq!(
            starting_pos.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }
}
