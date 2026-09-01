//! 🔁️ `replace-block` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceBlock {
    pub(crate) path: Vec<MdPathStep>,
    pub(crate) index: usize,
    pub(crate) block: MdBlock,
}

impl protocol::MutationKind<MdSnapshot, MdMutation> for ReplaceBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "block", kind: "replace-block", record: "ReplaceBlock" };

    fn diff(&self, base: &MdSnapshot) -> protocol::MutationOutcome<<MdMutation as protocol::Mutation<MdSnapshot>>::Diff> {
        agg_diff(&MdMutation::ReplaceBlock(self.clone()), base)
    }
    fn inverse(&self, base: &MdSnapshot) -> Vec<MdMutation> {
        agg_inverse(&MdMutation::ReplaceBlock(self.clone()), base)
    }
    fn label(&self) -> String {
        "replace-block".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
