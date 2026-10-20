use semio_framework_test_fixtures::fixture_text;

#[allow(unused_imports)]
use crate::editor::note::terminology::NotePlayLabels;
#[allow(unused_imports)]
use crate::editor::note::support::ui_label;
#[allow(unused_imports)]
use crate::schema::flatten_blocks;
#[allow(unused_imports)]
use semio_s_artifact_note_note_schema::NoteSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::tree_item_desc;
#[allow(unused_imports)]
use semio_framework_plugin::ui_node_list;
#[allow(unused_imports)]
use semio_framework_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::PanelGroup;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabKind;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTreeBuilder;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
use crate::editor::note::panels::inspection::*;
#[test]
fn note_semantic_panels_match_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️summary.json")).expect("neutral UI vectors");
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::empty_note_snapshot();
    snapshot.snap_enabled = Some(false);
    for row in fixture["cases"].as_array().expect("locale cases") {
        let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::from_language_tag(row["locale"].as_str().expect("locale")).expect("declared fixture locale"), ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag(row["locale"].as_str().expect("locale")).expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native) };
        let labels = crate::editor::note::terminology::note_play_labels(&view_state);
        let inspector = render(&snapshot, fixture["utility"].as_str().expect("utility"), labels).expect("inspector");
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(inspector)).expect("project and retire inspector");
        let actual: serde_json::Value = serde_json::from_str(&projection).expect("independent JSON oracle");
        let section = &actual["children"][0];
        assert_eq!(section["component"]["label"], row["heading"]);
        let lines: Vec<_> = section["children"]
            .as_array()
            .expect("summary")
            .iter()
            .map(|child| {
                let label = child["component"]["label"].as_str().expect("summary row label");
                let description = child["component"]["description"].as_str().expect("summary row description");
                serde_json::Value::String(format!("{label}: {description}"))
            })
            .collect();
        assert_eq!(lines, *row["lines"].as_array().expect("summary lines"));
        let catalogue = crate::editor::note::panels::catalogue::render(labels).expect("catalogue");
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(catalogue)).expect("project and retire catalogue");
        let actual: serde_json::Value = serde_json::from_str(&projection).expect("catalogue JSON oracle");
        assert_eq!(actual["children"][0]["component"]["label"], row["catalogue"]);
        assert_eq!(actual["children"][0]["children"][0]["component"]["value"], row["firstKind"]);
    }
}

#[test]
fn note_ink_canvas_payload_matches_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️summary.json")).expect("neutral canvas vectors");
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::empty_note_snapshot();
    let camera = serde_json::from_value(fixture["camera"].clone()).expect("camera oracle");
    for mode in ["composite", "navigator"] {
        let node = crate::editor::note::modes::edit::windows::composite::render_canvas_scene(&snapshot, &camera, fixture["utility"].as_str().expect("utility"), "note.fixture.canvas", mode).expect("canvas");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("semantic canvas surface") };
        let scene: semio_framework_ui_scene::InkCanvasScene = semio_framework_ui_scene::decode(props).expect("decode actual packed scene");
        let actual: serde_json::Value = serde_json::from_str(&scene.document_json).expect("independent canvas JSON oracle");
        assert_eq!(actual["schema"], fixture["canvasSchema"], "the canvas speaks the framework ink-canvas wire, not note's own document schema");
        assert_eq!(actual["camera"], fixture["camera"]);
        assert_eq!(actual["blocks"], serde_json::json!([]));
        assert_eq!(scene.active_utility, fixture["utility"].as_str().expect("utility"));
        assert_eq!(scene.view_mode, mode);
        assert_eq!(scene.interactive, mode == "composite");
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire canvas");
    }
}
