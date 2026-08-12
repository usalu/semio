//! 🧬️ Draw mutation — `DuplicateLayer`: copies an existing layer to a new, content-addressed id
//! right after its source.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧬️ `duplicate-layer` payload — source address only; the duplicate's id is deterministic
/// (content-addressed from the source, see `engine::clone_draw_layer_node`), so `diff`/`inverse`
/// recompute it from BASE rather than carrying it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub layer_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_layer(layer_id: String) -> DrawMutation {
    DrawMutation::DuplicateLayer(DuplicateLayer { layer_id })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for DuplicateLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "duplicate", entity: "layer", kind: "duplicate-layer", record: "DuplicatedLayer" };

    fn diff(&self, base: &DrawSnapshot) -> DrawDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Duplicate layer \"{}\"", self.layer_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
