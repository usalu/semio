//! 🧪️ Descriptor laws of the severity mutation fixtures: every leaf descriptor equals its committed JSON descriptor.
//!
//! 🧭️ Consolidated from one test file per leaf; each leaf keeps its own module so the test names are unchanged.

mod assign_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, severity::AssignN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<AssignN>(include_str!("../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/↩️assign-n/🔣️.json"));
    }
}

mod set_error_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, severity::SetErrorN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetErrorN>(include_str!("../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🚫️set-error-n/🔣️.json"));
    }
}

mod set_fatal_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, severity::SetFatalN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetFatalN>(include_str!("../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🛑️set-fatal-n/🔣️.json"));
    }
}

mod set_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, severity::SetN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetN>(include_str!("../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/🔢️set-n/🔣️.json"));
    }
}

mod set_warning_n {
    use semio_framework_os_store_testing_fixtures::{assert_fixture_descriptor, severity::SetWarningN};

    #[test]
    fn direct_fixture_leaf_contract() {
        assert_fixture_descriptor::<SetWarningN>(include_str!("../../🧪️testing/🧬️mutations/🚦️severity/🧬️mutations/⚠️set-warning-n/🔣️.json"));
    }
}
