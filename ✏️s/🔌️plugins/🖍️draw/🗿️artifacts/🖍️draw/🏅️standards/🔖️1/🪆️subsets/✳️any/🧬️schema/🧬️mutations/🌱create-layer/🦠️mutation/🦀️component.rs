//! 🌱 Draw mutation — `CreateLayer`: brings a new id-keyed layer into existence at an address
//! (root when `parent_id` is `None`, `index` FINAL-state — appends when `None`).
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-layer` payload — full initial payload plus optional (parent, index) address.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-layer")]
pub struct CreateLayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    #[dsl(statements)]
    pub layer: Box<DrawLayerNode>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_layer(parent_id: Option<String>, index: Option<usize>, layer: DrawLayerNode) -> DrawMutation {
    DrawMutation::CreateLayer(CreateLayer { parent_id, index, layer: Box::new(layer) })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for CreateLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "layer", kind: "create-layer", record: "CreatedLayer" };

    async fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create layer \"{}\"", crate::artifacts::draw::schema::layer_id(&self.layer))
    }
    async fn target(&self) -> Vec<String> {
        vec![crate::artifacts::draw::schema::layer_id(&self.layer).to_string()]
    }
}
//#endregion 🔖️Mutation
