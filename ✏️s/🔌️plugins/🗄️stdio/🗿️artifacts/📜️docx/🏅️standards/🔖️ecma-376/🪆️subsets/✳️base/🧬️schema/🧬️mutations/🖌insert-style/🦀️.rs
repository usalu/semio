//! 🖌️ `insert-style` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertStyle {
    pub(crate) style: DocxStyle,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for InsertStyle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "style", kind: "insert-style", record: "InsertStyle" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::InsertStyle(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxMutation> {
        agg_inverse(&DocxMutation::InsertStyle(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-style".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
