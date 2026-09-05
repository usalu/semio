//! 👁️ Remodeling play app — the Model window's `remodeling.layers` toggle group: which point-cloud/mesh
//! layers the 3D scene draws. Supplied per frame from the LIVE config by
//! `ArtifactEditor::window_measures`, never frozen into the manifest (a manifest-frozen snapshot could not
//! reflect a toggle the user just flipped).

use crate::editor::remodeling::config::RemodelingLayerVisibility;
use crate::editor::remodeling::remodeling_action;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{LabelText, WindowMeasure};
use serde_json::json;

//#region 🔖️Measure
pub async fn measure(layers: &RemodelingLayerVisibility, labels: &RemodelingLabels) -> WindowMeasure {
    let toggle = |id: &str, icon: &str, label: LabelText, pressed: bool, layer: &str| WindowMeasure::Toggle {
        id: format!("remodeling-measure-layer-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: remodeling_action("setLayerVisibility", Some(json!({ "layer": layer, "visible": !pressed }))),
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
mod tests {
    use super::*;
    use crate::editor::remodeling::config::RemodelingConfig;

    #[semio_framework_async_macros::async_test]
    async fn every_layer_gets_its_own_toggle_and_a_default_open_group() {
        let config = RemodelingConfig::default();
        let labels = semio_framework_plugin::resolve_labels_for_locale::<RemodelingLabels>("en-US");
        let WindowMeasure::Group { children, default_open, .. } = measure(&config.layers, labels) else { panic!("expected a Group measure") };
        assert_eq!(children.len(), 5);
        assert_eq!(default_open, Some(true));
    }
}
//#endregion 🧪️Tests
