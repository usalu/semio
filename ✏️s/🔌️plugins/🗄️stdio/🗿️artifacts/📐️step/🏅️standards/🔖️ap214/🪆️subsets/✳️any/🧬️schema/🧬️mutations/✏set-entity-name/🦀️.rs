//! ✏️️ `set-entity-name` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntityName {
    pub(crate) id: u64,
    pub(crate) name: String,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for SetEntityName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity-name", kind: "set-entity-name", record: "SetEntityName" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepMutation as protocol::Mutation<StepSnapshot>>::Diff> {
        agg_diff(&StepMutation::SetEntityName(self.clone()), base)
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepMutation> {
        agg_inverse(&StepMutation::SetEntityName(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-entity-name".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
