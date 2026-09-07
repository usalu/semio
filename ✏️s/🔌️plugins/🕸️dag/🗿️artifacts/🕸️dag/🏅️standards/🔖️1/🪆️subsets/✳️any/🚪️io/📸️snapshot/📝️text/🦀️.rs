//! 📜️ DAG document text codec. The graph snapshot owns the canonical node/edge wire grammar;
//! the artifact reconstructs its composed child owner when decoding that graph.

use crate::artifacts::dag::{DagSnapshot, DAG_DOCUMENT_SCHEMA};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ The canonical DAG fixture, handcrafted in the `.dag` DSL — the same file the DAG kernel's own
/// tests parse.
pub const DAG_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.dag` DSL text into a `DagSnapshot`.
pub fn parse_dsl(text: &str) -> Result<DagSnapshot, store::TextError> {
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DagSnapshot` back to `.dag` DSL text.
pub fn print_dsl(document: &DagSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🔖️CodecPrimitives
/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style stdio's own `✳️graph`/`✳️text` facets already establish, duplicated locally (not imported
/// across crates) to keep this facet independently compilable.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

//#endregion 🔖️CodecPrimitives

//#region 🔖️HandcraftedArtifactDsl
impl store::ArtifactDsl for DagSnapshot {
    const EXTENSION: &'static str = "dag";
    fn envelope_id() -> &'static str {
        "dag.dag"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let graph = <infinite_board_port_directed_dag::DagSnapshot as store::ArtifactDsl>::parse_dsl(text)?;
        let mut snapshot: Self = graph.into();
        snapshot.schema = DAG_DOCUMENT_SCHEMA.into();
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let graph = infinite_board_port_directed_dag::DagSnapshot::from(self);
        store::ArtifactDsl::print_dsl(&graph)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn dump_example_dsl_when_requested() {
        if std::env::var("DUMP_DAG_EXAMPLE").is_ok() {
            use crate::artifacts::dag::snapshot::schema::DagSnapshot;
            use crate::artifacts::dag::{dag_content_child_with_owner, DagFixtureEdge, DagNodeSpec, DAG_DOCUMENT_SCHEMA};
            let nodes = vec![DagNodeSpec { id: "slider-a".into(), name: "A".into(), ..Default::default() }, DagNodeSpec { id: "slider-b".into(), name: "B".into(), x: 200.0, ..Default::default() }];
            let edges = vec![DagFixtureEdge { id: "edge-1".into(), source: "slider-a@out".into(), target: "slider-b@in".into(), ..Default::default() }];
            let content = dag_content_child_with_owner(nodes, edges);
            let snapshot = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), content };
            println!("{}", print_dsl(&snapshot));
        }
    }

    #[test]
    fn demo_graph_matches_the_language_neutral_json_oracle() {
        let snapshot = parse_dsl(DAG_EXAMPLE_TEXT).expect("demo DSL");
        let graph = infinite_board_port_directed_dag::DagSnapshot::from(&snapshot);
        let expected: serde_json::Value = serde_json::from_str(include_str!("../../../📚️examples/🎬️demo/🧪️fixtures/🧾️scene.json")).expect("demo JSON oracle");
        let observed = serde_json::json!({
            "nodes": graph.nodes.iter().map(|node| (&node.id, &node.name, node.x, node.y)).collect::<Vec<_>>(),
            "edges": graph.edges.iter().map(|edge| (&edge.id, &edge.source, &edge.target)).collect::<Vec<_>>(),
        });
        assert_eq!(observed, expected);
        let reparsed = <infinite_board_port_directed_dag::DagSnapshot as store::ArtifactDsl>::parse_dsl(&print_dsl(&snapshot)).expect("shared graph grammar");
        assert_eq!(reparsed, graph);
    }

    #[semio_framework_async_macros::async_test]
    async fn example_fixture_dsl_round_trips() {
        let document = parse_dsl(DAG_EXAMPLE_TEXT).expect("parse default fixture");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn fused_edge_arrow_wire_parses_labeled_endpoints() {
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

    #[semio_framework_async_macros::async_test]
    async fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}
