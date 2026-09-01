//! 🎓 `insert-master` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertMaster {
    pub(crate) master: SlideMaster,
}

impl protocol::MutationKind<SemioPresentationSnapshot, SemioPresentationMutation> for InsertMaster {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "master", kind: "insert-master", record: "InsertMaster" };

    fn diff(&self, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<<SemioPresentationMutation as protocol::Mutation<SemioPresentationSnapshot>>::Diff> {
        agg_diff(&SemioPresentationMutation::InsertMaster(self.clone()), base)
    }
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Vec<SemioPresentationMutation> {
        agg_inverse(&SemioPresentationMutation::InsertMaster(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-master".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
