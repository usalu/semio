//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddCounter>(0, CounterMutation::AddCounter, include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json"), include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/➕️add-counter/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}

/// ➕️ The lawful counter addition: its inverse diffs sum to the negative of the forward diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for delta in [5, -7, 0] {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&CounterMutation::AddCounter(AddCounter { delta }), &10i64).await;
    }
}
