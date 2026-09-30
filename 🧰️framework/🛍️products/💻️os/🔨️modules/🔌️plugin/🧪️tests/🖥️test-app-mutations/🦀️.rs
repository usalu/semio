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
    ::store::os_store::test_support::assert_wire_witness::<TestConfigMutation>(include_str!("../../🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<TestMutation>(include_str!("../../🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}
//#endregion 🧪️TestAppMutationFixtures
