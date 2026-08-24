//! 🧊️ Puzzle 3d viewer — the Mesh window: a read-only world-3d render of the real
//! `Puzzle3dSnapshot`, built directly from the framework `MeshWindowKit` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6) plus this file's own small, pure
//! snapshot→view-model helpers — no selection, no engagement, no gumball/utilities: a viewer has no
//! utilities that edit and emits no mutations by construction (`ViewEmit`). This file imports nothing
//! from the sibling editor module (`policyViewerPurityBreaches` forbids it outright).

use crate::artifacts::puzzle3d::{Puzzle3dObject, Puzzle3dScale, Puzzle3dSnapshot};
use semio_framework_plugin::app::{MeshView, MeshWindowKit, WindowKit};
use semio_framework_plugin::{world3d_default_camera, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_selection_json, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
/// 👁️ Matches the editor's `PUZZLE3D_FALLBACK_MESH_KIND` literal ("box") — duplicated on purpose
/// rather than imported through the sibling editor module, which `policyViewerPurityBreaches` forbids
/// outright.
const PUZZLE3D_VIEW_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::puzzle3d::create_puzzle3d_viewer` — the
/// framework kit's own read-only `WindowKindDefinition`, unmodified.
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️SceneJson
/// 👁️ Read-only twin of the editor's `object_scale_json`, over the real typed `Puzzle3dScale` (a
/// scalar-or-`[x,y,z]` closed union) instead of a `serde_json::Value` scratch fixture.
fn puzzle3d_view_object_scale(object: &Puzzle3dObject) -> [f64; 3] {
    match &object.scale {
        Some(Puzzle3dScale::Uniform(scale)) => [*scale, *scale, *scale],
        Some(Puzzle3dScale::Vec3(vec3)) => *vec3,
        None => [1.0, 1.0, 1.0],
    }
}

/// 👁️ Real object instances read straight off the document — no selection/hover paint, no reveal
/// cutoff (a viewer has no fill-planning session to reveal against).
fn puzzle3d_view_instances_json(document: &Puzzle3dSnapshot) -> String {
    let instances: Vec<serde_json::Value> = document
        .objects
        .iter()
        .map(|object| {
            let mesh_id = object.mesh_url.as_deref().filter(|url| !url.is_empty()).map(world3d_mesh_id_from_url).unwrap_or_else(|| PUZZLE3D_VIEW_FALLBACK_MESH_KIND.to_string());
            let scale = if object.hidden { [0.0, 0.0, 0.0] } else { puzzle3d_view_object_scale(object) };
            serde_json::json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": object.origin,
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": scale,
                "label": object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
                "disabled": object.locked,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Real mesh registry — the fallback box kind plus every distinct `meshUrl` on the document's
/// objects. A viewer never had a browser-registered GLB round-trip (`registerBrushMesh` is an editor
/// action), so every url-backed mesh resolves through the same `world3d_mesh_id_from_url` id scheme
/// the host renderer already knows how to fetch.
fn puzzle3d_view_meshes_json(document: &Puzzle3dSnapshot) -> String {
    let mut urls: Vec<String> = document.objects.iter().filter_map(|object| object.mesh_url.clone()).filter(|url| !url.is_empty()).collect();
    urls.sort();
    urls.dedup();
    world3d_meshes_json_from_kinds_and_urls(&[PUZZLE3D_VIEW_FALLBACK_MESH_KIND.to_string()], &urls)
}
//#endregion 🔖️SceneJson

//#region 🔖️Render
/// 👁️ Pure `Puzzle3dSnapshot -> UiNode` read: default camera (a viewer has no persisted per-session
/// camera — `Config = NoConfig`), no selection/gumball/engagement overlay, real objects read straight
/// off the document through the framework `MeshWindowKit`.
pub fn render(document: &Puzzle3dSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::BuiltNode> {
    let view = MeshView {
        camera_json: world3d_default_camera(),
        meshes_json: puzzle3d_view_meshes_json(document),
        instances_json: puzzle3d_view_instances_json(document),
        selection_json: world3d_selection_json("pick", &[], None),
    };
    MeshWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_the_framework_mesh_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.id, "framework.window.mesh");
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = Puzzle3dSnapshot::default();
        let _node = render(&document);
    }

    #[test]
    fn instances_json_carries_one_entry_per_object() {
        let mut document = Puzzle3dSnapshot::default();
        document.objects.push(Puzzle3dObject {
            id: "o1".into(),
            label: None,
            object_kind: None,
            anchor: Default::default(),
            origin: [1.0, 2.0, 3.0],
            orientation: None,
            scale: Some(Puzzle3dScale::Uniform(2.0)),
            mesh_url: None,
            vortices: Vec::new(),
            hidden: false,
            locked: false,
        });
        let instances: serde_json::Value = serde_json::from_str(&puzzle3d_view_instances_json(&document)).expect("instances json");
        assert_eq!(instances.as_array().map(Vec::len), Some(1));
        assert_eq!(instances[0]["scale"], serde_json::json!([2.0, 2.0, 2.0]));
    }
}
//#endregion 🧪️Tests
