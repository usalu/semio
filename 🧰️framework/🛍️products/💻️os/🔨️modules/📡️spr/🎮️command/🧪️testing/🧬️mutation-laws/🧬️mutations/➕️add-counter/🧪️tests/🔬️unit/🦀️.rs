//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounter>(include_str!("../../🔣️.json"));
}

/// ➕️ L3: a delta, a negative delta, the zero delta and the delta that needs two wide steps sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for (delta, base) in [(5, 10i64), (-7, 10), (0, 10), (i64::MIN, 10), (i64::MAX, -10)] {
        semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&CounterMutation::AddCounter(AddCounter { delta }), &base).await;
    }
}
