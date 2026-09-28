use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_structurally_editable_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    for action in ["set-cell", "add-row", "remove-row", "add-column", "remove-column"] {
        assert!(def.actions.iter().any(|candidate| candidate.id == action), "missing {action}");
    }
    assert!(!def.actions.iter().any(|candidate| candidate.id == "set-header"));
}

#[semio_framework_async_macros::async_test]
async fn render_keeps_every_record_as_windowed_editable_data() {
    let document = TsvSnapshot { schema: "stdio.tsv".into(), records: vec![vec!["name".into(), "role".into()], vec!["ada".into(), "engineer".into()]], trailing_newline: false, line_ending: Default::default() };
    let node = render_revisioned(&document, "store-revision", semio_framework_plugin::Locale::De, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    assert!(matches!(node.component, Component::Container(_)));
    let json = serde_json::to_string(&node).expect("declarative table json");
    for witness in ["name", "role", "ada", "engineer", "Spalte 1", "Spalte 2", "store-revision", "set-cell", "Zeile hinzufügen", "Spalte hinzufügen", "Zeile entfernen"] {
        assert!(json.contains(witness), "missing {witness} in {json}");
    }
    assert!(!json.contains("set-header"));
}

#[semio_framework_async_macros::async_test]
async fn render_keeps_a_single_record_editable_instead_of_hiding_it_as_a_header() {
    let document = TsvSnapshot { schema: "stdio.tsv".into(), records: vec![vec!["only-row".into()]], trailing_newline: false, line_ending: Default::default() };
    let node = render_revisioned(&document, "store-revision", semio_framework_plugin::Locale::En, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    let json = serde_json::to_string(&node).expect("declarative table json");
    assert!(json.contains("only-row"));
    assert!(json.contains("Column 1"));
}
