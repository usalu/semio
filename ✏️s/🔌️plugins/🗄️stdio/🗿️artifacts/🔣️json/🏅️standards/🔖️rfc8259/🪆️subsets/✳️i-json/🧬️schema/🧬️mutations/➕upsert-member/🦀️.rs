//! ➕ `upsert-member` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpsertMember {
    pub(crate) path: JsonPath,
    pub(crate) key: String,
    pub(crate) value: JsonValue,
}

impl protocol::MutationKind<JsonSnapshot, JsonIJsonMutation> for UpsertMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "member", kind: "upsert-member", record: "UpsertMember" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonIJsonMutation as protocol::Mutation<JsonSnapshot>>::Diff> {
        agg_diff(&JsonIJsonMutation::UpsertMember(self.clone()), base)
    }
    fn inverse(&self, base: &JsonSnapshot) -> Vec<JsonIJsonMutation> {
        agg_inverse(&JsonIJsonMutation::UpsertMember(self.clone()), base)
    }
    fn label(&self) -> String {
        "upsert-member".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
