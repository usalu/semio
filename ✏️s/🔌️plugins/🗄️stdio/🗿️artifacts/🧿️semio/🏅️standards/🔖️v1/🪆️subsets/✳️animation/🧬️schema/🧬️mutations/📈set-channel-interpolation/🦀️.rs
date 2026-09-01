//! 📈️ `set-channel-interpolation` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct SetChannelInterpolation {
    pub(crate) timeline_index: usize,
    pub(crate) index: usize,
    pub(crate) interpolation: AnimInterpolation,
}

impl protocol::MutationKind<SemioAnimationSnapshot, SemioAnimationMutation> for SetChannelInterpolation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "channel-interpolation", kind: "set-channel-interpolation", record: "SetChannelInterpolation" };

    fn diff(&self, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<<SemioAnimationMutation as protocol::Mutation<SemioAnimationSnapshot>>::Diff> {
        agg_diff(&SemioAnimationMutation::SetChannelInterpolation(self.clone()), base)
    }
    fn inverse(&self, base: &SemioAnimationSnapshot) -> Vec<SemioAnimationMutation> {
        agg_inverse(&SemioAnimationMutation::SetChannelInterpolation(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-channel-interpolation".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
