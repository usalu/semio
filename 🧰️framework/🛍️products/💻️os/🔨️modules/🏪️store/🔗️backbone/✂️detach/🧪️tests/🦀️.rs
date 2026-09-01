use super::*;

//#region ✂️DetachRefusal
async fn observe_refusal(case_id: &str) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let case = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == case_id).unwrap();
    let mut store = super::super::ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "detach-refusal", DemoSnapshot { n: Some(4) }, None)).await.expect("real initialized Store");
    store.install_member_store_owners_exact(demo_closable_store_owners());
    let (mut local, peer) = MemoryBackbone::pair("detach-local", "detach-peer").await;
    let payload_byte = u8::try_from(fixture["payload"]["byte"].as_u64().unwrap()).unwrap();
    let payload_length = usize::try_from(fixture["payload"]["length"].as_u64().unwrap()).unwrap();
    let payload = vec![payload_byte; payload_length];
    let expected_pointer = payload.as_ptr();
    let expected_capacity = payload.capacity();
    local.send(BackboneMessage::Mutations { envelopes: payload }).await.expect("original queued payload");
    let original_outbox = Arc::as_ptr(local.outbox.as_ref().unwrap());
    store.envelope.backbone = Some(local.descriptor().await);
    *store.backbone = Some(Backbones::Memory(local));
    let original_generation = store.generation;
    let occupied = if case["failure"] == "capacity" {
        Some(store.displaced_retirements.reserve_owner_slots(ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY).expect("reserve the actual full destination"))
    } else {
        store.generation = fixture["generationMaximum"].as_str().unwrap().parse().unwrap();
        None
    };
    let before_generation = store.generation;
    let before_descriptor = serde_json::to_value(&store.envelope.backbone).unwrap();
    let before_revision = store.content_revision;
    let mut detached = None;
    let attempt = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        match store.detach_backbone() {
            Ok(owner) => { detached = owner; false }
            Err(_) => true,
        }
    }));
    let refused = matches!(attempt, Ok(true));
    let panicked = attempt.is_err();
    let descriptor_preserved = serde_json::to_value(&store.envelope.backbone).unwrap() == before_descriptor;
    let generation_preserved = store.generation == before_generation;
    let revision_preserved = store.content_revision == before_revision;
    let backbone_preserved = matches!(store.backbone.as_ref(), Some(Backbones::Memory(local)) if Arc::as_ptr(local.outbox.as_ref().unwrap()) == original_outbox);
    let payload_preserved = {
        let queue = peer.inbox.as_ref().unwrap().lock().unwrap();
        matches!(queue.front(), Some(BackboneMessage::Mutations { envelopes }) if envelopes.as_ptr() == expected_pointer && envelopes.capacity() == expected_capacity && envelopes.len() == payload_length && envelopes.iter().all(|byte| *byte == payload_byte))
    };
    if let Some(reservation) = occupied { store.displaced_retirements.release_owner_slots(reservation).expect("release exact fixture reservation"); }
    store.generation = original_generation;
    drop(peer);
    if let Some(owner) = detached.take() {
        let mut retirement = ArtifactStoreBackboneRetirement::new(owner);
        for _ in 0..4_096 {
            if retirement.terminal_is_empty() { break; }
            retirement.close_step(1, 17).expect("retire exact returned owner before assertions");
        }
        assert!(retirement.terminal_is_empty());
    }
    for _ in 0..4_096 {
        if SpaceMember::close_owned_terminal_is_empty(&store) { break; }
        SpaceMember::close_owned_step(&mut store, 1, 512).expect("close original Store before assertions");
    }
    assert!(SpaceMember::close_owned_terminal_is_empty(&store));
    eprintln!("[DEBUG] {case_id}: refused={refused} panicked={panicked} descriptor={descriptor_preserved} generation={generation_preserved} backbone={backbone_preserved} payload={payload_preserved} revision={revision_preserved}; original payload length={payload_length} capacity={expected_capacity}; cleanup terminal=true");
    assert_eq!((refused, panicked, descriptor_preserved, generation_preserved, backbone_preserved, payload_preserved, revision_preserved), (true, false, true, true, true, true, true), "{case_id}");
}

#[semio_framework_async_macros::async_test]
async fn backbone_detach_refusal_preserves_descriptor_and_payload_at_full_destination() {
    observe_refusal("occupied-owner-destination").await;
}

#[semio_framework_async_macros::async_test]
async fn backbone_detach_refusal_preserves_descriptor_and_payload_at_generation_overflow() {
    observe_refusal("generation-overflow").await;
}
//#endregion ✂️DetachRefusal
