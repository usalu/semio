//! 📋️ Fixed-list initialized ownership and cold serde oracle laws.

use super::*;

//#region 🧪️InitializedOwnership
#[test]
fn instance_lifetime_ui_fixed_list_initializes_only_owned_payloads() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut values: UiFixedList<serde_json::Value, 4> = serde_json::from_value(row["values"].clone()).unwrap();
        assert_eq!(serde_json::to_value(&values).unwrap(), row["values"]);
        let exact = values.items.as_ref().map_or(0, |items| items.len()) == values.len();
        let mut popped = Vec::new();
        while let Some(value) = values.pop() { popped.push(value); }
        assert_eq!(popped, *row["popped"].as_array().unwrap());
        assert_eq!(exact, fixture["ownership"]["initializedSlotsEqualLogicalLength"].as_bool().unwrap(), "{}", row["name"]);
    }
}

#[derive(Debug)]
struct CountedPayload(std::sync::Arc<std::sync::atomic::AtomicUsize>);

impl Drop for CountedPayload {
    fn drop(&mut self) { self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
}

#[test]
fn instance_lifetime_ui_fixed_list_empty_release_preserves_transferred_payload() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
    let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut values = UiFixedList::<CountedPayload, 4>::default();
    assert!(values.try_reserve().unwrap());
    values.try_push_reserved(CountedPayload(drops.clone())).unwrap();
    let identity = values.get(0).unwrap() as *const CountedPayload;
    assert_eq!(values.release_empty_allocation().is_ok(), fixture["ownership"]["releaseWithPayloadAccepted"].as_bool().unwrap());
    assert_eq!(values.get(0).unwrap() as *const CountedPayload, identity);
    let transferred = values.pop().unwrap();
    assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst) != 0, fixture["ownership"]["popDestroysTransferredPayload"].as_bool().unwrap());
    assert_eq!(!values.terminal_is_empty(), fixture["ownership"]["terminalRequiresNoAllocation"].as_bool().unwrap());
    assert!(values.release_empty_allocation().unwrap());
    assert!(values.terminal_is_empty());
    assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst) != 0, fixture["ownership"]["emptyReleaseDestroysTransferredPayload"].as_bool().unwrap());
    assert_eq!(usize::from(values.release_empty_allocation().unwrap()), fixture["ownership"]["secondEmptyReleaseCount"].as_u64().unwrap() as usize);
    drop(transferred);
    assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
fn instance_lifetime_ui_fixed_list_reservation_preserves_fixed_envelope() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
    let mut values = UiFixedList::<u32, 4>::default();
    assert_eq!(values.try_push_reserved(99).is_ok(), fixture["ownership"]["unreservedPushAccepted"].as_bool().unwrap());
    assert!(values.terminal_is_empty());
    assert!(values.try_reserve().unwrap());
    let identity = values.items.as_ref().unwrap().as_ptr();
    for value in 0..4 { values.try_push_reserved(value).unwrap(); }
    assert_eq!(identity != values.items.as_ref().unwrap().as_ptr(), fixture["ownership"]["reservedPushChangesBacking"].as_bool().unwrap());
    assert_eq!(values.try_push_reserved(99) == Err(99), fixture["ownership"]["overflowPreservesRejectedOwner"].as_bool().unwrap());
    assert_eq!(values.capacity(), fixture["capacity"].as_u64().unwrap() as usize);
    assert_eq!(values.swap_remove(1), Some(1));
    assert_eq!(values.iter().copied().collect::<Vec<_>>(), vec![0, 3, 2]);
    while values.pop().is_some() {}
    assert!(values.release_empty_allocation().unwrap());
    let mut zero = UiFixedList::<u32, 0>::default();
    assert_eq!(zero.try_reserve().unwrap(), fixture["ownership"]["zeroCapacityReservesBacking"].as_bool().unwrap());
    assert_eq!(zero.try_push_reserved(17), Err(17));
    assert!(zero.terminal_is_empty());
    let mut zero_sized = UiFixedList::<(), 2>::default();
    zero_sized.try_reserve().unwrap();
    zero_sized.try_push_reserved(()).unwrap();
    zero_sized.try_push_reserved(()).unwrap();
    assert_eq!(zero_sized.try_push_reserved(()), Err(()));
    assert_eq!(zero_sized.capacity(), 2);
    assert_eq!(zero_sized.pop(), Some(()));
    assert_eq!(zero_sized.pop(), Some(()));
    assert!(zero_sized.release_empty_allocation().unwrap());
    assert!(zero_sized.terminal_is_empty());
}
//#endregion 🧪️InitializedOwnership
