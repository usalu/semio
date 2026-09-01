//! 📝️ `set-raw-text` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRawText {
    pub(crate) path: NodePath,
    pub(crate) text: String,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetRawText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "raw-text", kind: "set-raw-text", record: "SetRawText" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as protocol::Mutation<HtmlSnapshot>>::Diff> {
        agg_diff(&HtmlMutation::SetRawText(self.clone()), base)
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Vec<HtmlMutation> {
        agg_inverse(&HtmlMutation::SetRawText(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-raw-text".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
