#![allow(dead_code)]
use std::{collections::HashSet, thread};

use sdk::{
    fen::Fen,
    position::{Color, Position},
};
use serde::Deserialize;

use crate::{utils::logger::configure_logger, generators::movegen::MoveGen};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Test {
    description: String,
    test_cases: Vec<TestCase>,
}

#[derive(Deserialize, Debug)]
struct TestCase {
    start: StartPosition,
    expected: Vec<MoveFen>,
}

#[derive(Deserialize, Debug)]
struct StartPosition {
    description: String,
    fen: String,
}

#[derive(Deserialize, Debug)]
struct MoveFen {
    r#move: String,
    fen: String,
}

fn load_test(file_name: String) -> Test {
    let home = env!("CARGO_MANIFEST_DIR");
    let test = std::fs::read_to_string(format!("{home}/src/test_cases/{file_name}")).unwrap();

    serde_json::from_str(&test).unwrap()
}

#[test]
fn test_all() {
    configure_logger();
    info!("Starting tests");

    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(test_pawns)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn test_pawns() {
    let move_gen = MoveGen::new();
    let test_cases = load_test("pawns.json".to_string());

    for (idx, test_case) in test_cases.test_cases.iter().enumerate() {
        let pos = Position::from_fen(test_case.start.fen.clone()).unwrap();

        let expected_moves: HashSet<String> = test_case
            .expected
            .iter()
            .map(|expected| expected.r#move.clone())
            .collect();

        let actual_moves: HashSet<String> =
            move_gen.generate_legal_moves(&pos).chess_notation_moves();

        let expected_not_actual: HashSet<String> =
            expected_moves.difference(&actual_moves).cloned().collect();

        let actual_not_expected: HashSet<String> =
            actual_moves.difference(&expected_moves).cloned().collect();

        assert!(
            expected_not_actual.is_empty(),
            "{pos}\nActual: {:?}\nExpected not actual: {:?}\nDescription: {}\nFen: {}",
            actual_moves,
            expected_not_actual,
            test_case.start.description,
            test_case.start.fen
        );
        assert!(
            actual_not_expected.is_empty(),
            "{pos}\nActual: {:?}\nActual not expected: {:?}\nDescription: {}\nFen: {}",
            actual_moves,
            actual_not_expected,
            test_case.start.description,
            test_case.start.fen
        );
        info!("[Pawns-{}] passed.", idx + 1);
    }
}
