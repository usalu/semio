//! ➕️ `insert-track` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "insert-track")]
pub struct InsertTrack {
    pub index: usize,
    #[dsl(block)]
    pub track: Mp4Track,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for InsertTrack {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "track", kind: "insert-track", record: "InsertTrack" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::Diff> {
        agg_diff(&Mp4Mutation::InsertTrack(self.clone()), base)
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Vec<Mp4Mutation> {
        agg_inverse(&Mp4Mutation::InsertTrack(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-track".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
