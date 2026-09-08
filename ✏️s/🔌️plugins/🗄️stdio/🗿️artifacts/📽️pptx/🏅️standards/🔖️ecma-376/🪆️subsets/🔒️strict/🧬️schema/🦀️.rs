//! 🧬️ PptxSnapshot schema (ecma-376/🔒️strict) — reuses the 🧱️base subset's `PptxSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.pptx` schema id). ISO/IEC 29500-1:2016 Strict is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/🔒️strict/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: real ISO/IEC 29500-1 Strict
//! conformance-class subset, same shared pattern as `📜️docx`/`📕️xlsx` ecma-376 🔒️strict.

pub use crate::standards::v_ecma_376::subsets::base::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// 🧱️base subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of 🧱️base's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::strict::schema::mutations` while 🧱️base's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::standards::v_ecma_376::subsets::base::schema::PptxBuilder as PptxAnyBuilder;
    use crate::standards::v_ecma_376::subsets::strict::schema::check_strict_conformance;
    #[cfg(test)]
    use crate::schema::mutations::set_snapshot;
    use crate::{PptxDiff, PptxMutation, PptxSnapshot};
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct PptxStrictBuilderConstruction {
        inner: PptxAnyBuilder,
    }

    impl ArtifactBuilder for PptxStrictBuilderConstruction {
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
        /// ISO/IEC 29500-1 Strict violation fails `build()` -- soft diagnostics (missing
        /// `conformance="strict"`, `mc:AlternateContent`) pass through as advisory `Diagnostic`s;
        /// the `Err` path is NOT taken for those, only hard ones block.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let snapshot = self.inner.build()?;
            let hard: Vec<Diagnostic> = check_strict_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };

    //#region 🔖️Namespaces
    pub const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/presentationml/main";
    pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
    pub const TRANSITIONAL_DRAWING_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
    pub const VML_NS: &str = "urn:schemas-microsoft-com:vml";
    pub const STRICT_REL_BASE: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
    pub const TRANSITIONAL_REL_BASE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    //#endregion 🔖️Namespaces

    //#region 🔖️Conformance
    pub const CODE_MAIN_NS: &str = "stdio.pptx.strict.main-ns-not-strict";
    pub const CODE_TRANSITIONAL_NS_PRESENT: &str = "stdio.pptx.strict.transitional-ns-present";
    pub const CODE_VML_PRESENT: &str = "stdio.pptx.strict.vml-present";
    pub const CODE_REL_BASE: &str = "stdio.pptx.strict.relationship-base-not-strict";
    pub const CODE_CONFORMANCE_ATTR: &str = "stdio.pptx.strict.conformance-attr-missing";
    pub const CODE_ALTERNATE_CONTENT: &str = "stdio.pptx.strict.alternate-content-present";

    /// 🧭️ Locates the root officeDocument part regardless of whether the package declares the
    /// Transitional or Strict officeDocument relationship type.
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

    /// 🛡️ Real ISO/IEC 29500-1:2016 Strict conformance checks against one already-decoded
    /// `PptxSnapshot`. Shared single source of truth: `PptxStrictComposer::compose` hard-gates on
    /// this (pre-serialization, authoritative), `PptxStrictBuilder::build` hard-gates on this too,
    /// and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_strict_conformance(snapshot: &PptxSnapshot) -> Vec<Diagnostic> {
        let opc = &snapshot.opc;
        let mut out = Vec::new();

        match main_part_path(opc) {
            Some(path) => match part_text(opc, &path) {
                Some(text) => {
                    if !text.contains(STRICT_MAIN_NS) {
                        out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} does not declare the Strict PresentationML main namespace ({STRICT_MAIN_NS})")));
                    }
                    if !text.contains("conformance=\"strict\"") {
                        out.push(soft(CODE_CONFORMANCE_ATTR, format!("root officeDocument part {path}'s <p:presentation> does not declare conformance=\"strict\"")));
                    }
                }
                None => out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} is missing or not valid utf-8 -- cannot verify the Strict PresentationML main namespace"))),
            },
            None => out.push(hard(CODE_MAIN_NS, "package has no resolvable officeDocument relationship -- cannot verify the Strict PresentationML main namespace".into())),
        }

        for part in &opc.parts {
            let Some(text) = std::str::from_utf8(&part.bytes).ok() else { continue };
            if text.contains(TRANSITIONAL_MAIN_NS) || text.contains(TRANSITIONAL_DRAWING_NS) {
                out.push(hard(CODE_TRANSITIONAL_NS_PRESENT, format!("part {} declares a Transitional OOXML main namespace -- ISO/IEC 29500-1 Strict forbids it", part.path)));
            }
            if text.contains(VML_NS) {
                out.push(hard(CODE_VML_PRESENT, format!("part {} contains VML markup ({VML_NS}) -- ISO/IEC 29500-1 Strict forbids VML", part.path)));
            }
            if text.contains("mc:AlternateContent") {
                out.push(soft(CODE_ALTERNATE_CONTENT, format!("part {} contains mc:AlternateContent markup-compatibility escape hatch", part.path)));
            }
        }

        let mut owners: Vec<&String> = opc.relationships.keys().collect();
        owners.sort();
        for owner in owners {
            for rel in &opc.relationships[owner] {
                if rel.rel_type.starts_with(TRANSITIONAL_REL_BASE) {
                    out.push(hard(CODE_REL_BASE, format!("relationship {} owned by '{owner}' uses the Transitional officeDocument relationships base ({}) -- Strict requires {STRICT_REL_BASE}", rel.id, rel.rel_type)));
                }
            }
        }

        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pptx` (ecma-376/🔒️strict): delegates the real parse to the 🧱️base subset's
    /// analyzer (same `PptxSnapshot`), then folds real ISO/IEC 29500-1 Strict conformance diagnostics
    /// on top. `sniff` also delegates -- a subset-level sniff for `strict` is "is this recognizable
    /// as a pptx at all", the same OPC-shaped probe every ecma-376 dialect shares; conformance is a
    /// separate, heavier question answered by `analyze`/`check_strict_conformance`, not by `sniff`.
    pub struct PptxStrictAnalyzerAnalysis;

    impl ArtifactAnalysis for PptxStrictAnalyzerAnalysis {
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
                let checks = check_strict_conformance(snapshot);
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
    pub spec PptxStrictBuilderFacets {
        construction: PptxStrictBuilderConstruction,
        analysis: PptxStrictAnalyzerAnalysis,
        composition: super::io::derived_composition::PptxStrictComposerComposition,
    }
    builder: PptxStrictBuilder,
    analyzer: PptxStrictAnalyzer,
    composer: PptxStrictComposer,
);
//#endregion 🧬️DerivedArtifactFacets
