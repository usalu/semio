//! ✋️ Puzzle5d mutation — `DragSelection2d`: a relative, parametric drag of a set of parts on the board by one flat offset. The gesture's own inputs (which
//! ids, which offset) are the payload, so editing the drag in history re-derives every board position from whatever base
//! it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items, puzzle5d_selection_number, Puzzle5dMutation};
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-selection2d` payload — part ids and the flat board offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "drag-selection2d")]
pub struct DragSelection2d {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for DragSelection2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "selection", kind: "drag-selection2d", record: "DraggedSelection2d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (puzzle5d_selection_number(self.dx), puzzle5d_selection_number(self.dy));
        let (en, de) = puzzle5d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en}) on the board"), &format!("{de} auf dem Brett um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_selection_2d(targets: Vec<String>, dx: f64, dy: f64) -> Puzzle5dMutation {
    Puzzle5dMutation::DragSelection2d(DragSelection2d { targets, dx, dy })
}
