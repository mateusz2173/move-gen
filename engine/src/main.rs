pub mod core;
mod uci;

use move_gen::r#move::MakeMove;
use sdk::position::Position;
use timeit::timeit_loops;
use uci::start_uci;

use crate::core::search::Search;
use crate::core::Engine;
use std::thread;

fn run() {
    start_uci();
}

fn main() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024 * 2)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
