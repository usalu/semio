//! ➖️ `remove-point` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! ➖️ Removes the point record at `index` — an out-of-range index is rejected by apply with the snapshot untouched.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemovePoint {
    pub index: usize,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for RemovePoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "point", kind: "remove-point", record: "RemovePoint" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(diff::diff_remove_point(base, *index))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.points.get(*index) {
                Some(p) => vec![LasMutation::InsertPoint(insert_point::InsertPoint { index: *index, point: p.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove point", "Punkt entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
