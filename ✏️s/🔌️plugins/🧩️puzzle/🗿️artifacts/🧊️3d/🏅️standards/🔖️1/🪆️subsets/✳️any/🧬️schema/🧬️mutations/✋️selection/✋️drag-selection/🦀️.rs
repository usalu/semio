//! ✋️ Puzzle3d mutation — `DragSelection`: a relative, parametric drag of a set of objects and target
//! volumes by one common world offset. The gesture's own inputs (which ids, which offset) are the
//! payload, so editing the drag in history re-derives every origin from whatever base it replays on.
use crate::standards::v1::subsets::any::schema::mutations::{MoveObject, MoveTargetVolume, ReplaceAttractionGeometry, RotateObject, ScaleObject};
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use {crate::standards::v1::subsets::any::schema::mutations::selection::puzzle3d_selection_items, crate::standards::v1::subsets::any::schema::mutations::selection::puzzle3d_selection_triple};

use crate::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-selection` payload — object and target-volume ids (classified by document membership) and
/// the world offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "drag-selection")]
pub struct DragSelection {
    pub targets: Vec<String>,
    pub offset: [f64; 3],
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for DragSelection
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<DragSelection> + From<MoveObject> + From<MoveTargetVolume> + From<ReplaceAttractionGeometry> + From<RotateObject> + From<ScaleObject>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "selection", kind: "drag-selection", record: "DraggedSelection" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (offset_en, offset_de) = puzzle3d_selection_triple(self.offset);
        let (en, de) = puzzle3d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by {offset_en}"), &format!("{de} um {offset_de} ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_selection<Op: From<DragSelection>>(targets: Vec<String>, offset: [f64; 3]) -> Op {
    Op::from(DragSelection { targets, offset })
}
