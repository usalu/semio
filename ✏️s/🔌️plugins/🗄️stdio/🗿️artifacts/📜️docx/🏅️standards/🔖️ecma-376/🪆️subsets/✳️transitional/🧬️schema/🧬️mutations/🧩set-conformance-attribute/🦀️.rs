//! 🧩️ `set-conformance-attribute` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetConformanceAttribute {
        pub(crate) value: String,
    }

impl protocol::MutationKind<DocxSnapshot, DocxTransitionalMutation> for SetConformanceAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "conformance-attribute", kind: "set-conformance-attribute", record: "SetConformanceAttribute" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxTransitionalMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxTransitionalMutation::SetConformanceAttribute(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxTransitionalMutation> {
        agg_inverse(&DocxTransitionalMutation::SetConformanceAttribute(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-conformance-attribute".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
