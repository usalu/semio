//! 🚪️ IO — composer + subset validator registration for `s.stdio.semio.table`, mirroring every
//! other semio subset's convention. Registration flows through `register()`, called from this
//! standard's `⚙️engine::register()`.
//!
//! ⚠️ OUT OF SCOPE for this wave (deliberately, per this ticket's brief, mirroring `🔤️text`'s own
//! same decision): the `📥️import`/`📤️export` leaves bridging `table` to the csv/tsv/xlsx format
//! artifacts. That is hub routing, a separate concern for a later wave — `io_entries()` below is
//! empty and `reads()` only advertises this subset's own native dialect.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
    use crate::standards::v1::subsets::table::io::SemioTableAnalyzer;
    use semio_framework_plugin::{
        register_composer_entries, register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator,
        SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("table") };

    //#region 🔖️Composer
    pub struct SemioTableComposerComposition;

    impl ArtifactComposition for SemioTableComposerComposition {
        type Snapshot = SemioTableSnapshot;
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
                return Err(ComposeError { message: "SemioTableComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioTableAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioTableComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decode-only validator (no referential-invariant diagnostics — `table` is a leaf with no
    /// child/link slots, so there is nothing cross-referential to check). The row/column alignment
    /// invariant is maintained by every mutation triad's own `🔺️diff` leaf, not re-validated here.
    pub struct SemioTableValidator;

    impl SubsetValidator for SemioTableValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioTableSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(_) => Vec::new(),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_table.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioTableValidator: payload did not decode as a SemioTableSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioTableValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ Empty — no csv/tsv/xlsx format bridges in this wave (see module doc comment).
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
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::table::schema::semio_table_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("table") }, store::ArtifactCodec::of::<SemioTableSnapshot, crate::standards::v1::subsets::table::schema::mutations::SemioTableMutation>(
            crate::standards::v1::subsets::table::schema::snapshot::STDIO_SEMIOTABLE_DOCUMENT_SCHEMA,
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
            .schemas([crate::standards::v1::subsets::table::schema::semio_table_artifact_schema_descriptor()])
            .document_codec_bare::<SemioTableSnapshot, crate::standards::v1::subsets::table::schema::mutations::SemioTableMutation>(crate::standards::v1::subsets::table::schema::snapshot::STDIO_SEMIOTABLE_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_plugin::StandardId("v1"), subset: semio_framework_plugin::SubsetId("table") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::table::schema::inferences::semio_table_artifact_inference_descriptor()])
            .composers(crate::semio_written(io_entries(), &COMPOSERS))
    }

    /// 💡️ Registers `s.stdio.semio.table.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::table::schema::inferences::semio_table_artifact_inference_descriptor()).expect("schema descriptor publication");
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
    use crate::standards::v1::subsets::table::schema::diff::SemioTableDiff;
    use crate::standards::v1::subsets::table::schema::mutations::{apply_semio_table_mutation, SemioTableMutation};
    use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioTableBuilderConstruction {
        snapshot: SemioTableSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioTableBuilderConstruction {
        /// 🏗️ Starts a fresh, empty table document.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: SemioTableSnapshot::default() }
        }
        /// 🏗️ Appends one column, in order.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_column(mut self, name: impl Into<String>, kind: SemioTableCellKind) -> Self {
            self.snapshot.columns.push(SemioTableColumn { name: name.into(), kind });
            self
        }
        /// 🏗️ Appends one row, in order.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_row(mut self, row: SemioTableRow) -> Self {
            self.snapshot.rows.push(row);
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioTableBuilderConstruction {
        type Snapshot = SemioTableSnapshot;
        type Mutation = SemioTableMutation;
        type Diff = SemioTableDiff;
        fn empty() -> Self {
            Self { snapshot: SemioTableSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioTableSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_table_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioTableDiff as protocol::MutationDiff<SemioTableSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioTableParts {
        pub snapshot: Option<SemioTableSnapshot>,
    }

    pub struct SemioTableAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioTableAnalyzerAnalysis {
        type Parts = SemioTableParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("table") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOTABLE_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioTableParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioTableSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
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
    pub spec SemioTableBuilderFacets {
        construction: SemioTableBuilderConstruction,
        analysis: SemioTableAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::table::io::derived_composition::SemioTableComposerComposition,
    }
    builder: SemioTableBuilder,
    analyzer: SemioTableAnalyzer,
    composer: SemioTableComposer,
);
