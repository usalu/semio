//! 🚮️ `remove-sample` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct RemoveSample {
    pub(crate) stream_index: usize,
    pub(crate) index: usize,
}

impl protocol::MutationKind<SemioVideoSnapshot, SemioVideoMutation> for RemoveSample {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "sample", kind: "remove-sample", record: "RemoveSample" };

    fn diff(&self, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<<SemioVideoMutation as protocol::Mutation<SemioVideoSnapshot>>::Diff> {
        agg_diff(&SemioVideoMutation::RemoveSample(self.clone()), base)
    }
    fn inverse(&self, base: &SemioVideoSnapshot) -> Vec<SemioVideoMutation> {
        agg_inverse(&SemioVideoMutation::RemoveSample(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-sample".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
