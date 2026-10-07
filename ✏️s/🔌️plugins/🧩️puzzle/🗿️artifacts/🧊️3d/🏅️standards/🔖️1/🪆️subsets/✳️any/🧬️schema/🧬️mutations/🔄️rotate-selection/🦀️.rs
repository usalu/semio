//! 🔄️ Puzzle3d mutation — `RotateSelection`: a relative, parametric turn of a set of objects and target
//! volumes, each about its own origin, by one world-axis rotation. The gesture's own inputs (which ids,
//! which axis, which angle) are the payload, so editing the turn in history re-derives every orientation
//! from whatever base it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_items,puzzle3d_selection_number,Puzzle3dMutation};

use crate::Puzzle3dSnapshot;

//#region 🔖️Mutation
/// 🔄️ `rotate-selection` payload — object and target-volume ids, the world axis and the angle in
/// radians (right-handed about the axis).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    pub targets: Vec<String>,
    pub axis: [f64; 3],
    #[dsl(angle = "rad")]
    pub angle: f64,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for RotateSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "selection", kind: "rotate-selection", record: "RotatedSelection" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
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
pub fn rotate_selection(targets: Vec<String>, axis: [f64; 3], angle: f64) -> Puzzle3dMutation {
    Puzzle3dMutation::RotateSelection(RotateSelection { targets, axis, angle })
}
