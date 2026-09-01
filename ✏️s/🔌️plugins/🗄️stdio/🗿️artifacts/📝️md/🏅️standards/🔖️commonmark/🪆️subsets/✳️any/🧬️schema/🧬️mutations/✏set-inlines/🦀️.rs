//! ✏️ `set-inlines` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetInlines {
    pub(crate) path: Vec<MdPathStep>,
    pub(crate) index: usize,
    pub(crate) inlines: Vec<MdInline>,
}

impl protocol::MutationKind<MdSnapshot, MdMutation> for SetInlines {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "inlines", kind: "set-inlines", record: "SetInlines" };

    fn diff(&self, base: &MdSnapshot) -> protocol::MutationOutcome<<MdMutation as protocol::Mutation<MdSnapshot>>::Diff> {
        agg_diff(&MdMutation::SetInlines(self.clone()), base)
    }
    fn inverse(&self, base: &MdSnapshot) -> Vec<MdMutation> {
        agg_inverse(&MdMutation::SetInlines(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-inlines".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
