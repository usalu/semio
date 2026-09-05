//! ⚙️ `set-relationship-base` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRelationshipBase {
        pub(crate) base: String,
    }

impl protocol::MutationKind<DocxSnapshot, DocxTransitionalMutation> for SetRelationshipBase {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "relationship-base", kind: "set-relationship-base", record: "SetRelationshipBase" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxTransitionalMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxTransitionalMutation::SetRelationshipBase(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxTransitionalMutation> {
        agg_inverse(&DocxTransitionalMutation::SetRelationshipBase(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-relationship-base".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
