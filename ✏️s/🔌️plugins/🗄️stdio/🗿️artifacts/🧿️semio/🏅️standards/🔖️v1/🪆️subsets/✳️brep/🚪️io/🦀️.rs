//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::semio::standards::v1::subsets::brep::io::export::serializers::artifacts::step::v_ap214::any::SemioBrepToStep;
    use crate::artifacts::semio::standards::v1::subsets::brep::io::import::deserializers::artifacts::step::v_ap214::any::SemioBrepFromStep;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::SemioBrepAnalyzer;
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition, Dialect, IoPayload,
        StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };
    use std::collections::HashSet;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("brep") };

    //#region 🔖️Composer
    pub struct SemioBrepComposerComposition;

    impl ArtifactComposition for SemioBrepComposerComposition {
        type Snapshot = SemioBrepSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SemioBrepComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioBrepAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioBrepComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes the payload as this subset's own `SemioBrepSnapshot` (D5's validate-on-build hook)
    /// and checks referential invariants BETWEEN the subset's own collections: every id an
    /// edge/loop/face/shell/solid references (start/end vertex, loop edge, outer/inner loop, shell
    /// face, solid shell) must resolve to a real entity of the referenced kind in the same snapshot.
    /// Dangling references are reported, never silently dropped or fabricated.
    pub struct SemioBrepValidator;

    impl SubsetValidator for SemioBrepValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioBrepSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_brep_referential_integrity(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_brep.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioBrepValidator: payload did not decode as a SemioBrepSnapshot".to_string())],
            }
        }
    }

    /// 🔗️ Real cross-collection referential-invariant check — dangling ids are reported as errors, not
    /// silently ignored (nothing here is decode-only anymore).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_brep_referential_integrity(snapshot: &SemioBrepSnapshot) -> Vec<dsl::Diagnostic> {
        let vertex_ids: HashSet<&str> = snapshot.vertices.iter().map(|v| v.id.as_str()).collect();
        let edge_ids: HashSet<&str> = snapshot.edges.iter().map(|e| e.id.as_str()).collect();
        let loop_ids: HashSet<&str> = snapshot.loops.iter().map(|l| l.id.as_str()).collect();
        let face_ids: HashSet<&str> = snapshot.faces.iter().map(|f| f.id.as_str()).collect();
        let shell_ids: HashSet<&str> = snapshot.shells.iter().map(|s| s.id.as_str()).collect();

        let mut diagnostics = Vec::new();
        let mut dangling = |code: &'static str, message: String| {
            diagnostics.push(dsl::Diagnostic::error(code, dsl::TextSpan::at(1, 1), message));
        };

        for e in &snapshot.edges {
            if !vertex_ids.contains(e.start_vertex.as_str()) {
                dangling("stdio.semio_brep.dangling-edge-start-vertex", format!("edge {:?} references unknown start vertex {:?}", e.id, e.start_vertex));
            }
            if !vertex_ids.contains(e.end_vertex.as_str()) {
                dangling("stdio.semio_brep.dangling-edge-end-vertex", format!("edge {:?} references unknown end vertex {:?}", e.id, e.end_vertex));
            }
        }
        for l in &snapshot.loops {
            for le in &l.edges {
                if !edge_ids.contains(le.edge.as_str()) {
                    dangling("stdio.semio_brep.dangling-loop-edge", format!("loop {:?} references unknown edge {:?}", l.id, le.edge));
                }
            }
        }
        for f in &snapshot.faces {
            if !loop_ids.contains(f.outer_loop.as_str()) {
                dangling("stdio.semio_brep.dangling-face-outer-loop", format!("face {:?} references unknown outer loop {:?}", f.id, f.outer_loop));
            }
            for inner in &f.inner_loops {
                if !loop_ids.contains(inner.as_str()) {
                    dangling("stdio.semio_brep.dangling-face-inner-loop", format!("face {:?} references unknown inner loop {:?}", f.id, inner));
                }
            }
        }
        for s in &snapshot.shells {
            for sf in &s.faces {
                if !face_ids.contains(sf.face.as_str()) {
                    dangling("stdio.semio_brep.dangling-shell-face", format!("shell {:?} references unknown face {:?}", s.id, sf.face));
                }
            }
        }
        for so in &snapshot.solids {
            for ss in &so.shells {
                if !shell_ids.contains(ss.shell.as_str()) {
                    dangling("stdio.semio_brep.dangling-solid-shell", format!("solid {:?} references unknown shell {:?}", so.id, ss.shell));
                }
            }
        }
        diagnostics
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioBrepValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::brep::schema::semio_brep_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioBrepSnapshot, crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation>(
            crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::STDIO_SEMIOBREP_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_bridge_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.brep.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::semio_brep_artifact_inference_descriptor());
    }

    /// 🌉️ W4 semio↔step bridge — one deserializer entry (writes brep, reads step) + one serializer
    /// entry (writes step, reads brep) give all 4 `IoKey`s via `register_composer_entries`'s own
    /// symmetric import/export insertion (see its doc comment) — no separate reverse registration
    /// needed.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn io_bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES.get_or_init(|| vec![deserializer_entry_of::<SemioBrepFromStep>(), serializer_entry_of::<SemioBrepToStep>()]).as_slice()
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex};

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn valid_snapshot() -> SemioBrepSnapshot {
            let mut s = SemioBrepSnapshot::default();
            s.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3::default() }];
            s.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } }];
            s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }];
            s.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true }];
            s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
            s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
            s
        }

        #[semio_framework_async_macros::async_test]
        async fn referential_integrity_passes_on_a_self_consistent_snapshot() {
            assert!(check_brep_referential_integrity(&valid_snapshot()).is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn referential_integrity_flags_every_kind_of_dangling_reference() {
            let mut s = valid_snapshot();
            s.edges[0].start_vertex = "v-missing".into();
            s.loops[0].edges[0].edge = "e-missing".into();
            s.faces[0].outer_loop = "l-missing".into();
            s.faces[0].inner_loops = vec!["l-missing-2".into()];
            s.shells[0].faces[0].face = "f-missing".into();
            s.solids[0].shells[0].shell = "s-missing".into();
            let diagnostics = check_brep_referential_integrity(&s);
            let codes: Vec<&str> = diagnostics.iter().map(|d| d.code.0.as_str()).collect();
            for expected in [
                "stdio.semio_brep.dangling-edge-start-vertex",
                "stdio.semio_brep.dangling-loop-edge",
                "stdio.semio_brep.dangling-face-outer-loop",
                "stdio.semio_brep.dangling-face-inner-loop",
                "stdio.semio_brep.dangling-shell-face",
                "stdio.semio_brep.dangling-solid-shell",
            ] {
                assert!(codes.contains(&expected), "expected {expected} among {codes:?}");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn validator_decodes_pack_payload_and_runs_referential_checks() {
            let bytes = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&valid_snapshot());
            assert!(SemioBrepValidator::validate(&IoPayload::Binary(bytes)).await.is_empty());

            let mut broken = valid_snapshot();
            broken.edges[0].end_vertex = "v-missing".into();
            let broken_bytes = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&broken);
            let diagnostics = SemioBrepValidator::validate(&IoPayload::Binary(broken_bytes)).await;
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_brep.dangling-edge-end-vertex"));
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.brep`'s three
        /// facets — following the ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION
        /// flow pilot's proven template (`ws-codec-workflow-report.md`). Lives in this composer's
        /// own test region: brep has no per-standard `⚙️engine` dir the way json/csv/zip/png do, and
        /// v1's SHARED `🏅️standards/🔖️v1/⚙️engine/🦀️.rs` only aggregates all 14 subsets'
        /// `register()` calls (no test module of its own, and out of this ticket's `✳️brep/`-only edit
        /// scope anyway).
        mod conformance_laws {

            use crate::artifacts::semio::standards::v1::subsets::brep::schema::{diff, mutations, snapshot};
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
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_brep_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioBrepMutation` variant (`mutations::demo_mutation_cases()`).
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
            /// for every representative `SemioBrepDiff` (`diff::demo_diff_cases()`), incl. the empty
            /// (no-op) diff.
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
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_brep_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_brep_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[semio_framework_async_macros::async_test]
            async fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/🧊️solid/🖼️assets/🗣️.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/🧊️solid/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_brep_snapshot();

                let parsed = <snapshot::SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_brep_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_brep_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioBrepSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_brep_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_brep_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
