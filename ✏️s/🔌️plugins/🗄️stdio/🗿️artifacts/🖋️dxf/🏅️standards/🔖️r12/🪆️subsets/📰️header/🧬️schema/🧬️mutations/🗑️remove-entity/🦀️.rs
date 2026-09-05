//! 🗑️ `remove-entity` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEntity {
    pub index: usize,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for RemoveEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entity", kind: "remove-entity", record: "RemoveEntity" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as protocol::Mutation<DxfSnapshot>>::Diff> {
        agg_diff(&DxfMutation::RemoveEntity(self.clone()), base)
    }
    fn inverse(&self, base: &DxfSnapshot) -> Vec<DxfMutation> {
        agg_inverse(&DxfMutation::RemoveEntity(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-entity".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
