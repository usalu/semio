//! 🔇️ `remove-channel` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct RemoveChannel {
    pub(crate) index: usize,
}

impl protocol::MutationKind<SemioAudioSnapshot, SemioAudioMutation> for RemoveChannel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "channel", kind: "remove-channel", record: "RemoveChannel" };

    fn diff(&self, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<<SemioAudioMutation as protocol::Mutation<SemioAudioSnapshot>>::Diff> {
        agg_diff(&SemioAudioMutation::RemoveChannel(self.clone()), base)
    }
    fn inverse(&self, base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
        agg_inverse(&SemioAudioMutation::RemoveChannel(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-channel".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
