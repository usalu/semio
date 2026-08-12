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
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::SemioImageAnalyzer;
    use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::png::v1_2::any::SemioImageFromPng;
    use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::png::v1_2::any::SemioImageToPng;
    use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::jpg::v_jfif_1_01::any::SemioImageFromJpg;
    use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::jpg::v_jfif_1_01::any::SemioImageToJpg;
    use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::gif::v89a::any::SemioImageFromGif;
    use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::gif::v89a::any::SemioImageToGif;
    use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::bmp::v_v3::any::SemioImageFromBmp;
    use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::bmp::v_v3::any::SemioImageToBmp;
    use crate::artifacts::semio::standards::v1::subsets::image::io::import::deserializers::artifacts::tiff::v6_0::any::SemioImageFromTiff;
    use crate::artifacts::semio::standards::v1::subsets::image::io::export::serializers::artifacts::tiff::v6_0::any::SemioImageToTiff;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

    //#region 🔖️Composer
    pub struct SemioImageComposerComposition;

    impl ArtifactComposition for SemioImageComposerComposition {
        type Snapshot = SemioImageSnapshot;
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
                return Err(ComposeError { message: "SemioImageComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioImageAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "SemioImageComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ 🚧 scaffolded by W1b — decode-only validator (no referential-invariant diagnostics yet;
    /// W2 adds real cross-reference checks).
    pub struct SemioImageValidator;

    impl SubsetValidator for SemioImageValidator {
        const DIALECT: Dialect = DIALECT;
        fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioImageSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(_) => Vec::new(),
                None => vec![dsl::Diagnostic::error(
                    "stdio.semio_image.validate-decode-failed",
                    dsl::TextSpan::at(1, 1),
                    "SemioImageValidator: payload did not decode as a SemioImageSnapshot".to_string(),
                )],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioImageValidator>) }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
    /// G4): the five raster-format bridges (png/jpg/gif/bmp/tiff), each a deserializer+serializer
    /// pair. Per `register_composer_entries`'s own doc comment, ONE entry registers BOTH its import
    /// AND (symmetrically) the counterpart's export `IoKey` — a deserializer (writes image, reads
    /// fmt) plus its mirror serializer (writes fmt, reads image) together cover all four `IoKey`s per
    /// format without hand-writing each direction separately.
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    fn io_entries() -> &'static [ComposerEntry] {
        IO_ENTRIES.get_or_init(|| vec![
            deserializer_entry_of::<SemioImageFromPng>(), serializer_entry_of::<SemioImageToPng>(),
            deserializer_entry_of::<SemioImageFromJpg>(), serializer_entry_of::<SemioImageToJpg>(),
            deserializer_entry_of::<SemioImageFromGif>(), serializer_entry_of::<SemioImageToGif>(),
            deserializer_entry_of::<SemioImageFromBmp>(), serializer_entry_of::<SemioImageToBmp>(),
            deserializer_entry_of::<SemioImageFromTiff>(), serializer_entry_of::<SemioImageToTiff>(),
        ]).as_slice()
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
    /// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::image::schema::semio_image_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioImageSnapshot, crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation>(crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA));
        register_subset_validator(validator_entry());
        register_composer_entries(io_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.image.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::image::schema::inferences::semio_image_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        //#region 🔖️ConformanceLaws
        /// 🧪️ The 6 real-codec conformance-law tests (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-
        /// REUSE-EVOLUTION's image wave), mirroring `✳️flow`'s/`✳️mesh`'s own proven, fully-verified
        /// template (`ws-codec-workflow-report.md`/`ws-codec-mesh-report.md`) — same 6 test names, same
        /// shape, only the facet modules and demo-case helpers differ.
        mod conformance_laws {
            use crate::artifacts::semio::standards::v1::subsets::image::schema::{diff, mutations, snapshot};
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
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_image_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioImageMutation` variant (`mutations::demo_mutation_cases()`).
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
            /// for every representative `SemioImageDiff` (`diff::demo_diff_cases()`), incl. the empty
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
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_image_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_image_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[test]
            fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/🖼️swatch/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/🖼️swatch/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_image_snapshot();

                let parsed = <snapshot::SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_image_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_image_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioImageSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_image_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_image_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
