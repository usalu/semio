//! 💬️ `set-comment` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetComment {
    pub(crate) path: NodePath,
    pub(crate) text: String,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "comment", kind: "set-comment", record: "SetComment" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as protocol::Mutation<HtmlSnapshot>>::Diff> {
        agg_diff(&HtmlMutation::SetComment(self.clone()), base)
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Vec<HtmlMutation> {
        agg_inverse(&HtmlMutation::SetComment(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-comment".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
