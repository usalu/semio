//! 🧪️ Verifies the mutation fixture leaf contract.
use super::*;
use semio_framework_test_fixtures::fixture_path;
#[test]
fn direct_counter_leaf_contract() {
    super::super::super::assert_counter_leaf_descriptor::<AddCounter>(std::fs::read_to_string(fixture_path!("../../../../🧪️testing/🧬️mutation-laws/🧬️mutations/➕️add-counter/🔣️.json")).expect("fixture").as_str());
}
