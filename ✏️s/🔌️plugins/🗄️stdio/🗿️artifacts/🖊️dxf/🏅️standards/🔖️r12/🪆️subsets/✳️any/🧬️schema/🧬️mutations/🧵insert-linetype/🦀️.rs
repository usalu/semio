//! 🧵️ `insert-linetype` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertLinetype {
    pub index: usize,
    pub linetype: DxfLinetype,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for InsertLinetype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "linetype", kind: "insert-linetype", record: "InsertLinetype" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as protocol::Mutation<DxfSnapshot>>::Diff> {
        agg_diff(&DxfMutation::InsertLinetype(self.clone()), base)
    }
    fn inverse(&self, base: &DxfSnapshot) -> Vec<DxfMutation> {
        agg_inverse(&DxfMutation::InsertLinetype(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-linetype".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
