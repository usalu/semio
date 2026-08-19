//! ✏️ Draw mutation — `RenameLayer`: changes one layer's identity `name` field.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ `rename-layer` payload — `new_name` per the taxonomy's naming convention for identity fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-layer")]
pub struct RenameLayer {
    pub layer_id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn rename_layer(layer_id: String, new_name: String) -> DrawMutation {
    DrawMutation::RenameLayer(RenameLayer { layer_id, new_name })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for RenameLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "layer", kind: "rename-layer", record: "RenamedLayer" };

    async fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename layer \"{}\" to \"{}\"", self.layer_id, self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
