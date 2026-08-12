//! 🚪️ IO — composer + subset validator registration for `s.stdio.semio.object`, mirroring every
//! other semio subset's convention. Registration flows through `register()`, called from this
//! standard's `⚙️engine::register()`.
//!
//! ⚠️ OUT OF SCOPE for this wave (deliberately, per this ticket's brief, same as `✳️text`): the
//! `📥️import`/`📤️export` leaves bridging `object` to any format artifact. `io_entries()` is empty;
//! `reads()` only advertises this subset's own native dialect.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{
        ArtifactComposition, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, ComposerEntry, Dialect, IoPayload, StandardId, SubsetId,
        SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of, register_composer_entries,
    };
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::SemioObjectAnalyzer;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

    //#region 🔖️Composer
    pub struct SemioObjectComposerComposition;

    impl ArtifactComposition for SemioObjectComposerComposition {
        type Snapshot = SemioObjectSnapshot;
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
                return Err(ComposeError { message: "SemioObjectComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioObjectAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "SemioObjectComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decode PLUS a real referential-invariant check: every present child handle's `target`
    /// dialect must name the kind the slot declares (`brep`→`s.stdio.semio`/`v1`/`brep`, etc.) —
    /// `object` is the first COMPOSITE subset, so unlike every leaf's decode-only validator, there
    /// is now something genuinely cross-referential to check at this level (the handle's own
    /// declared kind, not the child DOCUMENT's content — dereferencing the child is a host-level
    /// concern, out of scope for a pure decode-time validator).
    pub struct SemioObjectValidator;

    fn wrong_kind(field: &str, expected_subset: &str, target: &store::os_io::ArtifactRef) -> Option<dsl::Diagnostic> {
        if target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.subset != expected_subset {
            Some(dsl::Diagnostic::error(
                "stdio.semio_object.validate-child-kind-mismatch",
                dsl::TextSpan::at(1, 1),
                format!("SemioObjectValidator: `{field}` handle targets {}@{}/{}, expected kind s.stdio.semio subset {expected_subset}", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
            ))
        } else {
            None
        }
    }

    impl SubsetValidator for SemioObjectValidator {
        const DIALECT: Dialect = DIALECT;
        fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            let Some(snapshot) = decoded else {
                return vec![dsl::Diagnostic::error(
                    "stdio.semio_object.validate-decode-failed",
                    dsl::TextSpan::at(1, 1),
                    "SemioObjectValidator: payload did not decode as a SemioObjectSnapshot".to_string(),
                )];
            };
            let mut diagnostics = Vec::new();
            if let Some(brep) = &snapshot.brep { diagnostics.extend(wrong_kind("brep", "brep", &brep.target)); }
            if let Some(mesh) = &snapshot.mesh { diagnostics.extend(wrong_kind("mesh", "mesh", &mesh.target)); }
            if let Some(properties) = &snapshot.properties { diagnostics.extend(wrong_kind("properties", "value", &properties.target)); }
            diagnostics
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioObjectValidator>) }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    fn io_entries() -> &'static [ComposerEntry] { &[] }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::object::schema::semio_object_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioObjectSnapshot, crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation>(crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA));
        register_subset_validator(validator_entry());
        register_composer_entries(io_entries());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        //#region 🔖️ConformanceLaws
        mod conformance_laws {
            use crate::artifacts::semio::standards::v1::subsets::object::schema::{diff, mutations, snapshot};
            use protocol::{DiffCodec, OpBinary, OpText};

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

            #[test]
            fn grammar_conformance_law() {
                let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_object_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            #[test]
            fn ops_grammar_conformance_law() {
                let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                for mutation in mutations::text::demo_mutation_cases() {
                    let printed = mutation.print_op();
                    assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
                }
            }

            #[test]
            fn diff_grammar_conformance_law() {
                let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                for d in diff::demo_diff_cases() {
                    let printed = d.print_diff();
                    assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
                }
            }

            #[test]
            fn protocol_walk_law() {
                let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_object_snapshot());
                let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
                let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

                let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
                for mutation in mutations::text::demo_mutation_cases() {
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

            #[test]
            fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../📚️examples/📦️crate/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/📦️crate/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_object_snapshot();

                let parsed = <snapshot::SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_object_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_object_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioObjectSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_object_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_object_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
