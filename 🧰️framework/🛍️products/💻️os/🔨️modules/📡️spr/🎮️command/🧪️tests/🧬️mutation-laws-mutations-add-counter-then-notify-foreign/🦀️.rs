//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
use semio_framework_test_fixtures::fixture_path;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounterThenNotifyForeign>(std::fs::read_to_string(fixture_path!("../../../../🧪️testing/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify/🔣️.json")).expect("fixture").as_str());
}
#[test]
fn plan_keeps_local_add_before_foreign_steps() {
    let base = 0;
    let mut planner = Planner::new(&base);
    AddCounterThenNotifyForeign { delta: 2, foreign_count: 2 }.plan(&base, &mut planner).unwrap();
    assert_eq!(planner.steps().len(), 3);
}
