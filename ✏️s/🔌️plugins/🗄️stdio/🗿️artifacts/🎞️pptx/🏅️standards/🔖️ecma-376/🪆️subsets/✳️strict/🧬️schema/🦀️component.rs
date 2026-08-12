//! 🧬️ PptxSnapshot schema (ecma-376/✳️strict) — reuses the ✳️any subset's `PptxSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.pptx` schema id). ISO/IEC 29500-1:2016 Strict is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✳️strict/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: real ISO/IEC 29500-1 Strict
//! conformance-class subset, same shared pattern as `📜️docx`/`📕️xlsx` ecma-376 ✳️strict.

pub use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::schema::check_strict_conformance;
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::PptxBuilder as PptxAnyBuilder;
    use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};

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

        fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let (inner, diff) = self.inner.mutate(mutation);
            (Self { inner }, diff)
        }

        fn absorb(self, diff: Self::Diff) -> Self {
            Self { inner: self.inner.absorb(diff) }
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
    mod tests {
        use super::*;
        use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE};

        const STRICT_PRESENTATION_XML: &str = concat!(
            r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );

        fn strict_snapshot() -> PptxSnapshot {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", STRICT_PRESENTATION_XML.as_bytes().to_vec());
            opc.add_relationship("", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument", "ppt/presentation.xml");
            PptxSnapshot { opc, ..PptxSnapshot::default() }
        }

        #[test]
        fn empty_builder_has_no_office_document_relationship_and_fails_build() {
            let err = PptxStrictBuilderConstruction::empty().build().expect_err("an empty package has no officeDocument relationship, must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::schema::CODE_MAIN_NS));
        }

        #[test]
        fn conforming_strict_snapshot_builds_clean() {
            let snapshot = PptxStrictBuilderConstruction::from_snapshot(strict_snapshot()).build().expect("conforming Strict snapshot must build");
            assert!(snapshot.opc.part_bytes("ppt/presentation.xml").is_some());
        }

        #[test]
        fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut violating = strict_snapshot();
            violating.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<v:shape xmlns:v=\"urn:schemas-microsoft-com:vml\"/>".to_vec());
            let (mutated, _diff) = PptxStrictBuilderConstruction::from_snapshot(PptxSnapshot::default()).mutate(PptxMutation::SetSnapshot { snapshot: violating });
            let err = mutated.build().expect_err("VML markup must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::schema::CODE_VML_PRESENT));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::{PptxAnalyzer as PptxAnyAnalyzer, PptxParts};
    use crate::artifacts::pptx::PptxSnapshot;
    use crate::artifacts::zip::opc::OpcPackage;

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
    fn main_part_path(opc: &OpcPackage) -> Option<String> {
        crate::artifacts::pptx::standards::v_ecma_376::engine::resolve_office_document_relationship(opc)
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

    /// 🛡️ Real ISO/IEC 29500-1:2016 Strict conformance checks against one already-decoded
    /// `PptxSnapshot`. Shared single source of truth: `PptxStrictComposer::compose` hard-gates on
    /// this (pre-serialization, authoritative), `PptxStrictBuilder::build` hard-gates on this too,
    /// and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
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
                    out.push(hard(
                        CODE_REL_BASE,
                        format!("relationship {} owned by '{owner}' uses the Transitional officeDocument relationships base ({}) -- Strict requires {STRICT_REL_BASE}", rel.id, rel.rel_type),
                    ));
                }
            }
        }

        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pptx` (ecma-376/✳️strict): delegates the real parse to the ✳️any subset's
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
    mod tests {
        use super::*;
        use crate::artifacts::zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

        const STRICT_PRESENTATION_XML: &str = concat!(
            r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );

        fn strict_snapshot() -> PptxSnapshot {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", STRICT_PRESENTATION_XML.as_bytes().to_vec());
            opc.add_relationship("", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument", "ppt/presentation.xml");
            PptxSnapshot { opc, ..PptxSnapshot::default() }
        }

        #[test]
        fn conforming_strict_snapshot_reports_nothing() {
            let diagnostics = check_strict_conformance(&strict_snapshot());
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        #[test]
        fn transitional_main_ns_on_root_part_is_hard() {
            let mut snapshot = strict_snapshot();
            let transitional_xml = STRICT_PRESENTATION_XML.replace(STRICT_MAIN_NS, TRANSITIONAL_MAIN_NS);
            snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", transitional_xml.into_bytes());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn transitional_namespace_anywhere_in_package_is_hard() {
            let mut snapshot = strict_snapshot();
            snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", format!("<p:sld xmlns:p=\"{TRANSITIONAL_MAIN_NS}\"/>").into_bytes());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TRANSITIONAL_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn vml_markup_is_hard() {
            let mut snapshot = strict_snapshot();
            snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", format!("<v:shape xmlns:v=\"{VML_NS}\"/>").into_bytes());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VML_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn transitional_relationship_base_is_hard() {
            let mut snapshot = strict_snapshot();
            snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld/>".to_vec());
            snapshot.opc.add_relationship("ppt/presentation.xml", "rId2", REL_TYPE_OFFICE_DOCUMENT, "slides/slide1.xml");
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_REL_BASE && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn missing_conformance_attribute_is_soft() {
            let mut snapshot = strict_snapshot();
            let no_conformance = STRICT_PRESENTATION_XML.replace(" conformance=\"strict\"", "");
            snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", no_conformance.into_bytes());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn alternate_content_markup_is_soft() {
            let mut snapshot = strict_snapshot();
            snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<mc:AlternateContent/>".to_vec());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ALTERNATE_CONTENT && d.severity == Severity::Warning), "got {diagnostics:?}");
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn missing_office_document_relationship_is_hard() {
            let snapshot = PptxSnapshot::default();
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS && d.severity == Severity::Error), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PptxStrictBuilderFacets {
        construction: derived_construction::PptxStrictBuilderConstruction,
        analysis: derived_analysis::PptxStrictAnalyzerAnalysis,
        composition: super::io::derived_composition::PptxStrictComposerComposition,
    }
    builder: PptxStrictBuilder,
    analyzer: PptxStrictAnalyzer,
    composer: PptxStrictComposer,
);
//#endregion 🧬️DerivedArtifactFacets
