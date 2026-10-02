//! 🎨️ `set-run-formatting` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRunFormatting {
    pub address: DocxXmlAddress,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for SetRunFormatting {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "run-formatting", kind: "set-run-formatting", record: "SetRunFormatting" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::SetRunFormatting(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxMutation> {
        agg_inverse(&DocxMutation::SetRunFormatting(self.clone()), base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set run formatting", "Formatierung des Textlaufs setzen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.part_path.clone()).chain(self.address.node_path.iter().map(usize::to_string)).collect()
    }
}
//#endregion 🔖️Payload
