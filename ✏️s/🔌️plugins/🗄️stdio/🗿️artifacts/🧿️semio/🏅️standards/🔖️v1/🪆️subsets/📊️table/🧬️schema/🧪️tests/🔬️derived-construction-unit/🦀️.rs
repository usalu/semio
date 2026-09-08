mod tests {
    use super::*;
    use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_a_populated_snapshot() {
        let snapshot = SemioTableBuilderConstruction::new().add_column("label", SemioTableCellKind::Str).add_row(SemioTableRow { cells: vec![SemioValue::Str { value: "hello".into() }] }).build().expect("build");
        assert_eq!(snapshot.columns.len(), 1);
        assert_eq!(snapshot.rows[0].cells.len(), 1);
    }
}
