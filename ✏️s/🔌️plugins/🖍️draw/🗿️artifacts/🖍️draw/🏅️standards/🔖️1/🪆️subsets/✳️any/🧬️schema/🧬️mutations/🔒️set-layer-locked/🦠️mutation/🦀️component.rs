//! 🔒️ Draw mutation — `SetLayerLocked`: flips one layer's `locked` flag.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔒️ `set-layer-locked` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "set-layer-locked")]
pub struct SetLayerLocked {
    pub layer_id: String,
    pub locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_layer_locked(layer_id: String, locked: bool) -> DrawMutation {
    DrawMutation::SetLayerLocked(SetLayerLocked { layer_id, locked })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for SetLayerLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer-locked", record: "SetLayerLocked" };

    fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set layer \"{}\" locked to {}", self.layer_id, self.locked)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
