//! 🔀 Draw mutation — `SetLayerBooleanOperation`: sets a boolean layer's `operation` scalar.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔀 `set-layer-boolean-operation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "set-layer-boolean-operation")]
pub struct SetLayerBooleanOperation {
    pub layer_id: String,
    pub boolean_operation: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_boolean_operation(layer_id: String, boolean_operation: String) -> DrawMutation {
    DrawMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id, boolean_operation })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for SetLayerBooleanOperation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-boolean-operation", record: "SetLayerBooleanOperation" };

    fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
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
