//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounterTwice>(include_str!("../../🧫️fixtures/🧬️mutation-laws/🧬️mutations/✌️add-counter-twice/🔣️.json"));
}
#[test]
fn plan_has_two_local_adds() {
    let base = 0;
    let mut planner = Planner::new(&base);
    AddCounterTwice { delta: 3 }.plan(&base, &mut planner).unwrap();
    assert_eq!(planner.steps().len(), 2);
}
