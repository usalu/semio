use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_one_row_per_top_level_step() {
    let scene = crate::examples::demo::scene();
    let expected = scene.path.steps.len();
    let node = render(&scene, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("viewer table");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("table surface") };
    let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("packed table");
    let rows: serde_json::Value = serde_json::from_str(&scene.rows_json).expect("independent row oracle");
    assert_eq!(rows.as_array().expect("rows").len(), expected);
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire table");
}

#[semio_framework_async_macros::async_test]
async fn columns_resolve_from_the_shared_view_locale() {
    let scene = crate::examples::demo::scene();
    let columns = |locale| {
        let node = render(&scene, &semio_framework_plugin::ViewModel { locale, ..semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native) }).expect("viewer table");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("table surface") };
        let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("packed table");
        // 📊️ `TableWindowKit::render` emits `columnsJson` as `{id, label}` records (position-keyed ids
        // the row records reuse), not bare label strings — this law is about the LABELS resolving from
        // the shared view locale, so it reads them out of those records.
        let columns = serde_json::from_str::<Vec<serde_json::Value>>(&scene.columns_json).expect("columns").into_iter().map(|column| column["label"].as_str().expect("column label").to_string()).collect::<Vec<String>>();
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire table");
        columns
    };
    assert_eq!(columns(semio_framework_ui_locale::Locale::En), ["#", "Id", "Kind"]);
    assert_eq!(columns(semio_framework_ui_locale::Locale::De), ["#", "ID", "Art"]);
}
