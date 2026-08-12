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

    //#region 🔖️ConformanceLaws
    /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.workflow`'s
    /// three facets — the FIRST real pilot for a semio subset (workflow), establishing the pattern
    /// the other 12 domain subsets replicate. Lives in this composer's own test region: workflow
    /// has no per-standard `⚙️engine` dir the way json/csv/zip/png do, and v1's SHARED
    /// `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` only aggregates all 14 subsets' `register()`
    /// calls (no test module of its own, and out of this ticket's `✳️workflow/`-only edit scope
    /// anyway).
    mod conformance_laws {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::workflow::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below.
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
        /// the demo snapshot — same preamble-stripped body reconstruction the eventual
        /// `m5_handcrafted_grammar_conformance` harness uses (envelope id prepended as the bare
        /// `artifact-mark` token), so this is a direct proof this facet will pass that harness once
        /// graduated.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&snapshot::demo_workflow_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
        /// for every `SemioWorkflowMutation` variant (`mutations::demo_mutation_cases()`).
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `SemioWorkflowDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty (no-op) diff.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first), every demo mutation's
        /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&snapshot::demo_workflow_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `snapshot::demo_workflow_snapshot()` —
        /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
        /// pack twin — so the fixtures can never silently drift back to a fake.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../../../📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../../../📚️examples/🌊️pipeline/🖼️assets/🎒️example.pack.semio");

            let demo = snapshot::demo_workflow_snapshot();

            let parsed = <snapshot::SemioWorkflowSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_workflow_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_workflow_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <snapshot::SemioWorkflowSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_workflow_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_workflow_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🔖️Tests
