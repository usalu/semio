//! 🎚️ `set-sample-rate` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSampleRate {
    pub(crate) sample_rate: u32,
}

impl protocol::MutationKind<SemioAudioSnapshot, SemioAudioMutation> for SetSampleRate {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "sample-rate", kind: "set-sample-rate", record: "SetSampleRate" };

    fn diff(&self, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<<SemioAudioMutation as protocol::Mutation<SemioAudioSnapshot>>::Diff> {
        agg_diff(&SemioAudioMutation::SetSampleRate(self.clone()), base)
    }
    fn inverse(&self, base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
        agg_inverse(&SemioAudioMutation::SetSampleRate(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-sample-rate".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
