use crate::standards::v_r12::subsets::any::schema::snapshot::DxfSnapshot;
use semio_framework_plugin::app::ImageWindowKit;
use semio_framework_plugin::WindowKit;
use crate::viewer::dxf::modes::view::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_image_window_kit() {
    assert_eq!(definition().id, ImageWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_node_for_the_default_document() {
    assert!(render(&DxfSnapshot::default()).is_ok());
}
