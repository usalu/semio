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
        let value: FlowFixture = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(fixture.get("fixture").unwrap())).unwrap();
        let oracle: FlowFixture = crate::os_dsl::FromValue::from_value(crate::os_dsl::ToValue::to_value(&value)).unwrap();
        assert_eq!(value, oracle);
        let oracle_released = drain(FlowRetirement::from_owner(FlowOwner::Fixture(oracle)), maximum);
        let mut retirement = FlowRetirement::from_owner(FlowOwner::Fixture(value));
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

fn exact_direct_backing(owner: FlowOwner, expected_bytes: usize) {
    assert!(expected_bytes > 4096);
    let mut retirement = FlowRetirement::from_owner(owner);
    assert_eq!(retirement.allocated_bytes(), expected_bytes);
    assert_eq!(retirement.next_allocation_bytes().unwrap(), None);
    assert_eq!(retirement.next_close_byte_demand().unwrap(), expected_bytes);
    assert!(matches!(retirement.close_step(0, expected_bytes).unwrap(), SnapshotRetirementStep::Blocked));
    assert!(matches!(retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked));
    assert_eq!(retirement.allocated_bytes(), expected_bytes);
    assert!(matches!(retirement.close_step(1, expected_bytes - 1).unwrap(), SnapshotRetirementStep::Blocked));
    assert_eq!(retirement.allocated_bytes(), expected_bytes);
    assert!(matches!(retirement.close_step(1, expected_bytes).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes } if released_bytes == expected_bytes));
    assert_eq!(retirement.allocated_bytes(), 0);
    assert!(matches!(retirement.close_step(0, 0).unwrap(), SnapshotRetirementStep::Complete));
    assert!(retirement.terminal_is_empty());
}

#[test]
fn flow_physical_retirement_every_direct_string_and_vec_releases_actual_capacity_once() {
    let mut bytes = Vec::with_capacity(8193);
    bytes.push(7);
    let bytes_capacity = bytes.capacity();
    exact_direct_backing(FlowOwner::Bytes(bytes), bytes_capacity);

    let values = oversized_empty::<String>(8193);
    let capacity = values.capacity() * size_of::<String>();
    exact_direct_backing(FlowOwner::Strings(values), capacity);
    let values = oversized_empty::<Widget>(8193);
    let capacity = values.capacity() * size_of::<Widget>();
    exact_direct_backing(FlowOwner::Widgets(values), capacity);
    let values = oversized_empty::<SynapseSpec>(8193);
    let capacity = values.capacity() * size_of::<SynapseSpec>();
    exact_direct_backing(FlowOwner::Specs(values), capacity);
    let values = oversized_empty::<neural::Neuron>(8193);
    let capacity = values.capacity() * size_of::<neural::Neuron>();
    exact_direct_backing(FlowOwner::Neurons(values), capacity);
    let values = oversized_empty::<neural::Synapse>(8193);
    let capacity = values.capacity() * size_of::<neural::Synapse>();
    exact_direct_backing(FlowOwner::Synapses(values), capacity);
    let values = oversized_empty::<FlowPreviewGui>(8193);
    let capacity = values.capacity() * size_of::<FlowPreviewGui>();
    exact_direct_backing(FlowOwner::Previews(values), capacity);
    let values = oversized_empty::<FlowLayoutEntry>(8193);
    let capacity = values.capacity() * size_of::<FlowLayoutEntry>();
    exact_direct_backing(FlowOwner::Layout(values), capacity);
}

#[test]
fn flow_physical_retirement_frontier_requires_exact_admission_and_releases_metadata_last() {
    let fixture: FlowFixture = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(
        crate::os_pack::json::parse(include_str!("../../🧫️fixtures/🔣️.json")).unwrap().get("fixture").unwrap(),
    )).unwrap();
    let mut retirement = FlowRetirement::from_owner(FlowOwner::Fixture(fixture));
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
                "[DEBUG] exact Flow allocation and close demands cannot block: demand={close_demand} allocated={} next_allocation={:?}",
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
fn flow_physical_retirement_multi_root_ingress_retains_refused_owner_until_exact_retry_and_close() {
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
    let mut retirement = FlowRetirement::from_owner(FlowOwner::Bytes(first));
    let mut refused = retirement.push(FlowOwner::Bytes(second)).expect_err("a second root requires admitted frontier backing");
    assert_eq!(retirement.allocated_bytes(), first_capacity);
    assert!(matches!(&refused, FlowOwner::Bytes(bytes) if bytes.capacity() == second_capacity));

    for _ in 0..8 {
        let demand = retirement.next_push_allocation_bytes().unwrap().expect("refused owner requires one exact frontier allocation");
        let before = retirement.allocated_bytes();
        assert_eq!(retirement.reserve_push_allocation(0).unwrap().allocated_bytes, 0);
        assert_eq!(retirement.reserve_push_allocation(demand - 1).unwrap().allocated_bytes, 0);
        assert_eq!(retirement.allocated_bytes(), before);
        refused = match retirement.push(refused) {
            Ok(()) => break,
            Err(owner) => owner,
        };
        let step = retirement.reserve_push_allocation(demand).unwrap();
        assert!(step.progressed && step.allocated_bytes >= demand);
        refused = match retirement.push(refused) {
            Ok(()) => break,
            Err(owner) => owner,
        };
    }
    assert!(retirement.next_push_allocation_bytes().unwrap().is_none());
    let admitted_bytes = retirement.allocated_bytes();
    assert!(admitted_bytes >= first_capacity + second_capacity);
    assert_eq!(drain(retirement, admitted_bytes), admitted_bytes);
}
//#endregion 🧪️Retirement
