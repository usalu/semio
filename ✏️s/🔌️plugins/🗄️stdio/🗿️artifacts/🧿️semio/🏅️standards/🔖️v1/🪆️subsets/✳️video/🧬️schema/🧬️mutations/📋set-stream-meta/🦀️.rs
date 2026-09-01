//! 📋️ `set-stream-meta` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStreamMeta {
    pub(crate) index: usize,
    pub(crate) kind: SemioVideoStreamKind,
    pub(crate) codec: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) rate: SemioRational,
}

impl protocol::MutationKind<SemioVideoSnapshot, SemioVideoMutation> for SetStreamMeta {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "stream-meta", kind: "set-stream-meta", record: "SetStreamMeta" };

    fn diff(&self, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<<SemioVideoMutation as protocol::Mutation<SemioVideoSnapshot>>::Diff> {
        agg_diff(&SemioVideoMutation::SetStreamMeta(self.clone()), base)
    }
    fn inverse(&self, base: &SemioVideoSnapshot) -> Vec<SemioVideoMutation> {
        agg_inverse(&SemioVideoMutation::SetStreamMeta(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-stream-meta".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
