//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddMissingCounter>(1, CounterMutation::AddMissingCounter, include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🔣️.json"), include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/🚫️add-missing-counter/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}

/// ➕️ A missing-target forward outcome is not an invertible mutation: the sum law refuses it.
#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must not have been rejected")]
async fn the_sum_law_refuses_a_missing_target_outcome() {
    crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&CounterMutation::AddMissingCounter(AddMissingCounter {}), &10i64).await;
}
