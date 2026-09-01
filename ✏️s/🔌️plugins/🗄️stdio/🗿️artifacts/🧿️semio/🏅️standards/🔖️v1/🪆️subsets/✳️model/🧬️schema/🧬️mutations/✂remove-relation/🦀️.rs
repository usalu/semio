//! ✂️ `remove-relation` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveRelation {
    pub(crate) id: String,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for RemoveRelation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "relation", kind: "remove-relation", record: "RemoveRelation" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::Diff> {
        agg_diff(&SemioModelMutation::RemoveRelation(self.clone()), base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
        agg_inverse(&SemioModelMutation::RemoveRelation(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-relation".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
