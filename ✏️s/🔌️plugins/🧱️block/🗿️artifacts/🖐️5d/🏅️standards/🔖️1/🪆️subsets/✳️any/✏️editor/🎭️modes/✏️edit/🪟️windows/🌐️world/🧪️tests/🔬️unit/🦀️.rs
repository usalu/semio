
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BLOCK5D_BODY_WORLD);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
}
