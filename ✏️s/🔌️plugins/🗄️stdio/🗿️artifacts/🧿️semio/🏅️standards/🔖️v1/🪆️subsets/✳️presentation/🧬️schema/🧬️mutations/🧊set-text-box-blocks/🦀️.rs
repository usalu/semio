//! 🧊 `set-textbox-blocks` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.
//!
//! 🧭️ `SEMANTICS.kind` — and this leaf's folder — is `set-text-box-blocks`:
//! `dsl::Mutations`' derive asserts `SEMANTICS.kind == to_kebab("SetTextBoxBlocks")`, and
//! `to_kebab` splits before every uppercase letter that follows a lowercase one, so `TextBox` ->
//! `text-box` (verified against the already-migrated `svg` baseline's `SetViewBox` ->
//! `set-view-box`). The op-text/binary keyword (`print_op`/`parse_op`/`OP_KEYWORDS`, this
//! artifact's grammar files and the committed test fixtures) is a SEPARATE vocabulary the derive
//! does not see, and keeps its established `set-textbox-blocks` spelling unchanged — only this
//! leaf's internal descriptor identity changes.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTextBoxBlocks {
    pub(crate) slide_index: usize,
    pub(crate) shape_index: usize,
    pub(crate) blocks: Vec<DocBlock>,
}

impl protocol::MutationKind<SemioPresentationSnapshot, SemioPresentationMutation> for SetTextBoxBlocks {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "text-box-blocks", kind: "set-text-box-blocks", record: "SetTextBoxBlocks" };

    fn diff(&self, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<<SemioPresentationMutation as protocol::Mutation<SemioPresentationSnapshot>>::Diff> {
        agg_diff(&SemioPresentationMutation::SetTextBoxBlocks(self.clone()), base)
    }
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Vec<SemioPresentationMutation> {
        agg_inverse(&SemioPresentationMutation::SetTextBoxBlocks(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-text-box-blocks".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
