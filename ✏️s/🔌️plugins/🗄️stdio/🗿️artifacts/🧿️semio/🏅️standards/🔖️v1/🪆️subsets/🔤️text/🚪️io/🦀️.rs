//! 🚪️ IO — composer + subset validator registration for `s.stdio.semio.text`, mirroring every
//! other semio subset's convention. Registration flows through `register()`, called from this
//! standard's `⚙️engine::register()`.
//!
//! ⚠️ OUT OF SCOPE for this wave (deliberately, per this ticket's brief): the `📥️import`/
//! `📤️export` leaves bridging `text` to the txt/md format artifacts. That is hub routing, a
//! separate concern for a later wave — `io_entries()` below is empty and `reads()` only advertises
//! this subset's own native dialect.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
    use crate::standards::v1::subsets::text::schema::SemioTextAnalyzer;
    use semio_framework_plugin::{
        register_composer_entries, register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator,
        SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("text") };

    //#region 🔖️Composer
    pub struct SemioTextComposerComposition;

    impl ArtifactComposition for SemioTextComposerComposition {
        type Snapshot = SemioTextSnapshot;
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
                return Err(ComposeError { message: "SemioTextComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioTextAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioTextComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decode-only validator (no referential-invariant diagnostics — `text` is a leaf with no
    /// child/link slots, so there is nothing cross-referential to check).
    pub struct SemioTextValidator;

    impl SubsetValidator for SemioTextValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioTextSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(_) => Vec::new(),
                None => vec![dsl::Diagnostic::error("stdio.semio_text.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioTextValidator: payload did not decode as a SemioTextSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioTextValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ Empty — no txt/md format bridges in this wave (see module doc comment).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn io_entries() -> &'static [ComposerEntry] {
        &[]
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called
    /// from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::text::schema::semio_text_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("text") }, store::ArtifactCodec::of::<SemioTextSnapshot, crate::standards::v1::subsets::text::schema::mutations::SemioTextMutation>(crate::standards::v1::subsets::text::schema::snapshot::STDIO_SEMIOTEXT_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(io_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        builder
            .schemas([crate::standards::v1::subsets::text::schema::semio_text_artifact_schema_descriptor()])
            .document_codec_bare::<SemioTextSnapshot, crate::standards::v1::subsets::text::schema::mutations::SemioTextMutation>(crate::standards::v1::subsets::text::schema::snapshot::STDIO_SEMIOTEXT_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("text") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::text::schema::inferences::semio_text_artifact_inference_descriptor()])
            .composers(crate::semio_written(io_entries(), &COMPOSERS))
    }

    /// 💡️ Registers `s.stdio.semio.text.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::text::schema::inferences::semio_text_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
