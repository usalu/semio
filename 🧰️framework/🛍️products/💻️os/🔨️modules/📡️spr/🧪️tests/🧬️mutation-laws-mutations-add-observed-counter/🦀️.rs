//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddObservedCounter>(3, CounterMutation::AddObservedCounter, include_str!("../../🧫️fixtures/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🔣️.json"));
}
