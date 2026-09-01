//! 🗝️ `set-map-entry` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMapEntry {
    pub(crate) path: SemioValuePath,
    pub(crate) key: String,
    pub(crate) value: SemioValue,
}

impl protocol::MutationKind<SemioValueSnapshot, SemioValueMutation> for SetMapEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "map-entry", kind: "set-map-entry", record: "SetMapEntry" };

    fn diff(&self, base: &SemioValueSnapshot) -> protocol::MutationOutcome<<SemioValueMutation as protocol::Mutation<SemioValueSnapshot>>::Diff> {
        agg_diff(&SemioValueMutation::SetMapEntry(self.clone()), base)
    }
    fn inverse(&self, base: &SemioValueSnapshot) -> Vec<SemioValueMutation> {
        agg_inverse(&SemioValueMutation::SetMapEntry(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-map-entry".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
