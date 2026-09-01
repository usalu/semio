//! 🔲 `set-cell` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCell {
    pub(crate) row_index: usize,
    pub(crate) field_index: usize,
    pub(crate) value: String,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for SetCell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "cell", kind: "set-cell", record: "SetCell" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as protocol::Mutation<TsvSnapshot>>::Diff> {
        agg_diff(&TsvMutation::SetCell(self.clone()), base)
    }
    fn inverse(&self, base: &TsvSnapshot) -> Vec<TsvMutation> {
        agg_inverse(&TsvMutation::SetCell(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-cell".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
