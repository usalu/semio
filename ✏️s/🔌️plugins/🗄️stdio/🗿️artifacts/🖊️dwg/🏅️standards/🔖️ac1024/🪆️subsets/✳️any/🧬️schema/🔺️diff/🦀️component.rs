//! 🔺️ Sparse field-by-field diff for logical DWG document state.

use crate::artifacts::dwg::schema::snapshot::{DwgApplicationHistory, DwgApplicationInfo, DwgAuxiliaryHeader, DwgClass, DwgDependency, DwgHeaderVariables, DwgIndexedPreview, DwgLogicalDrawing, DwgRevisionHistory, DwgSummaryInfo, DwgTemplate};
use crate::artifacts::dwg::DwgSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.dwg`; schema identity is intentionally immutable.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg.diff")]
pub struct DwgDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintenance_version: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codepage: Option<u16>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drawing: Option<DwgLogicalDrawing>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<DwgHeaderVariables>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<DwgClass>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<DwgDependency>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<DwgSummaryInfo>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<DwgApplicationInfo>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<DwgTemplate>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auxiliary_header: Option<DwgAuxiliaryHeader>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_history: Option<DwgRevisionHistory>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<DwgIndexedPreview>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_history: Option<DwgApplicationHistory>,
}

impl MutationDiff<DwgSnapshot> for DwgDiff {
    async fn apply(&self, base: &DwgSnapshot) -> MutationApplyResult<DwgSnapshot> {
        Ok(DwgSnapshot {
            schema: base.schema.clone(),
            version: self.version.clone().unwrap_or_else(|| base.version.clone()),
            maintenance_version: self.maintenance_version.unwrap_or(base.maintenance_version),
            codepage: self.codepage.unwrap_or(base.codepage),
            drawing: self.drawing.clone().unwrap_or_else(|| base.drawing.clone()),
            header: self.header.clone().unwrap_or_else(|| base.header.clone()),
            classes: self.classes.clone().unwrap_or_else(|| base.classes.clone()),
            dependencies: self.dependencies.clone().unwrap_or_else(|| base.dependencies.clone()),
            summary: self.summary.clone().unwrap_or_else(|| base.summary.clone()),
            application: self.application.clone().unwrap_or_else(|| base.application.clone()),
            template: self.template.clone().unwrap_or_else(|| base.template.clone()),
            auxiliary_header: self.auxiliary_header.clone().unwrap_or_else(|| base.auxiliary_header.clone()),
            revision_history: self.revision_history.clone().unwrap_or_else(|| base.revision_history.clone()),
            preview: self.preview.clone().unwrap_or_else(|| base.preview.clone()),
            application_history: self.application_history.clone().unwrap_or_else(|| base.application_history.clone()),
        })
    }

    /// ➕️ Coalesces sequential field replacements with last-write-wins semantics.
    async fn absorb(&mut self, other: Self) {
        if other.version.is_some() {
            self.version = other.version;
        }
        if other.maintenance_version.is_some() {
            self.maintenance_version = other.maintenance_version;
        }
        if other.codepage.is_some() {
            self.codepage = other.codepage;
        }
        if other.drawing.is_some() {
            self.drawing = other.drawing;
        }
        if other.header.is_some() {
            self.header = other.header;
        }
        if other.classes.is_some() {
            self.classes = other.classes;
        }
        if other.dependencies.is_some() {
            self.dependencies = other.dependencies;
        }
        if other.summary.is_some() {
            self.summary = other.summary;
        }
        if other.application.is_some() {
            self.application = other.application;
        }
        if other.template.is_some() {
            self.template = other.template;
        }
        if other.auxiliary_header.is_some() {
            self.auxiliary_header = other.auxiliary_header;
        }
        if other.revision_history.is_some() {
            self.revision_history = other.revision_history;
        }
        if other.preview.is_some() {
            self.preview = other.preview;
        }
        if other.application_history.is_some() {
            self.application_history = other.application_history;
        }
    }
}

impl DiffAlgebra<DwgSnapshot> for DwgDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta from
    /// `self.apply(base)` back to `base`.
    async fn inverse(&self, base: &DwgSnapshot) -> Self {
        let mutated = self.apply(base).unwrap();
        Self::between(&mutated, base)
    }

    /// 🧭️ Computes a field-by-field logical state delta.
    async fn between(base: &DwgSnapshot, other: &DwgSnapshot) -> Self {
        let version = (base.version != other.version).then(|| other.version.clone());
        let maintenance_version = (base.maintenance_version != other.maintenance_version).then_some(other.maintenance_version);
        let codepage = (base.codepage != other.codepage).then_some(other.codepage);
        let drawing = (base.drawing != other.drawing).then(|| other.drawing.clone());
        let header = (base.header != other.header).then(|| other.header.clone());
        let classes = (base.classes != other.classes).then(|| other.classes.clone());
        let dependencies = (base.dependencies != other.dependencies).then(|| other.dependencies.clone());
        let summary = (base.summary != other.summary).then(|| other.summary.clone());
        let application = (base.application != other.application).then(|| other.application.clone());
        let template = (base.template != other.template).then(|| other.template.clone());
        let auxiliary_header = (base.auxiliary_header != other.auxiliary_header).then(|| other.auxiliary_header.clone());
        let revision_history = (base.revision_history != other.revision_history).then(|| other.revision_history.clone());
        let preview = (base.preview != other.preview).then(|| other.preview.clone());
        let application_history = (base.application_history != other.application_history).then(|| other.application_history.clone());
        DwgDiff { version, maintenance_version, codepage, drawing, header, classes, dependencies, summary, application, template, auxiliary_header, revision_history, preview, application_history }
    }

    async fn is_empty(&self) -> bool {
        self.version.is_none()
            && self.maintenance_version.is_none()
            && self.codepage.is_none()
            && self.drawing.is_none()
            && self.header.is_none()
            && self.classes.is_none()
            && self.dependencies.is_none()
            && self.summary.is_none()
            && self.application.is_none()
            && self.template.is_none()
            && self.auxiliary_header.is_none()
            && self.revision_history.is_none()
            && self.preview.is_none()
            && self.application_history.is_none()
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `DwgDiff` to short-circuit into.
pub async fn diff_set_snapshot(base: &DwgSnapshot, next: &DwgSnapshot) -> DwgDiff {
    DwgDiff::between(base, next)
}

pub async fn diff_set_version_info(base: &DwgSnapshot, version: &str, maintenance_version: u8, codepage: u16) -> DwgDiff {
    let mut next = base.clone();
    crate::artifacts::dwg::schema::snapshot::synchronize_version_info(&mut next, version, maintenance_version, codepage).expect("SetVersionInfo requires a valid DWG version sentinel");
    DwgDiff::between(base, &next)
}

//#endregion 🔖️MutationDiffBuilders

//#region 🔖️DemoCases
/// 🎬️ Representative empty, full logical, and version-info diffs.
#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<DwgDiff> {
    vec![
        DwgDiff::default(),
        DwgDiff {
            version: Some("AC1032".into()),
            maintenance_version: Some(9),
            codepage: Some(65001),
            drawing: Some(DwgLogicalDrawing::default()),
            header: Some(DwgHeaderVariables::default()),
            classes: Some(vec![DwgClass { number: 500, dxf_name: "ACDBPLACEHOLDER".into(), ..Default::default() }]),
            dependencies: Some(vec![DwgDependency { feature: "xref".into(), relative_path: "site.dwg".into(), ..Default::default() }]),
            summary: Some(DwgSummaryInfo { title: "Architectural".into(), ..Default::default() }),
            application: Some(DwgApplicationInfo { product: "AutoCAD".into(), ..Default::default() }),
            template: Some(DwgTemplate::default()),
            auxiliary_header: Some(DwgAuxiliaryHeader::default()),
            revision_history: Some(DwgRevisionHistory::default()),
            preview: Some(DwgIndexedPreview::default()),
            application_history: Some(DwgApplicationHistory::default()),
        },
        diff_set_version_info(&crate::artifacts::dwg::standards::v_ac1024::engine::demo_dwg_snapshot(), "AC1024", 2, 30),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ Logical diff text and binary codecs retain every field.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = DwgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
            let decoded = DwgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
        }
    }
}
//#endregion 🧪️Tests
