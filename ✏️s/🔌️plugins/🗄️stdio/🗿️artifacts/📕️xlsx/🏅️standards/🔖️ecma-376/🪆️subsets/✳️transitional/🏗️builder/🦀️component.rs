//! 🏗️ XlsxTransitionalBuilder (ecma-376/✳️transitional) — a typed builder whose only construction
//! path stamps the resulting `XlsxSnapshot`'s `xl/workbook.xml` into genuinely ISO/IEC 29500-4
//! Transitional shape (Transitional SpreadsheetML namespace, Transitional officeDocument
//! relationships namespace, `conformance="transitional"`) before `build()` re-runs the SAME
//! `check_transitional_conformance` used by `XlsxTransitionalComposer` as a hard gate. Mirrors
//! ✳️strict's builder exactly, opposite polarity -- see that module's doc comment for the shared
//! rationale and the documented `encode_pack`/`encode_xlsx` writer-scope-cut caveat (this stamp
//! only survives until the snapshot is next pack-encoded, at which point it's regenerated as
//! Transitional-shaped ANYWAY -- so, unlike ✳️strict, a full round trip through the shared writer
//! never breaks this dialect's own hard gate).

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlNode};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::{XlsxSnapshot, XlsxWorkbook};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::analyzer::{check_transitional_conformance, TRANSITIONAL_R_NS, TRANSITIONAL_SML_NS};
use crate::artifacts::xlsx::{XlsxDiff, XlsxMutation};

const WORKBOOK_PART: &str = "xl/workbook.xml";
const WORKBOOK_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";

//#region 🔖️Stamp
/// 🖋️ Real-rewrites `snapshot.opc`'s `xl/workbook.xml` root attrs to explicit Transitional shape.
pub fn stamp_transitional_namespace(mut snapshot: XlsxSnapshot) -> XlsxSnapshot {
    if let Some(bytes) = snapshot.opc.part_bytes(WORKBOOK_PART) {
        if let Ok(text) = std::str::from_utf8(bytes) {
            if let Ok(mut doc) = xml_document_from_text(text) {
                if let Some(XmlNode::Element { attrs, .. }) = &mut doc.root {
                    set_attr(attrs, "xmlns", TRANSITIONAL_SML_NS);
                    set_attr(attrs, "xmlns:r", TRANSITIONAL_R_NS);
                    set_attr(attrs, "conformance", "transitional");
                }
                let bytes = xml_document_to_text(&doc).into_bytes();
                snapshot.opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, bytes);
            }
        }
    }
    snapshot
}

fn set_attr(attrs: &mut Vec<XmlAttr>, name: &str, value: &str) {
    if let Some(existing) = attrs.iter_mut().find(|a| a.name == name) {
        existing.value = value.into();
    } else {
        attrs.push(XmlAttr { name: name.into(), value: value.into() });
    }
}
//#endregion 🔖️Stamp

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct XlsxTransitionalBuilder {
    snapshot: XlsxSnapshot,
}

impl XlsxTransitionalBuilder {
    /// ➕️ The recommended entry point: builds a minimal package from `workbook` via the shared
    /// ecma-376 engine, then stamps it explicitly Transitional.
    pub fn new(workbook: XlsxWorkbook) -> Self {
        Self { snapshot: stamp_transitional_namespace(crate::artifacts::xlsx::standards::v_ecma_376::engine::build_minimal_xlsx(workbook)) }
    }
}

impl ArtifactBuilder for XlsxTransitionalBuilder {
    type Snapshot = XlsxSnapshot;
    type Mutation = XlsxMutation;
    type Diff = XlsxDiff;

    fn empty() -> Self {
        Self::new(XlsxWorkbook::default())
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot }
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }

    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard Transitional
    /// violation (Strict-shaped namespace/conformance attribute, or an unparsable workbook.xml)
    /// fails `build()`; the soft diagnostic passes through as advisory.
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let hard: Vec<Diagnostic> = check_transitional_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() {
            Ok(self.snapshot)
        } else {
            Err(hard)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stamps_transitional_and_builds_clean() {
        let snapshot = XlsxTransitionalBuilder::new(XlsxWorkbook::default()).build().expect("conforming construction must build");
        assert!(check_transitional_conformance(&snapshot).iter().all(|d| d.severity != Severity::Error), "got {:?}", check_transitional_conformance(&snapshot));
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = XlsxTransitionalBuilder::new(XlsxWorkbook::default()).build().unwrap();
        snapshot.opc.set_part(
            WORKBOOK_PART,
            WORKBOOK_CONTENT_TYPE,
            br#"<workbook xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict"/>"#.to_vec(),
        );
        let (mutated, _diff) = XlsxTransitionalBuilder::from_snapshot(XlsxSnapshot::default()).mutate(XlsxMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("a Strict-declared workbook.xml must fail build()");
        assert!(
            err.iter().any(|d| d.code.0 == crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::analyzer::CODE_CONFORMANCE_ATTRIBUTE),
            "got {err:?}"
        );
    }
}
