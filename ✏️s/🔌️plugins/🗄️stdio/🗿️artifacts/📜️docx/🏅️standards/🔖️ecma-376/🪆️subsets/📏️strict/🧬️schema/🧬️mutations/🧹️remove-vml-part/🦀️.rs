//! 📐️ `remove-vml-part` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveVmlPart {
        pub(crate) path: String,
    }

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for RemoveVmlPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "vml-part", kind: "remove-vml-part", record: "RemoveVmlPart" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxStrictMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxStrictMutation::RemoveVmlPart(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxStrictMutation> {
        agg_inverse(&DocxStrictMutation::RemoveVmlPart(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-vml-part".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
