//! 👁️ Draw mutation — `SetLayerVisible`: flips one layer's `visible` flag (addressed, single-field
//! setter — the taxonomy's own canonical `set` example).
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 👁️ `set-layer-visible` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "set-layer-visible")]
pub struct SetLayerVisible {
    pub layer_id: String,
    pub visible: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_visible(layer_id: String, visible: bool) -> DrawMutation {
    DrawMutation::SetLayerVisible(SetLayerVisible { layer_id, visible })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for SetLayerVisible {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-visible", record: "SetLayerVisible" };

    fn diff(&self, base: &DrawSnapshot) -> DrawDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set layer \"{}\" visible to {}", self.layer_id, self.visible)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
