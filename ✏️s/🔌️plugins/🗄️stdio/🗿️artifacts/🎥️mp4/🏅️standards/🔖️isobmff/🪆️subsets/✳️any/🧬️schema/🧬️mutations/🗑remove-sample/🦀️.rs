//! 🗑️ `remove-sample` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-sample")]
pub struct RemoveSample {
    pub track_index: usize,
    pub index: usize,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for RemoveSample {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "sample", kind: "remove-sample", record: "RemoveSample" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::Diff> {
        agg_diff(&Mp4Mutation::RemoveSample(self.clone()), base)
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Vec<Mp4Mutation> {
        agg_inverse(&Mp4Mutation::RemoveSample(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-sample".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
