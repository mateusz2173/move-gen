pub mod core;

use move_gen::r#move::MakeMove;
use sdk::position::Position;
use timeit::timeit_loops;

use crate::core::search::Search;
use crate::core::Engine;
use std::thread;

fn run() {
    let mut engine = Engine::default();

    let mut pos = Position::default();

    let depth = 3;

    let time = timeit_loops!(1, {
        while let Some((val, mv)) = engine.search(&pos, depth) {
            let c = pos.make_move(&mv).unwrap();
            dbg!(&c);
            println!("{}", pos.halfmove_clock);

            println!("{mv}: {val}");
            println!("{pos}")
        }
    });

    println!("No more moves.");

    let nps = engine.nodes_evaluated as f64 / time;
    println!("Nodes evaluated: {}, time: {time:.2}s, nps: {nps:.2}", engine.nodes_evaluated);
}

fn main() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024 * 2)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
