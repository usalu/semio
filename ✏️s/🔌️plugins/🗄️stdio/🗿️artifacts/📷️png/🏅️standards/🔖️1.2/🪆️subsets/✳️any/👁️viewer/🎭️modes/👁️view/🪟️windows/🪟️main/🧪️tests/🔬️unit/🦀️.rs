use crate::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;
use semio_framework_ui_locale::Locale;
use crate::viewer::png::modes::view::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = PngSnapshot::default();
    let _node = render(&document, semio_framework_ui_locale::Locale::En);
}
