//! 🧪️ Descriptor laws of the lossy mutation fixtures: every leaf descriptor equals its committed JSON descriptor.
//!
//! 🧭️ Consolidated from one test file per leaf; each leaf keeps its own module so the test names are unchanged.

mod set_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, lossy::SetN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetN>(include_str!("../../🧪️testing/🧬️mutations/🪤️lossy/🧬️mutations/🔢️set-n/🔣️.json"));
    }
}
