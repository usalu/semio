//! 🚩️ `set-sample-flags` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct SetSampleFlags {
    pub(crate) stream_index: usize,
    pub(crate) index: usize,
    pub(crate) pts: u64,
    pub(crate) key: bool,
}

impl protocol::MutationKind<SemioVideoSnapshot, SemioVideoMutation> for SetSampleFlags {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "sample-flags", kind: "set-sample-flags", record: "SetSampleFlags" };

    fn diff(&self, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<<SemioVideoMutation as protocol::Mutation<SemioVideoSnapshot>>::Diff> {
        agg_diff(&SemioVideoMutation::SetSampleFlags(self.clone()), base)
    }
    fn inverse(&self, base: &SemioVideoSnapshot) -> Vec<SemioVideoMutation> {
        agg_inverse(&SemioVideoMutation::SetSampleFlags(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-sample-flags".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
