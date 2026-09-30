//! 🧪️ Direct metadata-bearing counter fixtures for Plugin runtime tests.

#[path = "../🧬️mutation-fixtures-dummy/🦀️.rs"]
pub mod dummy;
#[path = "../🧬️mutation-fixtures-no-state/🦀️.rs"]
pub mod no_state;
#[path = "../🧬️mutation-fixtures-surface/🦀️.rs"]
pub mod surface;
#[path = "../🧬️mutation-fixtures-transaction/🦀️.rs"]
pub mod transaction;

/// 🧾️ The committed wire witnesses decode through the aggregate's `FromValue` and re-encode to exactly the committed JSON.
#[test]
fn committed_wire_witnesses_are_the_canonical_wire() {
    ::store::os_store::test_support::assert_wire_witness::<dummy::DummyMutation>(include_str!("../../🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<transaction::TxnMutation>(include_str!("../../🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<transaction::TxnMutation>(include_str!("../../🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<transaction::TxnMutation>(include_str!("../../🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
    ::store::os_store::test_support::assert_wire_witness::<surface::SurfaceMutation>(include_str!("../../🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}
