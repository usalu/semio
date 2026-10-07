//! ✋️ Puzzle2d mutation — `DragSelection`: a relative, parametric drag of a set of nodes and target
//! regions by one common offset. The gesture's own inputs (which ids, which offset) are the payload, so
//! editing the drag in history re-derives every position from whatever base it replays on.

use semio_framework_value::{list::PagedList, paged::PagedUtf8};

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_selection_items,puzzle2d_selection_number,Puzzle2dMutation};

use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-selection` payload — node and target-region ids (classified by document membership) and
/// the offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned)]
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
pub fn drag_selection(targets: PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>, dx: f64, dy: f64) -> Puzzle2dMutation {
    Puzzle2dMutation::DragSelection(DragSelection { targets, dx, dy })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for DragSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "selection", kind: "drag-selection", record: "DraggedSelection" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (puzzle2d_selection_number(self.dx), puzzle2d_selection_number(self.dy));
        let (en, de) = puzzle2d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en})"), &format!("{de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.iter().map(PagedUtf8::to_string_owner).collect()
    }
}
//#endregion 🔖️Mutation
