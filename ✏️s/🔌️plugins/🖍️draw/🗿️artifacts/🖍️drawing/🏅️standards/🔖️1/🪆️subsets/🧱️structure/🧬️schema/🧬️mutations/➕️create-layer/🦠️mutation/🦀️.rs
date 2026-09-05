//! 🌱 Drawing mutation — `CreateLayer`: brings a new id-keyed layer into existence at an address
//! (root when `parent_id` is `None`, `index` FINAL-state — appends when `None`).
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Mutation
/// 🌱 `create-layer` payload — full initial payload plus optional (parent, index) address.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-layer")]
pub struct CreateLayer {
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub index: Option<usize>,
    #[dsl(statements)]
    pub layer: Box<DrawingLayerNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_layer(parent_id: Option<String>, index: Option<usize>, layer: DrawingLayerNode) -> DrawingMutation {
    DrawingMutation::CreateLayer(CreateLayer { parent_id, index, layer: Box::new(layer) })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for CreateLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create layer \"{}\"", crate::artifacts::drawing::schema::layer_id(&self.layer))
    }
    fn target(&self) -> Vec<String> {
        vec![crate::artifacts::drawing::schema::layer_id(&self.layer).to_string()]
    }
}
//#endregion 🔖️Mutation
