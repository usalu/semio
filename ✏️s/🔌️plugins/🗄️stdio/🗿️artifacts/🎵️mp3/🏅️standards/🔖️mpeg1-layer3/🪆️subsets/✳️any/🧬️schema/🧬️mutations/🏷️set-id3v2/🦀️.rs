//! 🏷️ `set-id3v2` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetId3v2 {
    pub id3v2: Option<Id3v2Tag>,
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for SetId3v2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "id3v2", kind: "set-id3v2", record: "SetId3v2" };

    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as protocol::Mutation<Mp3Snapshot>>::Diff> {
        agg_diff(&Mp3Mutation::SetId3v2(self.clone()), base)
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Vec<Mp3Mutation> {
        agg_inverse(&Mp3Mutation::SetId3v2(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-id3v2".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
