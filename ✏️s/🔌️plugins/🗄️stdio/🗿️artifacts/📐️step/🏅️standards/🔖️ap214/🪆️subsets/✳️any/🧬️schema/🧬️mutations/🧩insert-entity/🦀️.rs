//! 🧩️ `insert-entity` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertEntity {
    pub(crate) index: usize,
    pub(crate) entity: StepEntity,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for InsertEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "entity", kind: "insert-entity", record: "InsertEntity" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepMutation as protocol::Mutation<StepSnapshot>>::Diff> {
        agg_diff(&StepMutation::InsertEntity(self.clone()), base)
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepMutation> {
        agg_inverse(&StepMutation::InsertEntity(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-entity".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
