use super::*;

/// ➕️ L3: setting a new count, the unchanged count and the integer edges sums to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = TestSnapshot { count: 3, label: "before".into(), slot: Vec::new() };
    for value in [9, 3, 0, i32::MIN, i32::MAX] {
        semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&TestMutation::from(SetCount { value }), &base).await;
    }
}
