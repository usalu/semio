
use super::*;
#[test]
fn note_semantic_panels_match_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️summary.json")).expect("neutral UI vectors");
    let mut snapshot = crate::schema::empty_note_snapshot();
    snapshot.snap_enabled = Some(false);
    for row in fixture["cases"].as_array().expect("locale cases") {
        let config = crate::editor::note::config::NoteConfig { locale: row["locale"].as_str().expect("locale").into(), ..Default::default() };
        let labels = crate::editor::note::terminology::note_play_labels(&config);
        let inspector = render(&snapshot, fixture["utility"].as_str().expect("utility"), labels).expect("inspector");
        let projection = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(inspector)).expect("project and retire inspector");
        let actual: serde_json::Value = serde_json::from_str(&projection).expect("independent JSON oracle");
        assert_eq!(actual["component"]["label"], row["heading"]);
        let lines: Vec<_> = actual["children"].as_array().expect("summary").iter().map(|child| child["component"]["value"].clone()).collect();
        assert_eq!(lines, *row["lines"].as_array().expect("summary lines"));
        let catalogue = crate::editor::note::panels::catalogue::render(labels).expect("catalogue");
        let projection = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(catalogue)).expect("project and retire catalogue");
        let actual: serde_json::Value = serde_json::from_str(&projection).expect("catalogue JSON oracle");
        assert_eq!(actual["children"][0]["component"]["label"], row["catalogue"]);
        assert_eq!(actual["children"][0]["children"][0]["component"]["value"], row["firstKind"]);
    }
}

#[test]
fn note_ink_canvas_payload_matches_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️summary.json")).expect("neutral canvas vectors");
    let snapshot = crate::schema::empty_note_snapshot();
    let camera = serde_json::from_value(fixture["camera"].clone()).expect("camera oracle");
    for mode in ["composite", "navigator"] {
        let node = crate::editor::note::modes::edit::windows::composite::render_canvas_scene(&snapshot, &camera, fixture["utility"].as_str().expect("utility"), "note.fixture.canvas", mode).expect("canvas");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("semantic canvas surface") };
        let scene: semio_framework_ui_scene::InkCanvasScene = semio_framework_ui_scene::decode(props).expect("decode actual packed scene");
        let actual: serde_json::Value = serde_json::from_str(&scene.document_json).expect("independent canvas JSON oracle");
        assert_eq!(actual["schema"], fixture["schema"]);
        assert_eq!(actual["camera"], fixture["camera"]);
        assert_eq!(actual["blocks"], serde_json::json!([]));
        assert_eq!(scene.active_utility, fixture["utility"].as_str().expect("utility"));
        assert_eq!(scene.view_mode, mode);
        assert_eq!(scene.interactive, mode == "composite");
        semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire canvas");
    }
}
