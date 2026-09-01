//! 🔢 `set-safe-number` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSafeNumber {
    pub(crate) path: JsonPath,
    pub(crate) lexeme: String,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for SetSafeNumber {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "safe-number", kind: "set-safe-number", record: "SetSafeNumber" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as protocol::Mutation<JsonSnapshot>>::Diff> {
        agg_diff(&JsonIJsonMutation::SetSafeNumber(self.clone()), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Vec<JsonIJsonMutation> {
        agg_inverse(&JsonIJsonMutation::SetSafeNumber(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-safe-number".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
