//#region 🧪️TestAppMutationFixtures
#[path = "../🖥️test-app-mutations-config/🦀️.rs"]
pub mod config;
pub(crate) use config::{ChangeTestConfigSelection, TestConfig, TestConfigMutation};
#[path = "../🖥️test-app-mutations-document/🦀️.rs"]
pub mod document;
pub(crate) use document::{SetCount, SetLabel, SetSlotChildren, TestMutation, TestSnapshot};

/// 🧾️ The committed wire witnesses decode through the aggregate's `FromValue` and re-encode to exactly the committed JSON.
#[test]
fn committed_wire_witnesses_are_the_canonical_wire() {
    ::store::os_store::test_support::assert_wire_witness::<TestConfigMutation>(include_str!("../../🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🧪️tests/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}
//#endregion 🧪️TestAppMutationFixtures
