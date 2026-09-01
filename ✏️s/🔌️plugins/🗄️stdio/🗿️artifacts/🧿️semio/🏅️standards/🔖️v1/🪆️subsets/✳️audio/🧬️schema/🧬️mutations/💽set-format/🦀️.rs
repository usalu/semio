//! 💽️ `set-format` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetFormat {
    pub(crate) format: SemioAudioFormat,
}

impl protocol::MutationKind<SemioAudioSnapshot, SemioAudioMutation> for SetFormat {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "format", kind: "set-format", record: "SetFormat" };

    fn diff(&self, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<<SemioAudioMutation as protocol::Mutation<SemioAudioSnapshot>>::Diff> {
        agg_diff(&SemioAudioMutation::SetFormat(self.clone()), base)
    }
    fn inverse(&self, base: &SemioAudioSnapshot) -> Vec<SemioAudioMutation> {
        agg_inverse(&SemioAudioMutation::SetFormat(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-format".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
