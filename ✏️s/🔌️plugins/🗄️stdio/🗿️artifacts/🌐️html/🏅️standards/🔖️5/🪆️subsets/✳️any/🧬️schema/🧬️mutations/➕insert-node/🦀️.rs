//! ➕️ `insert-node` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertNode {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
    pub(crate) node: HtmlNode,
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for InsertNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "node", kind: "insert-node", record: "InsertNode" };

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as protocol::Mutation<HtmlSnapshot>>::Diff> {
        agg_diff(&HtmlMutation::InsertNode(self.clone()), base)
    }
    fn inverse(&self, base: &HtmlSnapshot) -> Vec<HtmlMutation> {
        agg_inverse(&HtmlMutation::InsertNode(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-node".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
