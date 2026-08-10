//! 📜️ DAG artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::DocumentDsl for DagSnapshot` is implemented directly in the DAG kernel crate
//! (`infinite_board_port_directed_dag`); see `crate::artifacts::dag::op`'s doc for why. This module only
//! adds the thin artifact-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture
//! constant and its round-trip law.

use crate::artifacts::dag::DagSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


/// 📄️ The canonical DAG fixture, handcrafted in the `.dag` DSL — the same file the DAG kernel's own
/// tests parse.
pub const DAG_EXAMPLE_TEXT: &str =
    include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.dag` DSL text into a `DagSnapshot`.
pub fn parse_dsl(text: &str) -> Result<DagSnapshot, store::TextError> {
    <DagSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DagSnapshot` back to `.dag` DSL text.
pub fn print_dsl(document: &DagSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_example_dsl_when_requested() {
        if std::env::var("DUMP_DAG_EXAMPLE").is_ok() {
            use crate::artifacts::dag::snapshot::schema::DagSnapshot;
            use crate::artifacts::dag::{DagFixtureEdge, DagNodeSpec};
            let snapshot = DagSnapshot {
                schema: infinite_board_port_directed_dag::DAG_DOCUMENT_SCHEMA.into(),
                nodes: vec![
                    DagNodeSpec { id: "slider-a".into(), name: "A".into(), ..Default::default() },
                    DagNodeSpec { id: "slider-b".into(), name: "B".into(), x: 200.0, ..Default::default() },
                ],
                edges: vec![DagFixtureEdge {
                    id: "edge-1".into(),
                    source: "slider-a@out".into(),
                    target: "slider-b@in".into(),
                    ..Default::default()
                }],
            };
            println!("{}", print_dsl(&snapshot));
        }
    }

    #[test]
    fn example_fixture_dsl_round_trips() {
        let document = parse_dsl(DAG_EXAMPLE_TEXT).expect("parse default fixture");
        store::os_store::test_support::assert_dsl_round_trip(&document);
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

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

