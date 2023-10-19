use move_gen::generators::movegen::MoveGen;

pub mod evaluate;
pub mod search;

#[derive(Default)]
pub struct Engine {
    pub nodes_evaluated: usize,
    pub move_gen: MoveGen,
}
