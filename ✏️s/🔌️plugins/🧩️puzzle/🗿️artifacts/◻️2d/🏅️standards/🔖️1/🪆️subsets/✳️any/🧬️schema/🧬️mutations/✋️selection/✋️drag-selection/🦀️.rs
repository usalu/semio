//! 🗾️ regions by one common offset. The gesture's own inputs (which ids, which offset) are the payload, so
//! ✋️ Puzzle2d mutation — `DragSelection`: a relative, parametric drag of a set of nodes and target
//! editing the drag in history re-derives every position from whatever base it replays on.

use crate::standards::v1::subsets::any::schema::mutations::move_node::MoveNode;
use crate::standards::v1::subsets::any::schema::mutations::move_target_region::MoveTargetRegion;
use semio_framework_value_persistent::list::PagedList;
use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use {crate::standards::v1::subsets::any::schema::mutations::selection::puzzle2d_selection_items, crate::standards::v1::subsets::any::schema::mutations::selection::puzzle2d_selection_number};

use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-selection` payload — node and target-region ids (classified by document membership) and
/// the offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "drag-selection")]
pub struct DragSelection {
    pub targets: PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_selection<Op: From<DragSelection>>(targets: PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>, dx: f64, dy: f64) -> Op {
    Op::from(DragSelection { targets, dx, dy })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for DragSelection
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<DragSelection> + From<MoveNode> + From<MoveTargetRegion>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "selection", kind: "drag-selection", record: "DraggedSelection" };

    

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (puzzle2d_selection_number(self.dx), puzzle2d_selection_number(self.dy));
        let (en, de) = puzzle2d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en})"), &format!("{de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.iter().map(PagedUtf8::to_string_owner).collect()
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for DragSelection
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<DragSelection> + From<MoveNode> + From<MoveTargetRegion>,
 {
fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?

    })
}
}
//#endregion 🔖️Mutation
