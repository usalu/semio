//! 🔠️ `set-shared-string` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSharedString {
    pub(crate) index: usize,
    pub(crate) value: String,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for SetSharedString {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shared-string", kind: "set-shared-string", record: "SetSharedString" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<<XlsxMutation as protocol::Mutation<XlsxSnapshot>>::Diff> {
        agg_diff(&XlsxMutation::SetSharedString(self.clone()), base)
    }
    fn inverse(&self, base: &XlsxSnapshot) -> Vec<XlsxMutation> {
        agg_inverse(&XlsxMutation::SetSharedString(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-shared-string".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
