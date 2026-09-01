//! 🔖️ `set-id3v1` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetId3v1 {
    pub id3v1: Option<Id3v1Tag>,
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for SetId3v1 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "id3v1", kind: "set-id3v1", record: "SetId3v1" };

    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as protocol::Mutation<Mp3Snapshot>>::Diff> {
        agg_diff(&Mp3Mutation::SetId3v1(self.clone()), base)
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Vec<Mp3Mutation> {
        agg_inverse(&Mp3Mutation::SetId3v1(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-id3v1".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
