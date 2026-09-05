//! 🧊 Semio Brep viewer — the Main window: a read-only world-3d mesh scene built with the shared
//! `MeshWindowKit` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6), from the
//! artifact-level `SemioBrepSnapshot` — this file imports nothing from the sibling editor module
//! (a substring check on that module path is what `policyViewerPurityBreaches` forbids).
//!
//! 🔓 Landed in ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave W3-A: renders the REAL
//! tessellated geometry via `SemioBrepSnapshot -> Body -> tessellate_document` (`📸️snapshot/
//! 🔁️body/🦀️.rs` + `💡️inferences/🦀️.rs`, this same wave), one merged mesh + one instance for the
//! whole document (see `tessellate_document`'s own doc comment for why the inference's single-
//! `MeshTransfer` shape makes one merged mesh the honest choice over per-solid instancing).
//! Triangle picking resolves to a real face/solid via `face_groups`' persistent-label `entityId`s
//! (`semio_framework_plugin::mesh_from_indexed_with_face_groups`) — no more placeholder box.
//! `SEMIO_BREP_VIEW_FALLBACK_MESH_KIND` is gone: an empty/invalid document now renders an empty
//! mesh (zero triangles), not a fabricated stand-in shape.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::{tessellate_document, BREP_INFERENCE_DEFAULT_DEFLECTION};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_framework_plugin::{mesh_from_indexed_with_face_groups, world3d_camera_json, world3d_selection_json, BuiltNode, MeshData, MeshView, MeshWindowKit, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
const SEMIO_BREP_VIEW_MESH_ID: &str = "brep-document";
const SEMIO_BREP_VIEW_DEFAULT_CAMERA_POSITION: [f64; 3] = [8.0, -8.0, 6.0];
const SEMIO_BREP_VIEW_DEFAULT_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const SEMIO_BREP_VIEW_DEFAULT_CAMERA_FOV: f64 = 45.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by the surface root's `create_*_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Real `SemioBrepSnapshot -> MeshData`: builds a native `Body` (lossless, `📸️snapshot/
/// 🔁️body/🦀️.rs`), tessellates every solid, merges into one indexed triangle mesh with
/// persistent-label `face_groups` for picking, and carries the tessellated edge polylines through
/// as a wireframe overlay (`MeshData::edge_positions`). An empty/unparseable document (no solids,
/// or a `Body::from_snapshot` failure) yields an EMPTY mesh — never a fabricated placeholder.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn document_mesh_data(document: &SemioBrepSnapshot) -> MeshData {
    let Ok(body) = Body::from_snapshot(document) else { return MeshData::default() };
    let mesh = tessellate_document(&body, BREP_INFERENCE_DEFAULT_DEFLECTION);
    let face_groups: Vec<(u32, u32, u32)> = mesh.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
    let mut data = mesh_from_indexed_with_face_groups(&mesh.position, &mesh.normal, &mesh.index, &face_groups);
    data.edge_positions = mesh.edges;
    data
}

/// 👁️ One instance for the whole document, identity-placed (every solid's own geometry already
/// carries its absolute world position — see `document_mesh_data`'s merge).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn world_instances_json() -> String {
    pack::json_to_string(&pack::JsonValue::Array(vec![pack::json!({
        "id": "brep-document-instance",
        "meshId": SEMIO_BREP_VIEW_MESH_ID,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "label": "Semio Brep",
        "smoothShading": true,
    })]))
}

/// 👁️ Pure `SemioBrepSnapshot -> BuiltNode` read: default camera (a viewer has no persisted
/// per-session camera — `Config = NoConfig`), no selection/gumball/engagement overlay.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &SemioBrepSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mesh_data = document_mesh_data(document);
    let meshes_json = pack::json_to_string(&pack::JsonValue::Array(vec![pack::json!({ "id": SEMIO_BREP_VIEW_MESH_ID, "data": pack::JsonValue::from(mesh_data) })]));
    let view = MeshView {
        camera_json: world3d_camera_json(SEMIO_BREP_VIEW_DEFAULT_CAMERA_POSITION, SEMIO_BREP_VIEW_DEFAULT_CAMERA_TARGET, SEMIO_BREP_VIEW_DEFAULT_CAMERA_FOV),
        meshes_json,
        instances_json: world_instances_json(),
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
        let document = SemioBrepSnapshot::default();
        let _node = render(&document);
    }

    /// 👁️ The empty document (no solids) must render zero triangles, not a fabricated box —
    /// proves `SEMIO_BREP_VIEW_FALLBACK_MESH_KIND`'s removal actually took effect.
    #[semio_framework_async_macros::async_test]
    async fn empty_document_renders_an_empty_mesh() {
        let mesh = document_mesh_data(&SemioBrepSnapshot::default());
        assert!(mesh.positions.is_empty());
        assert!(mesh.indices.is_empty());
    }

    /// 👁️ A real solid (box, via the demo fixture's own round trip through `Body`) tessellates to
    /// an actual non-empty triangle mesh with real face-group entity ids, and `render` succeeds.
    #[semio_framework_async_macros::async_test]
    async fn a_real_box_solid_renders_a_non_empty_mesh() {
        let mut body = Body::new();
        let mut rec = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder::new();
        crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let document = body.to_snapshot();
        let mesh = document_mesh_data(&document);
        assert!(!mesh.positions.is_empty());
        assert!(!mesh.indices.is_empty());
        assert!(!mesh.face_ids.is_empty());
        let _node = render(&document).expect("render succeeds for a real solid");
    }
}
//#endregion 🧪️Tests
