//! 🏗️ PptxStrictBuilder (ecma-376/✳️strict) — a thin wrapper around the ✳️any subset's
//! `PptxBuilder` (same construction vocabulary: `from_snapshot`/`from_text`/`from_binary`/
//! `mutate`/`absorb`, and the ✳️any subset's own typed slide/paragraph constructors reachable
//! through `from_snapshot` + `mutate`). What makes this builder Strict-specific is `build()`
//! itself: however `self` got built, it re-runs the SAME `check_strict_conformance` used by
//! `PptxStrictComposer`, unconditionally -- so a hard ISO/IEC 29500-1 Strict violation can never
//! leave this builder as an `Ok(PptxSnapshot)` (mirrors `📄️pdf` 1.7 `✳️a`'s `PdfABuilder`).

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::analyzer::check_strict_conformance;
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::builder::PptxBuilder as PptxAnyBuilder;
use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct PptxStrictBuilder {
    inner: PptxAnyBuilder,
}

impl ArtifactBuilder for PptxStrictBuilder {
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
        let err = PptxStrictBuilder::empty().build().expect_err("an empty package has no officeDocument relationship, must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::analyzer::CODE_MAIN_NS));
    }

    #[test]
    fn conforming_strict_snapshot_builds_clean() {
        let snapshot = PptxStrictBuilder::from_snapshot(strict_snapshot()).build().expect("conforming Strict snapshot must build");
        assert!(snapshot.opc.part_bytes("ppt/presentation.xml").is_some());
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut violating = strict_snapshot();
        violating.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<v:shape xmlns:v=\"urn:schemas-microsoft-com:vml\"/>".to_vec());
        let (mutated, _diff) = PptxStrictBuilder::from_snapshot(PptxSnapshot::default()).mutate(PptxMutation::SetSnapshot { snapshot: violating });
        let err = mutated.build().expect_err("VML markup must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::analyzer::CODE_VML_PRESENT));
    }
}
