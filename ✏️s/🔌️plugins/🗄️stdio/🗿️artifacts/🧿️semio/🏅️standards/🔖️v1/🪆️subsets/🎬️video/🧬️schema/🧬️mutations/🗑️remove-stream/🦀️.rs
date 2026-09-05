//! 🗑️ `remove-stream` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveStream {
    pub(crate) index: usize,
}

impl protocol::MutationKind<SemioVideoSnapshot, SemioVideoMutation> for RemoveStream {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "stream", kind: "remove-stream", record: "RemoveStream" };

    fn diff(&self, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<<SemioVideoMutation as protocol::Mutation<SemioVideoSnapshot>>::Diff> {
        agg_diff(&SemioVideoMutation::RemoveStream(self.clone()), base)
    }
    fn inverse(&self, base: &SemioVideoSnapshot) -> Vec<SemioVideoMutation> {
        agg_inverse(&SemioVideoMutation::RemoveStream(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-stream".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
