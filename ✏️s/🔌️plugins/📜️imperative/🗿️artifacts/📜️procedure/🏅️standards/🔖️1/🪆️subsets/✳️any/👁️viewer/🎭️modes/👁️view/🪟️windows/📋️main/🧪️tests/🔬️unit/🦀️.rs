
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_top_level_step() {
    let document = crate::schema::default_snapshot();
    let expected = crate::procedure_working_scene(&document).path.steps.len();
    let node = render(&document).expect("viewer table");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("table surface") };
    let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("packed table");
    let rows: serde_json::Value = serde_json::from_str(&scene.rows_json).expect("independent row oracle");
    assert_eq!(rows.as_array().expect("rows").len(), expected);
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire table");
    crate::retire_procedure_fixture(document);
}
