mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn new_builds_clean() {
        let snapshot = Ifc2x3Cv20BuilderConstruction::new().add_product(2, "IFCWALL", "Wall 1").build().expect("conforming construction must build");
        assert_eq!(snapshot.document.instances.len(), 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let violating = Part21Instance { id: 99, entities: vec![("IFCSTRUCTURALANALYSISMODEL".into(), vec![])] };
        let snapshot = Ifc2x3Cv20BuilderConstruction::new().build().unwrap();
        let (mutated, _diff) = Ifc2x3Cv20BuilderConstruction::from_snapshot(snapshot).mutate(Ifc2x3Mutation::UpsertInstance(crate::standards::v2x3::subsets::base::schema::mutations::upsert_instance::UpsertInstance { instance: violating, index: None }));
        let err = mutated.build().expect_err("a structural entity must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v2x3::subsets::cv20::io::CODE_STRUCTURAL_ENTITY));
    }
}
