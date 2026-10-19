use super::*;
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&ValueMutation::SetValue(SetValue { n: 9 }), &Value(3)).await;
}

#[test]
fn direct_fixture_leaf_contract() {
    super::super::assert_fixture_descriptor::<SetValue>(include_str!("../../🧪️testing/🧬️mutations/🔢️set-value/🔣️.json"));
}
