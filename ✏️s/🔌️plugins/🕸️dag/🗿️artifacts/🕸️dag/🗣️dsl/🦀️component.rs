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
    include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.dag.dag.dsl.semio");

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

    #[test]
    fn fused_edge_arrow_wire_parses_labeled_endpoints() {
        let parsed = dsl::parse_wire_text("a -e1:Connection> b:Node@out").expect("parse fused edge");
        assert_eq!(parsed.edge_label.id.as_deref(), Some("e1"));
        assert_eq!(parsed.edge_label.kind.as_deref(), Some("Connection"));
        assert_eq!(parsed.from.id, "a");
        assert!(parsed.edge.as_ref().map(|(d, _)| *d).unwrap_or(false));
    }
}
//#endregion 🧪️Tests
