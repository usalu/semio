//! 🧬️ Drawing mutation — `DuplicateLayer`: copies an existing layer to a new, content-addressed id
//! right after its source.
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Mutation
/// 🧬️ `duplicate-layer` payload — source address only; the duplicate's id is deterministic
/// (content-addressed from the source, see `engine::clone_drawing_layer_node`), so `diff`/`inverse`
/// recompute it from BASE rather than carrying it.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub layer_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn duplicate_layer(layer_id: String) -> DrawingMutation {
    DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for DuplicateLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "duplicate", entity: "layer", kind: "duplicate-layer", record: "DuplicatedLayer" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
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
