//! 🔀 Drawing mutation — `SetLayerBooleanOperation`: sets a boolean layer's `operation` scalar.
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Mutation
/// 🔀 `set-layer-boolean-operation` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-layer-boolean-operation")]
pub struct SetLayerBooleanOperation {
    pub layer_id: String,
    pub boolean_operation: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_boolean_operation(layer_id: String, boolean_operation: String) -> DrawingMutation {
    DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id, boolean_operation })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetLayerBooleanOperation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-boolean-operation", record: "SetLayerBooleanOperation" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set layer \"{}\" boolean operation to {}", self.layer_id, self.boolean_operation)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
