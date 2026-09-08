//! 🧬️ PptxSnapshot schema (ecma-376/🌉️transitional) — reuses the 🧱️base subset's `PptxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.pptx` schema id). ISO/IEC 29500-4:2016
//! Transitional is a validation-gated dialect STAMP on top of that existing schema, not a new
//! one -- see D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/🌉️transitional/🧬️schema/`
//! is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema
//! definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: real ISO/IEC 29500-4 Transitional
//! conformance-class subset, same shared pattern as `📜️docx`/`📕️xlsx` ecma-376 🌉️transitional.

pub use crate::standards::v_ecma_376::subsets::base::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// 🧱️base subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of 🧱️base's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::transitional::schema::mutations` while 🧱️base's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::standards::v_ecma_376::subsets::base::schema::PptxBuilder as PptxAnyBuilder;
    use crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    #[cfg(test)]
    use crate::schema::mutations::set_snapshot;
    use crate::{PptxDiff, PptxMutation, PptxSnapshot};
    use dsl::{Diagnostic, Severity};
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

        fn from_text(text: &str) -> Result<Self, store::TextError> {
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
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v_ecma_376::subsets::base::schema::{PptxAnalyzer as PptxAnyAnalyzer, PptxParts};
    use crate::PptxSnapshot;
    use semio_s_artifact_stdio_zip::opc::OpcPackage;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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
    fn part_text<'a>(opc: &'a OpcPackage, path: &str) -> Option<&'a str> {
        opc.part_bytes(path).and_then(|b| std::str::from_utf8(b).ok())
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
            Some(path) => match part_text(opc, &path) {
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

        for part in &opc.parts {
            let Some(text) = std::str::from_utf8(&part.bytes).ok() else { continue };
            if text.contains(STRICT_NS_MARKER) {
                out.push(hard(CODE_STRICT_NS_PRESENT, format!("part {} declares an ISO/IEC 29500-1 Strict namespace -- ISO/IEC 29500-4 Transitional forbids it", part.path)));
            }
        }

        let mut owners: Vec<&String> = opc.relationships.keys().collect();
        owners.sort();
        for owner in owners {
            for rel in &opc.relationships[owner] {
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
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
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
//#endregion 🧬️DerivedArtifactFacets
