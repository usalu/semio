//! 🚫️ `remove-block` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveBlock {
    pub(crate) name: String,
}

impl protocol::MutationKind<SemioCadSnapshot, SemioCadMutation> for RemoveBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "block", kind: "remove-block", record: "RemoveBlock" };

    fn diff(&self, base: &SemioCadSnapshot) -> protocol::MutationOutcome<<SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::Diff> {
        agg_diff(&SemioCadMutation::RemoveBlock(self.clone()), base)
    }
    fn inverse(&self, base: &SemioCadSnapshot) -> Vec<SemioCadMutation> {
        agg_inverse(&SemioCadMutation::RemoveBlock(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-block".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
