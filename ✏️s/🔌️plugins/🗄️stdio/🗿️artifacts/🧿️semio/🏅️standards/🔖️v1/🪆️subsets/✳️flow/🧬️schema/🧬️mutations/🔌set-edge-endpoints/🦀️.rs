//! 🔌️ `set-edge-endpoints` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEdgeEndpoints {
    pub(crate) id: String,
    pub(crate) from: PortRef,
    pub(crate) to: PortRef,
}

impl protocol::MutationKind<SemioFlowSnapshot, SemioFlowMutation> for SetEdgeEndpoints {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "edge-endpoints", kind: "set-edge-endpoints", record: "SetEdgeEndpoints" };

    fn diff(&self, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<<SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::Diff> {
        agg_diff(&SemioFlowMutation::SetEdgeEndpoints(self.clone()), base)
    }
    fn inverse(&self, base: &SemioFlowSnapshot) -> Vec<SemioFlowMutation> {
        agg_inverse(&SemioFlowMutation::SetEdgeEndpoints(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-edge-endpoints".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
