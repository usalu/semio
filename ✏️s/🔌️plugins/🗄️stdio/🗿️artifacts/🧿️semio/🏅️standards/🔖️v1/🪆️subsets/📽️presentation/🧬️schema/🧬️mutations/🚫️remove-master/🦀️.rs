//! 🪄 `remove-master` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveMaster {
    pub(crate) id: String,
}

impl protocol::MutationKind<SemioPresentationSnapshot, SemioPresentationMutation> for RemoveMaster {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "master", kind: "remove-master", record: "RemoveMaster" };

    fn diff(&self, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<<SemioPresentationMutation as protocol::Mutation<SemioPresentationSnapshot>>::Diff> {
        agg_diff(&SemioPresentationMutation::RemoveMaster(self.clone()), base)
    }
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Vec<SemioPresentationMutation> {
        agg_inverse(&SemioPresentationMutation::RemoveMaster(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-master".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
