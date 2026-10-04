//! 🔀 Drawing mutation — `SetLayerBooleanOperation`: sets a boolean layer's `operation` scalar.
use crate::diff::DrawingDiff;
use crate::mutations::DrawingMutation;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 🔀 `set-layer-boolean-operation` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
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
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer \"{}\" boolean operation to {}", self.layer_id, self.boolean_operation), &format!("Boolesche Operation von Ebene \"{}\" auf {} setzen", self.layer_id, self.boolean_operation))
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
