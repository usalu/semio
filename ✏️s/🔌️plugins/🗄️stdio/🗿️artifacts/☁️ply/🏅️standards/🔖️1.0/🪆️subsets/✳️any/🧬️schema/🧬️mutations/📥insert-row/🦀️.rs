//! 📥️ `insert-row` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRow {
    pub(crate) element_name: String,
    pub(crate) index: usize,
    pub(crate) row: PlyRow,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for InsertRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "row", kind: "insert-row", record: "InsertRow" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as protocol::Mutation<PlySnapshot>>::Diff> {
        agg_diff(&PlyMutation::InsertRow(self.clone()), base)
    }
    fn inverse(&self, base: &PlySnapshot) -> Vec<PlyMutation> {
        agg_inverse(&PlyMutation::InsertRow(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-row".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
