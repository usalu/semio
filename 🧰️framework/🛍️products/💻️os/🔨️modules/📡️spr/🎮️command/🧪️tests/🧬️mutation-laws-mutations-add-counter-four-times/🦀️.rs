//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounterFourTimes>(include_str!("../../🧫️fixtures/🧬️mutation-laws/🧬️mutations/4️⃣add-counter-four-times/🔣️.json"));
}
#[test]
fn plan_nests_two_twice_plans() {
    let base = 0;
    let mut planner = Planner::new(&base);
    AddCounterFourTimes { delta: 2 }.plan(&base, &mut planner).unwrap();
    assert_eq!(planner.steps().len(), 4);
}
