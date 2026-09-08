mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn new_builds_clean() {
        let snapshot = Ifc2x3CobieBuilderConstruction::new().add_space("Room 101").build().expect("conforming construction must build");
        assert_eq!(snapshot.document.instances.len(), 5);
    }

    #[semio_framework_async_macros::async_test]
    async fn wrong_schema_via_raw_mutate_still_fails_build() {
        let snapshot = Ifc2x3CobieBuilderConstruction::new().build().unwrap();
        let mut bad = snapshot.clone();
        bad.document.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("IFC4".into())])];
        let (mutated, _diff) = Ifc2x3CobieBuilderConstruction::from_snapshot(Ifc2x3Snapshot::default()).mutate(Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(bad) }));
        let err = mutated.build().expect_err("a non-IFC2X3 FILE_SCHEMA must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v2x3::subsets::cobie::schema::CODE_FILE_SCHEMA));
    }
}
