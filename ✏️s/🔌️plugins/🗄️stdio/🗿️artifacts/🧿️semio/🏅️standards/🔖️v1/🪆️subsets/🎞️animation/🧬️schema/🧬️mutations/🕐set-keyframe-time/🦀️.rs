//! 🕐️ `set-keyframe-time` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetKeyframeTime {
    pub timeline_index: usize,
    pub channel_index: usize,
    pub index: usize,
    pub t: f64,
}

impl protocol::MutationKind<SemioAnimationSnapshot, SemioAnimationMutation> for SetKeyframeTime {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "keyframe-time", kind: "set-keyframe-time", record: "SetKeyframeTime" };

    fn diff(&self, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<<SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::Diff> {
        agg_diff(&SemioAnimationMutation::SetKeyframeTime(self.clone()), base)
    }
    fn inverse(&self, base: &SemioAnimationSnapshot) -> Vec<SemioAnimationMutation> {
        agg_inverse(&SemioAnimationMutation::SetKeyframeTime(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("set-keyframe-time", "Schlüsselbildzeit setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
