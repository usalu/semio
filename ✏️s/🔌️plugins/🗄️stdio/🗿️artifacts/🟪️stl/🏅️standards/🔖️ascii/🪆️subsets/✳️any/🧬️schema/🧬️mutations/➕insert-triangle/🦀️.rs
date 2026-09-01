//! ➕️ `insert-triangle` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
/// ➕️ Inserts a fully-specified triangle at `index` (final position, clamped to `len`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertTriangle {
    pub(crate) index: usize,
    pub(crate) triangle: StlTriangle,
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for InsertTriangle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "triangle", kind: "insert-triangle", record: "InsertTriangle" };

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as protocol::Mutation<StlSnapshot>>::Diff> {
        agg_diff(&StlMutation::InsertTriangle(self.clone()), base)
    }
    fn inverse(&self, base: &StlSnapshot) -> Vec<StlMutation> {
        agg_inverse(&StlMutation::InsertTriangle(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-triangle".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
