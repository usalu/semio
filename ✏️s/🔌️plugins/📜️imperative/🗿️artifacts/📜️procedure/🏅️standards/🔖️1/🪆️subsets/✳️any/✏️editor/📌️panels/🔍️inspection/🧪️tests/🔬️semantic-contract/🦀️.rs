use super::*;

fn project(node: semio_framework_plugin::BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
    serde_json::from_str(&text).expect("independent UI oracle")
}

#[test]
fn imperative_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️panels.json")).expect("neutral UI vectors");
    let document = ProcedureSnapshot::default();
    for row in vectors["cases"].as_array().expect("locales") {
        let labels = semio_framework_plugin::resolve_labels::<ImperativeLabels>(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::locale_from_str(row["locale"].as_str().expect("locale")), ..Default::default() });
        let tree = project(crate::editor::procedure::panels::document::render(&document, labels, &semio_framework_plugin::TreeWindows::unhosted()).expect("document"));
        assert_eq!(tree["children"][0]["component"]["label"], row["document"]);
        let catalogue = crate::editor::procedure::panels::catalogue::render(labels, &semio_framework_plugin::TreeWindows::unhosted()).expect("catalogue");
        let actions = catalogue.children[0].children.iter().map(|node| serde_json::to_value(&node.bindings[0].args).expect("independent action oracle")).collect::<Vec<_>>();
        let tree = project(catalogue);
        assert_eq!(tree["children"][0]["component"]["label"], row["catalogue"]);
        for (index, kind) in vectors["actionKinds"].as_array().expect("actions").iter().enumerate() {
            assert_eq!(actions[index]["kind"], *kind);
        }
        let tree = project(render(&document, labels).expect("inspector"));
        assert_eq!(tree["children"][0]["component"]["label"], row["inspection"]);
        assert_eq!(tree["children"][0]["children"][0]["component"]["label"], row["steps"]);
        let node = crate::editor::procedure::modes::edit::windows::main::render(&document, &vectors["runOutput"].to_string(), labels).expect("table");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("table surface") };
        let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(props).expect("packed table");
        // 📊️ `TableWindowKit::render` emits the renderer table contract both hosts read: `columnsJson`
        // as `{id, label}` records and `rowsJson` as `{id, "<column index>": cell}` records. The
        // committed oracle spells what the USER reads — the locale-resolved column labels and the cell
        // text in column order — so the records are projected back to that, rather than the oracle being
        // rewritten into the renderer's keying.
        let columns: Vec<serde_json::Value> = serde_json::from_str::<Vec<serde_json::Value>>(&scene.columns_json).expect("columns oracle").into_iter().map(|column| column["label"].clone()).collect();
        assert_eq!(serde_json::Value::Array(columns.clone()), row["columns"]);
        let rows: Vec<serde_json::Value> = serde_json::from_str::<Vec<serde_json::Value>>(&scene.rows_json)
            .expect("rows oracle")
            .into_iter()
            .map(|record| serde_json::Value::Array((0..columns.len()).map(|column| record[column.to_string().as_str()].clone()).collect()))
            .collect();
        assert_eq!(serde_json::Value::Array(rows), vectors["rows"]);
        project(node);
    }
    for node in [crate::editor::procedure::modes::edit::windows::script::render(&document).expect("editor text"), crate::viewer::procedure::modes::view::windows::script::render(&document).expect("viewer text")] {
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("text surface") };
        let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("packed text");
        assert_eq!(scene.language.as_deref(), Some("imperative"));
        assert_eq!(serde_json::from_str::<serde_json::Value>(scene.settings_json.as_deref().expect("settings")).expect("settings oracle"), vectors["settings"]);
        project(node);
    }
}
