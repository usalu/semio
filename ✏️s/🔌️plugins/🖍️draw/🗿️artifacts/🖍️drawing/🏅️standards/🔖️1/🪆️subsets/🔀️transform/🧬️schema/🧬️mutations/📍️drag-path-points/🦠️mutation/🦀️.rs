//! 📍️ Drawing mutation — `DragPathPoints`: a relative, parametric drag of path anchors and handles by one world-space
//! offset — the node-editing gesture, replayable on any base; an anchor carries its attached tangents.
use crate::mutations::{drawing_label_number, DrawingMutation};
use crate::schema::geometry::editing::PathPoint;
use crate::DrawingSnapshot;

//#region 🔖️Mutation
/// 🎯️ One dragged path point: the path layer, the segment that owns the point and which of its points.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "path-point-target")]
pub struct DrawingPathPointTarget {
    pub layer_id: String,
    pub index: usize,
    pub point: PathPoint,
}

/// 📍️ `drag-path-points` payload — the dragged points and the world-space offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "drag-path-points")]
pub struct DragPathPoints {
    pub targets: Vec<DrawingPathPointTarget>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_path_points(targets: Vec<DrawingPathPointTarget>, dx: f64, dy: f64) -> DrawingMutation {
    DrawingMutation::DragPathPoints(DragPathPoints { targets, dx, dy })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for DragPathPoints {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "path-points", kind: "drag-path-points", record: "DraggedPathPoints" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (drawing_label_number(self.dx), drawing_label_number(self.dy));
        let (en, de) = match self.targets.len() {
            1 => ("1 path point".to_string(), "1 Pfadpunkt".to_string()),
            count => (format!("{count} path points"), format!("{count} Pfadpunkte")),
        };
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en})"), &format!("{de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        drag_path_points_layers(&self.targets)
    }
}

/// 🗂️ The addressed path layer ids in first-seen order, each once.
pub fn drag_path_points_layers(targets: &[DrawingPathPointTarget]) -> Vec<String> {
    let mut layers: Vec<String> = Vec::new();
    for target in targets {
        if !layers.contains(&target.layer_id) {
            layers.push(target.layer_id.clone());
        }
    }
    layers
}
//#endregion 🔖️Mutation
