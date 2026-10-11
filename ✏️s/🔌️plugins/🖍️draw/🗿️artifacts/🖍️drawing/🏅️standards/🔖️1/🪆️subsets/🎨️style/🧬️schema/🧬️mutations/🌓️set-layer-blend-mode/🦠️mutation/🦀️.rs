//! 🖌️ Drawing mutation — `SetLayerBlendMode`: sets one layer's `blend_mode` scalar.
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 🖌️ `set-layer-blend-mode` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-blend-mode")]
pub struct SetLayerBlendMode {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub blend_mode: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_blend_mode(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, blend_mode: semio_framework_value::paged::PagedUtf8<{usize::MAX}>) -> DrawingMutation {
    DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id, blend_mode })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetLayerBlendMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-blend-mode", record: "SetLayerBlendMode" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer \"{}\" blend mode to {}", self.layer_id, self.blend_mode), &format!("Füllmethode von Ebene \"{}\" auf {} setzen", self.layer_id, self.blend_mode))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
