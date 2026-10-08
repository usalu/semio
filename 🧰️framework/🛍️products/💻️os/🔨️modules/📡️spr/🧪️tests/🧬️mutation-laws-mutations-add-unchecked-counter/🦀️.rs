//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddUncheckedCounter>(2, CounterMutation::AddUncheckedCounter, include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🔣️.json"), include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/🐛️add-unchecked-counter/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}

/// ➕️ The deliberately incorrect command re-issues itself as its own inverse: the sum law catches that it does not restore.
#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must restore base")]
async fn the_sum_law_catches_an_inverse_that_does_not_restore() {
    crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&CounterMutation::AddUncheckedCounter(AddUncheckedCounter {}), &10i64).await;
}
