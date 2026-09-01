//! 💬️ `insert-comment` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertComment {
    pub(crate) index: usize,
    pub(crate) comment: String,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for InsertComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "comment", kind: "insert-comment", record: "InsertComment" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as protocol::Mutation<PlySnapshot>>::Diff> {
        agg_diff(&PlyMutation::InsertComment(self.clone()), base)
    }
    fn inverse(&self, base: &PlySnapshot) -> Vec<PlyMutation> {
        agg_inverse(&PlyMutation::InsertComment(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-comment".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
