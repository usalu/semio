//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{
        ArtifactComposition, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, ComposerEntry, Dialect, IoPayload, StandardId, SubsetId,
        SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of, register_composer_entries, deserializer_entry_of, serializer_entry_of,
    };
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::SemioDrawingAnalyzer;
    use crate::artifacts::semio::standards::v1::subsets::drawing::io::import::deserializers::artifacts::svg::v1_1::any::SemioDrawingFromSvg;
    use crate::artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
    use crate::artifacts::semio::standards::v1::subsets::drawing::io::import::deserializers::artifacts::dxf::v_r12::any::SemioDrawingFromDxf;
    use crate::artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::dxf::v_r12::any::SemioDrawingToDxf;
    use crate::artifacts::semio::standards::v1::subsets::drawing::io::import::deserializers::artifacts::pdf::v1_7::any::SemioDrawingFromPdf;
    use crate::artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::SemioDrawingToPdf;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

    //#region 🔖️Composer
    pub struct SemioDrawingComposerComposition;

    impl ArtifactComposition for SemioDrawingComposerComposition {
        type Snapshot = SemioDrawingSnapshot;
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
                return Err(ComposeError { message: "SemioDrawingComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioDrawingAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "SemioDrawingComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes the payload as `SemioDrawingSnapshot` and checks two real referential invariants
    /// (both real cross-collection lookups, not decode-only): (1) every `Path`/`Text` node's
    /// `style` reference resolves to a name present in `styles` (dangling-ref detection); (2) every
    /// `DrawLayer.id` is unique across `layers` (duplicate-id detection).
    pub struct SemioDrawingValidator;

    impl SubsetValidator for SemioDrawingValidator {
        const DIALECT: Dialect = DIALECT;
        fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_drawing_invariants(&snapshot),
                None => vec![dsl::Diagnostic::error(
                    "stdio.semio_drawing.validate-decode-failed",
                    dsl::TextSpan::at(1, 1),
                    "SemioDrawingValidator: payload did not decode as a SemioDrawingSnapshot".to_string(),
                )],
            }
        }
    }

    /// 🔎️ Real referential-invariant checks over `SemioDrawingSnapshot`'s own collections (no
    /// cross-artifact lookups needed -- both invariants are internal to this subset).
    pub fn check_drawing_invariants(snapshot: &SemioDrawingSnapshot) -> Vec<dsl::Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut seen_layer_ids = std::collections::HashSet::new();
        for layer in &snapshot.layers {
            if !seen_layer_ids.insert(layer.id.clone()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_drawing.duplicate-layer-id",
                    dsl::TextSpan::at(1, 1),
                    format!("SemioDrawingValidator: duplicate layer id {:?}", layer.id),
                ));
            }
        }

        fn walk(node: &DrawNode, style_names: &std::collections::HashSet<&str>, diagnostics: &mut Vec<dsl::Diagnostic>) {
            match node {
                DrawNode::Path { style: Some(name), .. } | DrawNode::Text { style: Some(name), .. } => {
                    if !style_names.contains(name.as_str()) {
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.semio_drawing.dangling-style-ref",
                            dsl::TextSpan::at(1, 1),
                            format!("SemioDrawingValidator: node references undefined style {name:?}"),
                        ));
                    }
                }
                DrawNode::Group { children, .. } => {
                    for child in children {
                        walk(child, style_names, diagnostics);
                    }
                }
                _ => {}
            }
        }
        let style_names: std::collections::HashSet<&str> = snapshot.styles.iter().map(|s| s.name.as_str()).collect();
        for layer in &snapshot.layers {
            walk(&layer.root, &style_names, &mut diagnostics);
        }

        diagnostics
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioDrawingValidator>) }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
    /// G4): drawing↔svg/dxf/pdf. svg is the richest (recursive scene-graph↔scene-graph); dxf is a
    /// real entity↔path translation (exact circles, sampled-flattened curves on export); pdf is an
    /// honestly text-only bridge (this codec's own snapshot never exposes decoded content-stream
    /// vector ops) — see each pair's own leaf doc comment for the full rationale.
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    fn io_entries() -> &'static [ComposerEntry] {
        IO_ENTRIES.get_or_init(|| vec![
            deserializer_entry_of::<SemioDrawingFromSvg>(), serializer_entry_of::<SemioDrawingToSvg>(),
            deserializer_entry_of::<SemioDrawingFromDxf>(), serializer_entry_of::<SemioDrawingToDxf>(),
            deserializer_entry_of::<SemioDrawingFromPdf>(), serializer_entry_of::<SemioDrawingToPdf>(),
        ]).as_slice()
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
    /// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::drawing::schema::semio_drawing_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioDrawingSnapshot, crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation>(crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA));
        register_subset_validator(validator_entry());
        register_composer_entries(io_entries());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioTransform};
        use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawStyle};

        #[test]
        fn dangling_style_ref_and_duplicate_layer_id_are_both_reported() {
            let snapshot = SemioDrawingSnapshot {
                styles: vec![DrawStyle { name: "ok".into(), fill: None, stroke: None, stroke_width: None, opacity: None }],
                layers: vec![
                    DrawLayer { id: "dup".into(), name: "a".into(), visible: true, root: DrawNode::Path { segments: vec![], style: Some("missing".into()) } },
                    DrawLayer { id: "dup".into(), name: "b".into(), visible: true, root: DrawNode::Text { value: "t".into(), at: SemioPoint2::default(), style: Some("ok".into()) } },
                ],
                ..SemioDrawingSnapshot::default()
            };
            let diagnostics = check_drawing_invariants(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_drawing.dangling-style-ref"), "{diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_drawing.duplicate-layer-id"), "{diagnostics:?}");
        }

        #[test]
        fn clean_snapshot_reports_no_diagnostics() {
            let snapshot = SemioDrawingSnapshot {
                styles: vec![DrawStyle { name: "ok".into(), fill: None, stroke: None, stroke_width: None, opacity: None }],
                layers: vec![DrawLayer { id: "l0".into(), name: "a".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Path { segments: vec![], style: Some("ok".into()) }] } }],
                ..SemioDrawingSnapshot::default()
            };
            assert!(check_drawing_invariants(&snapshot).is_empty());
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.drawing`'s
        /// three facets — following the ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION
        /// flow/brep waves' proven template. Lives in this composer's own test region: drawing has
        /// no per-standard `⚙️engine` dir the way json/csv/zip/png do, and v1's SHARED
        /// `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` only aggregates all 14 subsets' `register()` calls
        /// (no test module of its own, and out of this ticket's `✳️drawing/`-only edit scope anyway).
        mod conformance_laws {
            use super::*;
            use crate::artifacts::semio::standards::v1::subsets::drawing::schema::{diff, mutations, snapshot};
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
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_drawing_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioDrawingMutation` variant (`mutations::demo_mutation_cases()`).
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
            /// for every representative `SemioDrawingDiff` (`diff::demo_diff_cases()`), incl. the empty
            /// (no-op) diff.
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
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_drawing_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_drawing_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[test]
            fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../../../../📚️examples/🖍️sketch/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../../../../📚️examples/🖍️sketch/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_drawing_snapshot();

                let parsed = <snapshot::SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_drawing_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_drawing_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_drawing_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_drawing_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
