//! ➖️ `remove-triangle` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
/// ➖️ Removes the triangle at `index` (no-op if out of range).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveTriangle {
    pub(crate) index: usize,
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for RemoveTriangle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "triangle", kind: "remove-triangle", record: "RemoveTriangle" };

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as Mutation<StlSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(diff::diff_remove_triangle(*index))
    }
    fn inverse(&self, base: &StlSnapshot) -> Result<Vec<StlMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.triangles.get(*index) {
                Some(t) => vec![StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index: *index, triangle: *t })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove triangle", "Dreieck entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
