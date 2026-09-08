mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn new_builds_clean() {
        let snapshot = Ifc2x3SavBuilderConstruction::new().add_load_group(2).build().expect("conforming construction must build");
        assert_eq!(snapshot.document.instances.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn removing_the_analysis_model_via_raw_mutate_still_fails_build() {
        let snapshot = Ifc2x3SavBuilderConstruction::new().build().unwrap();
        let (mutated, _diff) = Ifc2x3SavBuilderConstruction::from_snapshot(snapshot).mutate(Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 1 }));
        let err = mutated.build().expect_err("removing the only analysis model must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v2x3::subsets::sav::schema::CODE_NO_ANALYSIS_MODEL));
    }
}
