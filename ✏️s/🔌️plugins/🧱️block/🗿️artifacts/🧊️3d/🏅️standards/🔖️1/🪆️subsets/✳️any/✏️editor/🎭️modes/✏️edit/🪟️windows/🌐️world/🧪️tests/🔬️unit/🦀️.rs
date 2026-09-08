
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BLOCK3D_BODY_WORLD);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
