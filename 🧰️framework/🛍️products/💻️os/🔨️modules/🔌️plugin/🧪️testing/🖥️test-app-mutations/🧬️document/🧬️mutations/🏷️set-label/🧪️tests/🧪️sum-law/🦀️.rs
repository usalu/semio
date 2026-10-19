use super::*;

/// ➕️ L3: setting a new label, the unchanged label (a no-op warning) and the empty label sums to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = TestSnapshot { count: 3, label: "before".into(), slot: Vec::new() };
    for value in ["after", "before", ""] {
        semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&TestMutation::from(SetLabel { value: value.into() }), &base).await;
    }
}
