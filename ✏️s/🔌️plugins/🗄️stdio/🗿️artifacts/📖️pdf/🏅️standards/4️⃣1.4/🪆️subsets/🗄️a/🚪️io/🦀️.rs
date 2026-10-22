//! 🚪️ IO stdio.pdf (1.4/🗄️a) — reuses the 🧱️base subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! The standard-level aggregator registers the `ComposerEntry`; the declaration's subset-validator table
//! registers this subset's validator, whose `SnapshotSubsetValidator` half is the owned-snapshot policy the
//! base SQLite codec resolves by dialect.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_4::subsets::a::io::check_pdf_a_conformance;
    use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
    use crate::standards::v1_4::subsets::base::io::PdfComposer as PdfAnyComposer;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator};

    const DIALECT_A: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("a") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfAComposerComposition;

    impl ArtifactComposition for PdfAComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_A;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_A, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = PdfAnyComposer::compose(sources)?;
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(check_pdf_a_conformance(&inner.snapshot));
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfAValidator;

    impl SubsetValidator for PdfAValidator {
        const DIALECT: Dialect = DIALECT_A;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_pdf_a_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.a.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/A (1.4) SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    impl store::io::SnapshotSubsetValidator for PdfAValidator {
        type Snapshot = PdfSnapshot;

        fn validate_snapshot(snapshot: &PdfSnapshot, _database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
            control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot, 1, 1).map_err(store::io_schema::IoError::from_value_error)?;
            Ok(store::io_schema::IoOutcome { value: (), diagnostics: check_pdf_a_conformance(snapshot) })
        }
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

/// 🌉️ Applies one a conformance mutation through its leaf-owned diff and the base bridge.
pub mod mutation_bridge {
    use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
    use crate::standards::v1_4::subsets::a::schema::mutations::PdfA1Mutation;

    /// ▶️ Applies the authoritative leaf diff.
    pub fn apply_a_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfA1Mutation) -> protocol::MutationOutcome<PdfDiff> {
        use protocol::{ColdMutation, Mutation};
        let outcome = mutation.diff(snapshot);
        crate::standards::v1_4::subsets::base::io::mutation_bridge::apply_outcome(outcome, snapshot)
    }
}

pub mod derived_construction {
    use crate::standards::v1_4::subsets::a::io::check_pdf_a_conformance;
    use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff, mutations::PdfMutation, snapshot::PdfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct PdfABuilderConstruction {
        snapshot: PdfSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for PdfABuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        fn empty() -> Self {
            Self { snapshot: PdfSnapshot::default(), diagnostics: Vec::new() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::standards::v1_4::subsets::base::io::mutation_bridge::apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff_cold(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ Re-runs the honestly-scope-limited PDF/A-1 check -- always SOFT at this schema, so
        /// `build()` never fails; the diagnostics still surface via the analyzer/composer/validator
        /// paths for anyone inspecting them.
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            let _ = check_pdf_a_conformance(&self.snapshot);
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
    use crate::standards::v1_4::subsets::base::io::{PdfAnalyzer as PdfAnyAnalyzer, PdfParts};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("a") };

    //#region 🔖️Conformance
    pub const CODE_TEXT_EMPTY: &str = "stdio.pdf.a.text-empty";
    pub const CODE_SCHEMA_GAP: &str = "stdio.pdf.a.schema-gap-unverifiable";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Honestly-scope-limited PDF/A-1 conformance check against one already-decoded `PdfSnapshot`.
    /// Shared single source of truth: `PdfAComposer::compose` (pass-through, can't hard-gate without
    /// an object graph) and the registered `SubsetValidator` both call this.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_pdf_a_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        if snapshot.first_page_text().trim().is_empty() {
            out.push(soft(
                CODE_TEXT_EMPTY,
                "page 1 shows no extractable text -- read off the first page of the document's page tree, the page ISO 19005-1's readable-content axis is checked on here; a very weak signal, but a real one given PageDoc has no other checkable field"
                    .into(),
            ));
        }
        out.push(soft(
            CODE_SCHEMA_GAP,
            "PDF 1.4's retained snapshot has no object graph; full ISO 19005-1 / ISO 15930 conformance cannot be checked from this schema; upgrade 1.4's engine to retain the object graph (see 1.7's PdfSnapshot.objects: Vec<PdfIndirectObject>) to implement real checks here.".into(),
        ));
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pdf` (1.4/🗄️a): delegates the real parse to the 🧱️base subset's analyzer
    /// (same `PdfSnapshot`), then folds the honestly-scope-limited PDF/A-1 diagnostics on top.
    pub struct PdfAAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfAAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            PdfAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            if let Some(snapshot) = &inner.parts.snapshot {
                diagnostics.extend(check_pdf_a_conformance(snapshot));
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence: inner.confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfABuilderFacets {
        construction: PdfABuilderConstruction,
        analysis: PdfAAnalyzerAnalysis,
        composition: PdfAComposerComposition,
    }
    builder: PdfABuilder,
    analyzer: PdfAAnalyzer,
    composer: PdfAComposer,
);
