//! ➕️ `insert-triangle` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
/// ➕️ Inserts a fully-specified triangle at `index` (final position, clamped to `len`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertTriangle {
    pub(crate) index: usize,
    pub(crate) triangle: StlTriangle,
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for InsertTriangle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "triangle", kind: "insert-triangle", record: "InsertTriangle" };

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as Mutation<StlSnapshot>>::Diff> {
        let Self { index, triangle } = self;
        protocol::MutationOutcome::new(diff::diff_insert_triangle(*index, *triangle))
    }
    fn inverse(&self, base: &StlSnapshot) -> Result<Vec<StlMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            {
                vec![StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index: (*index).min(base.triangles.len()) })]
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert triangle", "Dreieck einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
