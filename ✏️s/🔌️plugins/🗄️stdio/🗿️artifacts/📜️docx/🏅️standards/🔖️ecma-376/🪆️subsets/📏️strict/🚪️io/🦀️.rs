//! 🚪️ IO stdio.docx (ecma-376/📏️strict) — doc-leaf only, referencing the owning ✳️any subset's
//! import/export tree (`🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/`) rather than duplicating
//! it. Registration flows through `🎹️composer::register` (the `SubsetValidator` directly, the
//! `ComposerEntry` via the standard-level aggregator), not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::DocxComposer as DocxAnyComposer;
    use crate::standards::v_ecma_376::subsets::strict::schema::conformance::check_strict_conformance;
    use crate::DocxSnapshot;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_STRICT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct DocxStrictComposerComposition;

    impl ArtifactComposition for DocxStrictComposerComposition {
        type Snapshot = DocxSnapshot;
        const WRITES: Dialect = DIALECT_STRICT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_STRICT, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = DocxAnyComposer::compose(sources)?;
            let checks = check_strict_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 29500-1 Strict conformance violated: {} hard issue(s) -- not stamping the strict dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ecma-376/strict` -- see the module doc comment for
    /// how this relates to the composer's own pre-serialization hard gate above.
    pub struct DocxStrictValidator;

    impl SubsetValidator for DocxStrictValidator {
        const DIALECT: Dialect = DIALECT_STRICT;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <DocxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_strict_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.docx.strict.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "docx strict SubsetValidator: payload did not decode as a DocxSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<DocxStrictValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the ecma-376 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is aggregated separately by the standard-level composer
    /// (`crate::standards::v_ecma_376::engine::io_registry::entries()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub mod derived_construction {
    #[cfg(test)]
    use crate::schema::snapshot::{DocxDocument, DocxParagraph, DocxRun, DocxXmlPart};
    use crate::standards::v_ecma_376::subsets::strict::schema::conformance::{check_strict_conformance, STRICT_REL_BASE};
    use crate::{DocxDiff, DocxMutation, DocxSnapshot};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;
    #[cfg(test)]
    use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text;
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, RELS_CONTENT_TYPE};

    use crate::standards::v_ecma_376::subsets::strict::schema::construction::build_minimal_strict_docx;


    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct DocxStrictBuilderConstruction {
        snapshot: DocxSnapshot,
    }

    impl DocxStrictBuilderConstruction {
        /// ➕️ Appends a paragraph, re-serializing the strict-namespaced `word/document.xml` part.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_paragraph(mut self, paragraph: DocxParagraph) -> Self {
            let mut document = self.snapshot.project_document().unwrap_or_default();
            document.body.push(crate::schema::snapshot::DocxBlock::Paragraph(paragraph));
            self.snapshot = build_minimal_strict_docx(document);
            self
        }

        /// ➕️ Appends a single-run plain-text paragraph.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_text_paragraph(self, text: impl Into<String>) -> Self {
            self.add_paragraph(DocxParagraph::text(text.into()))
        }

        /// ➕️ Appends a paragraph made of the given runs (basic bold/italic/underline formatting).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_runs(self, runs: Vec<DocxRun>) -> Self {
            self.add_paragraph(DocxParagraph { runs, style: None, extra_paragraph_properties: Vec::new() })
        }
    }

    impl ArtifactBuilder for DocxStrictBuilderConstruction {
        type Snapshot = DocxSnapshot;
        type Mutation = DocxMutation;
        type Diff = DocxDiff;

        fn empty() -> Self {
            Self { snapshot: build_minimal_strict_docx(DocxDocument::default()) }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (next, diff) = store::apply_outcome(&self.snapshot, protocol::Mutation::diff(&mutation, &self.snapshot));
            self.snapshot = next;
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: re-runs `check_strict_conformance` unconditionally,
        /// regardless of which path produced the in-flight snapshot (typed `add_paragraph`,
        /// `from_binary`, a raw `UpsertInstance`-style mutation) -- a hard violation can never leave `build()`
        /// as `Ok`.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_strict_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v_ecma_376::subsets::base::io::{DocxAnalyzer as DocxAnyAnalyzer, DocxParts};
    use crate::{schema::snapshot::DocxXmlPart, DocxSnapshot};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };

    use crate::standards::v_ecma_376::subsets::strict::schema::conformance::*;


    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.docx` (ecma-376/📏️strict): delegates the real parse to the ✳️any subset's
    /// analyzer (same `DocxSnapshot`), then folds real ISO/IEC 29500-1 Strict conformance diagnostics
    /// on top.
    pub struct DocxStrictAnalyzerAnalysis;

    impl ArtifactAnalysis for DocxStrictAnalyzerAnalysis {
        type Parts = DocxParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            DocxAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = DocxAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_strict_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = semio_framework_plugin::io::Confidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec DocxStrictBuilderFacets {
        construction: DocxStrictBuilderConstruction,
        analysis: DocxStrictAnalyzerAnalysis,
        composition: super::io::derived_composition::DocxStrictComposerComposition,
    }
    builder: DocxStrictBuilder,
    analyzer: DocxStrictAnalyzer,
    composer: DocxStrictComposer,
);

#[path = "📝️text"]
pub mod text { #[path = "🧬️mutations/🦀️.rs"] pub mod mutations; }
#[path = "💾️binary"]
pub mod binary { #[path = "🧬️mutations/🦀️.rs"] pub mod mutations; }
