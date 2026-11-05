/// 🗂️ Guest document codec: the guest-backed codec refusals, the unregistered route and the two operations the WIT does not carry.


/// 🗂️ Guest codec thunks resolve registered components; invalid bytes produce typed refusals without clearing shared routes.
#[tokio::test]
async fn a_guest_backed_codec_with_no_registered_route_refuses_and_counts_its_candidates() {
    let refused = guest_print_mirror(b"not-a-pack", b"not-an-spr").await.expect_err("no component prints bytes that are not a pack");
    let store::VcsError::Deserialize(message) = &refused else { panic!("print-mirror must refuse by decode, got {refused:?}") };
    assert!(message.contains("no registered guest codec prints this pair") && message.contains("candidate(s)"), "{message}");

    let applied = guest_apply_ops_binary(b"not-a-pack", b"not-an-spr", b"").await.expect_err("no component applies a batch onto bytes that are not a pair");
    let store::VcsError::Deserialize(message) = &applied else { panic!("apply-ops must refuse by decode, got {applied:?}") };
    assert!(message.contains("no registered guest codec applies this batch") && message.contains("candidate(s)"), "{message}");
}

/// 🚫️ …and the two operations `interface codec` does NOT export refuse by naming exactly that,
/// rather than by inventing text no component ever produced. Both belong to the folder `.dsl`/`.ops`
/// lane, which a guest-backed codec exists precisely because a hub binding does not have.
#[tokio::test]
async fn a_guest_backed_codec_refuses_the_two_operations_the_wit_does_not_export() {
    let compiled = guest_compile_dsl("", "").await.expect_err("there is no codec.compile-dsl");
    let store::VcsError::Deserialize(message) = &compiled else { panic!("compile-dsl must refuse by decode, got {compiled:?}") };
    assert!(message.contains("has no `compile-dsl`") && message.contains("pack-schema-hash, genesis, print-mirror, apply-ops and replay-envelopes"), "{message}");

    let envelope = store::os_spr::MutationEnvelope {
        mutation_id: store::os_spr::MutationId("m1".to_string()),
        document_id: store::os_spr::ArtifactId("doc".to_string()),
        actor: store::os_spr::ActorId("agent:test#sess".to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: store::os_spr::ArtifactDiff { schema: store::os_spr::SchemaId("gis.map".to_string()), payload: Vec::new() },
        inverse: store::os_spr::InverseMutation { schema: store::os_spr::SchemaId("gis.map".to_string()), payload: Vec::new() },
        timestamp: store::os_spr::HybridLogicalTimestamp::new(1, 1),
        transaction: None, verb: None, line: None,
    };
    let printed = guest_edit_text_from_envelope(&envelope).await.expect_err("there is no per-envelope codec printer");
    let store::VcsError::Deserialize(message) = &printed else { panic!("edit-text must refuse by decode, got {printed:?}") };
    assert!(message.contains("prints whole pairs, not single edits"), "{message}");
}

