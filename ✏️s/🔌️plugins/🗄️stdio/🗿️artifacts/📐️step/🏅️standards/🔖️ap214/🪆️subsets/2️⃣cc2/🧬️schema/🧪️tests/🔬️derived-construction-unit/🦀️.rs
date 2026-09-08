mod tests {
    use super::*;
    use crate::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    use crate::standards::v_ap214::subsets::cc2::schema::CODE_LADDER;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn conforming_snapshot() -> StepSnapshot {
        StepSnapshot::from_part21_document(&Part21Document {
            header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
            instances: vec![
                Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
            ],
        })
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_construction_builds() {
        let snapshot = StepCc2BuilderConstruction::from_snapshot(conforming_snapshot()).build().expect("conforming construction must build");
        assert!(crate::standards::v_ap214::engine::ladder::has_product_definition_chain(&snapshot.to_part21_document()));
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = conforming_snapshot();
        let mut doc = snapshot.to_part21_document();
        doc.instances.push(Part21Instance { id: 99, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![])] });
        snapshot = StepSnapshot::from_part21_document(&doc);
        let (mutated, _diff) = StepCc2BuilderConstruction::from_snapshot(StepSnapshot::default()).mutate(StepMutation::SetSnapshot(crate::standards::v_ap214::subsets::base::schema::mutations::set_snapshot::SetSnapshot { snapshot }));
        let err = mutated.build().expect_err("an ADVANCED_BREP_SHAPE_REPRESENTATION instance above rung 2 must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_LADDER));
    }
}
