//! 🪢️ `insert-relation` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRelation {
    pub(crate) relation: ModelRelation,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for InsertRelation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "relation", kind: "insert-relation", record: "InsertRelation" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::Diff> {
        agg_diff(&SemioModelMutation::InsertRelation(self.clone()), base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
        agg_inverse(&SemioModelMutation::InsertRelation(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-relation".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
