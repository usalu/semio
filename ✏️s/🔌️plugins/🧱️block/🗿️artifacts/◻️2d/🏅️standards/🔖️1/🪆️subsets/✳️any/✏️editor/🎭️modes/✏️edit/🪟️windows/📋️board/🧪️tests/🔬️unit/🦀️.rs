
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_board_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BLOCK2D_BODY_BOARD);
    assert!(matches!(definition.surface_kind, SurfaceKind::Board2d));
}
