//! 🔄️ Puzzle3d mutation — `RotateSelection`: a relative, parametric turn of a set of objects and target
//! volumes, each about its own origin, by one world-axis rotation. The gesture's own inputs (which ids,
//! which axis, which angle) are the payload, so editing the turn in history re-derives every orientation
//! from whatever base it replays on.
use {semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::MoveObject, semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry, semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::RotateObject, semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::RotateTargetVolume, semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::ScaleObject};
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use {crate::puzzle3d_selection_items, crate::puzzle3d_selection_number};

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 🔄️ `rotate-selection` payload — object and target-volume ids, the world axis and the angle in
/// radians (right-handed about the axis).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    pub targets: Vec<String>,
    pub axis: [f64; 3],
    #[dsl(angle = "rad")]
    pub angle: f64,
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for RotateSelection
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<RotateSelection> + From<MoveObject> + From<ReplaceAttractionGeometry> + From<RotateObject> + From<RotateTargetVolume> + From<ScaleObject>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "selection", kind: "rotate-selection", record: "RotatedSelection" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (degrees_en, degrees_de) = puzzle3d_selection_number(self.angle.to_degrees());
        let (en, de) = puzzle3d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {en} by {degrees_en}°"), &format!("{de} um {degrees_de}° drehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_selection<Op: From<RotateSelection>>(targets: Vec<String>, axis: [f64; 3], angle: f64) -> Op {
    Op::from(RotateSelection { targets, axis, angle })
}
