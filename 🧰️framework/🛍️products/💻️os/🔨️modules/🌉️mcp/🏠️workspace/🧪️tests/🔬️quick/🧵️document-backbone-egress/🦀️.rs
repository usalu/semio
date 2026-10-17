/// 🧵️ Document backbone egress: the backbone effect addressed to this channel and the committed message without a document actor.

//#region 🧵️DocumentBackboneEgress

/// 🧵️ ticket 26/09/18 slice M10: a guest's committed envelopes leave the process on the DOCUMENT
/// BACKBONE, and the only messages this channel may relay are the ones addressed at its own actor
/// uri — the same ownership fence the wgpu shell's `route_document_backbone_effects` applies. A
/// shell reply, a topic publish, and another actor's backbone are all three not this document's
/// egress, and confusing any of them for it would relay one document's bytes into another's socket.
#[test]
fn only_a_backbone_effect_addressed_at_this_channel_is_document_egress() {
    let actor = "agent:test#sess";
    let backbone = |uri: &str| semio_framework::kernel::Effect::SendMessage {
        target: semio_framework::kernel::MessageEndpoint::Backbone { uri: uri.to_string() },
        payload: vec![7, 8, 9],
    };
    assert_eq!(document_backbone_payload(actor, &backbone(actor)), Some([7u8, 8, 9].as_slice()));
    assert_eq!(document_backbone_payload(actor, &backbone("agent:other#sess")), None, "another actor's backbone is another document's egress");
    let shell = semio_framework::kernel::Effect::SendMessage {
        target: semio_framework::kernel::MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId("0".to_string()) },
        payload: vec![7, 8, 9],
    };
    assert_eq!(document_backbone_payload(actor, &shell), None, "the shell reply lane is an answer, never document egress");
    let topic = semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Topic { name: actor.to_string() }, payload: vec![7, 8, 9] };
    assert_eq!(document_backbone_payload(actor, &topic), None, "a topic that happens to be named like the actor is not the backbone");
}

/// 🚧️ …and a committed message that cannot reach the hub is a typed, named fault, never a silent
/// drop. The guest has already committed by the time these bytes exist, so "the edit went nowhere"
/// has to be something the agent is told — with the REASON the document actor is missing, which for
/// a hub document is today the unregistered artifact codec.
#[test]
fn a_committed_backbone_message_with_no_document_actor_faults_with_its_reason() {
    let bound = unbound_plugin_artifacts();
    let router = RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), None, "agent:test#sess".to_string(), Arc::clone(&bound), None);
    let note = AppRoute { plugin_id: "note".to_string(), app_id: Some("note.editor".to_string()) };
    let unbound = router.relay_backbone_egress(&note, vec![vec![1, 2, 3]]).expect_err("no bound document at all");
    assert_eq!(unbound.code, "channel.not-wired");
    assert!(unbound.message.contains("has bound no document to it"), "{}", unbound.message);
    bound.lock().expect("binding map").insert(
        "hub-note".to_string(),
        PluginArtifactBinding {
            schema: "s.note.note".to_string(),
            plugin_id: "note".to_string(),
            app_id: "note.editor".to_string(),
            surface_id: Some("s.note.note@1/*#editor".to_string()),
            document: None,
            backbone: None,
            backbone_blocked_by: Some("no `store::os_store::serialization::ArtifactCodec` is registered for artifact schema `s.note.note`".to_string()),
            relayed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            relay: Arc::default(),
            used: 0,
        },
    );
    let blocked = router.relay_backbone_egress(&note, vec![vec![1, 2, 3]]).expect_err("bound, but no document actor");
    assert_eq!(blocked.code, "channel.not-wired");
    assert!(blocked.message.contains("hub-note") && blocked.message.contains("no `store::os_store::serialization::ArtifactCodec` is registered"), "the fault carries the binding's own recorded reason: {}", blocked.message);
}

//#endregion 🧵️DocumentBackboneEgress
