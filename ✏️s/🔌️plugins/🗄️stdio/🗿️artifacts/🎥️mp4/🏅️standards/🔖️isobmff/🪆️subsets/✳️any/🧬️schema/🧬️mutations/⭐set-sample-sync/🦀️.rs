//! ⭐️ `set-sample-sync` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-sample-sync")]
pub struct SetSampleSync {
    pub track_index: usize,
    pub index: usize,
    pub sync: bool,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for SetSampleSync {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "sample-sync", kind: "set-sample-sync", record: "SetSampleSync" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::Diff> {
        agg_diff(&Mp4Mutation::SetSampleSync(self.clone()), base)
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Vec<Mp4Mutation> {
        agg_inverse(&Mp4Mutation::SetSampleSync(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-sample-sync".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
