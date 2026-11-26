//! 📮️ Original Store delivery through an attached actor Port.
use backbone::atomic_transport::AtomicBackboneDelivery;
use backbone::original_outbox::OriginalBackboneMessageBytes;
use semio_framework_value::{AuthoringWallet, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress};
use semio_os_spr::atomic_group::{ATOMIC_BACKBONE_PART_BYTES, AtomicBackbonePart, PREPARED_ATOMIC_EDIT_TAG};
use semio_os_store_testing_doubles::{DemoMemberStoreProducer, store::{ArtifactStore, close_demo_artifact_store}};
use semio_os_store_testing_fixtures::{DemoSnapshot, demo::DemoMutation};
use semio_os_store_testing_prelude::{BackboneMessage, Backbones, backbone, create_document_envelope};

fn funded<R>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> (R, u128) {
    let work = 1 << 20;
    AuthoringWallet::new(grant, u128::from(work)).run_invoked(grant, work, |outer| {
        let mut effect = outer.reborrow(grant, work).unwrap();
        let output = run(&mut effect);
        (output, effect.performed_work())
    }).unwrap().unwrap().output
}

#[semio_framework_async_macros::async_test]
async fn original_bound_document_uses_actor_order_and_retires_between_messages() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let field = |name: &str| fixture[name].as_u64().unwrap() as usize;
    let bytes = |name: &str| fixture[name].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect::<Vec<_>>();
    let ordinary = bytes("ordinary");
    let trailing = bytes("trailing");
    let mut atomic = PREPARED_ATOMIC_EDIT_TAG.to_vec();
    atomic.resize(field("atomicPayloadBytes"), field("atomicPayloadByte") as u8);
    assert!(atomic.len() > ATOMIC_BACKBONE_PART_BYTES);
    let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "bound-outbound", DemoSnapshot { n: Some(1) }, None, &semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &DemoMemberStoreProducer::<_>::new()).await;
    let uri = "actor://bound-outbound";
    let (channel, host) = backbone::ActorBackboneChannelOwner::pair(uri);
    *store.wire.backbone = Some(Backbones::Port(backbone::PortBackbone { uri: uri.into(), channel: Some(channel) }));
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1 << 17, maximum_release_bytes: 1 << 17, maximum_depth: 32 };
    for payload in [&ordinary, &atomic, &trailing] {
        let outbox = &mut store.wire.original_outbox;
        for _ in 0..field("maximumTurns") {
            if outbox.can_append(grant).unwrap() { break; }
            funded(grant, |effect| outbox.reserve_step(effect).unwrap());
        }
        let mut owner = Some(OriginalBackboneMessageBytes::try_from_slice(payload).unwrap());
        assert!(outbox.append_reserved(&mut owner, grant).unwrap().is_some());
    }
    let binding = [1; 32];
    let mut delivery: Option<AtomicBackboneDelivery> = None;
    let denied = RetainedCloneGrant { maximum_items: 0, ..grant };
    funded(denied, |effect| {
        assert_eq!(store.advance_bound_document_outbound(binding, &mut delivery, effect).unwrap(), Some(Default::default()));
    });
    assert!(delivery.is_none());
    for _ in 0..2 {
        funded(grant, |effect| store.advance_bound_document_outbound(binding, &mut delivery, effect).unwrap());
    }
    let (result, work) = funded(grant, |effect| store.advance_bound_document_outbound([2; 32], &mut delivery, effect));
    assert!(result.is_err());
    assert_eq!(work, 0);
    let cursor = delivery.as_mut().expect("actor wire is being built");
    assert!(!cursor.terminal_is_empty());
    assert!(host.take_outbound().unwrap().is_none());
    funded(grant, |effect| cursor.close(effect).unwrap());
    assert!(cursor.terminal_is_empty());
    delivery = None;
    assert!(host.take_outbound().unwrap().is_none());
    let mut messages = Vec::new();
    for _ in 0..field("maximumTurns") {
        if !store.original_document_outbound_pending() { break; }
        let (receipt, work) = funded(grant, |effect| store.advance_bound_document_outbound(binding, &mut delivery, effect).unwrap());
        if let Some(receipt) = receipt {
            assert!(receipt.fits(grant));
            assert!(receipt == RetainedCloneProgress::default() || work != 0);
        }
        while let Some(wire) = host.take_outbound().unwrap() {
            let BackboneMessage::Mutations { envelopes } = backbone::decode_hot_backbone_message_exact(&wire).unwrap() else { panic!("the Port sends mutation bodies") };
            messages.push(envelopes.to_vec_owner());
        }
    }
    assert!(delivery.is_none());
    assert!(!store.original_document_outbound_pending());
    assert!(store.wire.original_outbox.terminal_is_empty());
    let [first, part_a, part_b, last] = messages.as_slice() else { panic!("expected four FIFO frames, got {}", messages.len()) };
    assert_eq!(first, &ordinary);
    assert_eq!(last, &trailing);
    let (part_a, payload_a) = AtomicBackbonePart::parse(part_a).unwrap();
    let (part_b, payload_b) = AtomicBackbonePart::parse(part_b).unwrap();
    assert_eq!(part_a.key, part_b.key);
    let digest = part_a.key.digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(Some(digest.as_str()), fixture["atomicSha256"].as_str());
    assert_eq!([part_a.ordinal, part_b.ordinal], [0, 1]);
    assert_eq!([payload_a, payload_b].concat(), atomic);
    host.begin_retire().unwrap();
    close_demo_artifact_store(&mut store);
    assert!(store.close_owned_terminal_is_empty());
}
