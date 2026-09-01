//! 📐️ `insert-frame-component` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertFrameComponent {
        pub(crate) index: usize,
        pub(crate) component: JpgFrameComponent,
    }

impl protocol::MutationKind<JpgSnapshot, JpgBaselineMutation> for InsertFrameComponent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "frame-component", kind: "insert-frame-component", record: "InsertFrameComponent" };

    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<<JpgBaselineMutation as protocol::Mutation<JpgSnapshot>>::Diff> {
        agg_diff(&JpgBaselineMutation::InsertFrameComponent(self.clone()), base)
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgBaselineMutation> {
        agg_inverse(&JpgBaselineMutation::InsertFrameComponent(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-frame-component".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
