//! 🔒️ Drawing mutation — `SetLayerLocked`: flips one layer's `locked` flag.
use semio_s_artifact_draw_drawing_diff::DrawingDiff;

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Mutation
/// 🔒️ `set-layer-locked` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree,semio_framework_value::NativeReceive)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-locked")]
pub struct SetLayerLocked {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_locked<Op: From<SetLayerLocked>>(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, locked: bool) -> Op {
    Op::from(SetLayerLocked { layer_id, locked })
}

impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for SetLayerLocked where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<SetLayerLocked> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-locked", record: "SetLayerLocked" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer \"{}\" locked to {}", self.layer_id, self.locked), &format!("Sperre von Ebene \"{}\" auf {} setzen", self.layer_id, self.locked))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.to_string_owner()]
    }
}

impl<Op> protocol::ColdMutationKind<DrawingSnapshot, Op> for SetLayerLocked where Op: protocol::ColdMutation<DrawingSnapshot, ReferenceDiff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<SetLayerLocked> {
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Mutation
