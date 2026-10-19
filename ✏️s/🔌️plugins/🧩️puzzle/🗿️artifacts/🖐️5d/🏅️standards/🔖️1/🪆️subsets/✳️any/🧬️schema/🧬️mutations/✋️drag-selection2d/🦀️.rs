//! ↔️ ids, which offset) are the payload, so editing the drag in history re-derives every board position from whatever base
//! ✋️ Puzzle5d mutation — `DragSelection2d`: a relative, parametric drag of a set of parts on the board by one flat offset. The gesture's own inputs (which
//! it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items, puzzle5d_selection_number};

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;

use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-selection2d` payload — part ids and the flat board offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
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

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for DragSelection2d
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<DragSelection2d> + From<crate::standards::v1::subsets::any::schema::mutations::move_part_2d::MovePart2d>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "selection", kind: "drag-selection2d", record: "DraggedSelection2d" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (puzzle5d_selection_number(self.dx), puzzle5d_selection_number(self.dy));
        let (en, de) = puzzle5d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en}) on the board"), &format!("{de} auf dem Brett um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}


impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for DragSelection2d
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<DragSelection2d>,
{
    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_selection_2d<Op>(targets: Vec<String>, dx: f64, dy: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::drag_selection_2d::DragSelection2d>,
{
    Op::from(DragSelection2d { targets, dx, dy })
}
