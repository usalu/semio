//! 🎙️ `insert-channel` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct InsertChannel {
    pub(crate) index: usize,
    pub(crate) channel: SemioAudioChannel,
}

impl protocol::MutationKind<SemioAudioSnapshot, SemioAudioMutation> for InsertChannel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "channel", kind: "insert-channel", record: "InsertChannel" };

    fn diff(&self, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<<SemioAudioMutation as protocol::Mutation<SemioAudioSnapshot>>::Diff> {
        agg_diff(&SemioAudioMutation::InsertChannel(self.clone()), base)
    }
    fn inverse(&self, base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
        agg_inverse(&SemioAudioMutation::InsertChannel(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-channel".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
