//! 🧪️ Descriptor laws of the timestamped mutation fixtures: every leaf descriptor equals its committed JSON descriptor.
//!
//! 🧭️ Consolidated from one test file per leaf; each leaf keeps its own module so the test names are unchanged.

mod assign_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, timestamped::AssignN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<AssignN>(include_str!("../../🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/↩️assign-n/🔣️.json"));
    }
}

mod set_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, timestamped::SetN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetN>(include_str!("../../🧪️testing/🧬️mutations/⏱️timestamped/🧬️mutations/🔢️set-n/🔣️.json"));
    }
}
