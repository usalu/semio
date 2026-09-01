//! ➖ `remove-row` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveRow {
    pub(crate) index: usize,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for RemoveRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "row", kind: "remove-row", record: "RemoveRow" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as protocol::Mutation<TsvSnapshot>>::Diff> {
        agg_diff(&TsvMutation::RemoveRow(self.clone()), base)
    }
    fn inverse(&self, base: &TsvSnapshot) -> Vec<TsvMutation> {
        agg_inverse(&TsvMutation::RemoveRow(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-row".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
