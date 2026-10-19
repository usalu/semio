//! 🧪️ Descriptor laws of the timestamped mutation fixtures: every leaf descriptor equals its committed JSON descriptor.
//!
//! 🧭️ Consolidated from one test file per leaf; each leaf keeps its own module so the test names are unchanged.

mod assign_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, timestamped::AssignN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<AssignN>(include_str!("../../🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/↩️assign-n/🔣️.json"));
    }

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    crate::os_store::component::tests::assert_fixture_sum_law(&TimestampedMutation::AssignN(AssignN { n: Some(5), physical_ms: 7 }), &[Some(3), None]).await;
}
}

mod set_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, timestamped::SetN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetN>(include_str!("../../🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/🔢️set-n/🔣️.json"));
    }

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    crate::os_store::component::tests::assert_fixture_sum_law(&TimestampedMutation::SetN(SetN { n: 9, physical_ms: 7 }), &[Some(3)]).await;
}
}
