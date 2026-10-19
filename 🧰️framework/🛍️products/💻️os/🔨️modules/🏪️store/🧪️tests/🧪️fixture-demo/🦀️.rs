//! 🧪️ Descriptor laws of the demo mutation fixtures: every leaf descriptor equals its committed JSON descriptor.
//!
//! 🧭️ Consolidated from one test file per leaf; each leaf keeps its own module so the test names are unchanged.

mod add_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, demo::AddN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<AddN>(include_str!("../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/➕️add-n/🔣️.json"));
    }

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::AddN(AddN { delta: 4 }), &[Some(3)]).await;
}
}

mod assign_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, demo::AssignN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<AssignN>(include_str!("../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/↩️assign-n/🔣️.json"));
    }

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::AssignN(AssignN { n: Some(5) }), &[Some(3), None]).await;
    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::AssignN(AssignN { n: None }), &[Some(3)]).await;
}
}

mod delete_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, demo::DeleteN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<DeleteN>(include_str!("../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🗑️delete-n/🔣️.json"));
    }

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::DeleteN(DeleteN {}), &[Some(3)]).await;
}
}

mod set_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, demo::SetN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetN>(include_str!("../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json"));
    }

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    crate::os_store::component::tests::assert_fixture_sum_law(&DemoMutation::SetN(SetN { n: 9 }), &[Some(3)]).await;
}
}
