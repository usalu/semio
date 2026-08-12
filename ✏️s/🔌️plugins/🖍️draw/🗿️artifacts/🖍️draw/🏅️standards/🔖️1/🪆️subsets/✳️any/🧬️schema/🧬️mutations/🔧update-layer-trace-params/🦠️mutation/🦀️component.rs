//! 🔧 Draw mutation — `UpdateLayerTraceParams`: sets a trace layer's `params` facet (threshold +
//! simplify epsilon, always validated/persisted together — the `update` verb's cohesive-facet case).
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, DrawTraceParams};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔧 `update-layer-trace-params` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-layer-trace-params")]
pub struct UpdateLayerTraceParams {
    pub layer_id: String,
    #[dsl(block)]
    pub params: DrawTraceParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_layer_trace_params(layer_id: String, params: DrawTraceParams) -> DrawMutation {
    DrawMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id, params })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for UpdateLayerTraceParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "layer", kind: "update-layer-trace-params", record: "UpdatedLayerTraceParams" };

    fn diff(&self, base: &DrawSnapshot) -> DrawDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update layer \"{}\" trace params", self.layer_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
