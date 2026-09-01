//! 🧱️ `add-element` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddElement {
    pub(crate) index: usize,
    pub(crate) element: PlyElement,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for AddElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "element", kind: "add-element", record: "AddElement" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as protocol::Mutation<PlySnapshot>>::Diff> {
        agg_diff(&PlyMutation::AddElement(self.clone()), base)
    }
    fn inverse(&self, base: &PlySnapshot) -> Vec<PlyMutation> {
        agg_inverse(&PlyMutation::AddElement(self.clone()), base)
    }
    fn label(&self) -> String {
        "add-element".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
