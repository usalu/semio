//! 🧊 Semio Text editor — the Main window: the SAME world-3d mesh scene shape as the sibling
//! viewer window, built with the shared `MeshWindowKit`'s EDITABLE variant (contract §2.6, action id
//! `set-vertex`). Render is identical to the viewer's read: both surfaces are pure reads of the same
//! artifact-level `SemioTextSnapshot` — this window itself never mutates; mutation is the surface
//! root's `handle()` responsibility.

use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use semio_framework_plugin::{mesh_from_kind, world3d_camera_json, world3d_selection_json, BuiltNode, MeshView, MeshWindowKit, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
const SEMIO_TEXT_EDIT_FALLBACK_MESH_KIND: &str = "box";
const SEMIO_TEXT_EDIT_DEFAULT_CAMERA_POSITION: [f64; 3] = [8.0, -8.0, 6.0];
const SEMIO_TEXT_EDIT_DEFAULT_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const SEMIO_TEXT_EDIT_DEFAULT_CAMERA_FOV: f64 = 45.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by the surface root's `create_*_editor`. The EDITABLE
/// variant — `MeshWindowKit::editable_window_kind()` — carries the frozen `set-vertex` action.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::editable_window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Same kind-agnostic content signal as the sibling viewer window (duplicated, not imported —
/// `policyViewerPurityBreaches` is one-directional but the two surfaces still never reference each
/// other by design).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_count(document: &SemioTextSnapshot) -> usize {
    dsl::ToValue::to_value(document).as_object().map(|object| object.iter().filter_map(|(_, field)| field.as_array().map(|array| array.len())).max().unwrap_or(0)).unwrap_or(0).clamp(1, 6)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn world_instances_json(document: &SemioTextSnapshot) -> String {
    let count = entity_count(document);
    let instances: Vec<pack::JsonValue> = (0..count)
        .map(|index| {
            pack::json!({
                "id": format!("semio_text-{index}"),
                "meshId": SEMIO_TEXT_EDIT_FALLBACK_MESH_KIND,
                "position": [index as f64 * 2.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": format!("Semio Text {index}"),
                "smoothShading": false,
            })
        })
        .collect();
    pack::json_to_string(&pack::JsonValue::from(instances))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &SemioTextSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let meshes_json = pack::json_to_string(&pack::JsonValue::from(vec![pack::json_object([
        ("id".to_string(), pack::JsonValue::from(SEMIO_TEXT_EDIT_FALLBACK_MESH_KIND)),
        ("data".to_string(), pack::json_from_dsl_value(&dsl::to_dsl_value(&mesh_from_kind(SEMIO_TEXT_EDIT_FALLBACK_MESH_KIND)).expect("MeshData serializes"))),
    ])]));
    let view = MeshView {
        camera_json: world3d_camera_json(SEMIO_TEXT_EDIT_DEFAULT_CAMERA_POSITION, SEMIO_TEXT_EDIT_DEFAULT_CAMERA_TARGET, SEMIO_TEXT_EDIT_DEFAULT_CAMERA_FOV),
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
        let document = SemioTextSnapshot::default();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
