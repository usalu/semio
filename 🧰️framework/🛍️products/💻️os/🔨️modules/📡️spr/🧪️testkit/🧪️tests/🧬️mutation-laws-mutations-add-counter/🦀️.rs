//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddCounter>(0, CounterMutation::AddCounter, include_str!("../../🧩️support/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json"));
}
