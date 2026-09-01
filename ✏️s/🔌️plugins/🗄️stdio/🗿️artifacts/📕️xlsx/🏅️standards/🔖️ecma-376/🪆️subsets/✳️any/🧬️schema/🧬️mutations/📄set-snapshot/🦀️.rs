//! 📄️ `set-snapshot` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub(crate) snapshot: XlsxSnapshot,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<<XlsxMutation as protocol::Mutation<XlsxSnapshot>>::Diff> {
        agg_diff(&XlsxMutation::SetSnapshot(self.clone()), base)
    }
    fn inverse(&self, base: &XlsxSnapshot) -> Vec<XlsxMutation> {
        agg_inverse(&XlsxMutation::SetSnapshot(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-snapshot".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
