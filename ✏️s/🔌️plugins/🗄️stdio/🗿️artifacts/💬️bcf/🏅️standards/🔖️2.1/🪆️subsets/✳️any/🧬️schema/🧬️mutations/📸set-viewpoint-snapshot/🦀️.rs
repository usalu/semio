//! 📸️ `set-viewpoint-snapshot` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by construction
//! rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetViewpointSnapshot {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
    pub(crate) snapshot: Option<Vec<u8>>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetViewpointSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "viewpoint-snapshot", kind: "set-viewpoint-snapshot", record: "SetViewpointSnapshot" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as protocol::Mutation<BcfSnapshot>>::Diff> {
        agg_diff(&BcfMutation::SetViewpointSnapshot(self.clone()), base)
    }
    fn inverse(&self, base: &BcfSnapshot) -> Vec<BcfMutation> {
        agg_inverse(&BcfMutation::SetViewpointSnapshot(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-viewpoint-snapshot".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
