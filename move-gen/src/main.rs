#![allow(unused_imports)]
use std::thread;

use movegen::MoveGen;
use sdk::{position::Position, square::Square};

use crate::r#move::{Move, MakeMove};

pub mod lookup;
pub mod r#move;
pub mod movegen;

fn run() {
    let mut pos = Position::default();
    let m1 = Move::new(Square::E2, Square::E4, None);
    let m2 = Move::new(Square::G7, Square::G5, None);
    let m3 = Move::new(Square::E4, Square::E5, None);
    let m4 = Move::new(Square::D7, Square::D5, None);
    println!("{pos}");
    pos.make_move(&m1);
    println!("{pos}");
    pos.make_move(&m2);
    println!("{pos}");
    pos.make_move(&m3);
    println!("{pos}");
    pos.make_move(&m4);
    println!("{pos}");
}

fn main() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
