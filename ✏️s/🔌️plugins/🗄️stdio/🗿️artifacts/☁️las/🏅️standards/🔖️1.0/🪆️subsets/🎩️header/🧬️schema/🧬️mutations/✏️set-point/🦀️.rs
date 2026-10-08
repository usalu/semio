//! ✏️ `set-point` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! ✏️ Replaces a point record wholesale.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPoint {
    pub index: usize,
    pub point: LasPoint,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetPoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "point", kind: "set-point", record: "SetPoint" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { index, point } = self;
        protocol::MutationOutcome::new(diff::diff_set_point(base, *index, point))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({
            match base.points.get(*index) {
                Some(p) => vec![LasMutation::SetPoint(set_point::SetPoint { index: *index, point: p.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set point", "Punkt setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
