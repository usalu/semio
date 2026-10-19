use super::*;

/// ➕️ L3: the concrete inverse's diffs sum to the negative of the forward diff, on a middle value and on the type's edges.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for (value, delta) in [(0, 5), (7, -3), (0, i32::MIN), (i32::MAX, i32::MIN)] {
        semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&WireTestMutation::AddValue(AddValue { delta }), &WireTestSnapshot { value }).await;
    }
}
