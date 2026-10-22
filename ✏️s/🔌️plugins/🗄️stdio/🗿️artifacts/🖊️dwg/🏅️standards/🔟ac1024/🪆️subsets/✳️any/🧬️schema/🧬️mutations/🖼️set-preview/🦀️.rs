//! 🖼️ `set-preview` — replaces the preview image block of the container. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPreview {
    pub preview: DwgIndexedPreview,
}

impl protocol::MutationKind<DwgSnapshot, DwgMutation> for SetPreview {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preview", kind: "set-preview", record: "SetPreview" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set preview image", "Vorschaubild setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<DwgSnapshot, DwgMutation> for SetPreview {
    fn diff(&self, base: &DwgSnapshot) -> protocol::MutationOutcome<<DwgMutation as protocol::ColdMutation<DwgSnapshot>>::ReferenceDiff> {
        protocol::MutationOutcome::new(DwgDiff { preview: (base.preview != self.preview).then(|| self.preview.clone()), ..DwgDiff::default() })
    }
    fn inverse(&self, base: &DwgSnapshot) -> Result<Vec<DwgMutation>, semio_framework_value::ValueError> {
        Ok((base.preview != self.preview).then(|| DwgMutation::SetPreview(SetPreview { preview: base.preview.clone() })).into_iter().collect())
    }
}
//#endregion 🔖️Payload
