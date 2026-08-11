//! 🏗️ PptxTransitionalBuilder (ecma-376/✳️transitional) — a thin wrapper around the ✳️any
//! subset's `PptxBuilder` (same construction vocabulary: `from_snapshot`/`from_text`/
//! `from_binary`/`mutate`/`absorb`, and the ✳️any subset's own typed slide/paragraph
//! constructors reachable through `from_snapshot` + `mutate`). What makes this builder
//! Transitional-specific is `build()` itself: however `self` got built, it re-runs the SAME
//! `check_transitional_conformance` used by `PptxTransitionalComposer`, unconditionally -- so a
//! hard ISO/IEC 29500-4 Transitional violation can never leave this builder as an
//! `Ok(PptxSnapshot)` (mirrors `📄️pdf` 1.7 `✳️a`'s `PdfABuilder`).

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::analyzer::check_transitional_conformance;
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::builder::PptxBuilder as PptxAnyBuilder;
use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct PptxTransitionalBuilder {
    inner: PptxAnyBuilder,
}

impl ArtifactBuilder for PptxTransitionalBuilder {
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
    use crate::artifacts::zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

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
        let err = PptxTransitionalBuilder::empty().build().expect_err("an empty package has no officeDocument relationship, must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::analyzer::CODE_MAIN_NS));
    }

    #[test]
    fn conforming_transitional_snapshot_builds_clean() {
        let snapshot = PptxTransitionalBuilder::from_snapshot(transitional_snapshot()).build().expect("conforming Transitional snapshot must build");
        assert!(snapshot.opc.part_bytes("ppt/presentation.xml").is_some());
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut violating = transitional_snapshot();
        violating.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld xmlns:p=\"http://purl.oclc.org/ooxml/presentationml/main\"/>".to_vec());
        let (mutated, _diff) = PptxTransitionalBuilder::from_snapshot(PptxSnapshot::default()).mutate(PptxMutation::SetSnapshot { snapshot: violating });
        let err = mutated.build().expect_err("a Strict namespace anywhere must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::analyzer::CODE_STRICT_NS_PRESENT));
    }
}
