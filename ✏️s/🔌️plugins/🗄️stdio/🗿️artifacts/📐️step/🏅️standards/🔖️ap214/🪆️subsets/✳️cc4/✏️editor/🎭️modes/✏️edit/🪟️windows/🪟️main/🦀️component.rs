//! 🧊 Step CC4 editor — the Main window: the SAME world-3d mesh scene shape as the sibling
//! viewer window, built with the shared `MeshWindowKit`'s EDITABLE variant (contract §2.6, action id
//! `set-vertex`). Render is identical to the viewer's read: both surfaces are pure reads of the same
//! artifact-level `StepSnapshot` — this window itself never mutates; mutation is the surface
//! root's `handle()` responsibility.

use crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::snapshot::StepSnapshot;
use semio_framework_plugin::{mesh_from_kind, world3d_camera_json, world3d_selection_json, MeshView, MeshWindowKit, UiNode, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
const STEP_CC4_EDIT_FALLBACK_MESH_KIND: &str = "box";
const STEP_CC4_EDIT_DEFAULT_CAMERA_POSITION: [f64; 3] = [8.0, -8.0, 6.0];
const STEP_CC4_EDIT_DEFAULT_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const STEP_CC4_EDIT_DEFAULT_CAMERA_FOV: f64 = 45.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by the surface root's `create_*_editor`. The EDITABLE
/// variant — `MeshWindowKit::editable_window_kind()` — carries the frozen `set-vertex` action.
pub async fn definition() -> WindowKindDefinition {
    MeshWindowKit::editable_window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Same kind-agnostic content signal as the sibling viewer window (duplicated, not imported —
/// `policyViewerPurityBreaches` is one-directional but the two surfaces still never reference each
/// other by design).
async fn entity_count(document: &StepSnapshot) -> usize {
    serde_json::to_value(document)
        .ok()
        .and_then(|value| value.as_object().map(|object| object.values().filter_map(|field| field.as_array().map(|array| array.len())).max().unwrap_or(0)))
        .unwrap_or(0)
        .clamp(1, 6)
}

async fn world_instances_json(document: &StepSnapshot) -> String {
    let count = entity_count(document);
    let instances: Vec<serde_json::Value> = (0..count)
        .map(|index| {
            serde_json::json!({
                "id": format!("step_cc4-{index}"),
                "meshId": STEP_CC4_EDIT_FALLBACK_MESH_KIND,
                "position": [index as f64 * 2.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": format!("Step CC4 {index}"),
                "smoothShading": false,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

pub async fn render(document: &StepSnapshot) -> UiNode {
    let meshes_json = serde_json::to_string(&[serde_json::json!({ "id": STEP_CC4_EDIT_FALLBACK_MESH_KIND, "data": mesh_from_kind(STEP_CC4_EDIT_FALLBACK_MESH_KIND) })]).unwrap_or_else(|_| "[]".into());
    let view = MeshView {
        camera_json: world3d_camera_json(STEP_CC4_EDIT_DEFAULT_CAMERA_POSITION, STEP_CC4_EDIT_DEFAULT_CAMERA_TARGET, STEP_CC4_EDIT_DEFAULT_CAMERA_FOV),
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
    async fn definition_declares_the_editable_mesh_window_kit() {
        let def = definition();
        assert_eq!(def.id, MeshWindowKit::KIND_ID);
        assert!(def.actions.iter().any(|action| action.id == "set-vertex"));
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = StepSnapshot::default();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
