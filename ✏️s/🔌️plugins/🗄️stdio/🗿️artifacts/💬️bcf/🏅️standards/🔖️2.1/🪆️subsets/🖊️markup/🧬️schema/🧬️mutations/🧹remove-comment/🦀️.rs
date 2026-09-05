//! 🧹️ `remove-comment` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveComment {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for RemoveComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "comment", kind: "remove-comment", record: "RemoveComment" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as protocol::Mutation<BcfSnapshot>>::Diff> {
        agg_diff(&BcfMutation::RemoveComment(self.clone()), base)
    }
    fn inverse(&self, base: &BcfSnapshot) -> Vec<BcfMutation> {
        agg_inverse(&BcfMutation::RemoveComment(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-comment".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
