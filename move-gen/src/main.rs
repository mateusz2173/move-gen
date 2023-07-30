#![allow(unused_imports)]
use std::thread;

use movegen::MoveGen;
use sdk::{position::{Position, Piece, Color}, square::Square};

use crate::r#move::{Move, MakeMove};

pub mod lookup;
pub mod r#move;
pub mod movegen;

fn run() {
    let mut pos = Position::default();
    let mut movegen = MoveGen::new();
    dbg!(pos.pieces[Color::White as usize][Piece::Pawn as usize]);

    for i in 1..=1 {
        let mv = movegen.make_random_move(&mut pos);

        dbg!(pos.pieces[Color::White as usize][Piece::Pawn as usize]);
        println!("{i}. Move: {mv}");
    }
    println!("{pos}");
    for mv in movegen.generate_legal_moves(&pos) {
        println!("Legal move: {}", mv.to_chess_notation(&pos));
    }
}

fn main() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
