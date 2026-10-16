//! 🧪️ Descriptor laws of the demo mutation fixtures: every leaf descriptor equals its committed JSON descriptor.
//!
//! 🧭️ Consolidated from one test file per leaf; each leaf keeps its own module so the test names are unchanged.

mod add_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, demo::AddN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<AddN>(include_str!("../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/➕️add-n/🔣️.json"));
    }
}

mod assign_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, demo::AssignN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<AssignN>(include_str!("../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/↩️assign-n/🔣️.json"));
    }
}

mod delete_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, demo::DeleteN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<DeleteN>(include_str!("../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🗑️delete-n/🔣️.json"));
    }
}

mod set_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, demo::SetN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetN>(include_str!("../../🧪️testing/🧬️mutations/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json"));
    }
}
