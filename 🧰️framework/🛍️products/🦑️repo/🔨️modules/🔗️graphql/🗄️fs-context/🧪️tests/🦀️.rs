//! 🧭️ The analyze scope vectors: every raw scope string parses to the scope kind and operand the reference `ParseScope` reads.

use super::*;

const VECTORS: &str = include_str!("🧫️fixtures/🧭️analyze-scope.csv");

fn describe(scope: &AnalyzeScope) -> (&'static str, &str) {
    match scope {
        AnalyzeScope::Repo => ("repo", ""),
        AnalyzeScope::Definition(value) => ("definition", value),
        AnalyzeScope::Section(value) => ("section", value),
        AnalyzeScope::File(value) => ("file", value),
        AnalyzeScope::Technology(value) => ("technology", value),
        AnalyzeScope::Folder(value) => ("folder", value),
    }
}

#[test]
fn parses_every_scope_vector() {
    let rows: Vec<&str> = VECTORS.lines().skip(1).filter(|row| !row.is_empty()).collect();
    assert!(!rows.is_empty());
    for row in rows {
        let cells: Vec<&str> = row.split(',').collect();
        let parsed = AnalyzeScope::parse(cells[0]);
        assert_eq!(describe(&parsed), (cells[1], cells[2]), "{:?}", cells[0]);
    }
}
