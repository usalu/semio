mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn pass_through_build_never_fails_on_conformance_grounds() {
        let snapshot = TiffBaselineBuilderConstruction::empty().build().expect("all conformance findings are soft by policy; build must succeed");
        assert!(snapshot.ifds.is_empty());
    }
}
