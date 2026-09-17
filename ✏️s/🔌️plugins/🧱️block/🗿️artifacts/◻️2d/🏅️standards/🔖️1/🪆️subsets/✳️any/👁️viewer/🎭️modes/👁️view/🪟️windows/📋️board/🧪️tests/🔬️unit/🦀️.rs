
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_board_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(matches!(definition.surface_kind, SurfaceKind::Board2d));
}

#[semio_framework_async_macros::async_test]
async fn render_lists_real_handle_kind_and_handle_geometry() {
    use crate::{Block2dHandleKind, Block2dHandleTemplate};
    let mut document = crate::standards::v1::subsets::any::schema::empty_block2d_snapshot();
    document.handle_kinds.push(Block2dHandleKind { id: "k1".into(), name: "k1".into(), label: "Cable".into(), color: "#ff0000".into(), default_wire_kind: "cable.link".into() });
    document.handles.push(Block2dHandleTemplate { id: "h1".into(), handle_kind: "k1".into(), angle: std::f64::consts::PI, radius: 0.5 });
    // 🚚️ Read through the retiring PROJECTION, never `serde_json::to_string` on a `BuiltNode`: a built
    // node's `BuiltChildren` only serialises through the retained page transport.
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&document).expect("board render"))).expect("board projection");
    assert!(json.contains("Cable"), "board render must surface real handle-kind geometry: {json}");
    assert!(json.contains("radius 0.50"), "board render must surface real handle-template geometry: {json}");
}
