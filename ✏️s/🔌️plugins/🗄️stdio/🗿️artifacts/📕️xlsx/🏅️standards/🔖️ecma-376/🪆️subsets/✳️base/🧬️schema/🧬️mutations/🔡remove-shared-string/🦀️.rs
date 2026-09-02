//! 🔡️ `remove-shared-string` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSharedString {
    pub(crate) index: usize,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for RemoveSharedString {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "shared-string", kind: "remove-shared-string", record: "RemoveSharedString" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<<XlsxMutation as protocol::Mutation<XlsxSnapshot>>::Diff> {
        agg_diff(&XlsxMutation::RemoveSharedString(self.clone()), base)
    }
    fn inverse(&self, base: &XlsxSnapshot) -> Vec<XlsxMutation> {
        agg_inverse(&XlsxMutation::RemoveSharedString(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-shared-string".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
