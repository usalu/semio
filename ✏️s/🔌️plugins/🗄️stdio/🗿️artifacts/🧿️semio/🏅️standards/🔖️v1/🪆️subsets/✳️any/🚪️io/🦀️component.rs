//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::semio::standards::v1::subsets::animation::io::SemioAnimationValidator;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
    use crate::artifacts::semio::standards::v1::subsets::any::schema::SemioAnalyzer;
    use crate::artifacts::semio::standards::v1::subsets::audio::io::SemioAudioValidator;
    use crate::artifacts::semio::standards::v1::subsets::brep::io::SemioBrepValidator;
    use crate::artifacts::semio::standards::v1::subsets::cad::io::SemioCadValidator;
    use crate::artifacts::semio::standards::v1::subsets::document::io::SemioDocumentValidator;
    use crate::artifacts::semio::standards::v1::subsets::drawing::io::SemioDrawingValidator;
    use crate::artifacts::semio::standards::v1::subsets::flow::io::SemioFlowValidator;
    use crate::artifacts::semio::standards::v1::subsets::graph::io::SemioGraphValidator;
    use crate::artifacts::semio::standards::v1::subsets::image::io::SemioImageValidator;
    use crate::artifacts::semio::standards::v1::subsets::kit::io::SemioKitValidator;
    use crate::artifacts::semio::standards::v1::subsets::mesh::io::SemioMeshValidator;
    use crate::artifacts::semio::standards::v1::subsets::model::io::SemioModelValidator;
    use crate::artifacts::semio::standards::v1::subsets::object::io::SemioObjectValidator;
    use crate::artifacts::semio::standards::v1::subsets::presentation::io::SemioPresentationValidator;
    use crate::artifacts::semio::standards::v1::subsets::table::io::SemioTableValidator;
    use crate::artifacts::semio::standards::v1::subsets::text::io::SemioTextValidator;
    use crate::artifacts::semio::standards::v1::subsets::value::io::SemioValueValidator;
    use crate::artifacts::semio::standards::v1::subsets::video::io::SemioVideoValidator;
    use dsl::Diagnostic;
    use semio_framework_plugin::{
        register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };
    use std::sync::OnceLock;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct SemioComposerComposition;

    impl ArtifactComposition for SemioComposerComposition {
        type Snapshot = SemioSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SemioComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The envelope's own `SubsetValidator` for the `"*"` dialect (D5's generic
    /// validate-on-build hook — required by policy, same as every one of the 13 subsets' own
    /// validators, `pdf`'s `✳️a` composer is the copy template). Decodes the payload as a
    /// `SemioSnapshot`, then DELEGATES to whichever one of the 13 subsets' OWN, already-real
    /// `SubsetValidator`s matches the decoded snapshot's active kind — this validator owns zero
    /// invariant logic itself, only the envelope-level decode + dispatch, exactly mirroring how
    /// `SemioDiff`/`SemioMutation` themselves only own routing, never re-derived per-subset rules.
    pub struct SemioValidator;

    /// 🔎️ Real dispatch: re-encodes the decoded inner snapshot through ITS OWN subset's
    /// `ArtifactPack`, then calls that subset's own registered `SubsetValidator::validate` — genuine
    /// reuse of all 13 already-tested invariant checks, never duplicated here.
    async fn dispatch_validate(snapshot: &SemioSnapshot) -> Vec<Diagnostic> {
        match &snapshot.subset {
            SemioSubsetSnapshot::Brep(s) => SemioBrepValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot as store::ArtifactPack>::encode_pack(s).await)).await,
            SemioSubsetSnapshot::Mesh(s) => SemioMeshValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot as store::ArtifactPack>::encode_pack(s).await)).await,
            SemioSubsetSnapshot::Model(s) => SemioModelValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot as store::ArtifactPack>::encode_pack(s).await)).await,
            SemioSubsetSnapshot::Value(s) => SemioValueValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot as store::ArtifactPack>::encode_pack(s).await)).await,
            SemioSubsetSnapshot::Document(s) => SemioDocumentValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(s).await)).await,
            SemioSubsetSnapshot::Cad(s) => SemioCadValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot as store::ArtifactPack>::encode_pack(s).await)).await,
            SemioSubsetSnapshot::Drawing(s) => SemioDrawingValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Image(s) => SemioImageValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Video(s) => SemioVideoValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Audio(s) => SemioAudioValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Animation(s) => SemioAnimationValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Presentation(s) => {
                SemioPresentationValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(s).await)).await
            }
            SemioSubsetSnapshot::Flow(s) => SemioFlowValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Text(s) => SemioTextValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Table(s) => SemioTableValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Graph(s) => SemioGraphValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Object(s) => SemioObjectValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot as store::ArtifactPack>::encode_pack(s).await)),
            SemioSubsetSnapshot::Kit(s) => SemioKitValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot as store::ArtifactPack>::encode_pack(s).await)),
        }
    }

    impl SubsetValidator for SemioValidator {
        const DIALECT: Dialect = DIALECT;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioSnapshot as store::ArtifactPack>::decode_pack(bytes).await.ok(),
                IoPayload::Text(text) => <SemioSnapshot as store::ArtifactDsl>::parse_dsl(text).await.ok(),
            };
            match decoded {
                Some(snapshot) => dispatch_validate(&snapshot).await,
                None => vec![Diagnostic {
                    code: dsl::FaultCode::new("stdio.semio.any.validate-decode-failed"),
                    severity: dsl::Severity::Warning,
                    span: dsl::TextSpan::at(1, 1),
                    message: "SemioValidator: payload did not decode as a SemioSnapshot — skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    async fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and its `SubsetValidator`.
    /// Called from this artifact's standard-level `engine::register()`.
    pub async fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::any::schema::semio_artifact_schema_descriptor().await);
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioSnapshot, crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation>(
            crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA,
        ).await);
        let _ = register_subset_validator(validator_entry().await);
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.inference`'s facet leaves into the OS-wide inference catalog —
    /// sibling to `register_artifact_schema_descriptor` above (separate registry, ticket
    /// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    pub async fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::any::schema::inferences::semio_artifact_inference_descriptor().await);
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioFormat, SemioAudioSnapshot};

        /// 🧪️ A clean, valid audio snapshot delegates to `SemioAudioValidator` and reports no hard
        /// (error-severity) diagnostics.
        #[semio_framework_async_macros::async_test]
        async fn clean_audio_snapshot_delegates_and_reports_no_errors() {
            let snapshot = SemioSnapshot { subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate: 44_100, format: SemioAudioFormat::Pcm16, ..Default::default() }), ..Default::default() };
            let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = SemioValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().all(|d| d.severity != dsl::Severity::Error), "clean snapshot must not report hard errors: {diagnostics:?}");
        }

        /// 🧪️ An invalid audio snapshot (`sample_rate == 0`, a real invariant `SemioAudioValidator`
        /// checks per `subsets::audio::composer`'s own doc comment) delegates through and the
        /// underlying subset's real diagnostic surfaces unchanged.
        #[semio_framework_async_macros::async_test]
        async fn invalid_audio_snapshot_surfaces_the_delegated_subsets_own_diagnostic() {
            let snapshot = SemioSnapshot { subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate: 0, format: SemioAudioFormat::Pcm16, ..Default::default() }), ..Default::default() };
            let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = SemioValidator::validate(&IoPayload::Binary(bytes));
            assert!(!diagnostics.is_empty(), "zero sample_rate must be flagged");
        }

        /// 🧪️ A payload that doesn't decode as a `SemioSnapshot` at all degrades to the documented
        /// soft warning, never a panic.
        #[semio_framework_async_macros::async_test]
        async fn undecodable_payload_returns_soft_warning_not_panic() {
            let diagnostics = SemioValidator::validate(&IoPayload::Binary(vec![0xff, 0x00, 0x01]));
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(diagnostics[0].severity, dsl::Severity::Warning);
        }

        //#region 🔖️ConformanceLaws
        /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio`'s (the
        /// `✳️any` envelope union) three facets — following `flow`'s/`value`'s own established
        /// pilot pattern. Lives in this composer's own test region: `any` has no per-standard
        /// `⚙️engine` dir the way json/csv/zip/png do, and v1's SHARED
        /// `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` only aggregates all 14 subsets' `register()`
        /// calls (no test module of its own, and out of this ticket's `✳️any/`-only edit scope anyway).
        mod conformance_laws {
            
            use crate::artifacts::semio::standards::v1::subsets::any::schema::{diff, mutations, snapshot};
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

            /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
            /// for the demo snapshot — same preamble-stripped body reconstruction the eventual
            /// `m5_handcrafted_grammar_conformance` harness uses (envelope id prepended as the bare
            /// `artifact-mark` token), so this is a direct proof this facet will pass that harness
            /// once graduated.
            #[semio_framework_async_macros::async_test]
            async fn grammar_conformance_law() {
                let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_semio_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
            /// output for every `SemioMutation` case (`mutations::demo_mutation_cases()`) — all 15
            /// top-level tags, incl. all 13 wrapped subset kinds.
            #[semio_framework_async_macros::async_test]
            async fn ops_grammar_conformance_law() {
                let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                for mutation in mutations::demo_mutation_cases() {
                    let printed = mutation.print_op();
                    assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
                }
            }

            /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
            /// output for every representative `SemioDiff` (`diff::demo_diff_cases()`), incl.
            /// `NoChange`, all 13 same-kind nested tags, and `Replace`.
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
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_semio_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_semio_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[semio_framework_async_macros::async_test]
            async fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/🌐️envelope/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/🌐️envelope/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_semio_snapshot();

                let parsed = <snapshot::SemioSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_semio_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_semio_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_semio_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_semio_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former standard-level `⚙️engine` (ticket 26/08/12/ENGINELESS-
/// ARTIFACTS-AND-APP-STATE-MACHINES) — pure `ComposerEntry` aggregation across all 19 subsets
/// (the 13 domain subsets + `text` + this `✳️any` envelope's own), no engine needed.
pub mod io_registry {
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::SemioAnimationComposer;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::SemioComposer as SemioRawAnyComposer;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::SemioAudioComposer;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::SemioBrepComposer;
    use crate::artifacts::semio::standards::v1::subsets::cad::schema::SemioCadComposer;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::SemioDocumentComposer;
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::SemioDrawingComposer;
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::SemioFlowComposer;
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::SemioGraphComposer;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::SemioImageComposer;
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::SemioKitComposer;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::SemioMeshComposer;
    use crate::artifacts::semio::standards::v1::subsets::model::schema::SemioModelComposer;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::SemioObjectComposer;
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::SemioPresentationComposer;
    use crate::artifacts::semio::standards::v1::subsets::table::schema::SemioTableComposer;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::SemioTextComposer;
    use crate::artifacts::semio::standards::v1::subsets::value::schema::SemioValueComposer;
    use crate::artifacts::semio::standards::v1::subsets::video::schema::SemioVideoComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<SemioBrepComposer>(),
                    composer_entry_of::<SemioMeshComposer>(),
                    composer_entry_of::<SemioModelComposer>(),
                    composer_entry_of::<SemioValueComposer>(),
                    composer_entry_of::<SemioDocumentComposer>(),
                    composer_entry_of::<SemioCadComposer>(),
                    composer_entry_of::<SemioDrawingComposer>(),
                    composer_entry_of::<SemioImageComposer>(),
                    composer_entry_of::<SemioVideoComposer>(),
                    composer_entry_of::<SemioAudioComposer>(),
                    composer_entry_of::<SemioAnimationComposer>(),
                    composer_entry_of::<SemioPresentationComposer>(),
                    composer_entry_of::<SemioFlowComposer>(),
                    composer_entry_of::<SemioTextComposer>(),
                    composer_entry_of::<SemioTableComposer>(),
                    composer_entry_of::<SemioGraphComposer>(),
                    composer_entry_of::<SemioObjectComposer>(),
                    composer_entry_of::<SemioKitComposer>(),
                    composer_entry_of::<SemioRawAnyComposer>(),
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
