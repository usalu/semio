//! ✏️ Draw mutation — `RenameLayer`: changes one layer's identity `name` field.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Mutation
/// ✏️ `rename-layer` payload — `new_name` per the taxonomy's naming convention for identity fields.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-layer")]
pub struct RenameLayer {
    pub layer_id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_layer(layer_id: String, new_name: String) -> DrawMutation {
    DrawMutation::RenameLayer(RenameLayer { layer_id, new_name })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for RenameLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "layer", kind: "rename-layer", record: "RenamedLayer" };

    fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename layer \"{}\" to \"{}\"", self.layer_id, self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
