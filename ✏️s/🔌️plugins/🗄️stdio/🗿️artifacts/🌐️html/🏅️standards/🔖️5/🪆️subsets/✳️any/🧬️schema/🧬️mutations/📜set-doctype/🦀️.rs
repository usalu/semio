//! 📜️ `set-doctype` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDoctype {
    pub(crate) doctype: Option<String>,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SetDoctype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "doctype", kind: "set-doctype", record: "SetDoctype" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as protocol::Mutation<HtmlSnapshot>>::Diff> {
        agg_diff(&HtmlMutation::SetDoctype(self.clone()), base)
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Vec<HtmlMutation> {
        agg_inverse(&HtmlMutation::SetDoctype(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-doctype".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
