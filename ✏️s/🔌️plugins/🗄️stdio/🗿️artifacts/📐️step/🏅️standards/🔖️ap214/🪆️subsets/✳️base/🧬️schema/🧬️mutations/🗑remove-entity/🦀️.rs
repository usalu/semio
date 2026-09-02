//! 🗑️ `remove-entity` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEntity {
    pub(crate) id: u64,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for RemoveEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entity", kind: "remove-entity", record: "RemoveEntity" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepMutation as protocol::Mutation<StepSnapshot>>::Diff> {
        agg_diff(&StepMutation::RemoveEntity(self.clone()), base)
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepMutation> {
        agg_inverse(&StepMutation::RemoveEntity(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-entity".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
