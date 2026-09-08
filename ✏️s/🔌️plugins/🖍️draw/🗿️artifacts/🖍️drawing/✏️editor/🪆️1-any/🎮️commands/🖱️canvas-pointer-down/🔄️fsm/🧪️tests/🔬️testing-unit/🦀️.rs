mod tests {
    use super::super::testing::support::{UnitToggleEvent, UnitToggleMachine};
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn explore_reaches_both_toggle_states() {
        let model = Model::<UnitToggleMachine>::new(vec![UnitToggleEvent::Flip]);
        let coverage = explore(&model, ());
        assert!(coverage.reached_stable_ids.contains(&"off"));
        assert!(coverage.reached_stable_ids.contains(&"on"));
        assert_eq!(coverage.visited_configurations, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn conformance_fixture_passes_for_matching_sequence() {
        let steps = [ConformanceStep { event: UnitToggleEvent::Flip, expect_active: &["on"] }, ConformanceStep { event: UnitToggleEvent::Flip, expect_active: &["off"] }];
        assert!(run_conformance::<UnitToggleMachine>((), &steps).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn conformance_fixture_fails_with_descriptive_message() {
        let steps = [ConformanceStep { event: UnitToggleEvent::Flip, expect_active: &["off"] }];
        let err = run_conformance::<UnitToggleMachine>((), &steps).unwrap_err().to_string();
        assert!(err.contains("step 0"));
        assert!(err.contains("off"));
    }

    #[semio_framework_async_macros::async_test]
    async fn invariant_reports_violation_by_name() {
        let mut sink: Vec<Command<UnitToggleMachine>> = Vec::new();
        let snapshot = init::<UnitToggleMachine>((), &mut sink);
        let invariants = [Invariant { name: "never off", check: |s: &Snapshot<UnitToggleMachine>| if s.matches("off") { Err(FsmError::Violation("was off".to_string())) } else { Ok(()) } }];
        let violations = check_invariants(&snapshot, &invariants);
        assert_eq!(violations, vec!["never off: was off".to_string()]);
    }
}
