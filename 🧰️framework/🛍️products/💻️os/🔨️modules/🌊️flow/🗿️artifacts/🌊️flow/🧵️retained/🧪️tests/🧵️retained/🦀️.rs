//! 🧪️ Shared typed Flow retirement byte and ownership laws.

use super::*;
use crate::os_store::ErasedSnapshotRetirement;
use std::mem::size_of;

//#region 🧪️Retirement
fn drain(mut retirement: FlowRetirement, minimum_close_bytes: usize) -> usize {
    let mut released = 0usize;
    for _ in 0..200_000 {
        if let Some(bytes) = retirement.next_allocation_bytes().unwrap() {
            retirement.reserve_allocation(bytes).unwrap();
            continue;
        }
        let maximum_bytes = retirement.next_close_byte_demand().unwrap().max(minimum_close_bytes).max(1);
        match retirement.close_step(1, maximum_bytes).unwrap() {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= maximum_bytes);
                released += released_bytes;
            }
            SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                return released;
            }
            SnapshotRetirementStep::Blocked => panic!("exact Flow demand blocked"),
        }
    }
    panic!("Flow retirement did not reach terminal-empty")
}

#[test]
fn flow_retirement_typed_serde_oracle_and_exact_bytes_survive_worker_transfer() {
    let fixture = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for maximum in [1, 4096] {
        let value: FlowHostSnapshot = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(fixture.get("hostSnapshot").unwrap())).unwrap();
        let oracle: FlowHostSnapshot = crate::os_dsl::FromValue::from_value(crate::os_dsl::ToValue::to_value(&value)).unwrap();
        assert_eq!(value, oracle);
        let oracle_released = drain(FlowRetirement::from_owner(FlowOwner::HostSnapshot(oracle)), maximum);
        let mut retirement = FlowRetirement::from_owner(FlowOwner::HostSnapshot(value));
        assert!(matches!(retirement.close_step(0, maximum).unwrap(), SnapshotRetirementStep::Blocked));
        assert!(matches!(retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked));
        let released = std::thread::spawn(move || drain(retirement, maximum)).join().unwrap();
        assert_eq!(released, oracle_released);
        assert!(released >= fixture.get("expected").and_then(|v| v.get("releasedBytes")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
    }
}

#[test]
fn flow_retirement_populated_drop_is_guarded_and_unwind_does_not_double_panic() {
    let mut retirement = FlowRetirement::default();
    retirement.push(FlowOwner::Bytes(vec![0; 8192]));
    assert!(std::panic::catch_unwind(|| drop(retirement)).is_err());
    assert!(std::thread::spawn(|| {
        let mut retirement = FlowRetirement::default();
        retirement.push(FlowOwner::Bytes(vec![0; 8192]));
        panic!("primary Flow retirement fault");
    }).join().is_err());
}

fn oversized_empty<T>(minimum_bytes: usize) -> Vec<T> {
    let slots = minimum_bytes.div_ceil(size_of::<T>()).max(1);
    Vec::with_capacity(slots)
}

/// 🎟️ Two currencies, one owner. The PAYLOAD (`len`) is portable and divisible, so it is charged
/// to the caller one grant at a time and a one-byte grant always progresses; the ALLOCATION
/// (`capacity * size_of::<T>()`) is machine-width dependent, is never charged to a caller, and
/// leaves `allocated_bytes` WHOLE, in exactly one step. A drained element vector owes no payload at
/// all — it owes only its allocation.
fn exact_direct_backing(owner: FlowOwner, expected_allocation_bytes: usize, expected_payload_bytes: usize) {
    assert!(expected_allocation_bytes > 4096);
    let mut retirement = FlowRetirement::from_owner(owner);
    assert_eq!(retirement.allocated_bytes(), expected_allocation_bytes);
    assert_eq!(retirement.next_allocation_bytes().unwrap(), None);
    assert_eq!(retirement.next_close_byte_demand().unwrap(), 1);
    assert!(matches!(retirement.close_step(0, expected_allocation_bytes).unwrap(), SnapshotRetirementStep::Blocked));
    assert!(matches!(retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked));
    assert_eq!(retirement.allocated_bytes(), expected_allocation_bytes);

    let mut released = 0usize;
    let mut allocation_steps = 0usize;
    for _ in 0..expected_payload_bytes + 2 {
        if retirement.terminal_is_empty() {
            break;
        }
        let held = retirement.allocated_bytes();
        let SnapshotRetirementStep::Pending { released_items, released_bytes } = retirement.close_step(1, 1).unwrap() else {
            panic!("a positive grant always charges a direct backing")
        };
        assert!(released_items <= 1 && released_bytes <= 1);
        released += released_bytes;
        if retirement.allocated_bytes() != held {
            assert_eq!(held - retirement.allocated_bytes(), expected_allocation_bytes, "an allocation leaves whole or not at all");
            allocation_steps += 1;
        }
    }
    assert_eq!(released, expected_payload_bytes, "only portable payload bytes are charged to the caller's grant");
    assert_eq!(allocation_steps, 1, "the actual capacity is released exactly once");
    assert_eq!(retirement.allocated_bytes(), 0);
    assert!(matches!(retirement.close_step(0, 0).unwrap(), SnapshotRetirementStep::Complete));
    assert!(retirement.terminal_is_empty());
}

#[test]
fn flow_physical_retirement_every_direct_string_and_vec_releases_actual_capacity_once() {
    let mut bytes = Vec::with_capacity(8193);
    bytes.resize(4097, 7);
    let bytes_capacity = bytes.capacity();
    exact_direct_backing(FlowOwner::Bytes(bytes), bytes_capacity, 4097);

    let values = oversized_empty::<String>(8193);
    let capacity = values.capacity() * size_of::<String>();
    exact_direct_backing(FlowOwner::Strings(values), capacity, 0);
    let values = oversized_empty::<Widget>(8193);
    let capacity = values.capacity() * size_of::<Widget>();
    exact_direct_backing(FlowOwner::Widgets(values), capacity, 0);
    let values = oversized_empty::<SynapseSpec>(8193);
    let capacity = values.capacity() * size_of::<SynapseSpec>();
    exact_direct_backing(FlowOwner::Specs(values), capacity, 0);
    let values = oversized_empty::<neural::Neuron>(8193);
    let capacity = values.capacity() * size_of::<neural::Neuron>();
    exact_direct_backing(FlowOwner::Neurons(values), capacity, 0);
    let values = oversized_empty::<neural::Synapse>(8193);
    let capacity = values.capacity() * size_of::<neural::Synapse>();
    exact_direct_backing(FlowOwner::Synapses(values), capacity, 0);
    let values = oversized_empty::<FlowPreviewGui>(8193);
    let capacity = values.capacity() * size_of::<FlowPreviewGui>();
    exact_direct_backing(FlowOwner::Previews(values), capacity, 0);
    let values = oversized_empty::<FlowLayoutEntry>(8193);
    let capacity = values.capacity() * size_of::<FlowLayoutEntry>();
    exact_direct_backing(FlowOwner::Layout(values), capacity, 0);
}

#[test]
fn flow_physical_retirement_frontier_requires_exact_admission_and_releases_metadata_last() {
    let host_snapshot: FlowHostSnapshot = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(
        crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap().get("hostSnapshot").unwrap(),
    )).unwrap();
    let mut retirement = FlowRetirement::from_owner(FlowOwner::HostSnapshot(host_snapshot));
    let demand = retirement.next_allocation_bytes().unwrap().expect("fixture decomposition needs a frontier page");
    assert!(demand > 0);
    assert_eq!(retirement.reserve_allocation(0).unwrap().allocated_bytes, 0);
    assert_eq!(retirement.reserve_allocation(demand - 1).unwrap().allocated_bytes, 0);
    assert_eq!(retirement.allocated_bytes(), 0);
    assert_eq!(retirement.reserve_allocation(demand).unwrap().allocated_bytes, demand);
    assert_eq!(retirement.allocated_bytes(), demand);
    assert!(matches!(retirement.close_step(0, usize::MAX).unwrap(), SnapshotRetirementStep::Blocked));

    let mut released = 0usize;
    let mut last_release = 0usize;
    for _ in 0..200_000 {
        if let Some(bytes) = retirement.next_allocation_bytes().unwrap() {
            let step = retirement.reserve_allocation(bytes).unwrap();
            assert!(step.allocated_bytes >= bytes);
            continue;
        }
        let close_demand = retirement.next_close_byte_demand().unwrap();
        match retirement.close_step(1, close_demand).unwrap() {
            SnapshotRetirementStep::Pending { released_bytes, .. } => {
                released = released.checked_add(released_bytes).unwrap();
                if released_bytes != 0 { last_release = released_bytes; }
            }
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Blocked => panic!(
                "exact Flow allocation and close demands cannot block: demand={close_demand} allocated={} next_allocation={:?}",
                retirement.allocated_bytes(),
                retirement.next_allocation_bytes(),
            ),
        }
    }
    assert!(released > 4096);
    assert!(last_release > 0, "the frontier backing is the final physical release");
    assert_eq!(retirement.allocated_bytes(), 0);
    assert!(retirement.terminal_is_empty());
}

#[test]
fn flow_physical_retirement_multi_root_ingress_records_fault_without_admission_then_admits_exactly() {
    let fixture = crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let contract = fixture.get("physicalRetirement").and_then(|value| value.get("multiRootIngress")).unwrap();
    let first_capacity = contract.get("firstCapacityBytes").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize;
    let second_capacity = contract.get("secondCapacityBytes").and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize;
    let mut first = Vec::with_capacity(first_capacity);
    first.push(1);
    let first_capacity = first.capacity();
    let mut second = Vec::with_capacity(second_capacity);
    second.push(2);
    let second_capacity = second.capacity();

    let mut first = Vec::with_capacity(first_capacity);
    first.push(1);
    let mut second = Vec::with_capacity(second_capacity);
    second.push(2);
    let mut retirement = FlowRetirement::from_owner(FlowOwner::Bytes(first));
    retirement.push(FlowOwner::Bytes(second));
    let admitted_bytes = retirement.allocated_bytes();
    assert!(admitted_bytes >= first_capacity + second_capacity);
    assert_eq!(drain(retirement, admitted_bytes), 2, "both admitted allocations reach terminal-empty; only their two PAYLOAD bytes are charged to the caller");
}

/// 🎟️ A seven-byte scene charges seven bytes, on every target. The capacity behind it — the
/// oversized byte buffer's own, and a drained element vector's `capacity * size_of::<String>()` — is
/// allocation admission, visible through `allocated_bytes`, and is never charged to the caller's
/// payload grant (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END).
#[test]
fn flow_physical_retirement_charges_payload_bytes_not_machine_width_capacity() {
    let mut scene = Vec::with_capacity(32 * 1024);
    scene.extend_from_slice(b"7 bytes");
    let scene_capacity = scene.capacity();
    let drained = oversized_empty::<String>(8193);
    let drained_capacity = drained.capacity() * size_of::<String>();
    let mut retirement = FlowRetirement::from_owner(FlowOwner::Bytes(scene));
    retirement.push(FlowOwner::Strings(drained));
    let admitted = retirement.allocated_bytes();
    assert!(admitted >= scene_capacity + drained_capacity);
    assert!(admitted > 30_000, "the admitted allocation dwarfs the payload");
    assert_eq!(drain(retirement, 1), 7, "a seven-byte scene charges seven bytes, whatever size_of::<String>() is");
}
//#endregion 🧪️Retirement
