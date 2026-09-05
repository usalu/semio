//! ✖️ `remove-map-entry` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by construction
//! rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveMapEntry {
    pub(crate) path: SemioValuePath,
    pub(crate) key: String,
}

impl protocol::MutationKind<SemioValueSnapshot, SemioValueMutation> for RemoveMapEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "map-entry", kind: "remove-map-entry", record: "RemoveMapEntry" };

    fn diff(&self, base: &SemioValueSnapshot) -> protocol::MutationOutcome<<SemioValueMutation as protocol::Mutation<SemioValueSnapshot>>::Diff> {
        agg_diff(&SemioValueMutation::RemoveMapEntry(self.clone()), base)
    }
    fn inverse(&self, base: &SemioValueSnapshot) -> Vec<SemioValueMutation> {
        agg_inverse(&SemioValueMutation::RemoveMapEntry(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-map-entry".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
