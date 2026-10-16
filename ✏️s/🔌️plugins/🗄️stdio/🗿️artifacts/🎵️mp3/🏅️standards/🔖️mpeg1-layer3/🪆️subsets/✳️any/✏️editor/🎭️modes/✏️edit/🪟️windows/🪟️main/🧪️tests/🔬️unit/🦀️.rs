use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use semio_framework_ui_locale::Locale;
use crate::editor::mp3::modes::edit::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = Mp3Snapshot::default();
    let _node = render(&document, Locale::En, None);
}
