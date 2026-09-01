//! 🕳️ `remove-spatial-node` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSpatialNode {
    pub(crate) id: String,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for RemoveSpatialNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "spatial-node", kind: "remove-spatial-node", record: "RemoveSpatialNode" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::Diff> {
        agg_diff(&SemioModelMutation::RemoveSpatialNode(self.clone()), base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
        agg_inverse(&SemioModelMutation::RemoveSpatialNode(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-spatial-node".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
