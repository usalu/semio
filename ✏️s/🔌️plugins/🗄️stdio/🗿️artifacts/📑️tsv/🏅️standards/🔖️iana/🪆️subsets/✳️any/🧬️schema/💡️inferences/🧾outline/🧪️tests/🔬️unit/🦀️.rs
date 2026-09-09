use super::*;

#[semio_framework_async_macros::async_test]
async fn reports_widest_record_as_column_count() {
    let snapshot = TsvSnapshot { schema: "stdio.tsv".into(), records: vec![vec!["a".into(), "b".into()], vec!["c".into()]], trailing_newline: false, line_ending: Default::default() };
    let outline = TsvOutline::compute(&snapshot);
    assert_eq!(outline.record_count, 2);
    assert_eq!(outline.column_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = TsvSnapshot::default();
    assert_eq!(TsvOutline::compute(&snapshot), TsvOutline::compute(&snapshot));
}
