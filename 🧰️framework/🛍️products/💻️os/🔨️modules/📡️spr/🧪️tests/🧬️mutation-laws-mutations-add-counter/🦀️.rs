//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddCounter>(0, CounterMutation::AddCounter, include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json"), include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/➕️add-counter/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}
