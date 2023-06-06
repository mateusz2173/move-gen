#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub mod bitboard;
pub mod fen;
pub mod genmove_tests;
pub mod position;
pub mod square;
pub mod lookup;

extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;
