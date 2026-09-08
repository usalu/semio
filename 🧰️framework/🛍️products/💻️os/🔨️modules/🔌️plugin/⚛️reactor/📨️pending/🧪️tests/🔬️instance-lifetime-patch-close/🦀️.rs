use super::*;

fn patch(surface: &str) -> UiPatch {
    UiPatch { surface: ui_contract::SurfaceId::try_from(surface).unwrap(), base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(1), ops: Default::default() }
}

#[test]
fn guest_instance_lifecycle_pending_patch_handback_preserves_rejected_owner_and_exact_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🚪️lifetime/🧫️fixture/🔣️.json")).unwrap();
    for grant in fixture["pendingPatch"]["grants"].as_array().unwrap() {
        let mut pending = PendingPatchAuthority::new();
        let first = fixture["pendingPatch"]["surfaces"][0].as_str().unwrap();
        let second = fixture["pendingPatch"]["surfaces"][1].as_str().unwrap();
        pending.push_external(patch(first)).unwrap();
        assert!(pending.take_one(65536).unwrap().is_none());
        let first_patch = pending.take_one(65536).unwrap().unwrap();
        pending.hand_back_turn(first_patch).unwrap();
        let rejected = pending.hand_back_turn(patch(second)).unwrap_err();
        assert_eq!(serde_json::to_value(&rejected).unwrap()["surface"], second);
        let mut rejected_owner = ui_contract::UiPendingPatch::default();
        *rejected_owner.source_mut().unwrap() = Some(rejected);
        while !rejected_owner.terminal_is_empty() {
            rejected_owner.close_step(1, 4096).unwrap();
        }
        let key = super::super::instance_lifetime::NativeCloseKey::fixture(7, 1);
        pending.reserve_close_instance(key).unwrap();
        pending.activate_close_instance(key).unwrap();
        let before = serde_json::to_value(pending.turn_handback.get().unwrap()).unwrap();
        assert_eq!(pending.close_instance_step(7, 0, 4096).unwrap(), ui_contract::UiValueRetirementStep::default());
        assert_eq!(pending.close_instance_step(7, 1, 0).unwrap(), ui_contract::UiValueRetirementStep::default());
        assert_eq!(serde_json::to_value(pending.turn_handback.get().unwrap()).unwrap(), before);
        let mut bytes = 0;
        let grant = grant.as_u64().unwrap() as usize;
        for turn in 0..1024 {
            let step = pending.close_instance_step(7, 1, grant).unwrap();
            assert!(step.released_items <= 1 && step.released_bytes <= grant);
            bytes += step.released_bytes;
            if step.complete {
                break;
            }
            assert!(turn < 1023);
        }
        assert_eq!(bytes, first.as_bytes().len());
        assert!(pending.turn_handback.terminal_is_empty());
        assert!(!pending.close_instance_complete(key).unwrap());
        pending.close_step().unwrap();
        assert!(pending.close_instance_complete(key).unwrap());
        pending.release_close_instance(key).unwrap();
    }
}

#[test]
fn guest_instance_lifecycle_pending_patch_unwind_keeps_the_exact_typed_cursor_mounted() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🚪️lifetime/🧫️fixture/🔣️.json")).unwrap();
    let mut pending = PendingPatchAuthority::new();
    pending.push_external(patch(fixture["pendingPatch"]["surfaces"][0].as_str().unwrap())).unwrap();
    let key = super::super::instance_lifetime::NativeCloseKey::fixture(7, 1);
    pending.reserve_close_instance(key).unwrap();
    pending.activate_close_instance(key).unwrap();
    let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pending.close_instance_step(7, 1, 1).unwrap();
        panic!("injected after partial typed retirement");
    }));
    assert!(failure.is_err());
    assert_eq!(pending.slots[0].is_some(), fixture["pendingPatch"]["faultLeavesStructuralOwner"].as_bool().unwrap());
    assert!(!pending.close_instance_complete(key).unwrap());
    for turn in 0..4096 {
        pending.close_step().unwrap();
        if pending.close_instance_complete(key).unwrap() {
            break;
        }
        assert!(turn < 4095);
    }
    pending.release_close_instance(key).unwrap();
    assert!(pending.slots.iter().all(Option::is_none));
}

#[test]
fn instance_lifetime_pending_patch_keeps_scope_after_payload_surface_retires() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json")).unwrap();
    let mut pending = PendingPatchAuthority::new();
    pending.push_external(UiPatch { surface: ui_contract::SurfaceId::try_from("7:retained").unwrap(), base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(1), ops: Default::default() }).unwrap();
    let key = super::super::instance_lifetime::NativeCloseKey::fixture(7, 1);
    pending.reserve_close_instance(key).unwrap();
    pending.activate_close_instance(key).unwrap();
    if let Some(PendingPatchSlot { owner: PendingPatchOwner::External(owner), .. }) = pending.slots[0].as_mut() {
        owner.source_mut().unwrap().as_mut().unwrap().surface = Default::default();
    }
    for turn in 0..1024 {
        pending.close_step().unwrap();
        if pending.slots[0].is_none() {
            break;
        }
        assert!(turn < 1023);
    }
    assert_eq!(pending.slots[0].is_none(), fixture["nativeCases"]["scopeAfterSurfaceClear"].as_bool().unwrap());
    assert!(!pending.close_step().unwrap());
    assert!(pending.close_step().unwrap());
    assert!(pending.close_instance_complete(key).unwrap());
    pending.release_close_instance(key).unwrap();
    assert!(pending.close_instance_complete(key).is_err());
}
