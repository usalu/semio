//! 🌐️ Block 5D viewer — the World window: a read-only mesh render of the part kind's first
//! representation, built with the framework's `MeshWindowKit` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6). This file itself imports
//! nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No
//! selection, no gumball, no engagement: a viewer has no utilities that edit and emits no mutations
//! by construction (`ViewEmit`).

use crate::artifacts::block5d::Block5dSnapshot;
// 🚧️ SDK GAP: the seven framework `WindowKit`s (contract §2.6, `MeshWindowKit` included) live in the
// `//#region 🔖️WindowKits` region nested inside `pub mod app { … }` but are NOT in the crate-root
// curated `pub use app::{ … };` re-export list (unlike `ArtifactViewer`/`Viewer`/`ViewEmit`, whose
// Gap 1 already closed) — only reachable through `app::`. Flagged in this packet's migration report.
use semio_framework_plugin::app::{MeshView, MeshWindowKit, WindowKit};
use semio_framework_plugin::{world3d_camera_projection_json, world3d_meshes_json_from_kinds, world3d_meshes_json_from_urls, world3d_selection_json, UiNode, WindowKindDefinition, WorldProjectionConfig};

//#region 🔖️Constants
/// 👁️ `MeshWindowKit::KIND_ID` — frozen id/body-key pair (contract §2.6): `"framework.window.mesh"`.
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
/// 👁️ Matches the editor's fallback mesh kind when a representation carries no `mesh_url` — same
/// literal duplicated on purpose rather than imported through the sibling editor module, which
/// `policyViewerPurityBreaches` forbids outright.
const BLOCK5D_VIEW_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::block5d::create_block5d_viewer`. Read-only
/// (`window_kind()`, not `editable_window_kind()`) — a viewer never emits the `set-vertex` mutation.
pub async fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `Block5dSnapshot -> UiNode` read: default camera (a viewer has no persisted per-session
/// camera — `Config = NoConfig`), no selection/gumball overlay. Real per-representation mesh urls
/// render as real meshes (`world3d_meshes_json_from_urls`); a part kind with no representation mesh
/// url yet falls back to the same placeholder box the editor's own world window implicitly assumes
/// when describing "mesh: —" (documented simplification, not a regression).
pub async fn render(document: &Block5dSnapshot) -> UiNode {
    let camera_json = world3d_camera_projection_json([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], None, 1.0, &WorldProjectionConfig::default());
    let urls: Vec<String> = document.representations.iter().filter_map(|representation| representation.mesh_url.clone()).collect();
    let meshes_json = if urls.is_empty() { world3d_meshes_json_from_kinds(&[BLOCK5D_VIEW_FALLBACK_MESH_KIND.to_string()]) } else { world3d_meshes_json_from_urls(&urls) };
    let instances_json = "[]".to_string();
    let selection_json = world3d_selection_json("rectangle", &[], None);
    MeshWindowKit::render(&MeshView { camera_json, meshes_json, instances_json, selection_json })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_frozen_mesh_window_kind() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::block5d::schema::empty_block5d_snapshot();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
