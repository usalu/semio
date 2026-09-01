//! 🎼️ `set-frames` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFrames {
    pub frames: Vec<Mp3Frame>,
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for SetFrames {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frames", kind: "set-frames", record: "SetFrames" };

    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as protocol::Mutation<Mp3Snapshot>>::Diff> {
        agg_diff(&Mp3Mutation::SetFrames(self.clone()), base)
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Vec<Mp3Mutation> {
        agg_inverse(&Mp3Mutation::SetFrames(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-frames".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
