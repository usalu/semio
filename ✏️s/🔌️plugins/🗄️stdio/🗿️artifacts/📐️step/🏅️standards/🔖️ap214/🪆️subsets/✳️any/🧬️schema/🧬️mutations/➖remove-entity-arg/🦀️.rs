//! ➖️ `remove-entity-arg` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEntityArg {
    pub(crate) id: u64,
    pub(crate) arg_index: usize,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for RemoveEntityArg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entity-arg", kind: "remove-entity-arg", record: "RemoveEntityArg" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepMutation as protocol::Mutation<StepSnapshot>>::Diff> {
        agg_diff(&StepMutation::RemoveEntityArg(self.clone()), base)
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepMutation> {
        agg_inverse(&StepMutation::RemoveEntityArg(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-entity-arg".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
