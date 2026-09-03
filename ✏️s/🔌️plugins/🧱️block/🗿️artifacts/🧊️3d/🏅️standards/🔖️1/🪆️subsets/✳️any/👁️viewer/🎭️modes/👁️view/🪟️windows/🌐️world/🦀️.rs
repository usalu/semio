//! 🌐️ Block 3D viewer — the world window: a read-only render of the object kind's representations and
//! rim-vortex templates, built from the same framework-level `world3d_*` helpers the editor's own
//! `🌍️world` compute facet uses for the equivalent geometry — this file itself imports nothing from
//! the sibling editor module (`policyViewerPurityBreaches` forbids it outright). No selection, no
//! engagement, no brush: a viewer has no utilities that edit and emits no mutations by construction
//! (`ViewEmit`). Uses the shared `MeshWindowKit`'s window-kind id/definition (contract §2.6,
//! `framework.window.mesh`) but writes its own extended render — `MeshWindowKit::render` alone has no
//! vortex slot, and this window's whole point is representations AND rim vortices together.

use crate::artifacts::block3d::{vortex_kinds_of, Block3dSnapshot};
use crate::BlockRepresentation;
use semio_framework_plugin::{build_world_3d_scene, world3d_camera_projection_json, world3d_mesh_id_from_url, world3d_scene_extended, world3d_selection_json, WindowKindDefinition, WorldProjectionConfig};
// 🚧️ SDK GAP: `MeshWindowKit`/`WindowKit` (contract §2.6) are only reachable through the `app`
// submodule they're declared in — not (yet) in `semio_framework_plugin`'s curated crate-root
// re-export list, mirroring the sibling editor module's own gap note about `Dialect`.
use semio_framework_plugin::app::{MeshWindowKit, WindowKit};
use dsl::json;
use dsl::os_pack::json::{to_string, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
pub const SURFACE_ID: &str = "block3d.view.world";
/// 👁️ Read-only counterpart of the editor's `BLOCK3D_PLAY_APP_ID` controller id — kept distinct so a
/// viewer session's world-3d controller can never be mistaken for an editor session's.
const BLOCK3D_VIEW_CONTROLLER_ID: &str = "block3d-view";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::block3d::create_block3d_viewer` — the
/// shared, read-only `MeshWindowKit` window kind (contract §2.6).
pub async fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `Block3dSnapshot -> UiNode` read: real representations become meshes/instances (zero
/// arrangement offset — a viewer has no persisted per-session window view, `Config = NoConfig`), real
/// rim-vortex templates render at their document position/color, no brush preview, no selection.
pub async fn render(document: &Block3dSnapshot) -> semio_framework_plugin::UiNode {
    let camera = &document.camera3d;
    let camera_json = world3d_camera_projection_json(camera.position, camera.target, None, camera.zoom, &WorldProjectionConfig::default());
    let meshes_json = meshes_json(&document.representations);
    let instances_json = instances_json(document, &document.representations);
    let selection_json = world3d_selection_json("rectangle", &[], None);
    let vortices_json = vortices_json(document);
    build_world_3d_scene(
        SURFACE_ID,
        BLOCK3D_VIEW_CONTROLLER_ID,
        world3d_scene_extended(camera_json, meshes_json, instances_json, selection_json, Some(vortices_json), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
    )
}

/// 👁️ Read-only twin of the editor `🌍️world` facet's `representation_mesh_id` — duplicated on purpose
/// rather than imported through the sibling editor module, which `policyViewerPurityBreaches` forbids
/// outright.
async fn representation_mesh_id(representation: &BlockRepresentation) -> String {
    representation.mesh_url.as_deref().map_or_else(|| format!("block3d-rep-{}", representation.id), world3d_mesh_id_from_url)
}

async fn meshes_json(representations: &[BlockRepresentation]) -> String {
    let meshes: Vec<Value> = representations
        .iter()
        .filter_map(|representation| {
            let url = representation.mesh_url.as_deref()?;
            Some(json!({ "id": representation_mesh_id(representation), "url": url }))
        })
        .collect();
    to_string(&Value::from(meshes))
}

async fn instances_json(document: &Block3dSnapshot, representations: &[BlockRepresentation]) -> String {
    let label = if document.object_kind.label.is_empty() { document.object_kind.name.clone() } else { document.object_kind.label.clone() };
    let instances: Vec<Value> = representations
        .iter()
        .map(|representation| {
            json!({
                "id": representation.id.as_str(),
                "meshId": representation_mesh_id(representation),
                "position": [0.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": format!("{} — {}", label, representation.name),
                "objectKind": document.object_kind.id.as_str(),
            })
        })
        .collect();
    to_string(&Value::from(instances))
}

async fn vortex_kind_color(document: &Block3dSnapshot, vortex_kind_id: &str) -> String {
    vortex_kinds_of(document).iter().find(|kind| kind.id == vortex_kind_id).map_or_else(|| "#888888".into(), |kind| kind.color.clone())
}

async fn vortices_json(document: &Block3dSnapshot) -> String {
    let vec3 = |v: [f64; 3]| Value::from(v.iter().map(|c| Value::from(*c)).collect::<Vec<Value>>());
    let records: Vec<Value> = document
        .vortices
        .iter()
        .map(|vortex| {
            json!({
                "fullId": format!("{}:{}", BLOCK3D_VIEW_CONTROLLER_ID, vortex.id),
                "objectId": document.object_kind.id.as_str(),
                "vortexKind": vortex.vortex_kind.as_str(),
                "position": vec3(vortex.position),
                "direction": vec3(vortex.direction),
                "radius": vortex.radius,
                "color": vortex_kind_color(document, &vortex.vortex_kind),
            })
        })
        .collect();
    to_string(&Value::from(records))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_shared_mesh_window_kind() {
        let def = definition();
        assert_eq!(def.id, "framework.window.mesh");
        assert_eq!(def.body_key, "framework.window.mesh");
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_empty_document() {
        let document = crate::artifacts::block3d::schema::empty_block3d_snapshot();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
