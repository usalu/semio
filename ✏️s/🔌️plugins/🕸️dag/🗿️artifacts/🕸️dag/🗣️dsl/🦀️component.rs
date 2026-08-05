//! 📜️ DAG artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::DocumentDsl for DagDocument` is implemented directly in the DAG kernel crate
//! (`infinite_board_port_directed_dag`); see `crate::artifacts::dag::op`'s doc for why. This module only
//! adds the thin artifact-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture
//! constant and its round-trip law.

use crate::artifacts::dag::DagDocument;

/// 📄️ The canonical DAG fixture, handcrafted in the `.dag` DSL — the same file the DAG kernel's own
/// tests parse.
pub const DAG_EXAMPLE_TEXT: &str =
    include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/⚡️implementations/🦀️rust/📚️examples/🕸️demo.dag");

/// 📖️ Parses `.dag` DSL text into a `DagDocument`.
pub fn parse_dsl(text: &str) -> Result<DagDocument, store::TextError> {
    <DagDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DagDocument` back to `.dag` DSL text.
pub fn print_dsl(document: &DagDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_fixture_dsl_round_trips() {
        let document = parse_dsl(DAG_EXAMPLE_TEXT).expect("parse default fixture");
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
