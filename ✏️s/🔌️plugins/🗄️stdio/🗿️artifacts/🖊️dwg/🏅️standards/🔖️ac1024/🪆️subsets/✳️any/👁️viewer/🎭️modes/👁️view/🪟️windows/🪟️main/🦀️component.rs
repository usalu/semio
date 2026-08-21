//! 🧊 DWG AC1024 viewer — the Main window: a read-only world-3d mesh scene built with the shared
//! `MeshWindowKit` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6), from the
//! artifact-level `DwgSnapshot` — this file imports nothing from the sibling editor module
//! (a substring check on that module path is what `policyViewerPurityBreaches` forbids). One
//! placeholder-box instance per entry of the snapshot's largest top-level collection stands in for
//! real per-kind geometry — deliberately generic across all subsets this kit serves (see the
//! packet's own report for the tradeoff).

use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_framework_plugin::{mesh_from_kind, world3d_camera_json, world3d_selection_json, BuiltNode, MeshView, MeshWindowKit, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
const DWG_AC1024_VIEW_FALLBACK_MESH_KIND: &str = "box";
const DWG_AC1024_VIEW_DEFAULT_CAMERA_POSITION: [f64; 3] = [8.0, -8.0, 6.0];
const DWG_AC1024_VIEW_DEFAULT_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const DWG_AC1024_VIEW_DEFAULT_CAMERA_FOV: f64 = 45.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by the surface root's `create_*_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure, kind-agnostic content signal: the length of the snapshot's largest top-level JSON
/// array field, clamped to a small display range — real (not fabricated), deliberately generic so
/// this same shape replicates uniformly across every subset this window kit serves without coupling
/// to field names a live peer ticket may still be refactoring.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_count(document: &DwgSnapshot) -> usize {
    serde_json::to_value(document).ok().and_then(|value| value.as_object().map(|object| object.values().filter_map(|field| field.as_array().map(|array| array.len())).max().unwrap_or(0))).unwrap_or(0).clamp(1, 6)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn world_instances_json(document: &DwgSnapshot) -> String {
    let count = entity_count(document);
    let instances: Vec<serde_json::Value> = (0..count)
        .map(|index| {
            serde_json::json!({
                "id": format!("dwg_ac1024-{index}"),
                "meshId": DWG_AC1024_VIEW_FALLBACK_MESH_KIND,
                "position": [index as f64 * 2.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": format!("DWG AC1024 {index}"),
                "smoothShading": false,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Pure `DwgSnapshot -> BuiltNode` read: default camera (a viewer has no persisted
/// per-session camera — `Config = NoConfig`), no selection/gumball/engagement overlay.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &DwgSnapshot) -> BuiltNode {
    let meshes_json = serde_json::to_string(&[serde_json::json!({ "id": DWG_AC1024_VIEW_FALLBACK_MESH_KIND, "data": mesh_from_kind(DWG_AC1024_VIEW_FALLBACK_MESH_KIND) })]).unwrap_or_else(|_| "[]".into());
    let view = MeshView {
        camera_json: world3d_camera_json(DWG_AC1024_VIEW_DEFAULT_CAMERA_POSITION, DWG_AC1024_VIEW_DEFAULT_CAMERA_TARGET, DWG_AC1024_VIEW_DEFAULT_CAMERA_FOV),
        meshes_json,
        instances_json: world_instances_json(document),
        selection_json: world3d_selection_json("rectangle", &[], None),
    };
    MeshWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_shared_mesh_window_kit() {
        assert_eq!(definition().id, MeshWindowKit::KIND_ID);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = DwgSnapshot::default();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
