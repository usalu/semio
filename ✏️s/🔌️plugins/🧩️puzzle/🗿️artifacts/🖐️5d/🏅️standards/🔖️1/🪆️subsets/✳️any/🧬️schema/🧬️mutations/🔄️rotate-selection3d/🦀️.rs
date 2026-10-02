//! 🔄️ Puzzle5d mutation — `RotateSelection3d`: a relative, parametric turn of a set of parts and target volumes, each about its own
//! world origin, by one world-axis rotation. The gesture's own inputs (which ids, which axis, which angle) are the
//! payload, so editing the turn in history re-derives every orientation from whatever base it replays on.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_items, puzzle5d_selection_number, Puzzle5dMutation};
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔄️ `rotate-selection3d` payload — part and target-volume ids, the world axis and the angle in radians (right-handed about the axis).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rotate-selection3d")]
pub struct RotateSelection3d {
    pub targets: Vec<String>,
    pub axis: [f64; 3],
    #[dsl(angle = "rad")]
    pub angle: f64,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for RotateSelection3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "selection", kind: "rotate-selection3d", record: "RotatedSelection3d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (degrees_en, degrees_de) = puzzle5d_selection_number(self.angle.to_degrees());
        let (en, de) = puzzle5d_selection_items(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {en} by {degrees_en}°"), &format!("{de} um {degrees_de}° drehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_selection_3d(targets: Vec<String>, axis: [f64; 3], angle: f64) -> Puzzle5dMutation {
    Puzzle5dMutation::RotateSelection3d(RotateSelection3d { targets, axis, angle })
}
