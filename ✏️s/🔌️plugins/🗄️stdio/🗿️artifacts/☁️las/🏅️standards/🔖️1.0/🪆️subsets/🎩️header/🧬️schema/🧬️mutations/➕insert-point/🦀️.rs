//! ➕️ `insert-point` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! ➕️ Inserts a fully-specified point record at `index` (final position, clamped to `len`).
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertPoint {
    pub index: usize,
    pub point: LasPoint,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for InsertPoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "point", kind: "insert-point", record: "InsertPoint" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { index, point } = self;
        protocol::MutationOutcome::new(diff::diff_insert_point(base, *index, point.clone()))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({ vec![LasMutation::RemovePoint(remove_point::RemovePoint { index: (*index).min(base.points.len()) })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert point", "Punkt einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
