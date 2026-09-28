use super::*;
use semio_framework_plugin::Component;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_structurally_editable_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    for action in ["set-cell", "add-row", "remove-row", "add-column", "remove-column", "set-header"] {
        assert!(def.actions.iter().any(|candidate| candidate.id == action), "missing {action}");
    }
}

#[semio_framework_async_macros::async_test]
async fn render_splits_header_from_windowed_data_rows_and_binds_structure() {
    let document = CsvSnapshot {
        schema: "stdio.csv".into(),
        has_header: true,
        records: vec![crate::CsvRecord { fields: vec![crate::CsvField { value: "name".into(), quoted: false }] }, crate::CsvRecord { fields: vec![crate::CsvField { value: "ada".into(), quoted: false }] }],
    };
    let node = render_revisioned(&document, "store-revision", semio_framework_plugin::Locale::En, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    assert!(matches!(node.component, Component::Container(_)));
    let json = serde_json::to_string(&node).expect("declarative table json");
    for witness in ["name", "ada", "store-revision", "set-cell", "set-header", "add-row", "add-column", "remove-row", "remove-column"] {
        assert!(json.contains(witness), "missing {witness} in {json}");
    }
}
