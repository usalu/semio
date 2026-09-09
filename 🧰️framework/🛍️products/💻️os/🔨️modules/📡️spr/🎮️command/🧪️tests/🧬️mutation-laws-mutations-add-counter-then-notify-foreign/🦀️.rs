//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounterThenNotifyForeign>(include_str!("../../🧪️testkit/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign/🔣️.json"));
}
#[test]
fn plan_keeps_local_add_before_foreign_steps() {
    let base = 0;
    let mut planner = Planner::new(&base);
    AddCounterThenNotifyForeign { delta: 2, foreign_count: 2 }.plan(&base, &mut planner).unwrap();
    assert_eq!(planner.steps().len(), 3);
}
