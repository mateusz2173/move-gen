#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

#[macro_use]
extern crate log;

pub mod bitboard;
pub mod fen;
pub mod lookup;
pub mod position;
pub mod square;
