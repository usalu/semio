//! 🔄️ `rotate-elements` — a relative turn of a set of model elements in place about one world axis (ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12, §17.6, §20.15): every element's BASE orientation is composed with the
//! turn, so editing the axis or the angle in history re-derives every orientation from whatever base it replays on.

use super::*;

//#region 🔖️Payload
/// 🔄️ `rotate-elements` payload — the element ids it turns, the world axis and the right-handed angle (radians).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RotateElements {
    pub targets: Vec<String>,
    pub axis: [f64; 3],
    pub angle: f64,
}

impl RotateElements {
    /// 🧭️ The unit `(x, y, z, w)` turn this leaf composes onto every element — `None` for a non-finite payload or a
    /// zero-length axis (the declared `axis-nonzero` invariant).
    pub fn delta(&self) -> Option<SemioQuaternion> {
        let length = (self.axis[0] * self.axis[0] + self.axis[1] * self.axis[1] + self.axis[2] * self.axis[2]).sqrt();
        if !length.is_finite() || length == 0.0 || !self.angle.is_finite() {
            return None;
        }
        let (sin, cos) = (self.angle * 0.5).sin_cos();
        Some(SemioQuaternion { x: self.axis[0] / length * sin, y: self.axis[1] / length * sin, z: self.axis[2] / length * sin, w: cos })
    }
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for RotateElements {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "elements", kind: "rotate-elements", record: "RotatedElements" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
        let Some(delta) = self.delta() else {
            return protocol::MutationOutcome::fatal("mutation.invariant", "axis-nonzero: a turn needs a finite angle about a finite axis of non-zero length", self.targets.clone());
        };
        relative_placement_diff(&self.targets, self.angle == 0.0, base, |placement| placement.rotation = quaternion_product(delta, placement.rotation))
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
        Ok(relative_placement_inverse(&self.targets, self.angle == 0.0 || self.delta().is_none(), base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (items_en, items_de) = element_count_label(self.targets.len());
        let (degrees_en, degrees_de) = number_label(self.angle.to_degrees());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {items_en} by {degrees_en}°"), &format!("{items_de} um {degrees_de}° drehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Payload
