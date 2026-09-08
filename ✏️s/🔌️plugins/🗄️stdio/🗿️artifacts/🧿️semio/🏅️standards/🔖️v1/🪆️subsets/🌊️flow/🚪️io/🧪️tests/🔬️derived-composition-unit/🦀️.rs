mod tests {
    use super::*;
    use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
    use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef};
    use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn node(id: &str) -> FlowNode {
        FlowNode { id: id.into(), kind: "k".into(), label: "L".into(), params: Vec::new(), position: SemioPoint2::default() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn edge(id: &str, from: &str, to: &str) -> FlowEdge {
        FlowEdge { id: id.into(), from: PortRef { node: from.into(), port: "out".into() }, to: PortRef { node: to.into(), port: "in".into() }, kind: "data".into() }
    }

    #[semio_framework_async_macros::async_test]
    async fn well_formed_graph_has_no_diagnostics() {
        let snap = SemioFlowSnapshot { nodes: vec![node("a"), node("b")], edges: vec![edge("e1", "a", "b")], ..SemioFlowSnapshot::default() };
        assert!(check_flow_referential_invariants(&snap).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn dangling_edge_endpoint_is_flagged() {
        let snap = SemioFlowSnapshot { nodes: vec![node("a")], edges: vec![edge("e1", "a", "missing")], ..SemioFlowSnapshot::default() };
        let diagnostics = check_flow_referential_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_flow.dangling-edge-endpoint"), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_node_and_edge_ids_are_flagged() {
        let snap = SemioFlowSnapshot { nodes: vec![node("a"), node("a")], edges: vec![edge("e1", "a", "a"), edge("e1", "a", "a")], ..SemioFlowSnapshot::default() };
        let diagnostics = check_flow_referential_invariants(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_flow.duplicate-node-id"), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_flow.duplicate-edge-id"), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn validator_recheck_on_wire_payload_flags_the_same_invariants() {
        let snap = SemioFlowSnapshot { nodes: vec![node("a")], edges: vec![edge("e1", "a", "ghost")], ..SemioFlowSnapshot::default() };
        let bytes = <SemioFlowSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let diagnostics = SemioFlowValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_flow.dangling-edge-endpoint"), "got {diagnostics:?}");
    }

    /// 🔁️ W4 G6 fixture-backed round trip: json1 -(deserialize)-> semio1 -(serialize)-> json2
    /// -(deserialize)-> semio2, asserting semio1 == semio2 — this pair is lossless (every field
    /// has a direct JSON member), so the round trip is exact, not just "modulo documented losses".
    #[semio_framework_async_macros::async_test]
    async fn json_round_trip_is_stable() {
        let semio1 = SemioFlowSnapshot { schema: crate::standards::v1::subsets::flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes: vec![node("a"), node("b")], edges: vec![edge("e1", "a", "b")] };
        let json1 = semio_framework_plugin::resolve_ready(SemioFlowToJson::serialize(&semio1)).expect("serialize");
        let semio2 = semio_framework_plugin::resolve_ready(SemioFlowFromJson::deserialize(&json1)).expect("deserialize");
        assert_eq!(semio1, semio2);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.flow`'s
    /// three facets — the FIRST real pilot for a semio subset (flow), establishing the pattern
    /// the other 12 domain subsets replicate. Lives in this composer's own test region: flow
    /// has no per-standard `⚙️engine` dir the way json/csv/zip/png do, and v1's SHARED
    /// `🏅️standards/🔖️v1/⚙️engine/🦀️.rs` only aggregates all 14 subsets' `register()`
    /// calls (no test module of its own, and out of this ticket's `🌊️flow/`-only edit scope
    /// anyway).
    mod conformance_laws {

        use crate::standards::v1::subsets::flow::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below.
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
        /// the demo snapshot — same preamble-stripped body reconstruction the eventual
        /// `m5_handcrafted_grammar_conformance` harness uses (envelope id prepended as the bare
        /// `artifact-mark` token), so this is a direct proof this facet will pass that harness once
        /// graduated.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&snapshot::demo_flow_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
        /// for every `SemioFlowMutation` variant (`mutations::demo_mutation_cases()`).
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `SemioFlowDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty (no-op) diff.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
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
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&snapshot::demo_flow_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `snapshot::demo_flow_snapshot()` —
        /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
        /// pack twin — so the fixtures can never silently drift back to a fake.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../../✉️base/📚️examples/🌊️pipeline/🖼️assets/🗣️.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../../✉️base/📚️examples/🌊️pipeline/🖼️assets/🎒️.pack.semio");

            let demo = snapshot::demo_flow_snapshot();

            let parsed = <snapshot::SemioFlowSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_flow_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_flow_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <snapshot::SemioFlowSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_flow_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_flow_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
