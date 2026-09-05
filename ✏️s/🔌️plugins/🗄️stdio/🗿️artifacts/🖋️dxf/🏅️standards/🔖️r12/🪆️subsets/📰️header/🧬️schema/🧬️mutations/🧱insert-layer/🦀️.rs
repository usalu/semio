//! 🧱️ `insert-layer` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertLayer {
    pub index: usize,
    pub layer: DxfLayer,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for InsertLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "layer", kind: "insert-layer", record: "InsertLayer" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as protocol::Mutation<DxfSnapshot>>::Diff> {
        agg_diff(&DxfMutation::InsertLayer(self.clone()), base)
    }
    fn inverse(&self, base: &DxfSnapshot) -> Vec<DxfMutation> {
        agg_inverse(&DxfMutation::InsertLayer(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-layer".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
