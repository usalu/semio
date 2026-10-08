//! 🚪️ IO — composer + subset validator registration for `s.stdio.semio.kit`, mirroring every
//! other semio subset's convention. Registration flows through `register()`, called from this
//! standard's `⚙️engine::register()`.
//!
//! ⚠️ OUT OF SCOPE for this wave (deliberately, per this ticket's brief, same as `🔤️text`/`📦️object`):
//! the `📥️import`/`📤️export` leaves bridging `kit` to any format artifact, and dissolving puzzle/
//! three-block's separately-declared `kit.catalog` artifact kind into this subset (a later wave's
//! concern — repointing those apps' `AppSchema::artifact_kind()` registrations). `io_entries()` is
//! empty; `reads()` only advertises this subset's own native dialect.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
    use crate::standards::v1::subsets::kit::io::SemioKitAnalyzer;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::ComposerEntry,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("kit") };

    //#region 🔖️Composer
    pub struct SemioKitComposerComposition;

    impl ArtifactComposition for SemioKitComposerComposition {
        type Snapshot = SemioKitSnapshot;
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
                return Err(ComposeError { message: "SemioKitComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioKitAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioKitComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes and verifies exact child identity plus the complete Semio dialect tuple. The LINK
    /// pool (`representations`) is intentionally not kind-checked here — a
    /// link may legitimately point at any independent artifact kind (image/mesh/whatever a
    /// catalog's representation happens to be), so there is no single expected kind to assert.
    pub struct SemioKitValidator;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn wrong_child<S>(field: &str, expected_subset: &str, child: &store::ArtifactChild<S>) -> Option<semio_framework_diagnostic::Diagnostic> {
        if let Err(message) = crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&child.child_id, &child.target, expected_subset) {
            Some(semio_framework_diagnostic::Diagnostic::error(
                "stdio.semio_kit.validate-child-identity-mismatch",
                semio_framework_diagnostic::TextSpan::at(1, 1),
                format!("SemioKitValidator: `{field}` {message}"),
            ))
        } else {
            None
        }
    }

    impl SubsetValidator for SemioKitValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioKitSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            let Some(snapshot) = decoded else {
                return vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_kit.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioKitValidator: payload did not decode as a SemioKitSnapshot".to_string())];
            };
            let mut diagnostics = Vec::new();
            for object in &snapshot.objects {
                diagnostics.extend(wrong_child("objects", "object", object));
            }
            for model in &snapshot.models {
                diagnostics.extend(wrong_child("models", "model", model));
            }
            if let Some(properties) = &snapshot.properties {
                diagnostics.extend(wrong_child("properties", "value", properties));
            }
            diagnostics
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioKitValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn io_entries() -> &'static [ComposerEntry] {
        &[]
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::kit::schema::semio_kit_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("kit") }, store::ArtifactCodec::of::<SemioKitSnapshot, crate::standards::v1::subsets::kit::schema::mutations::SemioKitMutation>(crate::standards::v1::subsets::kit::schema::snapshot::STDIO_SEMIOKIT_DOCUMENT_SCHEMA))
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
            .schemas([crate::standards::v1::subsets::kit::schema::semio_kit_artifact_schema_descriptor()])
            .document_codec_bare::<SemioKitSnapshot, crate::standards::v1::subsets::kit::schema::mutations::SemioKitMutation>(crate::standards::v1::subsets::kit::schema::snapshot::STDIO_SEMIOKIT_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("kit") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::kit::schema::inferences::semio_kit_artifact_inference_descriptor()])
            .composers(crate::semio_written(io_entries(), &COMPOSERS))
    }

    /// 💡️ Registers `s.stdio.semio.kit.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::kit::schema::inferences::semio_kit_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
    use crate::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation};
    use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioKitBuilderConstruction {
        snapshot: SemioKitSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioKitBuilderConstruction {
        /// 🏗️ Starts a fresh, empty kit (no types/designs/geometry).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: SemioKitSnapshot::default() }
        }
        /// 🏷️ Appends one TYPE to the catalog.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_type(mut self, id: impl Into<String>, name: impl Into<String>, category: impl Into<String>) -> Self {
            self.snapshot.types.push(SemioKitType { id: id.into(), name: name.into(), category: category.into() });
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioKitBuilderConstruction {
        type Snapshot = SemioKitSnapshot;
        type Mutation = SemioKitMutation;
        type Diff = SemioKitDiff;
        fn empty() -> Self {
            Self { snapshot: SemioKitSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioKitSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => (Self { snapshot, ..self }, outcome),
                Err(error) => (self, protocol::MutationOutcome::fatal(error.code, error.message, error.target)),
            }
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, STDIO_SEMIOKIT_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioKitParts {
        pub snapshot: Option<SemioKitSnapshot>,
    }

    pub struct SemioKitAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioKitAnalyzerAnalysis {
        type Parts = SemioKitParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("kit") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOKIT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOKIT_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioKitParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.kit.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioKitSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.kit.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioKitBuilderFacets {
        construction: SemioKitBuilderConstruction,
        analysis: SemioKitAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::kit::io::derived_composition::SemioKitComposerComposition,
    }
    builder: SemioKitBuilder,
    analyzer: SemioKitAnalyzer,
    composer: SemioKitComposer,
);
