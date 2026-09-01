//! ➕ `insert-row` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRow {
    pub(crate) index: usize,
    pub(crate) row: Vec<String>,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for InsertRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "row", kind: "insert-row", record: "InsertRow" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as protocol::Mutation<TsvSnapshot>>::Diff> {
        agg_diff(&TsvMutation::InsertRow(self.clone()), base)
    }
    fn inverse(&self, base: &TsvSnapshot) -> Vec<TsvMutation> {
        agg_inverse(&TsvMutation::InsertRow(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-row".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
