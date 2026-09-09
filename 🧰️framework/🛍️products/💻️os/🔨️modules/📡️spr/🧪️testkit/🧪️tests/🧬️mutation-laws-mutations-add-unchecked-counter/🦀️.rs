//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddUncheckedCounter>(2, CounterMutation::AddUncheckedCounter, include_str!("../../🧫️fixtures/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🔣️.json"));
}
