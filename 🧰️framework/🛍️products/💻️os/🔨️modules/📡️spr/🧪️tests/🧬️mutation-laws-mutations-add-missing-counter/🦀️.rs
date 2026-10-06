//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddMissingCounter>(1, CounterMutation::AddMissingCounter, include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🔣️.json"), include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}
