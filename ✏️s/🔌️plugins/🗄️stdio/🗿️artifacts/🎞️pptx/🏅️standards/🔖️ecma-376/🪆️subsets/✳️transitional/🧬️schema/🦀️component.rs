//! 🧬️ PptxSnapshot schema (ecma-376/✳️transitional) — reuses the ✳️any subset's `PptxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.pptx` schema id). ISO/IEC 29500-4:2016
//! Transitional is a validation-gated dialect STAMP on top of that existing schema, not a new
//! one -- see D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️transitional/🧬️schema/`
//! is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema
//! definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: real ISO/IEC 29500-4 Transitional
//! conformance-class subset, same shared pattern as `📜️docx`/`📕️xlsx` ecma-376 ✳️transitional.

pub use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::PptxBuilder as PptxAnyBuilder;
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};
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
    mod tests {
        use super::*;
        use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

        const TRANSITIONAL_PRESENTATION_XML: &str = concat!(
            r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );

        fn transitional_snapshot() -> PptxSnapshot {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", TRANSITIONAL_PRESENTATION_XML.as_bytes().to_vec());
            opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
            PptxSnapshot { opc, ..PptxSnapshot::default() }
        }

        #[test]
        fn empty_builder_has_no_office_document_relationship_and_fails_build() {
            let err = PptxTransitionalBuilderConstruction::empty().build().expect_err("an empty package has no officeDocument relationship, must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::schema::CODE_MAIN_NS));
        }

        #[test]
        fn conforming_transitional_snapshot_builds_clean() {
            let snapshot = PptxTransitionalBuilderConstruction::from_snapshot(transitional_snapshot()).build().expect("conforming Transitional snapshot must build");
            assert!(snapshot.opc.part_bytes("ppt/presentation.xml").is_some());
        }

        #[test]
        fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut violating = transitional_snapshot();
            violating.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld xmlns:p=\"http://purl.oclc.org/ooxml/presentationml/main\"/>".to_vec());
            let (mutated, _diff) = PptxTransitionalBuilderConstruction::from_snapshot(PptxSnapshot::default()).mutate(PptxMutation::SetSnapshot { snapshot: violating });
            let err = mutated.build().expect_err("a Strict namespace anywhere must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::schema::CODE_STRICT_NS_PRESENT));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::{PptxAnalyzer as PptxAnyAnalyzer, PptxParts};
    use crate::artifacts::pptx::PptxSnapshot;
    use crate::artifacts::zip::opc::OpcPackage;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

    //#region 🔖️Namespaces
    pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
    /// 🏅️ Marker prefix common to EVERY ISO/IEC 29500-1 Strict namespace URI (markup namespaces
    /// AND the officeDocument relationships base alike) -- see `🪆️subsets/✳️strict`'s
    /// `STRICT_MAIN_NS`/`STRICT_REL_BASE`, both of which start with this prefix.
    pub const STRICT_NS_MARKER: &str = "purl.oclc.org/ooxml";
    //#endregion 🔖️Namespaces

    //#region 🔖️Conformance
    pub const CODE_MAIN_NS: &str = "stdio.pptx.transitional.main-ns-not-transitional";
    pub const CODE_STRICT_NS_PRESENT: &str = "stdio.pptx.transitional.strict-ns-present";
    pub const CODE_CONFORMANCE_ATTR: &str = "stdio.pptx.transitional.conformance-attr-not-transitional";

    fn main_part_path(opc: &OpcPackage) -> Option<String> {
        crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::resolve_office_document_relationship(opc)
    }

    fn part_text<'a>(opc: &'a OpcPackage, path: &str) -> Option<&'a str> {
        opc.part_bytes(path).and_then(|b| std::str::from_utf8(b).ok())
    }

    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO/IEC 29500-4:2016 Transitional conformance checks against one already-decoded
    /// `PptxSnapshot`. Shared single source of truth: `PptxTransitionalComposer::compose` hard-gates
    /// on this (pre-serialization, authoritative), `PptxTransitionalBuilder::build` hard-gates on
    /// this too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
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
    /// 🧐️ Analyzes `stdio.pptx` (ecma-376/✳️transitional): delegates the real parse to the ✳️any
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
    mod tests {
        use super::*;
        use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

        const TRANSITIONAL_PRESENTATION_XML: &str = concat!(
            r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );

        fn transitional_snapshot() -> PptxSnapshot {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", TRANSITIONAL_PRESENTATION_XML.as_bytes().to_vec());
            opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
            PptxSnapshot { opc, ..PptxSnapshot::default() }
        }

        #[test]
        fn conforming_transitional_snapshot_reports_nothing() {
            let diagnostics = check_transitional_conformance(&transitional_snapshot());
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        #[test]
        fn strict_main_ns_on_root_part_is_hard() {
            let mut snapshot = transitional_snapshot();
            let strict_xml = TRANSITIONAL_PRESENTATION_XML.replace(TRANSITIONAL_MAIN_NS, "http://purl.oclc.org/ooxml/presentationml/main");
            snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", strict_xml.into_bytes());
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn strict_namespace_anywhere_in_package_is_hard() {
            let mut snapshot = transitional_snapshot();
            snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld xmlns:p=\"http://purl.oclc.org/ooxml/presentationml/main\"/>".to_vec());
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn strict_relationship_base_is_hard() {
            let mut snapshot = transitional_snapshot();
            snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld/>".to_vec());
            snapshot.opc.add_relationship("ppt/presentation.xml", "rId2", "http://purl.oclc.org/ooxml/officeDocument/relationships/slide", "slides/slide1.xml");
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn explicit_strict_conformance_attribute_is_soft() {
            let mut snapshot = transitional_snapshot();
            let with_conformance = TRANSITIONAL_PRESENTATION_XML.replace("<p:presentation ", "<p:presentation conformance=\"strict\" ");
            snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", with_conformance.into_bytes());
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn missing_office_document_relationship_is_hard() {
            let snapshot = PptxSnapshot::default();
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS && d.severity == Severity::Error), "got {diagnostics:?}");
        }
    }
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
