//! 🚪️ IO — structure only; registration flows through 🎹️composer::register (matching the
//! repo-wide convention — see gif's own io leaf doc comment). W4 adds the real semio↔dxf/dwg/step
//! import/export leaves under 📥️import/🧩️deserializers and 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{
        ArtifactComposition, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, ComposerEntry, Dialect, IoPayload, StandardId, SubsetId,
        SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of, register_composer_entries, deserializer_entry_of, serializer_entry_of,
    };
    use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadEntity, SemioCadSnapshot};
    use crate::artifacts::semio::standards::v1::subsets::cad::schema::SemioCadAnalyzer;
    use crate::artifacts::semio::standards::v1::subsets::cad::io::import::deserializers::artifacts::dxf::v_r12::any::SemioCadFromDxf;
    use crate::artifacts::semio::standards::v1::subsets::cad::io::export::serializers::artifacts::dxf::v_r12::any::SemioCadToDxf;
    use crate::artifacts::semio::standards::v1::subsets::cad::io::import::deserializers::artifacts::dwg::v_ac1024::any::SemioCadFromDwg;
    use crate::artifacts::semio::standards::v1::subsets::cad::io::export::serializers::artifacts::dwg::v_ac1024::any::SemioCadToDwg;
    use crate::artifacts::semio::standards::v1::subsets::cad::io::import::deserializers::artifacts::step::v_ap214::any::SemioCadFromStep;
    use crate::artifacts::semio::standards::v1::subsets::cad::io::export::serializers::artifacts::step::v_ap214::any::SemioCadToStep;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };

    //#region 🔖️Composer
    pub struct SemioCadComposerComposition;

    impl ArtifactComposition for SemioCadComposerComposition {
        type Snapshot = SemioCadSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] { &[DIALECT] }

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
                return Err(ComposeError { message: "SemioCadComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioCadAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "SemioCadComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Real referential-invariant checks over the subset's OWN collections: every
    /// `CadEntityRecord.layer` (top-level AND nested inside a block) must name a real `CadLayer`;
    /// every `CadEntity::Insert.block_name` must name a real `CadBlock` and must not name its OWN
    /// containing block (a self-referential insert is an infinite-recursion cycle, not valid content).
    fn cad_referential_diagnostics(snapshot: &SemioCadSnapshot) -> Vec<dsl::Diagnostic> {
        let mut diagnostics = Vec::new();
        let layer_names: std::collections::BTreeSet<&str> = snapshot.layers.iter().map(|l| l.name.as_str()).collect();
        let block_names: std::collections::BTreeSet<&str> = snapshot.blocks.iter().map(|b| b.name.as_str()).collect();

        let check_record = |diagnostics: &mut Vec<dsl::Diagnostic>, owning_block: Option<&str>, rec: &crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::CadEntityRecord| {
            if !layer_names.contains(rec.layer.as_str()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_cad.dangling-layer",
                    dsl::TextSpan::at(1, 1),
                    format!("entity {:?} (handle {:?}) references undefined layer {:?}", owning_block.unwrap_or("<top-level>"), rec.handle, rec.layer),
                ));
            }
            if let CadEntity::Insert { block_name, .. } = &rec.entity {
                if !block_names.contains(block_name.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error(
                        "stdio.semio_cad.dangling-block-insert",
                        dsl::TextSpan::at(1, 1),
                        format!("entity handle {:?} inserts undefined block {:?}", rec.handle, block_name),
                    ));
                }
                if owning_block == Some(block_name.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error(
                        "stdio.semio_cad.self-referential-insert",
                        dsl::TextSpan::at(1, 1),
                        format!("block {:?} contains an Insert of itself (handle {:?}) -- infinite recursion", block_name, rec.handle),
                    ));
                }
            }
        };

        for rec in &snapshot.entities {
            check_record(&mut diagnostics, None, rec);
        }
        for block in &snapshot.blocks {
            for rec in &block.entities {
                check_record(&mut diagnostics, Some(block.name.as_str()), rec);
            }
        }
        diagnostics
    }

    pub struct SemioCadValidator;

    impl SubsetValidator for SemioCadValidator {
        const DIALECT: Dialect = DIALECT;
        fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioCadSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => cad_referential_diagnostics(&snapshot),
                None => vec![dsl::Diagnostic::error(
                    "stdio.semio_cad.validate-decode-failed",
                    dsl::TextSpan::at(1, 1),
                    "SemioCadValidator: payload did not decode as a SemioCadSnapshot".to_string(),
                )],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioCadValidator>) }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
    /// G4): cad↔dxf/dwg/step. dxf is a real, complete entity-shaped bridge; dwg is an honestly
    /// unsupported-content bridge (this codec's D1/D2 decode depth never reaches entity bitcode);
    /// step bridges only the two AP214 curve entities (LINE/CIRCLE) with a real B-rep/solid
    /// equivalent — see each pair's own leaf doc comment for the full rationale.
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    fn io_entries() -> &'static [ComposerEntry] {
        IO_ENTRIES.get_or_init(|| vec![
            deserializer_entry_of::<SemioCadFromDxf>(), serializer_entry_of::<SemioCadToDxf>(),
            deserializer_entry_of::<SemioCadFromDwg>(), serializer_entry_of::<SemioCadToDwg>(),
            deserializer_entry_of::<SemioCadFromStep>(), serializer_entry_of::<SemioCadToStep>(),
        ]).as_slice()
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
    /// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::cad::schema::semio_cad_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioCadSnapshot, crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::SemioCadMutation>(crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA));
        register_subset_validator(validator_entry());
        register_composer_entries(io_entries());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
        use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntityRecord, CadLayer};

        #[test]
        fn validator_accepts_a_fully_referenced_snapshot() {
            let snapshot = SemioCadSnapshot {
                schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
                layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
                blocks: vec![CadBlock { name: "door".into(), base_point: SemioPoint2::default(), entities: Vec::new() }],
                entities: vec![CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "door".into(), insertion_point: SemioPoint2::default(), scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } }],
            };
            assert!(cad_referential_diagnostics(&snapshot).is_empty());
        }

        #[test]
        fn validator_flags_dangling_layer_and_dangling_block_insert() {
            let snapshot = SemioCadSnapshot {
                schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
                layers: Vec::new(),
                blocks: Vec::new(),
                entities: vec![CadEntityRecord { handle: "h1".into(), layer: "missing".into(), entity: CadEntity::Insert { block_name: "missing-block".into(), insertion_point: SemioPoint2::default(), scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } }],
            };
            let diagnostics = cad_referential_diagnostics(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_cad.dangling-layer"));
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_cad.dangling-block-insert"));
        }

        #[test]
        fn validator_flags_self_referential_block_insert() {
            let snapshot = SemioCadSnapshot {
                schema: crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
                layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
                blocks: vec![CadBlock {
                    name: "loopy".into(),
                    base_point: SemioPoint2::default(),
                    entities: vec![CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "loopy".into(), insertion_point: SemioPoint2::default(), scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } }],
                }],
                entities: Vec::new(),
            };
            let diagnostics = cad_referential_diagnostics(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_cad.self-referential-insert"));
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.cad`'s three
        /// facets — following the ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION
        /// flow/brep pilots' proven template (`ws-codec-workflow-report.md`/
        /// `ws-codec-brep-report.md`). Lives in this composer's own test region: cad has no
        /// per-standard `⚙️engine` dir the way json/csv/zip/png do, and v1's SHARED
        /// `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` only aggregates all 14 subsets' `register()`
        /// calls (no test module of its own, and out of this ticket's `✳️cad/`-only edit scope anyway).
        mod conformance_laws {
            use super::*;
            use crate::artifacts::semio::standards::v1::subsets::cad::schema::{diff, mutations, snapshot};
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
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_cad_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioCadMutation` variant (`mutations::demo_mutation_cases()`).
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
            /// for every representative `SemioCadDiff` (`diff::demo_diff_cases()`), incl. the empty
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
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_cad_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_cad_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[test]
            fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../../../../📚️examples/📐️drawing/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../../../../📚️examples/📐️drawing/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_cad_snapshot();

                let parsed = <snapshot::SemioCadSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_cad_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_cad_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioCadSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_cad_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_cad_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
