//! 📜️ DAG artifact — native text codec (`impl store::ArtifactDsl for DagSnapshot`), moved here
//! wholesale from the old `🧬️schema/📸️snapshot` codec home (design.md §1 CORRECTION: the native
//! codec is one bidirectional thing and sits unsplit at `🚪️io/<facet>/<representation>/`, not
//! mirrored under import/export — those only exist for FOREIGN dialects). Distinct from the
//! FRAMEWORK's own separate `infinite_board_port_directed_dag::DagSnapshot` type/codec, which this
//! plugin's `content` child bridges to via `crate::artifacts::dag::🔖️FrameworkBridge`, not by
//! sharing an impl.

use crate::artifacts::dag::{DagFixtureEdge, DagNodeSpec, DagSnapshot, DAG_DOCUMENT_SCHEMA};

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
pub async fn parse_dsl(text: &str) -> Result<DagSnapshot, store::TextError> {
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DagSnapshot` back to `.dag` DSL text.
pub async fn print_dsl(document: &DagSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🔖️CodecPrimitives
/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style stdio's own `✳️graph`/`✳️text` facets already establish, duplicated locally (not imported
/// across crates) to keep this facet independently compilable.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

async fn print_dag_snapshot_body(s: &DagSnapshot) -> String {
    let scene = crate::artifacts::dag::dag_working_scene(s);
    let nodes_json = serde_json::to_string(&scene.nodes).unwrap_or_default();
    let edges_json = serde_json::to_string(&scene.edges).unwrap_or_default();
    format!("schema={}\nnodes={}\nedges={}", enc_str(&s.schema), enc_str(&nodes_json), enc_str(&edges_json))
}
async fn parse_dag_snapshot_body(body: &str) -> Result<DagSnapshot, String> {
    let mut schema = None;
    let mut nodes: Option<Vec<DagNodeSpec>> = None;
    let mut edges: Option<Vec<DagFixtureEdge>> = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("nodes=") {
            nodes = Some(serde_json::from_str(&dec_str(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("edges=") {
            edges = Some(serde_json::from_str(&dec_str(rest)?).map_err(|e| e.to_string())?);
        } else {
            return Err(format!("dag snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "dag snapshot: missing schema line".to_string())?;
    let nodes = nodes.ok_or_else(|| "dag snapshot: missing nodes line".to_string())?;
    let edges = edges.ok_or_else(|| "dag snapshot: missing edges line".to_string())?;
    let content = crate::artifacts::dag::dag_content_child_handle_and_cache(nodes, edges);
    Ok(DagSnapshot { schema, content })
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️HandcraftedArtifactDsl
impl store::ArtifactDsl for DagSnapshot {
    const EXTENSION: &'static str = "dag";
    async fn envelope_id() -> &'static str {
        "dag.dag"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let mut snapshot = parse_dag_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
        snapshot.schema = DAG_DOCUMENT_SCHEMA.into();
        Ok(snapshot)
    }
    async fn print_dsl(&self) -> String {
        let body = print_dag_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
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
            use crate::artifacts::dag::{dag_content_child_handle_and_cache, DagFixtureEdge, DagNodeSpec, DAG_DOCUMENT_SCHEMA};
            let nodes = vec![
                DagNodeSpec { id: "slider-a".into(), name: "A".into(), ..Default::default() },
                DagNodeSpec { id: "slider-b".into(), name: "B".into(), x: 200.0, ..Default::default() },
            ];
            let edges = vec![DagFixtureEdge { id: "edge-1".into(), source: "slider-a@out".into(), target: "slider-b@in".into(), ..Default::default() }];
            let content = dag_content_child_handle_and_cache(nodes, edges);
            let snapshot = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), content };
            println!("{}", print_dsl(&snapshot));
        }
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

