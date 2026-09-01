//! 🔗️ `set-style-based-on` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStyleBasedOn {
    pub(crate) id: String,
    pub(crate) based_on: Option<String>,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetStyleBasedOn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "style-based-on", kind: "set-style-based-on", record: "SetStyleBasedOn" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::SetStyleBasedOn(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxMutation> {
        agg_inverse(&DocxMutation::SetStyleBasedOn(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-style-based-on".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
