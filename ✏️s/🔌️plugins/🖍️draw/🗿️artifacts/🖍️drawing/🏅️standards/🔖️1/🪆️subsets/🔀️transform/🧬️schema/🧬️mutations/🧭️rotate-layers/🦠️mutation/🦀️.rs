//! 🧭️ Drawing mutation — `RotateLayers`: a relative, parametric rotation of a set of layers by one angle about one
//! world-space pivot — the rotate handle's gesture, replayable on any base.
use crate::mutations::{drawing_label_layers, drawing_label_number, DrawingMutation};
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 🧭️ `rotate-layers` payload — the rotated layer ids, the world-space pivot and the angle in radians.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rotate-layers")]
pub struct RotateLayers {
    pub targets: Vec<String>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub angle: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_layers(targets: Vec<String>, pivot_x: f64, pivot_y: f64, angle: f64) -> DrawingMutation {
    DrawingMutation::RotateLayers(RotateLayers { targets, pivot_x, pivot_y, angle })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for RotateLayers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "layers", kind: "rotate-layers", record: "RotatedLayers" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (degrees_en, degrees_de) = drawing_label_number(self.angle.to_degrees());
        let (en, de) = drawing_label_layers(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {en} by {degrees_en}°"), &format!("{de} um {degrees_de}° drehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
