//#region 🧪️ExactPendingReceiver
use super::*;
use super::super::instance_lifetime::NativeCloseKey;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🛂️authority.json")).unwrap() }

fn patch() -> UiPatch {
    UiPatch { surface: ui_contract::SurfaceId::try_from(fixture()["current"]["surface"].as_str().unwrap()).unwrap(), base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(1), ops: Default::default() }
}

fn drain(pending: &mut PendingPatchAuthority, key: NativeCloseKey, bytes: usize) -> usize {
    pending.reserve_close_instance(key).unwrap();
    pending.activate_close_instance(key).unwrap();
    let mut released = 0;
    for turn in 0..4096 {
        let step = pending.close_instance_step(key, 1, bytes).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= bytes);
        released += step.released_bytes;
        if step.complete { break; }
        assert!(turn < 4095);
    }
    pending.close_step().unwrap();
    assert!(pending.close_instance_complete(key).unwrap());
    pending.release_close_instance(key).unwrap();
    released
}

#[test]
fn pending_patch_exact_receiver_reserves_before_source_transfer_and_preserves_refused_owner() {
    let fixture = fixture();
    let key = NativeCloseKey::fixture(7, 14);
    let mut pending = PendingPatchAuthority::new();
    let sequence = pending.next_sequence;
    let reservation_bytes = PendingPatchAuthority::required_reservation_bytes();
    assert!(reservation_bytes >= std::mem::size_of::<PendingPatchSlot>());
    assert!(reservation_bytes <= fixture["reservation"]["maximumPlacementBytes"].as_u64().unwrap() as usize);
    assert!(pending.reserve_external(key, 0).unwrap().is_none());
    assert_eq!(pending.next_sequence, sequence);
    assert!(pending.slots.iter().all(Option::is_none));
    let reservation = pending.reserve_external(key, PendingPatchAuthority::required_reservation_bytes()).unwrap().unwrap();
    let receipt = pending.reserved_receipt(reservation).unwrap();
    assert_eq!(receipt.lifetime, key.lifetime());
    assert_eq!(receipt.patch_sequence, sequence + 1);
    let mut source = Some(patch());
    let before = serde_json::to_value(source.as_ref().unwrap()).unwrap();
    assert!(!pending.place_external(reservation, &mut source, 0).unwrap());
    assert_eq!(serde_json::to_value(source.as_ref().unwrap()).unwrap(), before);
    assert!(pending.place_external(reservation, &mut source, std::mem::size_of::<UiPatch>()).unwrap());
    assert!(source.is_none());
    let mut replacement = Some(patch());
    assert!(!pending.place_external(reservation, &mut replacement, std::mem::size_of::<UiPatch>()).unwrap());
    assert_eq!(replacement.is_some(), fixture["reservation"]["occupiedTargetPreservesSource"].as_bool().unwrap());
    let mut rejected = ui_contract::UiPendingPatch::default();
    *rejected.source_mut().unwrap() = replacement.take();
    while !rejected.terminal_is_empty() { rejected.close_step(1, 4096).unwrap(); }
    assert_eq!(drain(&mut pending, key, 4096), fixture["current"]["surface"].as_str().unwrap().len());
}

#[test]
fn pending_patch_exact_receiver_exhaustion_captures_no_source_and_foreign_close_cannot_claim_it() {
    let fixture = fixture();
    let key = NativeCloseKey::fixture(7, 14);
    let mut pending = PendingPatchAuthority::new();
    pending.next_sequence = fixture["reservation"]["maximumSequence"].as_str().unwrap().parse().unwrap();
    assert!(pending.reserve_external(key, PendingPatchAuthority::required_reservation_bytes()).unwrap().is_none());
    assert_eq!(pending.slots.iter().flatten().count(), fixture["reservation"]["sequenceExhaustionCapturesSources"].as_u64().unwrap() as usize);
    let mut pending = PendingPatchAuthority::new();
    let reservation = pending.reserve_external(key, PendingPatchAuthority::required_reservation_bytes()).unwrap().unwrap();
    let mut source = Some(patch());
    pending.place_external(reservation, &mut source, std::mem::size_of::<UiPatch>()).unwrap();
    assert!(pending.reserve_close_instance(NativeCloseKey::fixture(7, 13)).is_err());
    assert!(pending.closing_instances.iter().all(Option::is_none));
    assert_eq!(pending.reserved_receipt(reservation).unwrap().lifetime, key.lifetime());
    assert_eq!(drain(&mut pending, key, 1), fixture["current"]["surface"].as_str().unwrap().len());
}
//#endregion 🧪️ExactPendingReceiver
