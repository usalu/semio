//! 🔒️ Drawing mutation — `SetLayerLocked`: flips one layer's `locked` flag.
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 🔒️ `set-layer-locked` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-locked")]
pub struct SetLayerLocked {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_locked(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, locked: bool) -> DrawingMutation {
    DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id, locked })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetLayerLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-locked", record: "SetLayerLocked" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer \"{}\" locked to {}", self.layer_id, self.locked), &format!("Sperre von Ebene \"{}\" auf {} setzen", self.layer_id, self.locked))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
