//! 🏷️ `set-worksheet-content-type` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWorksheetContentType {
        pub(crate) path: String,
        pub(crate) content_type: String,
    }

impl protocol::MutationKind<XlsxSnapshot, XlsxTransitionalMutation> for SetWorksheetContentType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "worksheet-content-type", kind: "set-worksheet-content-type", record: "SetWorksheetContentType" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<<XlsxTransitionalMutation as protocol::Mutation<XlsxSnapshot>>::Diff> {
        agg_diff(&XlsxTransitionalMutation::SetWorksheetContentType(self.clone()), base)
    }
    fn inverse(&self, base: &XlsxSnapshot) -> Vec<XlsxTransitionalMutation> {
        agg_inverse(&XlsxTransitionalMutation::SetWorksheetContentType(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-worksheet-content-type".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
