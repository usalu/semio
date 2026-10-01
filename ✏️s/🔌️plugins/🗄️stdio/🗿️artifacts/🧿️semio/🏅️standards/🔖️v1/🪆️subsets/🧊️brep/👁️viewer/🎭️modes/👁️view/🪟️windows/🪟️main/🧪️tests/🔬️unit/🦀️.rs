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
    let mut rec = semio_framework_3d::brep::representation::topology::history::OpRecorder::new();
    semio_framework_3d::brep::operations::primitives::make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let document = crate::standards::v1::subsets::brep::schema::snapshot::body::snapshot_from_body(&body);
    let mesh = document_mesh_data(&document);
    assert!(!mesh.positions.is_empty());
    assert!(!mesh.indices.is_empty());
    assert!(!mesh.face_ids.is_empty());
    let _node = render(&document).expect("render succeeds for a real solid");
}
