//! 🚪️ IO stdio.pptx (ecma-376/🌉️transitional) — reuses the 🧱️base subset's `zip`/`xml` raw-codec
//! DAG leaves rather than duplicating them (same `PptxSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `🧱️base/🚪️io` already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::PptxComposer as PptxAnyComposer;
    use crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use crate::PptxSnapshot;
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_diagnostic::FaultCode;
    use semio_framework_diagnostic::Severity;
    use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_TRANSITIONAL: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PptxTransitionalComposerComposition;

    impl ArtifactComposition for PptxTransitionalComposerComposition {
        type Snapshot = PptxSnapshot;
        const WRITES: Dialect = DIALECT_TRANSITIONAL;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_TRANSITIONAL, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = PptxAnyComposer::compose(sources)?;
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
    pub struct PptxTransitionalValidator;

    impl SubsetValidator for PptxTransitionalValidator {
        const DIALECT: Dialect = DIALECT_TRANSITIONAL;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PptxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PptxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_transitional_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pptx.transitional.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Transitional SubsetValidator: payload did not decode as a PptxSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PptxTransitionalValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from ecma-376's own `⚙️engine::register()`. The `ComposerEntry`
    /// itself is registered separately by the standard-level composer aggregator
    /// (`crate::standards::v_ecma_376::engine::io_registry::entries()`), matching how `🧱️base`'s
    /// own entry is registered.
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
    use crate::standards::v_ecma_376::subsets::base::io::PptxBuilder as PptxAnyBuilder;
    use crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use crate::{PptxDiff, PptxMutation, PptxSnapshot};
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct PptxTransitionalBuilderConstruction {
        inner: PptxAnyBuilder,
    }

    impl ArtifactBuilder for PptxTransitionalBuilderConstruction {
        type Snapshot = PptxSnapshot;
        type Mutation = PptxMutation;
        type Diff = PptxDiff;

        fn empty() -> Self {
            Self { inner: PptxAnyBuilder::empty() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { inner: PptxAnyBuilder::from_snapshot(snapshot) }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self { inner: PptxAnyBuilder::from_text(text)? })
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { inner: PptxAnyBuilder::from_binary(bytes)? })
        }

        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (inner, diff) = self.inner.mutate(mutation);
            (Self { inner }, diff)
        }

        fn absorb(self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            Ok(Self { inner: self.inner.absorb(diff)? })
        }

        /// 🛡️ The real construction gate: however `self`'s inner snapshot got here, a hard
        /// ISO/IEC 29500-4 Transitional violation fails `build()` -- the soft diagnostic (explicit
        /// `conformance="strict"` attribute) passes through as an advisory `Diagnostic`; the `Err`
        /// path is NOT taken for it, only hard ones block.
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
    use crate::standards::v_ecma_376::subsets::base::schema::{PptxAnalyzer as PptxAnyAnalyzer, PptxParts};
    use crate::PptxSnapshot;
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_diagnostic::FaultCode;
    use semio_framework_diagnostic::FaultScope;
    use semio_framework_diagnostic::Severity;
    use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use semio_s_artifact_stdio_zip::opc::OpcPackage;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

    //#region 🔖️Namespaces
    pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
    /// 🏅️ Marker prefix common to EVERY ISO/IEC 29500-1 Strict namespace URI (markup namespaces
    /// AND the officeDocument relationships base alike) -- see `🪆️subsets/🔒️strict`'s
    /// `STRICT_MAIN_NS`/`STRICT_REL_BASE`, both of which start with this prefix.
    pub const STRICT_NS_MARKER: &str = "purl.oclc.org/ooxml";
    //#endregion 🔖️Namespaces

    //#region 🔖️Conformance
    pub const CODE_MAIN_NS: &str = "stdio.pptx.transitional.main-ns-not-transitional";
    pub const CODE_STRICT_NS_PRESENT: &str = "stdio.pptx.transitional.strict-ns-present";
    pub const CODE_CONFORMANCE_ATTR: &str = "stdio.pptx.transitional.conformance-attr-not-transitional";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn main_part_path(opc: &OpcPackage) -> Option<String> {
        crate::standards::v_ecma_376::subsets::base::io::resolve_office_document_relationship(opc)
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
    /// `PptxSnapshot`. Shared single source of truth: `PptxTransitionalComposer::compose` hard-gates
    /// on this (pre-serialization, authoritative), `PptxTransitionalBuilder::build` hard-gates on
    /// this too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_transitional_conformance(snapshot: &PptxSnapshot) -> Vec<Diagnostic> {
        let opc = &snapshot.opc;
        let mut out = Vec::new();

        match main_part_path(opc) {
            Some(path) => match snapshot.part_text(&path) {
                Some(text) => {
                    if !text.contains(TRANSITIONAL_MAIN_NS) {
                        out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} does not declare the Transitional PresentationML main namespace ({TRANSITIONAL_MAIN_NS})")));
                    }
                    if text.contains("conformance=\"strict\"") {
                        out.push(soft(CODE_CONFORMANCE_ATTR, format!("root officeDocument part {path}'s <p:presentation> declares conformance=\"strict\" -- Transitional expects it absent or \"transitional\"")));
                    }
                }
                None => out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} is missing or not valid utf-8 -- cannot verify the Transitional PresentationML main namespace"))),
            },
            None => out.push(hard(CODE_MAIN_NS, "package has no resolvable officeDocument relationship -- cannot verify the Transitional PresentationML main namespace".into())),
        }

        for (path, text) in snapshot.part_texts() {
            if text.contains(STRICT_NS_MARKER) {
                out.push(hard(CODE_STRICT_NS_PRESENT, format!("part {path} declares an ISO/IEC 29500-1 Strict namespace -- ISO/IEC 29500-4 Transitional forbids it")));
            }
        }

        for (owner, relationships) in opc.relationships.groups() {
            for rel in relationships {
                if rel.rel_type.contains(STRICT_NS_MARKER) {
                    out.push(hard(CODE_STRICT_NS_PRESENT, format!("relationship {} owned by '{owner}' uses a Strict relationship base ({}) -- Transitional forbids it", rel.id, rel.rel_type)));
                }
            }
        }

        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pptx` (ecma-376/🌉️transitional): delegates the real parse to the 🧱️base
    /// subset's analyzer (same `PptxSnapshot`), then folds real ISO/IEC 29500-4 Transitional
    /// conformance diagnostics on top. `sniff` also delegates -- a subset-level sniff for
    /// `transitional` is "is this recognizable as a pptx at all", the same OPC-shaped probe every
    /// ecma-376 dialect shares; conformance is a separate, heavier question answered by
    /// `analyze`/`check_transitional_conformance`, not by `sniff`.
    pub struct PptxTransitionalAnalyzerAnalysis;

    impl ArtifactAnalysis for PptxTransitionalAnalyzerAnalysis {
        type Parts = PptxParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PptxAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PptxAnyAnalyzer::analyze(sources);
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
    pub spec PptxTransitionalBuilderFacets {
        construction: PptxTransitionalBuilderConstruction,
        analysis: PptxTransitionalAnalyzerAnalysis,
        composition: super::io::derived_composition::PptxTransitionalComposerComposition,
    }
    builder: PptxTransitionalBuilder,
    analyzer: PptxTransitionalAnalyzer,
    composer: PptxTransitionalComposer,
);
