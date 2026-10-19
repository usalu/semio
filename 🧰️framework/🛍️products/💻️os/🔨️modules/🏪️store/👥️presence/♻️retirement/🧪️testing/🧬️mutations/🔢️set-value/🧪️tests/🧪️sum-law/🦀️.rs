use super::*;

/// ➕️ L3: the concrete inverse's diffs sum to the negative of the forward diff, on a middle value and on the type's edges.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for n in [0, 9, i32::MIN] {
        semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&ValueMutation::SetValue(SetValue { n }), &Value(3)).await;
    }
}
