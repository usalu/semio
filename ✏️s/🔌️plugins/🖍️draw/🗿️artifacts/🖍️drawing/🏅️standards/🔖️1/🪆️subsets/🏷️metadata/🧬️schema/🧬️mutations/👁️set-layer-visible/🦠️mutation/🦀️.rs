//! 👁️ Drawing mutation — `SetLayerVisible`: flips one layer's `visible` flag (addressed, single-field
//! setter — the taxonomy's own canonical `set` example).
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 👁️ `set-layer-visible` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-visible")]
pub struct SetLayerVisible {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub visible: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_visible(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, visible: bool) -> DrawingMutation {
    DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id, visible })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetLayerVisible {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-visible", record: "SetLayerVisible" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer \"{}\" visible to {}", self.layer_id, self.visible), &format!("Sichtbarkeit von Ebene \"{}\" auf {} setzen", self.layer_id, self.visible))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
