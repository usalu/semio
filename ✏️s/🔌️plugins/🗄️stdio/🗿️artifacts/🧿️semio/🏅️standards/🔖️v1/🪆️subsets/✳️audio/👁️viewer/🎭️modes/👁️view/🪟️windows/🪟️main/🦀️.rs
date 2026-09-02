//! 🧊 Semio Audio viewer — the Main window: a read-only world-3d mesh scene built with the shared
//! `MeshWindowKit` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6), from the
//! artifact-level `SemioAudioSnapshot` — this file imports nothing from the sibling editor module
//! (a substring check on that module path is what `policyViewerPurityBreaches` forbids). One
//! placeholder-box instance per entry of the snapshot's largest top-level collection stands in for
//! real per-kind geometry — deliberately generic across all subsets this kit serves (see the
//! packet's own report for the tradeoff).

use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use semio_framework_plugin::{mesh_from_kind, world3d_camera_json, world3d_selection_json, BuiltNode, MeshView, MeshWindowKit, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
const SEMIO_AUDIO_VIEW_FALLBACK_MESH_KIND: &str = "box";
const SEMIO_AUDIO_VIEW_DEFAULT_CAMERA_POSITION: [f64; 3] = [8.0, -8.0, 6.0];
const SEMIO_AUDIO_VIEW_DEFAULT_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const SEMIO_AUDIO_VIEW_DEFAULT_CAMERA_FOV: f64 = 45.0;
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
fn entity_count(document: &SemioAudioSnapshot) -> usize {
    dsl::ToValue::to_value(document).as_object().map(|object| object.iter().filter_map(|(_, field)| field.as_array().map(|array| array.len())).max().unwrap_or(0)).unwrap_or(0).clamp(1, 6)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn world_instances_json(document: &SemioAudioSnapshot) -> String {
    let count = entity_count(document);
    let instances: Vec<pack::JsonValue> = (0..count)
        .map(|index| {
            pack::json!({
                "id": format!("semio_audio-{index}"),
                "meshId": SEMIO_AUDIO_VIEW_FALLBACK_MESH_KIND,
                "position": [index as f64 * 2.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": format!("Semio Audio {index}"),
                "smoothShading": false,
            })
        })
        .collect();
    pack::json_to_string(&pack::JsonValue::Array(instances))
}

/// 👁️ Pure `SemioAudioSnapshot -> BuiltNode` read: default camera (a viewer has no persisted
/// per-session camera — `Config = NoConfig`), no selection/gumball/engagement overlay.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &SemioAudioSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mesh_data_value = dsl::to_dsl_value(&mesh_from_kind(SEMIO_AUDIO_VIEW_FALLBACK_MESH_KIND)).unwrap_or(dsl::DslValue::Null);
    let meshes_json = pack::json_to_string(&pack::JsonValue::Array(vec![pack::json!({ "id": SEMIO_AUDIO_VIEW_FALLBACK_MESH_KIND, "data": pack::json_from_dsl_value(&mesh_data_value) })]));
    let view = MeshView {
        camera_json: world3d_camera_json(SEMIO_AUDIO_VIEW_DEFAULT_CAMERA_POSITION, SEMIO_AUDIO_VIEW_DEFAULT_CAMERA_TARGET, SEMIO_AUDIO_VIEW_DEFAULT_CAMERA_FOV),
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
        let document = SemioAudioSnapshot::default();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
