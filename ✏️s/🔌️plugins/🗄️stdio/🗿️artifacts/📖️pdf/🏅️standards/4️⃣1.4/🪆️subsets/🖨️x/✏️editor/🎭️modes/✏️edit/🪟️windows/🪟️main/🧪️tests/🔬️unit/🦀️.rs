use crate::editor::pdf14::page;
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_ui_locale::Locale;
use semio_framework_plugin::TreeWindows;
use semio_framework_ui_contract::UiPublicationRevision;
#[allow(unused_imports)]
use semio_framework_plugin::{WindowKindDefinition, UiAssemblyResult};
#[allow(unused_imports)]
use semio_framework_ui_contract::BuiltNode;
use crate::editor::pdf14x::modes::edit::windows::main::*;
static CORPUS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧱️base/✏️editor/🧪️tests/🧫️fixtures/📄️resolved-page-domain/🔣️.json"));
fn fixture() -> crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot {
    use crate::standards::v1_4::subsets::base::schema::snapshot::{PdfSnapshot, PageDoc};
    let fixture: serde_json::Value = serde_json::from_str((*CORPUS)).unwrap();
    PdfSnapshot { schema: fixture["schema"].as_str().unwrap().into(), pages: fixture["pages"].as_array().unwrap().iter().map(|page| PageDoc { width: page["width"].as_f64().unwrap(), height: page["height"].as_f64().unwrap(), text: page["text"].as_str().unwrap().into() }).collect() }
}

#[test]
fn definition_declares_the_own_resolved_page_actions() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert_eq!(definition.body_key, BODY_KEY);
    let mut actual = definition.actions.iter().map(|action| action.id.as_str()).collect::<Vec<_>>();
    actual.sort();
    let mut expected = vec!["set-page", "insert-page", "remove-page", "move-page", "set-page-size", "replace-page-text"];
    expected.sort();
    assert_eq!(actual, expected);
}
#[test]
fn render_keeps_each_own_page_draft_and_dimensions() {
    let document = fixture();
    let view = page::editable_view(&document, UiPublicationRevision(55)).unwrap();
    assert_eq!(view.pages[0].text, document.pages[0].text);
    assert_eq!(view.pages[1].text, document.pages[1].text);
    let node = render_windowed(&document, UiPublicationRevision(55), &TreeWindows::unhosted(), Locale::En).unwrap();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    assert!(projection.contains("page-0-item-0"));
    assert!(projection.contains("page-1-item-0"));
    assert!(projection.contains("page-dimensions"));
    let state=semio_framework_plugin::ViewModel{tree_windows:vec![
        semio_framework_plugin::TreeWindowRequest{body_key:BODY_KEY.into(),node_key:WINDOW_KIND_ID.into(),open:Some(true),offset:1,rows:1},
        semio_framework_plugin::TreeWindowRequest{body_key:BODY_KEY.into(),node_key:format!("{}{}page-1-item-0",WINDOW_KIND_ID,semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR),open:Some(true),offset:0,rows:1},
    ],..semio_framework_plugin::ViewModel::new(Locale::De,semio_framework_ui_locale::Terminology::Native)};
    let node=render_windowed(&document,UiPublicationRevision(55),&TreeWindows::for_body(&state,BODY_KEY),state.locale).unwrap();
    let projection=semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    let scene:semio_framework_ui_scene::TextEditorScene=semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&projection).unwrap();
    assert_eq!(scene.buffer,document.pages[1].text);
    let settings:serde_json::Value=serde_json::from_str(scene.settings_json.as_deref().unwrap()).unwrap();
    assert_eq!(settings["readOnly"],false);assert_eq!(settings["publicationRevision"],"55");assert_eq!(settings["applyLabel"],"Anwenden");assert_eq!(settings["editArguments"]["page"],1);assert_eq!(settings["editArguments"]["item"],0);assert_eq!(settings["editArguments"]["revision"],semio_framework_plugin::app::DocumentWindowKit::text_revision(&document.pages[1].text));
}
