mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_a_populated_snapshot() {
        let snapshot = SemioKitBuilderConstruction::new().add_type("chair", "Chair", "furniture").add_type("table", "Table", "furniture").build().expect("build");
        assert_eq!(snapshot.types.len(), 2);
        assert_eq!(snapshot.types[0].id, "chair");
    }
}
