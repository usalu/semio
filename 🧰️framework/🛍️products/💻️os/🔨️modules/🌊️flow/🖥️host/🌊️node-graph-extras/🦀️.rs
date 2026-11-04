//! 🌊️ Flow-backed node graph scene fields: the shared extras a plugin publishes beside its retained flow session.

use super::*;

/// 🌊️ Flow-backed NodeGraphScene fields required for wgpu FlowHost sync.
///
/// 🛍️ Deliberately carries NEITHER the operator records NOR the catalogue sections: both are
/// app-static and ride [`FlowAppCatalogue`] on the reserved catalogue surface exactly once per app
/// instance. See that type for the byte measurement that forced the split.
#[derive(Clone, Debug)]
pub struct FlowBackedNodeGraphExtras {
    pub host_snapshot_json: Option<String>,
    pub capabilities_json: Option<String>,
    pub lod_json: Option<String>,
    pub eval_json: Option<String>,
    pub computing_json: Option<String>,
    pub status_json: Option<String>,
}

/// 🌊️ Builds shared NodeGraphScene fields for flow-backed plugins. `session`, when set, contributes
/// `eval_json`/`status_json` from the in-process [`FlowEvalSession`] (never persisted in config).
///
/// 🪞️ Reads status directly from the original borrowed document and session authorities.
pub fn flow_backed_node_graph_extras(host_snapshot: &dyn FlowSnapshotSource, lod_mode: &str, proximity_distance: f64, grid_visible: bool, grid_snap_enabled: bool, grid_factor: f64, session: Option<&FlowEvalSession>) -> FlowBackedNodeGraphExtras {
    let automatic = lod_mode.is_empty() || lod_mode == FLOW_LOD_MODE_AUTOMATIC;
    let status_json=session.map(|session|session.status_json_for_snapshot(host_snapshot));
    FlowBackedNodeGraphExtras {
        host_snapshot_json: Some(semio_framework_pack_json::to_json_string(&FlowSnapshotView{schema:host_snapshot.schema(),camera:host_snapshot.camera(),widgets:host_snapshot.widgets(),synapses:host_snapshot.synapses(),layout:host_snapshot.layout()})),
        capabilities_json: Some(r#"{"engine":"flow","spotlight":true,"noteEdit":true,"clusters":true,"previewToggle":true}"#.into()),
        lod_json: Some(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("automatic".to_string(), semio_framework_pack_json::Value::Bool(automatic)),
            ("forcedLabel".to_string(), if automatic { semio_framework_pack_json::Value::Null } else { semio_framework_pack_json::Value::String(lod_mode.to_string()) }),
            ("proximityDistance".to_string(), semio_framework_pack_json::Value::Number(proximity_distance.into())),
            ("gridVisible".to_string(), semio_framework_pack_json::Value::Bool(grid_visible)),
            ("gridSnapEnabled".to_string(), semio_framework_pack_json::Value::Bool(grid_snap_enabled)),
            ("gridFactor".to_string(), semio_framework_pack_json::Value::Number(grid_factor.into())),
        ]))),
        eval_json: session.map(|session| session.eval_json().to_string()),
        computing_json: None,
        status_json,
    }
}
