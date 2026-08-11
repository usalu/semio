//! 🎹️ SemioWorkflowComposer (s.stdio.semio/v1/workflow) — analyzer-only compose (decodes the
//! subset's own JSON-pack payload; W4 adds real cross-format compose sources once semio↔format
//! import/export leaves land). `SemioWorkflowValidator` below decodes AND checks real referential
//! invariants (pdf 1.7 `✳️a` composer's `SubsetValidator` pattern, copied per this ticket's
//! instruction): node/edge id uniqueness and edge endpoint node-existence.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
    ComposerEntry, ArtifactDeserializer as _, ArtifactSerializer as _, deserializer_entry_of, serializer_entry_of, register_composer_entries,
};
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::SemioWorkflowSnapshot;
use crate::artifacts::semio::standards::v1::subsets::workflow::analyzer::SemioWorkflowAnalyzer;
use super::io::import::deserializers::artifacts::json::v_rfc8259::any::SemioWorkflowFromJson;
use super::io::export::serializers::artifacts::json::v_rfc8259::any::SemioWorkflowToJson;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("workflow") };

//#region 🔖️Composer
pub struct SemioWorkflowComposer;

impl ArtifactComposer for SemioWorkflowComposer {
    type Snapshot = SemioWorkflowSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] { &[DIALECT] }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "SemioWorkflowComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioWorkflowAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioWorkflowComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ Decodes AND checks real referential invariants — not decode-only. A workflow DAG snapshot is
/// only well-formed if: (1) every node id is unique, (2) every edge id is unique, (3) every edge's
/// `from.node`/`to.node` PortRef references an id that actually exists in `nodes`.
pub struct SemioWorkflowValidator;

/// 🔎️ Real referential-invariant checks over an already-decoded snapshot — factored out so the
/// composer (if it ever gains a pre-serialization hard gate, pdf `✳️a`-style) and this validator's
/// post-hoc wire recheck can share one implementation.
pub fn check_workflow_referential_invariants(snapshot: &SemioWorkflowSnapshot) -> Vec<dsl::Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen_node_ids = std::collections::HashSet::new();
    for node in &snapshot.nodes {
        if !seen_node_ids.insert(node.id.as_str()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_workflow.duplicate-node-id",
                dsl::TextSpan::at(1, 1),
                format!("SemioWorkflowValidator: duplicate node id {:?}", node.id),
            ));
        }
    }
    let mut seen_edge_ids = std::collections::HashSet::new();
    for edge in &snapshot.edges {
        if !seen_edge_ids.insert(edge.id.as_str()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_workflow.duplicate-edge-id",
                dsl::TextSpan::at(1, 1),
                format!("SemioWorkflowValidator: duplicate edge id {:?}", edge.id),
            ));
        }
        if !seen_node_ids.contains(edge.from.node.as_str()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_workflow.dangling-edge-endpoint",
                dsl::TextSpan::at(1, 1),
                format!("SemioWorkflowValidator: edge {:?}'s from.node {:?} references a node that does not exist", edge.id, edge.from.node),
            ));
        }
        if !seen_node_ids.contains(edge.to.node.as_str()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_workflow.dangling-edge-endpoint",
                dsl::TextSpan::at(1, 1),
                format!("SemioWorkflowValidator: edge {:?}'s to.node {:?} references a node that does not exist", edge.id, edge.to.node),
            ));
        }
    }
    diagnostics
}

impl SubsetValidator for SemioWorkflowValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioWorkflowSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioWorkflowSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_workflow_referential_invariants(&snapshot),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_workflow.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioWorkflowValidator: payload did not decode as a SemioWorkflowSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioWorkflowValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️IoEntries
/// 🚪️ workflow<->json bridge row (W4 G6) — one `deserializer_entry_of` (json -> semio) + one
/// `serializer_entry_of` (semio -> json), lossless (see `document`'s own composer for the fuller
/// doc comment on how `register_composer_entries` derives all 4 `IoKey`s from these 2 rows).
static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
fn io_entries() -> &'static [ComposerEntry] {
    IO_ENTRIES.get_or_init(|| vec![deserializer_entry_of::<SemioWorkflowFromJson>(), serializer_entry_of::<SemioWorkflowToJson>()])
}
//#endregion 🔖️IoEntries

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and the
/// workflow<->json io bridge row. Called from this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::workflow::schema::semio_workflow_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioWorkflowSnapshot, crate::artifacts::semio::standards::v1::subsets::workflow::schema::mutations::SemioWorkflowMutation>(crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_entries());
}
//#endregion 🔖️Register

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::{PortRef, WorkflowEdge, WorkflowNode};

    fn node(id: &str) -> WorkflowNode {
        WorkflowNode { id: id.into(), kind: "k".into(), label: "L".into(), params: Vec::new(), position: SemioPoint2::default() }
    }
    fn edge(id: &str, from: &str, to: &str) -> WorkflowEdge {
        WorkflowEdge { id: id.into(), from: PortRef { node: from.into(), port: "out".into() }, to: PortRef { node: to.into(), port: "in".into() }, kind: "data".into() }
    }

    #[test]
    fn well_formed_graph_has_no_diagnostics() {
        let snap = SemioWorkflowSnapshot { nodes: vec![node("a"), node("b")], edges: vec![edge("e1", "a", "b")], ..SemioWorkflowSnapshot::default() };
        assert!(check_workflow_referential_invariants(&snap).is_empty());
    }

    #[test]
    fn dangling_edge_endpoint_is_flagged() {
        let snap = SemioWorkflowSnapshot { nodes: vec![node("a")], edges: vec![edge("e1", "a", "missing")], ..SemioWorkflowSnapshot::default() };
        let diagnostics = check_workflow_referential_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_workflow.dangling-edge-endpoint"), "got {diagnostics:?}");
    }

    #[test]
    fn duplicate_node_and_edge_ids_are_flagged() {
        let snap = SemioWorkflowSnapshot { nodes: vec![node("a"), node("a")], edges: vec![edge("e1", "a", "a"), edge("e1", "a", "a")], ..SemioWorkflowSnapshot::default() };
        let diagnostics = check_workflow_referential_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_workflow.duplicate-node-id"), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_workflow.duplicate-edge-id"), "got {diagnostics:?}");
    }

    #[test]
    fn validator_recheck_on_wire_payload_flags_the_same_invariants() {
        let snap = SemioWorkflowSnapshot { nodes: vec![node("a")], edges: vec![edge("e1", "a", "ghost")], ..SemioWorkflowSnapshot::default() };
        let bytes = <SemioWorkflowSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let diagnostics = SemioWorkflowValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_workflow.dangling-edge-endpoint"), "got {diagnostics:?}");
    }

    /// 🔁️ W4 G6 fixture-backed round trip: json1 -(deserialize)-> semio1 -(serialize)-> json2
    /// -(deserialize)-> semio2, asserting semio1 == semio2 — this pair is lossless (every field
    /// has a direct JSON member), so the round trip is exact, not just "modulo documented losses".
    #[test]
    fn json_round_trip_is_stable() {
        let semio1 = SemioWorkflowSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA.into(),
            nodes: vec![node("a"), node("b")],
            edges: vec![edge("e1", "a", "b")],
        };
        let json1 = SemioWorkflowToJson::serialize(&semio1).expect("serialize");
        let semio2 = SemioWorkflowFromJson::deserialize(&json1).expect("deserialize");
        assert_eq!(semio1, semio2);
    }
}
//#endregion 🔖️Tests
