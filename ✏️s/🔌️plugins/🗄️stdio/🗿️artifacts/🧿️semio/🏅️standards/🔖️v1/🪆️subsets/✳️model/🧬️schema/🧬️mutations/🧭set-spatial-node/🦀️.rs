//! 🧭️ `set-spatial-node` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct SetSpatialNode {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) kind: Option<SpatialKind>,
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub(crate) parent_id: Option<Option<String>>,
    #[serde(default)]
    pub(crate) placement: Option<SemioTransform>,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for SetSpatialNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "spatial-node", kind: "set-spatial-node", record: "SetSpatialNode" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::Diff> {
        agg_diff(&SemioModelMutation::SetSpatialNode(self.clone()), base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
        agg_inverse(&SemioModelMutation::SetSpatialNode(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-spatial-node".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
