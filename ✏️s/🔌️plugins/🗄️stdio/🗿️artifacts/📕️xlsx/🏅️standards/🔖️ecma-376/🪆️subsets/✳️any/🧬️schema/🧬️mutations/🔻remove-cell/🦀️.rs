//! 🔻️ `remove-cell` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveCell {
    pub(crate) sheet_name: String,
    pub(crate) row: u32,
    pub(crate) col: u32,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxMutation> for RemoveCell {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "cell", kind: "remove-cell", record: "RemoveCell" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<<XlsxMutation as protocol::Mutation<XlsxSnapshot>>::Diff> {
        agg_diff(&XlsxMutation::RemoveCell(self.clone()), base)
    }
    fn inverse(&self, base: &XlsxSnapshot) -> Vec<XlsxMutation> {
        agg_inverse(&XlsxMutation::RemoveCell(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-cell".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
