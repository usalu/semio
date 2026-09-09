//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddRejectedCounter>(4, CounterMutation::AddRejectedCounter, include_str!("../../🧩️support/🧬️mutation-laws/🧬️mutations/⛔️add-rejected-counter/🔣️.json"));
}
