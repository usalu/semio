//! ✂️ `remove-edge` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEdge {
    pub(crate) id: String,
}

impl protocol::MutationKind<SemioFlowSnapshot, SemioFlowMutation> for RemoveEdge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "edge", kind: "remove-edge", record: "RemoveEdge" };

    fn diff(&self, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<<SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::Diff> {
        agg_diff(&SemioFlowMutation::RemoveEdge(self.clone()), base)
    }
    fn inverse(&self, base: &SemioFlowSnapshot) -> Vec<SemioFlowMutation> {
        agg_inverse(&SemioFlowMutation::RemoveEdge(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-edge".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
