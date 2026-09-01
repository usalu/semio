//! 🎨️ `set-run-formatting` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRunFormatting {
    pub(crate) path: DocxBlockPath,
    pub(crate) run_index: usize,
    pub(crate) bold: bool,
    pub(crate) italic: bool,
    pub(crate) underline: bool,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetRunFormatting {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run-formatting", kind: "set-run-formatting", record: "SetRunFormatting" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::SetRunFormatting(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxMutation> {
        agg_inverse(&DocxMutation::SetRunFormatting(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-run-formatting".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
