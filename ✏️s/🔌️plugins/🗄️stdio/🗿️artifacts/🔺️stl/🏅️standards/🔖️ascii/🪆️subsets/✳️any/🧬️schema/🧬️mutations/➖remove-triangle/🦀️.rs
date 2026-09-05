//! ➖️ `remove-triangle` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
/// ➖️ Removes the triangle at `index` (no-op if out of range).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveTriangle {
    pub(crate) index: usize,
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for RemoveTriangle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "triangle", kind: "remove-triangle", record: "RemoveTriangle" };

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as protocol::Mutation<StlSnapshot>>::Diff> {
        agg_diff(&StlMutation::RemoveTriangle(self.clone()), base)
    }
    fn inverse(&self, base: &StlSnapshot) -> Vec<StlMutation> {
        agg_inverse(&StlMutation::RemoveTriangle(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-triangle".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
