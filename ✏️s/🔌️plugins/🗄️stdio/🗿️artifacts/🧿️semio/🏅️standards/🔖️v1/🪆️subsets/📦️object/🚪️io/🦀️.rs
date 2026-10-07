//! 🚪️ IO — composer + subset validator registration for `s.stdio.semio.object`, mirroring every
//! other semio subset's convention. Registration flows through `register()`, called from this
//! standard's `⚙️engine::register()`.
//!
//! ⚠️ OUT OF SCOPE for this wave (deliberately, per this ticket's brief, same as `🔤️text`): the
//! `📥️import`/`📤️export` leaves bridging `object` to any format artifact. `io_entries()` is empty;
//! `reads()` only advertises this subset's own native dialect.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
    use crate::standards::v1::subsets::object::io::SemioObjectAnalyzer;
    use {semio_framework_plugin::register_composer_entries,semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::ComposerEntry,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

    //#region 🔖️Composer
    pub struct SemioObjectComposerComposition;

    impl ArtifactComposition for SemioObjectComposerComposition {
        type Snapshot = SemioObjectSnapshot;
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
                return Err(ComposeError { message: "SemioObjectComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioObjectAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioObjectComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn wrong_kind(field: &str, expected_subset: &str, target: &semio_framework_artifact_reference::ArtifactRef) -> Option<semio_framework_diagnostic::Diagnostic> {
        if target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.subset != expected_subset {
            Some(semio_framework_diagnostic::Diagnostic::error(
                "stdio.semio_object.validate-child-kind-mismatch",
                semio_framework_diagnostic::TextSpan::at(1, 1),
                format!("SemioObjectValidator: `{field}` handle targets {}@{}/{}, expected kind s.stdio.semio subset {expected_subset}", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
            ))
        } else {
            None
        }
    }

    impl SubsetValidator for SemioObjectValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            let Some(snapshot) = decoded else {
                return vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_object.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioObjectValidator: payload did not decode as a SemioObjectSnapshot".to_string())];
            };
            let mut diagnostics = Vec::new();
            if let Some(brep) = &snapshot.brep {
                diagnostics.extend(wrong_kind("brep", "brep", &brep.target));
            }
            if let Some(mesh) = &snapshot.mesh {
                diagnostics.extend(wrong_kind("mesh", "mesh", &mesh.target));
            }
            if let Some(properties) = &snapshot.properties {
                diagnostics.extend(wrong_kind("properties", "value", &properties.target));
            }
            diagnostics
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioObjectValidator>)
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
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::object::schema::semio_object_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("object") }, store::ArtifactCodec::of::<SemioObjectSnapshot, crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation>(
            crate::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA,
        ))
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
            .schemas([crate::standards::v1::subsets::object::schema::semio_object_artifact_schema_descriptor()])
            .document_codec_bare::<SemioObjectSnapshot, crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation>(crate::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("object") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::object::schema::inferences::semio_object_artifact_inference_descriptor()])
            .composers(crate::semio_written(io_entries(), &COMPOSERS))
    }

    /// 💡️ Registers `s.stdio.semio.object.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::object::schema::inferences::semio_object_artifact_inference_descriptor()).expect("schema descriptor publication");
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
    use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
    use crate::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
    use crate::standards::v1::subsets::object::schema::mutations::{apply_semio_object_mutation, SemioObjectMutation};
    use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioObjectBuilderConstruction {
        snapshot: SemioObjectSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioObjectBuilderConstruction {
        /// 🏗️ Starts a fresh object at the identity transform, no geometry/properties children.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: SemioObjectSnapshot::default() }
        }
        /// 🧭️ Overrides the object's placement.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_transform(mut self, transform: SemioTransform) -> Self {
            self.snapshot.transform = transform;
            self
        }
        /// 🧱️ Attaches an owned brep CHILD handle (never embedded content).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_brep(mut self, child_id: impl Into<String>, target: semio_framework_artifact_reference::ArtifactRef) -> Result<Self, String> {
            let child_id = child_id.into();
            crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&child_id, &target, "brep")?;
            self.snapshot.brep = Some(store::ArtifactChild::new(child_id, target));
            Ok(self)
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioObjectBuilderConstruction {
        type Snapshot = SemioObjectSnapshot;
        type Mutation = SemioObjectMutation;
        type Diff = SemioObjectDiff;
        fn empty() -> Self {
            Self { snapshot: SemioObjectSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_object_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioObjectDiff as protocol::MutationDiff<SemioObjectSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            self.snapshot.validate().map_err(|message| vec![semio_framework_diagnostic::Diagnostic::error("object.document", semio_framework_diagnostic::TextSpan::at(1, 1), message)])?;
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
    use crate::standards::v1::subsets::object::schema::snapshot::{SemioObjectSnapshot, STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioObjectParts {
        pub snapshot: Option<SemioObjectSnapshot>,
    }

    pub struct SemioObjectAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioObjectAnalyzerAnalysis {
        type Parts = SemioObjectParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioObjectParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.object.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.object.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
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
    pub spec SemioObjectBuilderFacets {
        construction: SemioObjectBuilderConstruction,
        analysis: SemioObjectAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::object::io::derived_composition::SemioObjectComposerComposition,
    }
    builder: SemioObjectBuilder,
    analyzer: SemioObjectAnalyzer,
    composer: SemioObjectComposer,
);
