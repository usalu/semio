//! ✋️ Puzzle3d mutation — `DragSelection`: a relative, parametric drag of a set of objects and target
//! volumes by one common world offset. The gesture's own inputs (which ids, which offset) are the
//! payload, so editing the drag in history re-derives every origin from whatever base it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_items, puzzle3d_selection_triple, Puzzle3dMutation};
use crate::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-selection` payload — object and target-volume ids (classified by document membership) and
/// the world offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "drag-selection")]
pub struct DragSelection {
    pub targets: Vec<String>,
    pub offset: [f64; 3],
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for DragSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "selection", kind: "drag-selection", record: "DraggedSelection" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let (offset_en, offset_de) = puzzle3d_selection_triple(self.offset);
        let (en, de) = puzzle3d_selection_items(self.targets.len());
        protocol::LocalizedLabel::native(&format!("Drag {en} by {offset_en}"), &format!("{de} um {offset_de} ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_selection(targets: Vec<String>, offset: [f64; 3]) -> Puzzle3dMutation {
    Puzzle3dMutation::DragSelection(DragSelection { targets, offset })
}
