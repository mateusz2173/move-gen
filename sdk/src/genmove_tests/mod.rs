#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TestData {
    description: Option<String>,
    test_cases: Vec<TestCase>,
}

#[derive(Deserialize, Debug)]
pub struct TestCase {
    start: TestInput,
    expected: Vec<TestOutput>,
}

#[derive(Deserialize, Debug)]
struct TestInput {
    description: Option<String>,
    fen: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct TestOutput {
    #[serde(rename = "move")]
    expected_move: String,
    fen: String,
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{fen::Fen, genmove_tests::TestOutput, movegen::MoveGen, position::Position};

    use super::TestData;

    #[test]
    fn test_pawns_positions() {
        let file = File::open("src/genmove_tests/test_cases/pawns.json").unwrap();

        let test_data: TestData = serde_json::from_reader(file).unwrap();
        for test_case in test_data.test_cases {
            let position = Position::from_fen(test_case.start.fen.clone()).unwrap();

            let generated_moves: Vec<TestOutput> = position
                .generate_moves()
                .iter()
                .map(|m| TestOutput {
                    expected_move: m.notation(&position).unwrap(),
                    fen: m.make(&position).to_fen(),
                })
                .collect();

            let diff: Vec<String> = generated_moves
                .iter()
                .filter(|elem| !test_case.expected.contains(elem))
                .zip(1..)
                .map(|(elem, idx)| {
                    format!(
                        "{idx}: {} -> {}\n{}",
                        elem.expected_move,
                        &elem.fen,
                        Position::from_fen(elem.fen.clone()).unwrap()
                    )
                })
                .collect();

            assert!(
                diff.is_empty(),
                "Move generator generated unexpected moves:\nFEN: {}\nDescription: {}\n{}\n",
                test_case.start.fen,
                test_case.start.description.unwrap_or_default(),
                diff.join("\n")
            );
            let diff: Vec<String> = test_case
                .expected
                .iter()
                .filter(|elem| !generated_moves.contains(elem))
                .zip(1..)
                .map(|(elem, idx)| {
                    format!(
                        "{idx}: {} -> {}\n{}",
                        elem.expected_move,
                        elem.fen,
                        Position::from_fen(elem.fen.clone()).unwrap()
                    )
                })
                .collect();

            assert!(
                diff.is_empty(),
                "Move generator hasn't generated expected moves:\nFEN: {}\nDescription: {}\n{}\n",
                test_case.start.fen,
                test_case.start.description.unwrap_or_default(),
                diff.join("\n")
            );
        }
    }
}
