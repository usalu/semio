//! 🔖️ `remove-conformance-attribute` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveConformanceAttribute {}

impl protocol::MutationKind<XlsxSnapshot, XlsxTransitionalMutation> for RemoveConformanceAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "conformance-attribute", kind: "remove-conformance-attribute", record: "RemoveConformanceAttribute" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<<XlsxTransitionalMutation as protocol::Mutation<XlsxSnapshot>>::Diff> {
        agg_diff(&XlsxTransitionalMutation::RemoveConformanceAttribute(self.clone()), base)
    }
    fn inverse(&self, base: &XlsxSnapshot) -> Vec<XlsxTransitionalMutation> {
        agg_inverse(&XlsxTransitionalMutation::RemoveConformanceAttribute(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-conformance-attribute".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
