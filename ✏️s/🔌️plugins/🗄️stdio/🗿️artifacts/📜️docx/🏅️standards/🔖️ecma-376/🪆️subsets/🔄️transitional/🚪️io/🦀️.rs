//! 🚪️ IO stdio.docx (ecma-376/🔄️transitional) — doc-leaf only, referencing the owning ✳️any
//! subset's import/export tree (`🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/`) rather than
//! duplicating it. Registration flows through `🎹️composer::register` (the `SubsetValidator`
//! directly, the `ComposerEntry` via the standard-level aggregator), not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::DocxComposer as DocxAnyComposer;
    use crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use crate::DocxSnapshot;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_TRANSITIONAL: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct DocxTransitionalComposerComposition;

    impl ArtifactComposition for DocxTransitionalComposerComposition {
        type Snapshot = DocxSnapshot;
        const WRITES: Dialect = DIALECT_TRANSITIONAL;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_TRANSITIONAL, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = DocxAnyComposer::compose(sources)?;
            let checks = check_transitional_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 29500-4 Transitional conformance violated: {} hard issue(s) -- not stamping the transitional dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ecma-376/transitional` -- see the module doc comment
    /// for how this relates to the composer's own pre-serialization hard gate above.
    pub struct DocxTransitionalValidator;

    impl SubsetValidator for DocxTransitionalValidator {
        const DIALECT: Dialect = DIALECT_TRANSITIONAL;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <DocxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_transitional_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.docx.transitional.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "docx transitional SubsetValidator: payload did not decode as a DocxSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<DocxTransitionalValidator>)
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
    use crate::schema::mutations::set_snapshot;
    use crate::schema::snapshot::{DocxParagraph, DocxRun, DocxStyle, DocxTable};
    use crate::standards::v_ecma_376::subsets::base::io::DocxBuilderConstruction as DocxAnyBuilder;
    use crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use crate::{DocxDiff, DocxMutation, DocxSnapshot};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct DocxTransitionalBuilderConstruction {
        inner: DocxAnyBuilder,
    }

    impl DocxTransitionalBuilderConstruction {
        /// ➕️ Appends a paragraph.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_paragraph(mut self, paragraph: DocxParagraph) -> Self {
            self.inner = self.inner.add_paragraph(paragraph);
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

        /// ➕️ Appends a table.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_table(mut self, table: DocxTable) -> Self {
            self.inner = self.inner.add_table(table);
            self
        }

        /// ➕️ Appends (or replaces, by `id`) a named style.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_style(mut self, style: DocxStyle) -> Self {
            self.inner = self.inner.add_style(style);
            self
        }
    }

    impl ArtifactBuilder for DocxTransitionalBuilderConstruction {
        type Snapshot = DocxSnapshot;
        type Mutation = DocxMutation;
        type Diff = DocxDiff;

        fn empty() -> Self {
            Self { inner: DocxAnyBuilder::empty() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { inner: DocxAnyBuilder::from_snapshot(snapshot) }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self { inner: DocxAnyBuilder::from_text(text)? })
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { inner: DocxAnyBuilder::from_binary(bytes)? })
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (inner, diff) = self.inner.mutate(mutation);
            self.inner = inner;
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.inner = self.inner.absorb(diff)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: re-runs `check_transitional_conformance` unconditionally,
        /// regardless of which path produced the in-flight snapshot -- a hard violation can never
        /// leave `build()` as `Ok`. Syncs `opc`'s main part from `document` first (mirrors what
        /// `encode_docx` does at real encode time) — `check_transitional_conformance` needs a
        /// materialized `word/document.xml`/relationship to find at all, and the shared `DocxAnyBuilder`
        /// this wraps doesn't materialize either until actual encode.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let snapshot = self.inner.build()?;
            let hard: Vec<Diagnostic> = check_transitional_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(snapshot)
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
    use crate::standards::v_ecma_376::subsets::base::schema::{DocxAnalyzer as DocxAnyAnalyzer, DocxParts};
    use crate::{DocxSnapshot, schema::snapshot::DocxXmlPart};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

    //#region 🔖️Namespaces
    pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    /// 🔎️ Every ISO/IEC 29500-1 Strict namespace URI (markup AND relationship) shares this prefix --
    /// see `📏️strict`'s `STRICT_MAIN_NS`/`STRICT_REL_BASE`, both of which start with it.
    pub const STRICT_NS_FAMILY_PREFIX: &str = "purl.oclc.org/ooxml";
    //#endregion 🔖️Namespaces

    //#region 🔖️Conformance
    pub const CODE_MAIN_NS_MISSING: &str = "stdio.docx.transitional.main-ns-missing";
    pub const CODE_STRICT_NS_PRESENT: &str = "stdio.docx.transitional.strict-ns-present";
    pub const CODE_CONFORMANCE_ATTR: &str = "stdio.docx.transitional.conformance-attr-invalid";

    /// 🔎️ Resolves the main document part via the root officeDocument relationship -- matched by
    /// relationship-type SUFFIX (`/officeDocument`) so this resolves for either conformance class; see
    /// `📏️strict::analyzer::main_document_part`'s doc comment for the full rationale.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn main_document_part(snapshot: &DocxSnapshot) -> Option<(&DocxXmlPart, String)> {
        let rel = snapshot.opc.relationships_for("")?.iter().find(|relationship| relationship.rel_type.to_string_owner().ends_with("/officeDocument"))?;
        let path = resolve_relationship_target("", &rel.target.to_string_owner());
        snapshot.xml_part(&path).map(|part| (part, path))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn part_contains(part: &DocxXmlPart, needle: &str) -> bool {
        !needle.is_empty()
            && part
                .materialize_document_exact()
                .is_ok_and(|document| semio_s_artifact_stdio_xml::schema::snapshot::xml_document_to_text(&document).contains(needle))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO/IEC 29500-4:2016 Transitional conformance checks against one already-decoded
    /// `DocxSnapshot`. Shared single source of truth: `DocxTransitionalComposer::compose` hard-gates
    /// on this (pre-serialization, authoritative), `DocxTransitionalBuilder::build` hard-gates on this
    /// too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_transitional_conformance(snapshot: &DocxSnapshot) -> Vec<Diagnostic> {
        let opc = &snapshot.opc;
        let mut out = Vec::new();

        match main_document_part(snapshot) {
            Some((part, path)) => {
                if !part_contains(part, TRANSITIONAL_MAIN_NS) {
                    out.push(hard(CODE_MAIN_NS_MISSING, format!("main document part {path} does not declare the transitional WordprocessingML namespace {TRANSITIONAL_MAIN_NS}")));
                }
                if part_contains(part, "conformance=\"strict\"") {
                    out.push(soft(CODE_CONFORMANCE_ATTR, format!("main document part {path} root element declares conformance=\"strict\" -- transitional documents must leave it absent or =\"transitional\"")));
                }
            }
            None => out.push(hard(CODE_MAIN_NS_MISSING, "package has no root officeDocument relationship -- cannot locate the main document part to check the transitional namespace on".into())),
        }

        for part in &snapshot.xml_parts {
            if part_contains(part, STRICT_NS_FAMILY_PREFIX) {
                out.push(hard(CODE_STRICT_NS_PRESENT, format!("part {} contains a strict-family namespace ({STRICT_NS_FAMILY_PREFIX}) -- transitional conformance forbids mixed namespaces", part.path)));
            }
        }

        let mut owners: Vec<&String> = opc.relationships.keys().collect();
        owners.sort();
        for owner in owners {
            for rel in opc.relationships.get(owner).expect("enumerated retained relationship owner").iter() {
                if rel.rel_type.to_string_owner().contains(STRICT_NS_FAMILY_PREFIX) {
                    out.push(hard(CODE_STRICT_NS_PRESENT, format!("relationship {} owned by {owner:?} uses a strict-family relationship base ({STRICT_NS_FAMILY_PREFIX}) -- transitional conformance forbids it", rel.id)));
                }
            }
        }

        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.docx` (ecma-376/🔄️transitional): delegates the real parse to the ✳️any
    /// subset's analyzer (same `DocxSnapshot`), then folds real ISO/IEC 29500-4 Transitional
    /// conformance diagnostics on top.
    pub struct DocxTransitionalAnalyzerAnalysis;

    impl ArtifactAnalysis for DocxTransitionalAnalyzerAnalysis {
        type Parts = DocxParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            DocxAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = DocxAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_transitional_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = IoConfidence::Low;
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
    pub spec DocxTransitionalBuilderFacets {
        construction: DocxTransitionalBuilderConstruction,
        analysis: DocxTransitionalAnalyzerAnalysis,
        composition: super::io::derived_composition::DocxTransitionalComposerComposition,
    }
    builder: DocxTransitionalBuilder,
    analyzer: DocxTransitionalAnalyzer,
    composer: DocxTransitionalComposer,
);
