//#region 🧪️TestAppMutationFixtures
#[path = "../🖥️test-app-mutations-config/🦀️.rs"]
pub mod config;
pub use crate::test_app_mutation_fixture::config::mutations::change_test_config_selection::ChangeTestConfigSelection;
pub use crate::test_app_mutation_fixture::config::TestConfig;
pub use crate::test_app_mutation_fixture::config::mutations::TestConfigMutation;
#[path = "../🖥️test-app-mutations-document/🦀️.rs"]
pub mod document;
pub use crate::test_app_mutation_fixture::document::mutations::set_count::SetCount;
pub use crate::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
pub use crate::test_app_mutation_fixture::document::mutations::set_slot_children::SetSlotChildren;
pub use crate::test_app_mutation_fixture::document::mutations::TestMutation;
pub use crate::test_app_mutation_fixture::document::TestSnapshot;

/// 🧾️ The committed wire witnesses decode through the aggregate's `FromValue` and re-encode to exactly the committed JSON.
#[doc(hidden)]
#[test]
pub fn committed_wire_witnesses_are_the_canonical_wire() {
    ::store::os_store::test_support::assert_wire_witness::<TestConfigMutation>(include_str!("../../🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}
//#endregion 🧪️TestAppMutationFixtures
