//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_fixture_contract() {
    super::super::super::tests::assert_leaf::<AddObservedCounter>(3, CounterMutation::AddObservedCounter, include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🔣️.json"), include_str!("../../🧪️testing/🧬️mutation-laws/🧬️mutations/👁️add-observed-counter/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}

/// ➕️ The deliberately nondeterministic command changes state once observed but returns no inverse: the sum law catches it.
#[semio_framework_async_macros::async_test]
#[should_panic(expected = "must not be empty")]
async fn the_sum_law_catches_an_empty_inverse_for_a_state_change() {
    let observed = AddObservedCounter::default();
    let _ = crate::os_spr::MutationKind::<i64, CounterMutation>::diff(&observed, &10i64);
    crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&CounterMutation::AddObservedCounter(observed), &10i64).await;
}
