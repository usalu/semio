//! 🔤 `set-string` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetString {
    pub(crate) path: JsonPath,
    pub(crate) value: String,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for SetString {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "string", kind: "set-string", record: "SetString" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as protocol::Mutation<JsonSnapshot>>::Diff> {
        agg_diff(&JsonIJsonMutation::SetString(self.clone()), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Vec<JsonIJsonMutation> {
        agg_inverse(&JsonIJsonMutation::SetString(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-string".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
