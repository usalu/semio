//! 📤️ `remove-row` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveRow {
    pub(crate) element_name: String,
    pub(crate) index: usize,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for RemoveRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "row", kind: "remove-row", record: "RemoveRow" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as Mutation<PlySnapshot>>::Diff> {
        agg_diff(&PlyMutation::RemoveRow(self.clone()), base)
    }
    fn inverse(&self, base: &PlySnapshot) -> Vec<PlyMutation> {
        agg_inverse(&PlyMutation::RemoveRow(self.clone()), base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove row", "Zeile entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
