use semio_ui_contract::BuiltNode;
#[allow(unused_imports)]
use crate::editor::page;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use semio_ui_locale::Locale;
#[allow(unused_imports)]
use semio_plugin::TreeWindows;
#[allow(unused_imports)]
use semio_plugin::WindowKindDefinition;
use crate::editor::pdf17e::modes::edit::windows::main::*;
use crate::standards::v1_7::subsets::base::io::export::demo_pdf17_snapshot;

#[test]
fn definition_declares_the_page_canvas() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "insert-text"));
    assert!(def.actions.iter().any(|action| action.id == "insert-rectangle"));
    assert!(def.actions.iter().any(|action| action.id == "insert-image"));
}

#[test]
fn render_paints_the_page_text_on_the_canvas() {
    let document = demo_pdf17_snapshot();
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let node = render(&document, semio_ui_locale::Locale::En).expect("page canvas");
    let projection = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).unwrap();
    let scene: semio_plugin::Canvas2dScene = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&projection).unwrap();
    assert!(scene.layers_json.contains("\"id\":\"p0:t0\"") && scene.layers_json.contains("\"kind\":\"cubic\""), "{}", scene.layers_json);
}
