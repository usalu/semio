//! 🧊️ volumes by one common world offset. The gesture's own inputs (which ids, which offset) are the
//! ✋️ Puzzle3d mutation — `DragSelection`: a relative, parametric drag of a set of objects and target
//! payload, so editing the drag in history re-derives every origin from whatever base it replays on.
use {semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::MoveObject, semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::MoveTargetVolume, semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry, semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::RotateObject, semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::ScaleObject};
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use {crate::puzzle3d_selection_items, crate::puzzle3d_selection_triple};

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-selection` payload — object and target-volume ids (classified by document membership) and
/// the world offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
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

    

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (offset_en, offset_de) = puzzle3d_selection_triple(self.offset);
        let (en, de) = puzzle3d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by {offset_en}"), &format!("{de} um {offset_de} ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle3dSnapshot, Op> for DragSelection
where
    Op: protocol::ColdMutation<Puzzle3dSnapshot, ReferenceDiff = Puzzle3dDiff> + From<DragSelection> + From<MoveObject> + From<MoveTargetVolume> + From<ReplaceAttractionGeometry> + From<RotateObject> + From<ScaleObject>,
 {
fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?

    })
}
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_selection<Op: From<DragSelection>>(targets: Vec<String>, offset: [f64; 3]) -> Op {
    Op::from(DragSelection { targets, offset })
}
