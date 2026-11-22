use super::*;

#[semio_framework_async_macros::async_test]
async fn create_xlsx_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_xlsx_viewer();
    assert_eq!(def.role, semio_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, XLSX_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<XlsxViewer as ArtifactViewer>::DIALECT, XLSX_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_main_window() {
    let def = create_xlsx_viewer();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn viewer_displays_numbers_with_the_locale_decimal_separator_and_keeps_canonical_values() {
    let number = XlsxCellValue::Number(1.5);
    assert_eq!(render_xlsx_cell_value(&number, &[], semio_ui_locale::Locale::En), "1.5");
    assert_eq!(render_xlsx_cell_value(&number, &[], semio_ui_locale::Locale::De), "1,5");
    let cached = XlsxCellValue::Formula { expr: "A1*3".into(), cached: Some(Box::new(number)) };
    assert_eq!(render_xlsx_cell_value(&cached, &[], semio_ui_locale::Locale::De), "=A1*3 (1,5)");
}

#[semio_framework_async_macros::async_test]
async fn viewer_labels_resolve_in_both_languages_from_the_catalog() {
    assert_eq!(xlsx_viewer_labels(semio_ui_locale::Locale::En).row.as_str(), "Row");
    assert_eq!(xlsx_viewer_labels(semio_ui_locale::Locale::De).column.as_str(), "Spalte");
    assert_eq!(xlsx_viewer_labels(semio_ui_locale::Locale::De).unknown_body.fill(&[("body", "x")]).as_str(), "Unbekannter Bereich: x");
}

#[semio_framework_async_macros::async_test]
async fn view_layout_titles_come_from_the_localized_window_kind_label() {
    let semio_plugin::WindowLayoutNode::Stack { children, .. } = view::layout().root else { panic!("expected a stack layout") };
    assert!(children.iter().all(|window| matches!(window, semio_plugin::WindowLayoutNode::Window { title: None, .. })));
}
