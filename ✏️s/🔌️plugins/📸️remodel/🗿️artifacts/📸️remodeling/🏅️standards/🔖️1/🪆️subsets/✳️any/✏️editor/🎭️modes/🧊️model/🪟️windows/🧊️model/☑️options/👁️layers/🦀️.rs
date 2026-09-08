//! 👁️ Remodeling play app — the Model window's `remodeling.layers` toggle group: which point-cloud/mesh
//! layers the 3D scene draws. Supplied per frame from the LIVE config by
//! `ArtifactEditor::window_measures`, never frozen into the manifest (a manifest-frozen snapshot could not
//! reflect a toggle the user just flipped).

use crate::editor::remodeling::config::RemodelingLayerVisibility;
use crate::editor::remodeling::remodeling_window_action;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{LabelText, WindowMeasure};

//#region 🔖️Measure
pub fn measure(layers: &RemodelingLayerVisibility, labels: &RemodelingLabels) -> WindowMeasure {
    let toggle = |id: &str, icon: &str, label: LabelText, pressed: bool, layer: &str| WindowMeasure::Toggle {
        id: format!("remodeling-measure-layer-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: remodeling_window_action("setLayerVisibility", Some(dsl::DslValue::object([("layer".to_string(), dsl::DslValue::String(layer.to_string())), ("visible".to_string(), dsl::DslValue::Bool(!pressed))]))),
    };
    WindowMeasure::Group {
        id: "remodeling-measure-layers".into(),
        label: labels.layers.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            toggle("mesh", "box", labels.layer_mesh, layers.mesh, "mesh"),
            toggle("dense", "cloud", labels.layer_dense, layers.dense, "dense"),
            toggle("sparse", "sparkles", labels.layer_sparse, layers.sparse, "sparse"),
            toggle("cameras", "camera", labels.layer_cameras, layers.cameras, "cameras"),
            toggle("gcps", "crosshair", labels.layer_gcps, layers.gcps, "gcps"),
        ],
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
