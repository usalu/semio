//! 🏗️ XlsxStrictBuilder (ecma-376/✳️strict) — a typed builder whose only construction path stamps
//! the resulting `XlsxSnapshot`'s `xl/workbook.xml` into genuinely ISO/IEC 29500-1 Strict shape
//! (Strict SpreadsheetML namespace, Strict officeDocument relationships namespace,
//! `conformance="strict"`) before `build()` re-runs the SAME `check_strict_conformance` used by
//! `XlsxStrictComposer` as a hard gate -- so a Strict violation can never leave this builder as an
//! `Ok(XlsxSnapshot)`, no matter which path produced the in-flight snapshot (including the
//! generic `SetSnapshot` escape hatch reachable through `mutate`).
//!
//! `stamp_strict_namespace` real-rewrites the already-generated `xl/workbook.xml` bytes (parses
//! them back with the shared XML component, patches the root element's three attributes,
//! re-serializes) rather than fabricating a hand-written literal -- genuinely Strict-shaped bytes,
//! not a placeholder that merely satisfies string equality. Note the SAME scope cut as
//! `🎹️composer`'s module doc comment: `ArtifactPack::encode_pack` (via `⚙️engine::encode_xlsx`)
//! always regenerates `xl/workbook.xml` as Transitional-shaped on the next full round trip, so
//! this stamp only survives until the snapshot is next pack-encoded.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlNode};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::{XlsxSnapshot, XlsxWorkbook};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::analyzer::{check_strict_conformance, STRICT_R_NS, STRICT_SML_NS};
use crate::artifacts::xlsx::{XlsxDiff, XlsxMutation};

const WORKBOOK_PART: &str = "xl/workbook.xml";
const WORKBOOK_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";

//#region 🔖️Stamp
/// 🖋️ Real-rewrites `snapshot.opc`'s `xl/workbook.xml` root attrs to Strict shape. A no-op on the
/// rest of the package (worksheets/sharedStrings/relationships) -- only the three attrs
/// `check_strict_conformance` actually inspects change.
pub fn stamp_strict_namespace(mut snapshot: XlsxSnapshot) -> XlsxSnapshot {
    if let Some(bytes) = snapshot.opc.part_bytes(WORKBOOK_PART) {
        if let Ok(text) = std::str::from_utf8(bytes) {
            if let Ok(mut doc) = xml_document_from_text(text) {
                if let Some(XmlNode::Element { attrs, .. }) = &mut doc.root {
                    set_attr(attrs, "xmlns", STRICT_SML_NS);
                    set_attr(attrs, "xmlns:r", STRICT_R_NS);
                    set_attr(attrs, "conformance", "strict");
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
pub struct XlsxStrictBuilder {
    snapshot: XlsxSnapshot,
}

impl XlsxStrictBuilder {
    /// ➕️ The recommended entry point: builds a minimal package from `workbook` via the shared
    /// ecma-376 engine, then stamps it Strict.
    pub fn new(workbook: XlsxWorkbook) -> Self {
        Self { snapshot: stamp_strict_namespace(crate::artifacts::xlsx::standards::v_ecma_376::engine::build_minimal_xlsx(workbook)) }
    }
}

impl ArtifactBuilder for XlsxStrictBuilder {
    type Snapshot = XlsxSnapshot;
    type Mutation = XlsxMutation;
    type Diff = XlsxDiff;

    /// ⚠️ `ArtifactBuilder::empty()` is mandated no-arg by the SDK trait (generic UI/mutation
    /// dispatch needs every builder facet uniform) -- falls back to an empty workbook, stamped
    /// Strict regardless. Prefer `XlsxStrictBuilder::new(workbook)` directly wherever real content
    /// is known up front.
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

    /// 🛡️ The real construction gate: however `self.snapshot` got here (`new`, `from_binary`, a
    /// raw `mutate(SetSnapshot { .. })`), a hard Strict violation fails `build()` -- the soft
    /// diagnostic (worksheet content-type mismatch) passes through as an advisory `Diagnostic`;
    /// the `Err` path is NOT taken for it, only hard ones block.
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
mod tests {
    use super::*;

    #[test]
    fn new_stamps_strict_and_builds_clean() {
        let snapshot = XlsxStrictBuilder::new(XlsxWorkbook::default()).build().expect("conforming construction must build");
        assert!(check_strict_conformance(&snapshot).iter().all(|d| d.severity != Severity::Error), "got {:?}", check_strict_conformance(&snapshot));
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = XlsxStrictBuilder::new(XlsxWorkbook::default()).build().unwrap();
        // Directly corrupt the stamped namespace, bypassing every typed constructor -- mirrors the
        // PDF/A pilot's raw-mutate escape-hatch test.
        snapshot.opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, b"<workbook xmlns=\"transitional\"/>".to_vec());
        let (mutated, _diff) = XlsxStrictBuilder::from_snapshot(XlsxSnapshot::default()).mutate(XlsxMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("a non-Strict workbook.xml must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::analyzer::CODE_NAMESPACE_MISMATCH), "got {err:?}");
    }
}
