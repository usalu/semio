//! 📥 `insert-array-element` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertArrayElement {
    pub(crate) path: JsonPath,
    pub(crate) index: usize,
    pub(crate) value: JsonValue,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for InsertArrayElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "array-element", kind: "insert-array-element", record: "InsertArrayElement" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as protocol::Mutation<JsonSnapshot>>::Diff> {
        agg_diff(&JsonIJsonMutation::InsertArrayElement(self.clone()), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Vec<JsonIJsonMutation> {
        agg_inverse(&JsonIJsonMutation::InsertArrayElement(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-array-element".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
