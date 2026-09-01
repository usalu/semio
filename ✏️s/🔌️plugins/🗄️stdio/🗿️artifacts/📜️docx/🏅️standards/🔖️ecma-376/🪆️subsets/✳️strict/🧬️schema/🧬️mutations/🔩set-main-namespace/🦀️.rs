//! 🔩️ `set-main-namespace` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMainNamespace {
        pub(crate) namespace: String,
    }

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for SetMainNamespace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "main-namespace", kind: "set-main-namespace", record: "SetMainNamespace" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxStrictMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxStrictMutation::SetMainNamespace(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxStrictMutation> {
        agg_inverse(&DocxStrictMutation::SetMainNamespace(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-main-namespace".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
