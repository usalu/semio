use super::*;

//#region 🧪️PatchHandoff
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture.json")).unwrap() }
fn ready() -> SurfaceReconcileReadyPatch {
    let fixture = fixture();
    let generation = fixture["generation"].as_u64().unwrap();
    let mut reservation = SurfaceReconcileReservation::try_new(generation).unwrap();
    let mut ops = ui_contract::UiPatchOps::default();
    ops.try_push(ui_contract::UiPatchOp::SetRoot { id: ui_contract::UiNodeId(41) }).unwrap();
    SurfaceReconcileReadyPatch { generation, patch: pending_surface_patch(Some(ui_contract::UiPatch { surface: ui_contract::SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap(), base_revision: ui_contract::UiRevision(6), revision: ui_contract::UiRevision(7), ops })), credit: reservation.credit.take(), handback: reservation.handback.take() }
}
fn close_ready(owner: &mut SurfaceReconcileReadyPatch, grant: usize) -> usize {
    let mut bytes = 0;
    for _ in 0..100_000 {
        let step = owner.close_step_with_grant(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        bytes += step.released_bytes;
        if step.complete { assert!(owner.terminal_is_empty()); return bytes; }
    }
    panic!("ready patch did not close");
}
fn close_published(owner: &mut SurfaceReconcilePublishedPatch, grant: usize) -> usize {
    let mut bytes = 0;
    for _ in 0..100_000 {
        let step = owner.close_step_with_grant(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        bytes += step.released_bytes;
        if step.complete { assert!(owner.terminal_is_empty()); return bytes; }
    }
    panic!("published patch did not close");
}

#[test]
fn retained_patch_handoff_keeps_exact_slots_until_preflight_and_acknowledgment() {
    let mut source = ready();
    let mut payload = ui_contract::UiPendingPatch::default();
    let mut published = None;
    assert_eq!(source.publish_into(&mut payload, &mut published, 0).unwrap(), 0);
    assert!(!source.terminal_is_empty() && payload.terminal_is_empty() && published.is_none());
    assert!(source.publish_into(&mut payload, &mut published, 32768).unwrap() > 0);
    assert!(source.terminal_is_empty());
    let exact = serde_json::to_value(payload.get().unwrap()).unwrap();
    let mut blocked = ready();
    assert_eq!(blocked.publish_into(&mut payload, &mut published, 32768).unwrap(), 0);
    assert_eq!(serde_json::to_value(payload.get().unwrap()).unwrap(), exact);
    close_ready(&mut blocked, 1);
    let mut ack = None;
    assert!(!SurfaceReconcilePublishedPatch::acknowledge_into(&mut published, &mut ack, "foreign", 7, 32768).unwrap());
    assert!(published.is_some() && ack.is_none());
    assert!(!SurfaceReconcilePublishedPatch::acknowledge_into(&mut published, &mut ack, "éé", 7, 0).unwrap());
    assert!(SurfaceReconcilePublishedPatch::acknowledge_into(&mut published, &mut ack, "éé", 7, 32768).unwrap());
    assert!(published.is_none());
    while !payload.close_step(1, 1).unwrap().complete {}
    let mut ack = ack.unwrap();
    let mut bytes = 0;
    for _ in 0..100_000 { let step = ack.close_step_with_grant(1, 1).unwrap(); bytes += step.released_bytes; if step.complete { break; } }
    assert!(ack.terminal_is_empty());
    assert_eq!(bytes, 4);
    eprintln!("[DEBUG] patch-handoff exact-slots=true occupied-target-preserved=true invalid-ack-preserved=true surface-bytes=4");
}

#[test]
fn retained_patch_handoff_close_respects_all_grants_and_contended_exact_credit() {
    for grant in fixture()["grants"].as_array().unwrap() { let mut owner = ready(); assert_eq!(close_ready(&mut owner, grant.as_u64().unwrap() as usize), 4); }
    let mut source = ready();
    let mut payload = ui_contract::UiPendingPatch::default();
    let mut published = None;
    source.publish_into(&mut payload, &mut published, 32768).unwrap();
    while !payload.close_step(1, 64).unwrap().complete {}
    let mut owner = published.unwrap();
    let ledger = ui_contract::UiResidentPermit::try_observe().unwrap();
    let mut saw_blocked = false;
    for _ in 0..100 {
        let step = owner.close_step_with_grant(1, 1).unwrap();
        if !step.progressed { saw_blocked = true; break; }
    }
    assert!(owner.credit.as_ref().is_some_and(|credit| ledger.owns(credit)));
    drop(ledger);
    let registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap();
    let mut handback_blocked = false;
    for _ in 0..100 { if !owner.close_step_with_grant(1, 1).unwrap().progressed { handback_blocked = true; break; } }
    assert!(owner.handback.is_some());
    drop(registry);
    close_published(&mut owner, 1);
    assert!(saw_blocked && handback_blocked);
    eprintln!("[DEBUG] patch-close grants=1,64,4096 exact-credit-contention=true exact-handback-contention=true");
}
#[test]
fn retained_patch_handoff_unwind_preserves_structural_payload_and_exact_authority() {
    for frontier in fixture()["unwindFrontiers"].as_array().unwrap() {
        let mut source = ready();
        let pointer = source.patch.get().unwrap().ops.get(0).unwrap() as *const _;
        let mut payload = ui_contract::UiPendingPatch::default();
        let mut published = None;
        let mut ack = None;
        let fault = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if frontier.as_u64().unwrap() >= 1 { source.publish_into(&mut payload, &mut published, 32768).unwrap(); }
            if frontier.as_u64().unwrap() >= 2 { SurfaceReconcilePublishedPatch::acknowledge_into(&mut published, &mut ack, "éé", 7, 32768).unwrap(); }
            panic!("fixture callback failure with all source and output slots retained");
        }));
        assert!(fault.is_err());
        let retained = source.patch.get().or_else(|| payload.get()).unwrap();
        assert_eq!(retained.ops.get(0).unwrap() as *const _, pointer);
        assert_eq!(usize::from(source.credit.is_some()) + usize::from(published.as_ref().is_some_and(|owner| owner.credit.is_some())) + usize::from(ack.as_ref().is_some_and(|owner| owner.owner.credit.is_some())), 1);
        close_ready(&mut source, 1);
        while !payload.close_step(1, 1).unwrap().complete {}
        if let Some(owner) = published.as_mut() { close_published(owner, 1); }
        if let Some(owner) = ack.as_mut() { for _ in 0..100_000 { if owner.close_step_with_grant(1, 1).unwrap().complete { break; } } assert!(owner.terminal_is_empty()); }
    }
    eprintln!("[DEBUG] patch-handoff unwind-frontiers=3 exact-payload-pointer=true exact-authority-count=1 publish-bytes={} ack-bytes={}", SurfaceReconcileReadyPatch::required_publish_bytes(), SurfaceReconcilePublishedPatch::required_acknowledge_bytes());
}
//#endregion 🧪️PatchHandoff
