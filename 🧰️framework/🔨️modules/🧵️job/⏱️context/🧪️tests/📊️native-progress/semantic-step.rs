use semio_framework_job_context::{Generation, OperationId, StepBudget, StepContext, root_cancel_token};

#[test]
fn the_semantic_step_limit_is_scoped_separately_from_physical_work() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/semantic-step.json")).unwrap();
    let number = |field| fixture[field].as_u64().unwrap();
    let (mut sequence, mut progress) = (0, Default::default());
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(number("physicalFuel"), 10, Default::default()), root_cancel_token(), || Some(0), &mut sequence, &mut progress);

    cx.with_original_work_loan_for_step(number("stepFuel"), 10, number("completed"), |cx| {
        assert_eq!(cx.fuel_remaining(), number("stepFuel"));
        assert_eq!(cx.semantic_step_limit(), Some(number("limit")));
        cx.with_original_work_loan(number("nestedFuel"), 10, |cx| {
            assert_eq!(cx.fuel_remaining(), number("nestedFuel"));
            assert_eq!(cx.semantic_step_limit(), Some(number("limit")));
            cx.consume_fuel(number("physicalSpend"));
            Ok(())
        })?;
        cx.with_original_work_loan_for_step(number("stepFuel"), 10, number("laterCompleted"), |cx| {
            assert_eq!(cx.semantic_step_limit(), Some(number("limit")));
            Ok(())
        })?;
        cx.with_original_work_loan_for_step(number("stepFuel"), 10, number("earlierCompleted"), |cx| {
            assert_eq!(cx.semantic_step_limit(), Some(number("narrowerLimit")));
            Ok(())
        })?;
        assert_eq!(cx.semantic_step_limit(), Some(number("limit")));
        Ok(())
    }).unwrap();

    assert_eq!(cx.semantic_step_limit(), None);
    assert_eq!(cx.fuel_remaining(), number("physicalFuelAfter"));
}
