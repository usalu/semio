use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::JpgSnapshot;
use crate::editor::jpg_any::modes::edit::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = JpgSnapshot::default();
    let _node = render(&document, semio_framework_ui_locale::Locale::En).expect("window with the re-encode notice");
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_region_painting_as_a_mutation_action() {
    let def = definition();
    let paint = def.actions.iter().find(|action| action.id == "paint-region").expect("region painting is a window action");
    assert_eq!(paint.kind, semio_framework_plugin::ActionKind::Mutation);
}

#[semio_framework_async_macros::async_test]
async fn re_encode_notice_is_localized() {
    for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
        render(&JpgSnapshot::default(), locale).expect("notice renders in every locale");
    }
}
