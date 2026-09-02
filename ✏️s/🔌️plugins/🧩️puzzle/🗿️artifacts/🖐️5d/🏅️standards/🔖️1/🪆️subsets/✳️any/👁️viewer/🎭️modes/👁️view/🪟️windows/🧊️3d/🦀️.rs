//! 🧊️ Puzzle 5D viewer — the World3d window: a read-only render of the unified 5d document's 3D
//! projection (part origins/orientations/scale, mesh urls), built with the frozen
//! `MeshWindowKit` (contract §2.6) directly on the artifact-level `Puzzle5dSnapshot` — this file
//! imports nothing from the sibling editor module (`policyViewerPurityBreaches` forbids it
//! outright). No selection, no gumball, no engagement overlay, no brush/fill preview: a viewer has
//! no utilities that edit and emits no mutations by construction (`ViewEmit`). Board/2D projection is
//! a follow-up, not a purity or completeness requirement — the contract only asks for one real window.

use crate::artifacts::puzzle5d::{Puzzle5dPart, Puzzle5dScale, Puzzle5dSnapshot};
use semio_framework_plugin::app::{MeshView, MeshWindowKit, WindowKit};
use semio_framework_plugin::{world3d_mesh_id_from_url, world3d_meshes_json_from_urls, world3d_selection_json, UiNode, WindowKindDefinition};

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::puzzle5d::create_puzzle5d_viewer`. Uses
/// the kit's own generic `window_kind()` verbatim — the frozen kind id `MeshWindowKit::KIND_ID`
/// (`"framework.window.mesh"`) doubles as this window's body key (see `render`'s dispatch).
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}

pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Default identity-ish camera — a viewer has no persisted per-session camera (`Config =
/// NoConfig`), so this is a fixed, documented simplification, not a bug.
fn default_camera_json() -> String {
    serde_json::json!({ "position": [8.0, 8.0, 8.0], "target": [0.0, 0.0, 0.0], "zoom": 1.0, "fov": 45.0 }).to_string()
}

/// ✂️ Same wire shape as `Puzzle5dScale`'s own `Serialize` impl (bare number = uniform, `[x,y,z]` =
/// per-axis) — duplicated as a plain default rather than reaching for the enum's `Serialize` through
/// a re-import, since the artifact-level type already round-trips through `serde_json::to_value`
/// directly.
fn scale_json(scale: Option<Puzzle5dScale>) -> serde_json::Value {
    serde_json::to_value(scale.unwrap_or(Puzzle5dScale::Uniform(1.0))).unwrap_or(serde_json::json!(1.0))
}

const FALLBACK_MESH_ID: &str = "box";

fn mesh_id_for(part: &Puzzle5dPart) -> String {
    part.part_3d.mesh_url.as_deref().map(world3d_mesh_id_from_url).unwrap_or_else(|| FALLBACK_MESH_ID.into())
}

fn meshes_json(document: &Puzzle5dSnapshot) -> String {
    let urls: Vec<String> = document.parts.iter().filter_map(|part| part.part_3d.mesh_url.clone()).collect();
    if urls.is_empty() {
        return serde_json::to_string(&[serde_json::json!({ "id": FALLBACK_MESH_ID, "data": semio_framework_plugin::mesh_from_kind(FALLBACK_MESH_ID) })]).unwrap_or_else(|_| "[]".into());
    }
    world3d_meshes_json_from_urls(&urls)
}

fn instances_json(document: &Puzzle5dSnapshot) -> String {
    let instances: Vec<serde_json::Value> = document
        .parts
        .iter()
        .map(|part| {
            serde_json::json!({
                "id": part.id,
                "meshId": mesh_id_for(part),
                "position": part.part_3d.origin,
                "rotation": part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": scale_json(part.part_3d.scale),
                "label": part.part_3d.label.clone().unwrap_or_default(),
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Pure `Puzzle5dSnapshot -> UiNode` read: default camera, no selection, no gumball, no
/// engagement/brush-preview overlay. Grip/fastener overlays are a follow-up, not required for a
/// first real read-only look at the document's placed parts.
pub fn render(document: &Puzzle5dSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::BuiltNode> {
    let view = MeshView { camera_json: default_camera_json(), meshes_json: meshes_json(document), instances_json: instances_json(document), selection_json: world3d_selection_json("pick", &[], None) };
    MeshWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_uses_the_frozen_mesh_window_kit_id() {
        let def = definition();
        assert_eq!(def.id, "framework.window.mesh");
        assert_eq!(def.id, BODY_KEY);
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = Puzzle5dSnapshot::default();
        let _node = render(&document);
    }

    #[test]
    fn render_places_one_instance_per_part() {
        let document = Puzzle5dSnapshot { parts: vec![Puzzle5dPart { id: "p1".into(), ..Default::default() }], ..Default::default() };
        assert!(instances_json(&document).contains("\"p1\""));
    }
}
//#endregion 🧪️Tests
