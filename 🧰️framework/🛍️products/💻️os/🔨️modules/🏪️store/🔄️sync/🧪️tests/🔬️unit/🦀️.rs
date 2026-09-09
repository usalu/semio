use super::*;
use crate::os_spr::{ArtifactId, Edit, Mutation, MutationComposition, MutationDiff, MutationDiffParticipation, MutationInvertibility, MutationLanguageSurface, MutationLeafDescriptor, MutationOutcomeClass, OpBinary, OpText};
use crate::os_store::{
    create_document_envelope, pack_rt, parse_document_pack, parse_document_text, print_document_pack, print_document_text, print_edit_lines, register_document_codec, ArtifactCodec, ArtifactCommand, ArtifactDsl, ArtifactPack, BlobStore,
    PackDecodeOptions, PackEncodeOptions, PackError, ParsedDocumentText,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

fn test_pool() -> Arc<semio_framework_async::WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<semio_framework_async::WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)))).clone()
}

fn document_backbone_envelope(id: &str, document_id: &str) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: MutationId(id.into()),
        document_id: ArtifactId(document_id.into()),
        actor: ActorId("actor-a".into()),
        dependencies: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".into()), payload: vec![1] },
        inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".into()), payload: vec![2] },
        timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 3, physical_ms: 9_007_199_254_740_992, logical: 5 },
    }
}

fn document_backbone_message(envelopes: &[MutationEnvelope]) -> Vec<u8> {
    BackboneMessage::Mutations { envelopes: encode_envelopes(envelopes) }.encode_op().expect("canonical document backbone message")
}

fn document_backbone_event_envelopes(event: &ArtifactEvent) -> Option<Vec<MutationEnvelope>> {
    let ArtifactEvent::DocumentBackbone { message } = event else { return None };
    decode_document_backbone_message_exact(message).ok()
}

#[semio_framework_async_macros::async_test]
async fn document_opening_attempt_wire_preserves_outer_owner_without_widening_actor_messages() {
    let attempt = "11111111-1111-4111-8111-111111111111";
    let request = backbone_worker_wire::BackboneWorkerRequest::Open { document_id: "document-a".into(), client_instance_id: Some(attempt.into()), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: Some(true), actor: "actor-a".into() };
    let wire = backbone_worker_wire::encode_request(&request).await.expect("encode exact opening owner");
    let decoded = backbone_worker_wire::decode_request(&wire).await.expect("decode exact opening owner");
    assert!(matches!(decoded, backbone_worker_wire::BackboneWorkerRequest::Open { client_instance_id: Some(owner), .. } if owner == attempt));

    let response = backbone_worker_wire::BackboneWorkerResponse::Event { document_id: "document-a".into(), client_instance_id: attempt.into(), event: ArtifactEvent::Status(ArtifactSyncStatus::default()) };
    let wire = backbone_worker_wire::encode_response(&response).await.expect("encode exact event owner");
    let decoded = backbone_worker_wire::decode_response(&wire).await.expect("decode exact event owner");
    assert!(matches!(decoded, backbone_worker_wire::BackboneWorkerResponse::Event { client_instance_id: owner, .. } if owner == attempt));
}

#[test]
fn artifact_mailbox_item_cap_plus_one_returns_exact_owner_and_preserves_fifo() {
    let (sender, receiver) = artifact_mailbox_pair();
    for seq in 0..ARTIFACT_MAILBOX_ITEMS as u64 {
        sender.send(ArtifactActorMsg::PublishPreview { key: format!("key-{seq}"), seq, payload: vec![seq as u8] }).expect("admit fixed mailbox owner");
    }
    let rejected = sender.send(ArtifactActorMsg::PublishPreview { key: "cap-plus-one".into(), seq: 256, payload: vec![7, 8] }).expect_err("item cap + 1 must reject");
    assert!(matches!(rejected.into_message(), ArtifactActorMsg::PublishPreview { key, seq: 256, payload } if key == "cap-plus-one" && payload == vec![7, 8]));
    for seq in 0..ARTIFACT_MAILBOX_ITEMS as u64 {
        assert!(matches!(receiver.try_recv(), Some(ArtifactActorMsg::PublishPreview { seq: actual, .. }) if actual == seq));
    }
    assert!(receiver.try_recv().is_none());
}

#[test]
fn artifact_mailbox_byte_cap_and_plus_one_preflight_before_mutation() {
    let exact_payload = vec![3; ARTIFACT_MAILBOX_BYTES - 17];
    let (exact_sender, exact_receiver) = artifact_mailbox_pair();
    exact_sender.send(ArtifactActorMsg::PublishPreview { key: String::new(), seq: 1, payload: exact_payload }).expect("exact byte cap admits");
    assert_eq!(exact_sender.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).bytes, ARTIFACT_MAILBOX_BYTES);
    assert!(matches!(exact_receiver.try_recv(), Some(ArtifactActorMsg::PublishPreview { seq: 1, .. })));

    let (overflow_sender, overflow_receiver) = artifact_mailbox_pair();
    let rejected = overflow_sender.send(ArtifactActorMsg::PublishPreview { key: String::new(), seq: 2, payload: vec![9; ARTIFACT_MAILBOX_BYTES - 16] }).expect_err("byte cap + 1 must reject");
    assert!(matches!(rejected.into_message(), ArtifactActorMsg::PublishPreview { seq: 2, payload, .. } if payload.len() == ARTIFACT_MAILBOX_BYTES - 16));
    assert!(overflow_receiver.try_recv().is_none(), "byte rejection cannot mutate FIFO state");
}

#[test]
fn document_backbone_mailbox_and_retention_are_exact_bounded_and_terminal() {
    let envelope = document_backbone_envelope("mutation-a", "document-a");
    let message = document_backbone_message(std::slice::from_ref(&envelope));
    let (sender, receiver) = artifact_mailbox_pair();
    sender.send(ArtifactActorMsg::DocumentBackbone { message: message.clone() }).expect("canonical mutation message is admitted");
    assert!(matches!(receiver.try_recv(), Some(ArtifactActorMsg::DocumentBackbone { message: actual }) if actual == message));
    let ack = BackboneMessage::Ack { op_ids: Vec::new() }.encode_op().expect("ack encodes");
    assert!(matches!(sender.send(ArtifactActorMsg::DocumentBackbone { message: ack }), Err(ArtifactMailboxSendError::Bytes { .. })), "host ingress refuses a Store acknowledgment");

    let mut retention = DocumentBackboneRetentionV1::default();
    retention.retain(message.len(), std::slice::from_ref(&envelope)).expect("first exact owner is retained");
    assert_eq!((retention.bytes, retention.messages, retention.entries.len()), (message.len(), 1, 1));
    assert!(retention.retain(message.len(), std::slice::from_ref(&envelope)).is_err(), "duplicate mutation identity cannot replace its owner");
    retention.release(std::slice::from_ref(&envelope));
    assert_eq!((retention.bytes, retention.messages, retention.entries.len()), (0, 0, 0));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn artifact_mailbox_wake_storm_coalesces_until_fifo_becomes_empty() {
    let (sender, receiver) = artifact_mailbox_pair();
    let wakes = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = wakes.clone();
    receiver.set_wake(Arc::new(move || {
        observed.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }));
    for _ in 0..ARTIFACT_MAILBOX_ITEMS {
        sender.send(ArtifactActorMsg::ExternalChanged).expect("wake-storm owner admitted");
    }
    assert_eq!(wakes.load(std::sync::atomic::Ordering::Acquire), 1);
    while receiver.try_recv().is_some() {}
    sender.send(ArtifactActorMsg::Detach).expect("new readiness edge admitted");
    assert_eq!(wakes.load(std::sync::atomic::Ordering::Acquire), 2);
}

#[test]
fn artifact_mailbox_stale_late_send_hands_back_exact_owner_and_interrupted_close_drains_one_per_grant() {
    let (sender, receiver) = artifact_mailbox_pair();
    for seq in 0..3 {
        sender.send(ArtifactActorMsg::PublishPreview { key: "close".into(), seq, payload: vec![seq as u8] }).expect("close fixture admission");
    }
    let close = receiver.close_handle();
    close.close();
    let late = sender.send(ArtifactActorMsg::PublishPreview { key: "late".into(), seq: 99, payload: vec![4, 5, 6] }).expect_err("late generation rejects");
    assert!(matches!(late, ArtifactMailboxSendError::Stale { message: ArtifactActorMsg::PublishPreview { key, seq: 99, payload } } if key == "late" && payload == vec![4, 5, 6]));
    for remaining in [2, 1, 0] {
        assert!(close.close_one());
        assert_eq!(close.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, remaining);
    }
    assert!(!close.close_one());
}

#[semio_framework_async_macros::async_test]
async fn artifact_mailbox_nested_identifier_bytes_and_backbone_one_pop_preserve_ownership_order() {
    use crate::os_store::Backbone;
    let nested = sample_presence_peer_with_interaction().await;
    let bare = PresencePeer {
        actor: nested.actor.clone(),
        connected_at_ms: nested.connected_at_ms,
        label: None,
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: None,
        color: None,
        surface: None,
        views: Vec::new(),
        ui: None,
    };
    let nested_bytes = artifact_actor_message_bytes(&ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(nested) }).expect("nested message fits");
    let bare_bytes = artifact_actor_message_bytes(&ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(bare) }).expect("bare message fits");
    assert!(nested_bytes > bare_bytes, "all nested identifiers and collections contribute byte credit");

    let (mut channel, remote) = ChannelBackbone::pair("store-sync-one-pop").await;
    channel.send(BackboneMessage::Ack { op_ids: vec!["first".into()] }).await.expect("first backbone owner");
    channel.send(BackboneMessage::Ack { op_ids: vec!["second".into()] }).await.expect("second backbone owner");
    assert!(matches!(remote.try_pop_front().expect("first opportunity"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["first"]));
    assert!(matches!(remote.try_pop_front().expect("second opportunity"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["second"]));
    assert!(remote.try_pop_front().expect("idle opportunity").is_none());
}

// 🎯️ `id` must be dotted `plugin.artifact` — `os_semio`'s preamble validator rejects a bare
// extension (this crate never compiled with `--features sync` before this packet, so the
// mismatch was never exercised at runtime). `extension_suffix` is the id's LAST segment, so
// `"demo.demo"` keeps `__DSL_EXTENSION` == "demo", unchanged from the old bare-extension form.
#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, crate::os_dsl::DslArtifact)]
#[dsl(id = "demo.demo")]
struct DemoSnapshot {
    n: i32,
}

impl ArtifactDsl for DemoSnapshot {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, crate::os_dsl::TextError> {
        let body = match semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Dsl, 1).expect("valid envelope_id");
        semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DemoSnapshot {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(|err| PackError::Schema(err.to_string()))
    }
    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
struct DemoDiff {
    n: Option<i32>,
}

impl MutationDiff<DemoSnapshot> for DemoDiff {
    fn apply(&self, snapshot: &DemoSnapshot) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
        Ok(DemoSnapshot { n: self.n.unwrap_or(snapshot.n) })
    }

    fn absorb(&mut self, other: Self) {
        if other.n.is_some() {
            self.n = other.n;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, crate::os_dsl::DslOps)]
#[serde(tag = "operation")]
#[value(tag = "operation")]
enum DemoMutation {
    #[dsl(key = "set-n")]
    SetN { n: i32 },
}

impl OpText for DemoMutation {
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

impl OpBinary for DemoMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (idx, (_, spec_fn)) = variants.iter().enumerate().find(|(_, (k, _))| k == &keyword).expect("variant spec must exist");
        let body = crate::os_pack::encode_record_body(&spec_fn(), &record, &PackEncodeOptions::default()).map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op pack", offset: 0, detail: e.to_string() })?;
        let mut out = Vec::with_capacity(2 + body.len());
        out.push(pack_rt::OP_BINARY_FORMAT);
        out.push(idx as u8);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: e.to_string() })?;
        if format != pack_rt::OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op binary format: {format}") });
        }
        let ordinal = reader.read_u8().map_err(|e| crate::os_spr::ProtocolError::Malformed { what: "op ordinal", offset: 1, detail: e.to_string() })?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or_else(|| crate::os_spr::ProtocolError::Malformed { what: "op ordinal", offset: 1, detail: format!("op ordinal {ordinal} out of range for {}", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let offset = reader.position() as u64;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset, detail: error.to_string() })
    }
}

impl Mutation<DemoSnapshot> for DemoMutation {
    type Diff = DemoDiff;
    const DESCRIPTORS: &'static [MutationLeafDescriptor] = &[MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync",
        semantic_kind: "set-n",
        display_name: "Set N",
        emoji: "🔢️",
        aggregate_variant: "SetN",
        payload_schema: "DemoMutation::SetN",
        text_opcode: Some("set-n"),
        binary_tag: Some(0),
        invertibility: MutationInvertibility::ExplicitMutation,
        diff_participation: MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[MutationOutcomeClass::Applied],
        composition: MutationComposition::Atomic,
        required_language_surfaces: &[MutationLanguageSurface::Rust, MutationLanguageSurface::Text, MutationLanguageSurface::Binary],
    }];

    fn descriptor(&self) -> &'static MutationLeafDescriptor {
        match self {
            Self::SetN { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, _snapshot: &DemoSnapshot) -> crate::os_spr::MutationOutcome<DemoDiff> {
        crate::os_spr::MutationOutcome::new(match self {
            DemoMutation::SetN { n } => DemoDiff { n: Some(*n) },
        })
    }

    fn inverse(&self, snapshot: &DemoSnapshot) -> Vec<Self> {
        vec![DemoMutation::SetN { n: snapshot.n }]
    }
}

/// @emoji 🎯️ Idempotently registers the `demo/v1` codec (process-global `OnceLock` registry,
/// shared across every test in this binary) — needed by any test exercising `FolderEndpoint`
/// end-to-end (both `Sqlite` and `Pack` now go through `document_codec` per the pack+spr flip),
/// mirroring a real app's program-init-time `register_document_codec_for_app` call.
async fn ensure_demo_codec_registered() {
    static ONCE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !ONCE.swap(true, std::sync::atomic::Ordering::AcqRel) {
        register_document_codec(ArtifactCodec::of::<DemoSnapshot, DemoMutation>("demo/v1")).expect("register demo codec");
    }
}

fn bootstrap_frontier(document_id: &str, ordinal: u64, edit_id: &str, commit: u64, chain: u8) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary { document_id: ArtifactId(document_id.into()), head_edit_ordinal: ordinal, head_edit_id: edit_id.into(), last_commit_seq: commit, chain_hash: [chain; 32] }
}

async fn demo_artifact_bootstrap(inline: bool) -> (ArtifactBootstrap, ArtifactBootstrapPair) {
    ensure_demo_codec_registered().await;
    let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 7 }, None);
    let files = print_document_pack(&envelope).await.expect("print bootstrap pair");
    let pair = ArtifactBootstrapPair { pack: files.pack, spr: files.spr };
    let pack_schema_hash = crate::os_store::document_codec("demo/v1").await.expect("codec lookup").expect("demo codec").pack_schema_hash;
    let bootstrap = ArtifactBootstrap {
        format_version: crate::os_spr::ARTIFACT_BOOTSTRAP_FORMAT_VERSION,
        descriptor_hash: [0x11; 32],
        artifact_schema: "demo/v1".into(),
        artifact_kind: "demo".into(),
        pack_schema_hash,
        baseline_frontier: bootstrap_frontier("demo", 7, "edit-7", 3, 0x33),
        pack_hash: semio_framework_hash::Sha256::digest(&pair.pack),
        spr_hash: semio_framework_hash::Sha256::digest(&pair.spr),
        pack_length: pair.pack.len() as u64,
        spr_length: pair.spr.len() as u64,
        chunk_count: if inline { 0 } else { 3 },
        aggregate_hash: crate::os_spr::artifact_bootstrap_aggregate_hash(&pair.pack, &pair.spr),
        required_tail_frontier: bootstrap_frontier("demo", 9, "edit-9", 4, 0x44),
        inline: inline.then(|| pair.clone()),
    };
    (bootstrap, pair)
}

#[test]
fn bootstrap_frontier_identity_rejects_same_ordinals_with_wrong_authenticated_head() {
    let required = bootstrap_frontier("demo", 9, "edit-9", 4, 0x44);
    let mut wrong_head = required.clone();
    wrong_head.head_edit_id = "edit-other".into();
    let mut wrong_chain = required.clone();
    wrong_chain.chain_hash = [0x55; 32];
    assert!(!frontier_reaches(&wrong_head, &required));
    assert!(!frontier_reaches(&wrong_chain, &required));
    assert!(frontier_reaches(&required, &required));
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn native_terminal_connection_failure_clears_receipt_actor_before_reissue() {
    use futures::StreamExt;
    use tokio_tungstenite::tungstenite::Message;

    async fn connect(actor: &mut native_actor::ArtifactActor, receipt_actor: &str) -> tokio_tungstenite::WebSocketStream<tokio::net::TcpStream> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind test socket");
        let url = format!("ws://{}", listener.local_addr().expect("test socket address"));
        let accepted = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept test socket");
            tokio_tungstenite::accept_async(stream).await.expect("upgrade test socket")
        });
        actor.connect_test_socket(&url, receipt_actor).await;
        accepted.await.expect("test socket task")
    }

    async fn receive_frame(socket: &mut tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) -> ClientFrame {
        let message = tokio::time::timeout(std::time::Duration::from_secs(1), socket.next()).await.expect("client frame deadline").expect("client frame owner").expect("client frame");
        let Message::Binary(bytes) = message else { panic!("expected binary client frame") };
        crate::os_spr::decode_client_frame(&bytes).await.expect("decode client frame").1
    }

    let (_, remote) = ChannelBackbone::pair("native-actor-epoch-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(8);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "local-only".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let stale = "hub.v1.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let fresh = "hub.v1.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    actor.install_test_socket_actor(stale);
    actor.inject_hub_frame(ServerFrame::Session { actor: stale.into(), color: 3 }).await;
    assert_eq!(actor.socket_epoch_test_state(), (Some(stale.into()), true, 0, Vec::new()));
    actor.fail_test_bootstrap().await;
    assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, Vec::new()));
    let first = sample_operation_envelope("after-bootstrap-failure", 1).await;
    let first_actor = first.actor.0.clone();
    actor.relay_test_envelope(first).await;
    assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, vec![first_actor.clone()]));

    actor.install_test_socket_actor(stale);
    actor.inject_hub_frame(ServerFrame::Session { actor: stale.into(), color: 4 }).await;
    actor.fail_test_connection().await;
    assert_eq!(actor.socket_epoch_test_state().0, None);
    assert!(!actor.socket_epoch_test_state().1);
    let second = sample_operation_envelope("after-eof", 2).await;
    let second_actor = second.actor.0.clone();
    actor.relay_test_envelope(second).await;
    assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, vec![first_actor.clone(), second_actor.clone()]));

    let mut socket = connect(&mut actor, fresh).await;
    assert!(matches!(receive_frame(&mut socket).await, ClientFrame::SocketHelloV1 { .. }));
    let before_session = sample_operation_envelope("before-fresh-session", 3).await;
    let before_session_actor = before_session.actor.0.clone();
    actor.relay_test_envelope(before_session).await;
    assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), false, 0, vec![first_actor, second_actor, before_session_actor]));
    assert!(tokio::time::timeout(std::time::Duration::from_millis(30), socket.next()).await.is_err(), "queued mutations cannot cross the socket before Session confirms the receipt actor");
    actor.inject_hub_frame(ServerFrame::Session { actor: fresh.into(), color: 5 }).await;
    let first_batch = receive_frame(&mut socket).await;
    let ClientFrame::Commands { batch_id, envelopes } = first_batch else { panic!("fresh Session must flush one command batch") };
    assert_eq!(envelopes.len(), 3);
    assert!(envelopes.iter().all(|envelope| envelope.actor.0 == fresh));
    assert!(tokio::time::timeout(std::time::Duration::from_millis(30), socket.next()).await.is_err(), "each queued mutation is sent exactly once after Session");
    assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), true, 1, Vec::new()));
    actor.inject_hub_frame(ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: bootstrap_frontier("demo", 3, "after-session", 3, 0x66) }).await;
    assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), true, 0, Vec::new()));

    actor.fail_test_connection().await;
    let reconnected = "hub.v1.cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
    let queued_during_reconnect = sample_operation_envelope("during-reconnect", 4).await;
    actor.relay_test_envelope(queued_during_reconnect).await;
    let mut reconnected_socket = connect(&mut actor, reconnected).await;
    assert!(matches!(receive_frame(&mut reconnected_socket).await, ClientFrame::SocketHelloV1 { .. }));
    assert!(tokio::time::timeout(std::time::Duration::from_millis(30), reconnected_socket.next()).await.is_err(), "reconnect cannot flush before its own Session");
    actor.inject_hub_frame(ServerFrame::Session { actor: reconnected.into(), color: 6 }).await;
    let ClientFrame::Commands { envelopes, .. } = receive_frame(&mut reconnected_socket).await else { panic!("reconnect Session must flush queued mutation") };
    assert_eq!(envelopes.len(), 1);
    assert_eq!(envelopes[0].actor.0, reconnected);
    assert!(tokio::time::timeout(std::time::Duration::from_millis(30), reconnected_socket.next()).await.is_err(), "reconnect flush is exactly once");
    assert_eq!(actor.socket_epoch_test_state(), (Some(reconnected.into()), true, 1, Vec::new()));

    actor.expire_test_socket_authority();
    let after_expiry = sample_operation_envelope("after-authority-expiry", 5).await;
    actor.relay_test_envelope(after_expiry).await;
    let (socket_actor, confirmed, pending, queued) = actor.socket_epoch_test_state();
    assert_eq!((socket_actor, confirmed, pending), (None, false, 0));
    assert_eq!(queued.len(), 2, "unacknowledged and post-expiry mutations stay queued for a fresh plan");
    let terminal = tokio::time::timeout(std::time::Duration::from_secs(1), reconnected_socket.next()).await.expect("expired authority closes promptly");
    assert!(!matches!(terminal, Some(Ok(Message::Binary(_)))), "expired plan authority cannot carry another command");
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn native_bootstrap_commits_pair_before_failed_local_replay_then_restarts_without_duplicate() {
    use crate::os_store::Backbone;
    let (bootstrap, pair) = demo_artifact_bootstrap(true).await;
    let required = bootstrap.required_tail_frontier.clone();
    let baseline = bootstrap.baseline_frontier.clone();
    let (mut channel, remote) = ChannelBackbone::pair("native-bootstrap-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, mut event_rx) = broadcast::channel(32);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "actor-bootstrap-test".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let local = sample_operation_envelope("pending-local", 8).await;
    actor.queue_test_outbox(vec![local.clone(), local.clone()]);
    actor.inject_bootstrap_local_replay_failure();
    let welcome =
        |bootstrap: ArtifactBootstrap| ServerFrame::Welcome { session_id: "session-bootstrap".into(), resume_token: "resume-bootstrap".into(), server_frontier: required.clone(), bootstrap: Bootstrap::ArtifactBootstrap(Box::new(bootstrap)) };

    actor.inject_hub_frame(welcome(bootstrap.clone())).await;
    let first = channel.receive().await.expect("baseline queue");
    assert_eq!(first.len(), 1, "failed replay queues only the committed baseline");
    assert!(matches!(&first[0], BackboneMessage::Snapshot { pack, spr } if pack == &pair.pack && spr == &pair.spr));
    let (pack, spr, frontier, pending_required, resume, pending_resume, remote_state, outbox) = actor.bootstrap_test_state();
    assert_eq!(pack.as_deref(), Some(pair.pack.as_slice()));
    assert_eq!(spr.as_deref(), Some(pair.spr.as_slice()));
    assert_eq!(frontier, Some(baseline.clone()));
    assert_eq!(pending_required, Some(required.clone()));
    assert_eq!(resume, None);
    assert_eq!(pending_resume.as_deref(), Some("resume-bootstrap"));
    assert!(!matches!(remote_state, RemoteState::Live { .. }));
    assert_eq!(outbox, vec![local.mutation_id.0.clone()], "failure preserves one deduplicated local owner");
    assert!(matches!(event_rx.try_recv(), Ok(ArtifactEvent::BootstrapProgress { .. })));

    actor.inject_hub_frame(ServerFrame::Presence { peers: Vec::new() }).await;
    assert!(!matches!(actor.bootstrap_test_state().6, RemoteState::Live { .. }), "presence cannot bypass authenticated catch-up");
    actor.inject_hub_frame(welcome(bootstrap)).await;
    let restarted = channel.receive().await.expect("restart queue");
    assert_eq!(restarted.iter().filter(|message| matches!(message, BackboneMessage::Snapshot { .. })).count(), 1);
    let replayed: Vec<MutationEnvelope> = restarted
        .iter()
        .filter_map(|message| match message {
            BackboneMessage::Mutations { envelopes } => Some(decode_envelopes(envelopes).expect("decode replay")),
            _ => None,
        })
        .flatten()
        .collect();
    assert_eq!(replayed.iter().map(|envelope| &envelope.mutation_id).collect::<Vec<_>>(), vec![&local.mutation_id], "restart performs one successful local replay");

    let mut wrong = required.clone();
    wrong.head_edit_id = "edit-wrong".into();
    wrong.chain_hash = [0x55; 32];
    actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: wrong }).await;
    assert!(!matches!(actor.bootstrap_test_state().6, RemoteState::Live { .. }), "same ordinals cannot authenticate a different chain");
    actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: required.clone() }).await;
    let (_, _, frontier, pending_required, resume, _, remote_state, outbox) = actor.bootstrap_test_state();
    assert_eq!(frontier, Some(required));
    assert_eq!(pending_required, None);
    assert_eq!(resume.as_deref(), Some("resume-bootstrap"));
    assert!(matches!(remote_state, RemoteState::Live { .. }));
    assert_eq!(outbox, vec![local.mutation_id.0], "offline hub retains the exact local owner after semantic replay");
    assert!(channel.receive().await.expect("no duplicate store replay").is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn native_inline_and_chunked_bootstrap_install_the_same_typed_pair_after_cancelled_restart() {
    use crate::os_store::Backbone;
    async fn actor_pair(uri: &str) -> (native_actor::ArtifactActor, ChannelBackbone) {
        let (channel, remote) = ChannelBackbone::pair(uri).await;
        let (_, receiver) = artifact_mailbox_pair();
        let (events, _) = broadcast::channel(32);
        let actor = native_actor::ArtifactActor::new(
            test_pool(),
            ArtifactActorConfig { document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "actor-bootstrap-test".into() },
            remote,
            receiver,
            events,
            Arc::new(std::sync::RwLock::new(None)),
            Arc::new(std::sync::RwLock::new(None)),
            None,
            semio_framework_async::CancelToken::root_now(),
        )
        .await;
        (actor, channel)
    }
    let (inline_bootstrap, pair) = demo_artifact_bootstrap(true).await;
    let (mut chunked_bootstrap, _) = demo_artifact_bootstrap(false).await;
    let required = inline_bootstrap.required_tail_frontier.clone();
    let welcome =
        |bootstrap: ArtifactBootstrap| ServerFrame::Welcome { session_id: "session-bootstrap".into(), resume_token: "resume-bootstrap".into(), server_frontier: required.clone(), bootstrap: Bootstrap::ArtifactBootstrap(Box::new(bootstrap)) };

    let (mut inline_actor, mut inline_channel) = actor_pair("native-bootstrap-inline").await;
    inline_actor.inject_hub_frame(welcome(inline_bootstrap)).await;
    let inline_messages = inline_channel.receive().await.expect("inline messages");
    assert!(matches!(&inline_messages[..], [BackboneMessage::Snapshot { pack, spr }] if pack == &pair.pack && spr == &pair.spr));

    let mut combined = pair.pack.clone();
    combined.extend_from_slice(&pair.spr);
    let chunk_size = combined.len().div_ceil(3);
    let chunks: Vec<Vec<u8>> = combined.chunks(chunk_size).map(<[u8]>::to_vec).collect();
    assert_eq!(chunks.len(), 3);
    chunked_bootstrap.chunk_count = chunks.len() as u32;
    let descriptor_hash = chunked_bootstrap.descriptor_hash;
    let (mut chunked_actor, mut chunked_channel) = actor_pair("native-bootstrap-chunked").await;
    chunked_actor.inject_hub_frame(welcome(chunked_bootstrap.clone())).await;
    chunked_actor.inject_hub_frame(ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index: 0, bytes: crate::os_spr::ArtifactBootstrapChunkBytes::try_from_slice(&chunks[0]).expect("bounded chunk") }).await;
    chunked_actor.cancel_test_bootstrap();
    assert!(chunked_channel.receive().await.expect("cancelled staging").is_empty(), "cancellation commits no partial pair");

    chunked_actor.inject_hub_frame(welcome(chunked_bootstrap)).await;
    for (index, chunk) in chunks.iter().enumerate() {
        chunked_actor.inject_hub_frame(ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index: index as u32, bytes: crate::os_spr::ArtifactBootstrapChunkBytes::try_from_slice(chunk).expect("bounded chunk") }).await;
        if index + 1 < chunks.len() {
            assert!(chunked_channel.receive().await.expect("staged chunk").is_empty(), "chunks stay invisible before done");
        }
    }
    chunked_actor.inject_hub_frame(ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count: chunks.len() as u32 }).await;
    let chunked_messages = chunked_channel.receive().await.expect("chunked messages");
    assert_eq!(chunked_messages, inline_messages, "inline and chunked replace with byte-identical typed pairs");
    chunked_actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: required.clone() }).await;
    let (actual_pack, actual_spr, frontier, pending_required, resume, _, remote_state, _) = chunked_actor.bootstrap_test_state();
    assert_eq!(actual_pack, Some(pair.pack));
    assert_eq!(actual_spr, Some(pair.spr));
    assert_eq!(frontier, Some(required));
    assert_eq!(pending_required, None);
    assert_eq!(resume.as_deref(), Some("resume-bootstrap"));
    assert!(matches!(remote_state, RemoteState::Live { .. }));
}

async fn sample_operation_envelope(edit_id: &str, n: i32) -> MutationEnvelope {
    let edit = Edit {
        id: edit_id.into(),
        actor: None,
        forwards: vec![DemoMutation::SetN { n }],
        inverse: vec![DemoMutation::SetN { n: 0 }],
        mutation_meta: Vec::new(),
        description: None,
        coalesce_key: None,
        sequence_number: 1,
        started_at: "0".into(),
        finished_at: None,
    };
    let document_id = ArtifactId("demo".to_string());
    let schema = crate::os_spr::SchemaId("demo/v1".to_string());
    let mut envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("operation envelope");
    envelopes.pop().expect("exactly one op envelope for a single-op edit")
}

//#region 🧪️SyncSession
#[semio_framework_async_macros::async_test]
async fn receive_materializes_remote_envelope_into_the_edit_timeline() {
    let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
    let store = ArtifactStore::new(envelope).await.expect("valid receive fixture");
    let mut session = SyncSession::new(store).await;
    session.receive(sample_operation_envelope("edit-1", 5).await).await.expect("receive");
    assert_eq!(session.store.snapshot().expect("snapshot").n, 5);
    assert_eq!(session.store.envelope().vcs.edits.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn receive_buffers_out_of_order_envelopes_until_dependencies_arrive() {
    let envelope: crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None);
    let store = ArtifactStore::new(envelope).await.expect("valid out-of-order fixture");
    let mut session = SyncSession::new(store).await;
    let first = sample_operation_envelope("edit-1", 5).await;
    let mut second = sample_operation_envelope("edit-2", 9).await;
    second.dependencies = vec![first.mutation_id.clone()];
    session.receive(second).await.expect("receive second first");
    assert_eq!(session.store.envelope().vcs.edits.len(), 0, "buffered until edit-1 arrives");
    session.receive(first).await.expect("receive first");
    assert_eq!(session.store.envelope().vcs.edits.len(), 2, "both edits now applied");
    assert_eq!(session.store.snapshot().expect("snapshot").n, 9);
}
//#endregion 🧪️SyncSession

//#region 🧪️Helpers
#[semio_framework_async_macros::async_test]
async fn hub_ws_url_derives_ws_endpoint_from_remote_uri() {
    assert_eq!(hub_ws_url("remote://host:6070", "studio-1", "doc-1", None).await, "ws://host:6070/spaces/studio-1/documents/doc-1/socket/v1");
    assert_eq!(hub_ws_url("https://semio_hub.example.com", "studio-1", "doc-2", None).await, "wss://semio_hub.example.com/spaces/studio-1/documents/doc-2/socket/v1");
    assert_eq!(hub_ws_url("ws://127.0.0.1:5000/prefix", "studio-1", "d", None).await, "ws://127.0.0.1:5000/spaces/studio-1/documents/d/socket/v1");
    assert_eq!(
        hub_ws_url("remote://host:6070", "studio /東京?", "doc#ä", Some("s.space.home@1/*#editor")).await,
        "ws://host:6070/spaces/studio%20%2F%E6%9D%B1%E4%BA%AC%3F/documents/doc%23%C3%A4/socket/v1?surface=s.space.home%401%2F%2A%23editor",
        "ticket 26/08/16/HUB-SPACES-…: surface travels out of band as ?surface= on the document WS URL"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn hostile_hub_binding_cannot_receive_a_credential_bound_document_grant() {
    use crate::os_directory::client::{DirectoryClientError, DocumentSocketAdmissionV1, DocumentSocketAuthorityV1, HubSocketGrantSource, LocalHubCredential, SocketGrantReceiptV1};
    use crate::os_directory::{
        DocumentOpenArtifactV1, DocumentOpenCatalogV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenRendererTargetV1, DocumentOpenRevalidationV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1, DocumentScope,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct BoundSource {
        admissions: AtomicUsize,
        trusted_origin: String,
    }

    impl HubSocketGrantSource for BoundSource {
        fn admit_document_socket(
            &self,
            _ctx: &semio_framework_async::OperationContext,
            space_id: &str,
            document_id: &str,
            expectation: &crate::os_directory::client::DocumentSocketExpectationV1,
            _client_instance_id: &str,
            _timeout_ms: u64,
        ) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
            self.admissions.fetch_add(1, Ordering::SeqCst);
            Ok(DocumentSocketAdmissionV1 {
                socket: SocketGrantReceiptV1 {
                    schema: "semio.hub.socket-grant/v1".into(),
                    protocol: "semio.socket.v1".into(),
                    grant: format!("socket.v1.{}.{}", "1".repeat(32), "2".repeat(64)),
                    actor_id: format!("hub.v1.{}", "3".repeat(64)),
                    expires_at_ms: i64::MAX,
                },
                authority: DocumentSocketAuthorityV1 {
                    admitted_lease: expectation.lease.clone(),
                    hub_origin: self.trusted_origin.clone(),
                    browser_actor: crate::os_directory::DocumentOpenBrowserActorV1::None,
                    expires_at_unix_ms: u64::try_from(i64::MAX).expect("positive max"),
                    scope: DocumentScope::new(space_id, document_id),
                    descriptor_digest_v1: "4".repeat(64),
                    catalog: DocumentOpenCatalogV1 { generation_id: "5".repeat(64) },
                    package: DocumentOpenPackageV1 {
                        plugin_id: "trusted.plugin".into(),
                        package_id: "trusted.package".into(),
                        version: "1.0.0".into(),
                        component_sha256: "6".repeat(64),
                        component_blake3: "7".repeat(64),
                        descriptor_byte_sha256: "8".repeat(64),
                        execution_protocol: crate::os_directory::DocumentExecutionProtocolV1 { app_channel_version: crate::os_spr::CHANNEL_VERSION },
                    },
                    artifact: DocumentOpenArtifactV1 { kind: "trusted.document".into(), schema: expectation.artifact_schema.clone(), pack_schema_hash: "1".repeat(64) },
                    parent_dialect: DocumentOpenParentDialectV1 { artifact_kind: "trusted.document".into(), standard: "1".into(), subset: "*".into() },
                    pack_schema_hash: [0x11; 32],
                    surface: DocumentOpenSurfaceV1 {
                        surface_id: "trusted.surface".into(),
                        app_id: "trusted.app".into(),
                        window_kind_id: "trusted.window".into(),
                        role: DocumentOpenSurfaceRoleV1::Editor,
                        renderer_target: DocumentOpenRendererTargetV1::Wgpu,
                    },
                    grant: DocumentOpenGrantV1 { read: true, write: true, observe: true },
                    checkpoint: crate::os_directory::DocumentOpenCheckpointV1 {
                        checkpoint_id: "9".repeat(64),
                        descriptor_digest_v1: "4".repeat(64),
                        baseline_frontier: crate::os_directory::ArtifactFrontier {
                            document_id: document_id.to_string(),
                            head_edit_ordinal: 0,
                            head_edit_id: String::new(),
                            last_commit_seq: 0,
                            chain_hash: crate::os_directory::ArtifactHash::new([0; 32]),
                        },
                        aggregate_sha256: "a".repeat(64),
                    },
                    revalidation: DocumentOpenRevalidationV1 { directory_revision: 1, membership_generation: 1, session_generation: Some(1), share_generation: None },
                },
            })
        }
    }

    let hostile = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("hostile listener");
    let hostile_origin = format!("http://{}", hostile.local_addr().expect("hostile address"));
    let source = Arc::new(BoundSource { admissions: AtomicUsize::new(0), trusted_origin: "http://127.0.0.1:1".into() });
    let (_, remote) = ChannelBackbone::pair("hostile-binding-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(8);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig {
            document_id: "document".into(),
            schema: "demo/v1".into(),
            bindings: vec![PersistenceBinding::Hub { base_url: hostile_origin, space_id: "space".into(), surface: Some("trusted.surface".into()) }],
            watch_external: false,
            actor: "local-untrusted".into(),
        },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(Some(Arc::new(LocalHubCredential::test("http://127.0.0.1:1", &format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64))))))),
        Arc::new(std::sync::RwLock::new(Some(source.clone()))),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    actor.run_test_connect_attempt().await;
    assert_eq!(source.admissions.load(Ordering::SeqCst), 1);
    assert!(tokio::time::timeout(std::time::Duration::from_millis(50), hostile.accept()).await.is_err(), "hostile binding receives no dial or grant header");
    assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, Vec::new()));
}

/// 🎨️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4: both
/// actors call `stamp_session` on every outbound `PresenceHeartbeat` right before
/// `presence_to_bytes` — shells never fill `color`/`surface` themselves. This proves the pure
/// helper both actors call: sets both fields from the actor's own `session_color`/`hub_surface`
/// state, overwriting whatever the shell handed in, and clears `surface` to `None` when the
/// document has no hub binding (folder-only) even if `session_color` is somehow set.
#[semio_framework_async_macros::async_test]
async fn actor_stamps_session_color_and_surface_on_outbound_heartbeat() {
    let mut peer = PresencePeer {
        actor: "actor-1".into(),
        connected_at_ms: 1000,
        label: None,
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: None,
        color: Some(99),
        surface: Some("shell-should-never-set-this".into()),
        views: Vec::new(),
        ui: None,
    };
    stamp_session(&mut peer, Some(7), Some("s.space.home@1/*#editor")).await;
    assert_eq!(peer.color, Some(7));
    assert_eq!(peer.surface.as_deref(), Some("s.space.home@1/*#editor"));

    stamp_session(&mut peer, None, None).await;
    assert_eq!(peer.color, None, "no session color yet (folder-only document, or hub not yet assigned one)");
    assert_eq!(peer.surface, None, "folder-only document carries no surface");
}
//#endregion 🧪️Helpers

//#region 🧪️WireBridge
// 🎯️ W6: `wire_bridge_round_trips_identity_and_diff_through_protocol_causal` is DELETED — the
// local/wire bridge it tested (`to_wire_envelope`/`from_wire_envelope`) no longer exists; local
// and wire envelopes are the same `crate::os_spr::MutationEnvelope` type now, an identity the type
// system enforces, not something a round-trip test needs to prove.
#[semio_framework_async_macros::async_test]
async fn rollback_envelope_synthesizes_an_undo_from_the_original_inverse() {
    let envelope = sample_operation_envelope("edit-1", 5).await;
    let rollback = rollback_envelope(&envelope).await;
    assert_eq!(rollback.dependencies, vec![envelope.mutation_id.clone()], "the undo depends on the operation it undoes");
    assert_eq!(rollback.diff.payload, envelope.inverse.payload, "the undo's forward diff IS the original's inverse");
    assert_ne!(rollback.mutation_id, envelope.mutation_id, "the undo gets its own operation id");
}

/// 🎬️ Compares nineteen committed wire specimens with Rust encoding and decoding without
/// modifying them. The TypeScript replication suite consumes the same semantic-owner files
/// under `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/📡️wire`.
/// The current socket hello round-trips in memory; the committed obsolete hello must reject.
#[semio_framework_async_macros::async_test]
async fn wire_fixtures_stay_byte_identical_across_rust_and_ts() {
    let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🔨️modules/📡️replication/🧫️fixtures/📡️wire");

    async fn check_client(dir: &std::path::Path, name: &str, frame: &ClientFrame, lane: Lane) {
        let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|error| panic!("read {name}: {error}"));
        assert_eq!(encode_client_frame(frame, lane).await, bytes, "{name} committed bytes");
        let (decoded_lane, decoded) = crate::os_spr::decode_client_frame(&bytes).await.unwrap_or_else(|error| panic!("decode {name}: {error}"));
        assert_eq!(decoded_lane, lane, "{name} lane round trip");
        assert_eq!(&decoded, frame, "{name} frame round trip");
    }

    async fn check_server(dir: &std::path::Path, name: &str, frame: &ServerFrame, lane: Lane) {
        let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|error| panic!("read {name}: {error}"));
        assert_eq!(crate::os_spr::encode_server_frame(frame, lane).await, bytes, "{name} committed bytes");
        let (decoded_lane, decoded) = decode_server_frame(&bytes).await.unwrap_or_else(|error| panic!("decode {name}: {error}"));
        assert_eq!(decoded_lane, lane, "{name} lane round trip");
        assert_eq!(&decoded, frame, "{name} frame round trip");
    }

    let frontier = RuntimeFrontierSummary { document_id: ArtifactId("doc-1".to_string()), head_edit_ordinal: 1, head_edit_id: "op-1".to_string(), last_commit_seq: 1, chain_hash: [9u8; 32] };
    let wire_envelope = MutationEnvelope {
        mutation_id: MutationId("op-1".to_string()),
        document_id: ArtifactId("doc-1".to_string()),
        actor: ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
        inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 0 }).expect("encode demo op") },
        timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 42, physical_ms: 1000, logical: 0 },
    };

    //#region 🔖️ClientFrame
    let hello = ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "demo/v1".to_string(), pack_schema_hash: [7u8; 32], resume_token: None, frontier: None };
    let hello_bytes = encode_client_frame(&hello, Lane::Command).await;
    let (hello_lane, decoded_hello) = crate::os_spr::decode_client_frame(&hello_bytes).await.expect("decode current socket hello");
    assert_eq!(hello_lane, Lane::Command, "current socket hello lane round trip");
    assert_eq!(decoded_hello, hello, "current socket hello frame round trip");
    let rejected_hello = std::fs::read(fixtures_dir.join("🚫️legacy-client-hello-rejected/💾️.bin")).expect("read rejected obsolete hello");
    assert!(crate::os_spr::decode_client_frame(&rejected_hello).await.is_err(), "obsolete hello must reject");
    check_client(&fixtures_dir, "🕹️client-commands/💾️.bin", &ClientFrame::Commands { batch_id: 1, envelopes: vec![wire_envelope.clone()] }, Lane::Command).await;
    check_client(&fixtures_dir, "🚩️client-frontier-advertise/💾️.bin", &ClientFrame::FrontierAdvertise { frontier: frontier.clone() }, Lane::Command).await;
    check_client(&fixtures_dir, "📣️client-preview-publish/💾️.bin", &ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview).await;
    check_client(&fixtures_dir, "🙋️client-presence/💾️.bin", &ClientFrame::Presence { peer: presence_to_bytes(&sample_presence_peer_with_interaction().await).await }, Lane::Preview).await;
    check_client(&fixtures_dir, "🎟️client-credit-grant/💾️.bin", &ClientFrame::CreditGrant { n: 16 }, Lane::Command).await;
    check_client(&fixtures_dir, "👋️client-bye/💾️.bin", &ClientFrame::Bye, Lane::Command).await;
    //#endregion 🔖️ClientFrame

    //#region 🔖️ServerFrame
    check_server(&fixtures_dir, "🔗️server-welcome-tail/💾️.bin", &ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail }, Lane::Command).await;
    check_server(
        &fixtures_dir,
        "📸️server-welcome-snapshot-inline/💾️.bin",
        &ServerFrame::Welcome { session_id: "session-2".to_string(), resume_token: "resume-2".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: Some(vec![9, 9, 9]) } },
        Lane::Command,
    )
    .await;
    check_server(&fixtures_dir, "🧩️server-snapshot-chunk/💾️.bin", &ServerFrame::SnapshotChunk { seq: 0, bytes: crate::os_spr::SnapshotChunkBytes::try_from_slice(&[1, 2, 3, 4]).unwrap() }, Lane::Command).await;
    check_server(&fixtures_dir, "🏁️server-snapshot-done/💾️.bin", &ServerFrame::SnapshotDone { seq_count: 4 }, Lane::Command).await;
    check_server(&fixtures_dir, "🎮️server-commands/💾️.bin", &ServerFrame::Commands { envelopes: vec![wire_envelope], origin: ActorId("actor-1".to_string()), frontier: frontier.clone() }, Lane::Command).await;
    check_server(
        &fixtures_dir,
        "✅️server-ack-accepted/💾️.bin",
        &ServerFrame::Ack { batch_id: 1, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: frontier.clone() },
        Lane::Command,
    )
    .await;
    check_server(
        &fixtures_dir,
        "🔀️server-ack-transformed/💾️.bin",
        &ServerFrame::Ack {
            batch_id: 2,
            stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Transformed { envelope: Box::new(sample_wire_envelope_for_fixtures().await) }) }],
            frontier: frontier.clone(),
        },
        Lane::Command,
    )
    .await;
    check_server(
        &fixtures_dir,
        "⛔️server-ack-rejected/💾️.bin",
        &ServerFrame::Ack { batch_id: 3, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "conflict".to_string(), messages: vec![1, 2, 3] }) }], frontier: frontier.clone() },
        Lane::Command,
    )
    .await;
    check_server(&fixtures_dir, "👁️server-preview/💾️.bin", &ServerFrame::Preview { actor: ActorId("actor-1".to_string()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview).await;
    check_server(&fixtures_dir, "👥️server-presence/💾️.bin", &ServerFrame::Presence { peers: vec![b"{\"id\":\"a\"}".to_vec(), presence_to_bytes(&sample_presence_peer_with_interaction().await).await] }, Lane::Preview).await;
    check_server(&fixtures_dir, "🎫️server-credit-grant/💾️.bin", &ServerFrame::CreditGrant { n: 32 }, Lane::Command).await;
    check_server(&fixtures_dir, "🚨️server-error/💾️.bin", &ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command).await;
    check_server(&fixtures_dir, "🪪️server-session/💾️.bin", &ServerFrame::Session { actor: "actor-1".to_string(), color: 5 }, Lane::Command).await;
    //#endregion 🔖️ServerFrame
}

/// 🧸️ A second, distinct `MutationEnvelope` for `🔀️server-ack-transformed/💾️.bin`'s
/// `ApplyOutcome::Transformed` payload — must differ from the primary `wire_envelope` fixture so
/// the vitest canary can assert it decodes as its own value, not an accidental copy.
async fn sample_wire_envelope_for_fixtures() -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: MutationId("op-2".to_string()),
        document_id: ArtifactId("doc-1".to_string()),
        actor: ActorId("actor-2".to_string()),
        dependencies: vec![MutationId("op-1".to_string())],
        diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 6 }).expect("encode demo op") },
        inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
        timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 42, physical_ms: 1001, logical: 0 },
    }
}

/// 🕹️ A `PresencePeer` whose `interaction` carries THREE domains (one selection-only, one
/// hover-only, one with both), TWO `views` (one Orbit with a pointer, one Canvas), a `color` +
/// `surface`, and a `ui` — `🙋️client-presence/💾️.bin`/`👥️server-presence/💾️.bin` match this so
/// the TS vitest twin exercises every `PresencePeer` v3 flag bit (§C7.1) with a realistic payload.
async fn sample_presence_peer_with_interaction() -> PresencePeer {
    PresencePeer {
        actor: "actor-1".to_string(),
        connected_at_ms: 1_700_000_000_000,
        label: Some("Ada".to_string()),
        presence_pack: None,
        user_id: Some("user-9".to_string()),
        role: Some("owner".to_string()),
        drag_ghost_json: None,
        interaction: Some(crate::os_spr::PresenceInteraction {
            app_id: "space".to_string(),
            domains: vec![
                crate::os_spr::PresenceDomain { domain: "outline".to_string(), granularity: "task".to_string(), selected: vec!["t1".to_string(), "t2".to_string()], hovered: vec![] },
                crate::os_spr::PresenceDomain { domain: "board".to_string(), granularity: "card".to_string(), selected: vec![], hovered: vec!["c1".to_string()] },
                crate::os_spr::PresenceDomain { domain: "canvas".to_string(), granularity: "node".to_string(), selected: vec!["n9".to_string()], hovered: vec!["n9".to_string(), "n10".to_string()] },
            ],
        }),
        color: Some(5),
        surface: Some("s.space.home@1/*#editor".to_string()),
        views: vec![
            crate::os_spr::PresenceWindowView {
                window_id: "w1".to_string(),
                space: "world".to_string(),
                kind: crate::os_spr::PresenceViewKind::Orbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 1.0, 0.0], fov: 45.0 },
                size: [1024.0, 768.0],
                pointer: Some([0.5, 0.5, 0.5]),
            },
            crate::os_spr::PresenceWindowView { window_id: "w2".to_string(), space: "canvas".to_string(), kind: crate::os_spr::PresenceViewKind::Canvas { x: 12.5, y: -4.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None },
        ],
        ui: Some(crate::os_spr::PresenceUi { hovered_path: Some("row[2]#t1".to_string()), focused_path: None, pressed_path: None }),
    }
}
//#endregion 🧪️WireBridge

// 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
// `assemble_presence_interaction` and its `🧪️PresenceInteraction` tests MOVED to
// `crate::os_spr::wire`'s `🔖️PresenceInteraction` region (`assemble_presence_interaction_tests`
// module there) alongside the function itself — see this file's `🔖️WireBridge` region for the
// pointer left behind.

//#region 🧪️PresenceInteraction
#[semio_framework_async_macros::async_test]
async fn presence_heartbeat_producer_publishes_immediately_then_coalesces_to_latest() {
    let mut producer = PresenceHeartbeatProducer::new(100);
    let mut first = sample_presence_peer_with_interaction().await;
    first.views = vec![crate::os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: crate::os_spr::PresenceViewKind::Canvas { x: 1.0, y: 2.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None }];
    assert_eq!(producer.offer(1_000, first.clone()), Some(first));

    let mut intermediate = sample_presence_peer_with_interaction().await;
    intermediate.views = vec![crate::os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: crate::os_spr::PresenceViewKind::Canvas { x: 3.0, y: 4.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None }];
    assert_eq!(producer.offer(1_040, intermediate), None);

    let mut latest = sample_presence_peer_with_interaction().await;
    latest.views = vec![crate::os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: crate::os_spr::PresenceViewKind::Canvas { x: 5.0, y: 6.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None }];
    assert_eq!(producer.offer(1_099, latest.clone()), None);
    assert_eq!(producer.pending(), Some(&latest));
    assert_eq!(producer.offer(1_100, latest.clone()), Some(latest));
    assert!(producer.pending().is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn artifact_host_presence_heartbeat_owns_cadence_per_document() {
    let host = ArtifactHost::new(test_pool());
    let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(1);
    let runner = native_actor::retained_turn_fixtures::fixture_runner_handle(host.pool.clone(), 1, cmd_rx.close_handle());
    host.inner.lock().unwrap().documents.insert(ArtifactDocumentKey::local("doc"), OpenDocument { generation: 1, cancel: semio_framework_async::CancelToken::root_now(), cmd_tx, events, presence: PresenceHeartbeatProducer::default(), runner });

    let first = sample_presence_peer_with_interaction().await;
    assert!(host.presence_heartbeat("doc", 500, first.clone()));
    assert!(matches!(cmd_rx.try_recv(), Some(ArtifactActorMsg::PresenceHeartbeat { peer }) if *peer == first));

    let mut latest = sample_presence_peer_with_interaction().await;
    latest.ui = Some(crate::os_spr::PresenceUi { hovered_path: Some("row[0]#changed".into()), focused_path: None, pressed_path: None });
    assert!(!host.presence_heartbeat("doc", 550, latest.clone()));
    assert!(cmd_rx.try_recv().is_none(), "sub-interval offer must not publish");
    assert!(host.presence_heartbeat("doc", 600, latest.clone()));
    assert!(matches!(cmd_rx.try_recv(), Some(ArtifactActorMsg::PresenceHeartbeat { peer }) if *peer == latest));
    assert!(!host.presence_heartbeat("missing", 700, sample_presence_peer_with_interaction().await));
}
//#endregion 🧪️PresenceInteraction

//#region 🧪️Helpers

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn op_envelope_from_stored_edit_round_trips_through_ingest() {
    let edit = crate::os_spr::HistoryEdit {
        id: "ext-1".into(),
        actor: Some("peer".into()),
        started_at: "0".into(),
        finished_at: None,
        coalesce_key: None,
        description: None,
        ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
        inverse: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 0 }.encode_op().expect("encode")) }],
        meta: None,
    };
    let envelopes = envelopes_from_history_edit(&edit, "demo", "demo/v1").await.expect("envelopes from history edit");
    assert_eq!(envelopes.len(), 1, "single-op edit yields one envelope");
    assert_eq!(envelopes[0].mutation_id.0, "ext-1#0", "meta-less fallback: edit id # op index");
    let recovered = <DemoMutation as OpBinary>::decode_op(&envelopes[0].diff.payload).expect("decode op");
    assert_eq!(recovered, DemoMutation::SetN { n: 42 });
}
//#endregion 🧪️Helpers

//#region 🧪️Actor
#[cfg(not(target_arch = "wasm32"))]
mod actor_tests {
    use super::*;
    use crate::os_spr::{decode_client_frame, encode_server_frame};
    use futures::{SinkExt, StreamExt};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::{broadcast as tokio_broadcast, Mutex};
    use tokio_tungstenite::tungstenite::Message as WsMessage;

    async fn demo_envelope(document_id: &str) -> crate::os_store::ArtifactEnvelope<DemoSnapshot, DemoMutation> {
        create_document_envelope("demo/v1", document_id, DemoSnapshot { n: 0 }, None)
    }

    async fn wait_until<Fut>(label: &str, mut predicate: impl FnMut() -> Fut)
    where
        Fut: std::future::Future<Output = bool>,
    {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        while !predicate().await {
            if tokio::time::Instant::now() >= deadline {
                panic!("{label} not satisfied before 5s deadline");
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn wait_until_value<T, Fut>(label: &str, mut predicate: impl FnMut() -> Fut) -> T
    where
        Fut: std::future::Future<Output = Option<T>>,
    {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(value) = predicate().await {
                return value;
            }
            if tokio::time::Instant::now() >= deadline {
                panic!("{label} not satisfied before 5s deadline");
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn wait_for_event(events: &mut broadcast::Receiver<ArtifactEvent>, predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        wait_for_named_event("event", events, predicate).await
    }

    async fn wait_for_named_event(label: &str, events: &mut broadcast::Receiver<ArtifactEvent>, mut predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            match tokio::time::timeout_at(deadline, events.recv()).await {
                Ok(Ok(event)) => {
                    if predicate(&event) {
                        return event;
                    }
                }
                Ok(Err(broadcast::error::RecvError::Lagged(_))) => continue,
                other => panic!("no matching {label} before deadline: {other:?}"),
            }
        }
    }

    async fn wait_for_mock_hub_event(label: &str, hub: &MockHub, events: &mut broadcast::Receiver<ArtifactEvent>, mut predicate: impl FnMut(&ArtifactEvent) -> bool) -> ArtifactEvent {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        let mut observed = Vec::new();
        loop {
            match tokio::time::timeout_at(deadline, events.recv()).await {
                Ok(Ok(event)) => {
                    if predicate(&event) {
                        return event;
                    }
                    observed.push(format!("{event:?}"));
                }
                Ok(Err(broadcast::error::RecvError::Lagged(count))) => observed.push(format!("lagged:{count}")),
                other => panic!("no matching {label} before deadline: {other:?}; actor-events={observed:?}; mock-hub={:?}", hub.trace()),
            }
        }
    }

    // 🔬️ External folder edit → RemoteMutations event + the store timeline grows on tick().
    #[tokio::test]
    async fn folder_external_edit_delivers_remote_operations() {
        ensure_demo_codec_registered().await;
        let dir = crate::os_store::test_support::tempdir().expect("tempdir");
        let host = ArtifactHost::new(test_pool());
        let channels = host.open(ArtifactActorConfig { document_id: "doc-a".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() }).await;
        let mut events = host.subscribe("doc-a").await;
        let mut store = ArtifactStore::new(demo_envelope("doc-a").await).await.expect("valid actor fixture");
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");

        // A local apply establishes a persisted edit on disk.
        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply");
        channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

        // Wait until the actor has persisted the local edit to the folder db as real pack+spr bytes.
        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
        let (pack, spr) = wait_until_value("persisted edit on disk", || async {
            let (pack, spr) = storage.read("doc-a").await.expect("read")?;
            if spr_op_ids(&spr).await.ok()?.is_empty() {
                None
            } else {
                Some((pack, spr))
            }
        })
        .await;

        // Out-of-band: append a foreign edit directly to the spr bytes (real binary op
        // payloads, no codec, no JSON) before writing pack+spr back.
        let external_edit = crate::os_spr::HistoryEdit {
            id: "external-1".into(),
            actor: Some("peer".into()),
            started_at: "0".into(),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
            inverse: vec![crate::os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 1 }.encode_op().expect("encode")) }],
            meta: None,
        };
        let new_spr = crate::os_store::append_history_edits_to_spr(&spr, &[external_edit]).await.expect("append external edit");
        storage.write("doc-a", "demo/v1", &pack, &new_spr).await.expect("out-of-band write");

        // Deterministically poke the actor to re-read (notify also wired, but timing-independent here).
        channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");

        let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::RemoteMutations { .. })).await;
        match event {
            ArtifactEvent::RemoteMutations { envelopes } => {
                assert_eq!(envelopes.len(), 1);
                assert_eq!(envelopes[0].mutation_id.0, "external-1#0", "single-op edit -> mutation_id is edit.id#0 (crate::os_spr::mutation_envelope_from_edit's ordinal-suffix convention)");
            }
            other => panic!("expected RemoteMutations, got {other:?}"),
        }

        // The store ingests the pushed operation on tick(); the timeline grows and snapshot updates.
        store.tick().await.expect("tick");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "external edit joined the timeline");
        assert_eq!(store.snapshot().expect("snapshot").n, 42);
        host.close("doc-a");
    }

    //#region 🔖️MockHub
    /// @emoji 🧪️ A minimal in-process semio_hub speaking the real, binary `crate::os_spr::wire::ClientFrame`/
    /// `ServerFrame` protocol, so the semio_hub endpoint is exercised end-to-end without linking a real
    /// `db`-backed semio_hub (that's CW6's job — this mock never touches `db`). Ordinal-indexed log,
    /// mirroring `db_sync`'s replica-catch-up shape (`Hello.frontier` -> filtered backlog ->
    /// `Welcome` then a follow-up `Commands`), but with a placeholder `chain_hash`/`resume_token`
    /// (this mock has no durable log to derive a real chain hash from).
    struct MockHub {
        log: Arc<Mutex<Vec<(u64, MutationEnvelope)>>>,
        broadcast: tokio_broadcast::Sender<ServerFrame>,
        connections: AtomicUsize,
        session_gate: Option<Arc<tokio::sync::Semaphore>>,
        trace: Arc<std::sync::Mutex<Vec<&'static str>>>,
    }

    impl MockHub {
        fn record(&self, event: &'static str) {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(event);
        }

        fn trace(&self) -> Vec<&'static str> {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
        }
    }

    struct MockHubSocketGrantSource {
        hub_origin: String,
        actor_id: String,
        grant_fill: char,
        trace: Arc<std::sync::Mutex<Vec<&'static str>>>,
    }

    impl crate::os_directory::client::HubSocketGrantSource for MockHubSocketGrantSource {
        fn admit_document_socket(
            &self,
            _ctx: &semio_framework_async::OperationContext,
            space_id: &str,
            document_id: &str,
            expectation: &crate::os_directory::client::DocumentSocketExpectationV1,
            _client_instance_id: &str,
            _timeout_ms: u64,
        ) -> Result<crate::os_directory::client::DocumentSocketAdmissionV1, crate::os_directory::client::DirectoryClientError> {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("grant.entered");
            let expires_at_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("system clock after epoch").as_millis().saturating_add(60_000).min(i64::MAX as u128) as i64;
            let admission = crate::os_directory::client::DocumentSocketAdmissionV1 {
                socket: crate::os_directory::client::SocketGrantReceiptV1 {
                    schema: "semio.hub.socket-grant/v1".into(),
                    protocol: "semio.socket.v1".into(),
                    grant: format!("socket.v1.{}.{}", self.grant_fill.to_string().repeat(32), self.grant_fill.to_string().repeat(64)),
                    actor_id: self.actor_id.clone(),
                    expires_at_ms,
                },
                authority: crate::os_directory::client::DocumentSocketAuthorityV1 {
                    admitted_lease: expectation.lease.clone(),
                    hub_origin: self.hub_origin.clone(),
                    browser_actor: crate::os_directory::DocumentOpenBrowserActorV1::None,
                    expires_at_unix_ms: u64::try_from(expires_at_ms).expect("positive mock expiry"),
                    scope: crate::os_directory::DocumentScope::new(space_id, document_id),
                    descriptor_digest_v1: "4".repeat(64),
                    catalog: crate::os_directory::DocumentOpenCatalogV1 { generation_id: "5".repeat(64) },
                    package: crate::os_directory::DocumentOpenPackageV1 {
                        plugin_id: "mock.plugin".into(),
                        package_id: "mock.package".into(),
                        version: "1.0.0".into(),
                        component_sha256: "6".repeat(64),
                        component_blake3: "7".repeat(64),
                        descriptor_byte_sha256: "8".repeat(64),
                        execution_protocol: crate::os_directory::DocumentExecutionProtocolV1 { app_channel_version: crate::os_spr::CHANNEL_VERSION },
                    },
                    artifact: crate::os_directory::DocumentOpenArtifactV1 { kind: "mock.document".into(), schema: expectation.artifact_schema.clone(), pack_schema_hash: "1".repeat(64) },
                    parent_dialect: crate::os_directory::DocumentOpenParentDialectV1 { artifact_kind: "mock.document".into(), standard: "1".into(), subset: "*".into() },
                    pack_schema_hash: expectation.pack_schema_hash,
                    surface: crate::os_directory::DocumentOpenSurfaceV1 {
                        surface_id: expectation.requested_surface_id.clone().unwrap_or_else(|| "mock.surface".into()),
                        app_id: "mock.app".into(),
                        window_kind_id: "mock.window".into(),
                        role: crate::os_directory::DocumentOpenSurfaceRoleV1::Editor,
                        renderer_target: crate::os_directory::DocumentOpenRendererTargetV1::Wgpu,
                    },
                    grant: crate::os_directory::DocumentOpenGrantV1 { read: true, write: true, observe: true },
                    checkpoint: crate::os_directory::DocumentOpenCheckpointV1 {
                        checkpoint_id: "9".repeat(64),
                        descriptor_digest_v1: "4".repeat(64),
                        baseline_frontier: crate::os_directory::ArtifactFrontier {
                            document_id: document_id.to_string(),
                            head_edit_ordinal: 0,
                            head_edit_id: String::new(),
                            last_commit_seq: 0,
                            chain_hash: crate::os_directory::ArtifactHash::new([0; 32]),
                        },
                        aggregate_sha256: "a".repeat(64),
                    },
                    revalidation: crate::os_directory::DocumentOpenRevalidationV1 { directory_revision: 1, membership_generation: 1, session_generation: Some(1), share_generation: None },
                },
            };
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("grant.returned");
            Ok(admission)
        }
    }

    fn configure_mock_hub(host: &ArtifactHost, hub_origin: &str, actor_fill: char, hub: &MockHub) {
        host.set_local_hub_credential(Arc::new(crate::os_directory::client::LocalHubCredential::test(hub_origin, &format!("session.v1.{}.{}", actor_fill.to_string().repeat(32), actor_fill.to_string().repeat(64)))));
        host.set_hub_socket_grant_source(Arc::new(MockHubSocketGrantSource { hub_origin: hub_origin.into(), actor_id: format!("hub.v1.{}", actor_fill.to_string().repeat(64)), grant_fill: actor_fill, trace: hub.trace.clone() }));
    }

    async fn mock_frontier(ordinal: u64) -> RuntimeFrontierSummary {
        RuntimeFrontierSummary { document_id: ArtifactId("mock".to_string()), head_edit_ordinal: ordinal, head_edit_id: format!("edit-{ordinal}"), last_commit_seq: ordinal, chain_hash: [0u8; 32] }
    }

    async fn spawn_mock_hub() -> (std::net::SocketAddr, Arc<MockHub>) {
        spawn_mock_hub_with_session_gate(false).await
    }

    async fn spawn_mock_hub_with_session_gate(gated: bool) -> (std::net::SocketAddr, Arc<MockHub>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let (broadcast, _rx) = tokio_broadcast::channel(256);
        let semio_hub = Arc::new(MockHub { log: Arc::new(Mutex::new(Vec::new())), broadcast, connections: AtomicUsize::new(0), session_gate: gated.then(|| Arc::new(tokio::sync::Semaphore::new(0))), trace: Arc::new(std::sync::Mutex::new(Vec::new())) });
        let accept_hub = semio_hub.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else { break };
                let conn_hub = accept_hub.clone();
                conn_hub.record("tcp.accepted");
                tokio::spawn(async move {
                    let actor_fill = if conn_hub.connections.fetch_add(1, Ordering::SeqCst) == 0 { 'a' } else { 'b' };
                    let accepted = tokio_tungstenite::accept_hdr_async(
                        stream,
                        |_request: &tokio_tungstenite::tungstenite::handshake::server::Request,
                         mut response: tokio_tungstenite::tungstenite::handshake::server::Response|
                         -> Result<_, tokio_tungstenite::tungstenite::handshake::server::ErrorResponse> {
                            response.headers_mut().insert("Sec-WebSocket-Protocol", "semio.socket.v1".parse().expect("static protocol header"));
                            Ok(response)
                        },
                    )
                    .await;
                    match accepted {
                        Ok(ws) => {
                            conn_hub.record("websocket.accepted");
                            mock_hub_connection(ws, conn_hub, format!("hub.v1.{}", actor_fill.to_string().repeat(64))).await;
                        }
                        Err(_) => conn_hub.record("websocket.refused"),
                    }
                });
            }
        });
        (addr, semio_hub)
    }

    async fn mock_hub_connection(ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, semio_hub: Arc<MockHub>, actor: String) {
        let (mut write, mut read) = ws.split();
        // Expect Hello first.
        let requested_ordinal = match read.next().await {
            Some(Ok(WsMessage::Binary(bytes))) => match decode_client_frame(&bytes).await {
                Ok((_, ClientFrame::SocketHelloV1 { frontier, .. })) => {
                    semio_hub.record("client.socket-hello");
                    frontier.map_or(0, |frontier| frontier.head_edit_ordinal)
                }
                Ok(_) => {
                    semio_hub.record("client.unexpected-first-frame");
                    return;
                }
                Err(_) => {
                    semio_hub.record("client.malformed-first-frame");
                    return;
                }
            },
            Some(Ok(_)) => {
                semio_hub.record("client.nonbinary-first-frame");
                return;
            }
            Some(Err(_)) => {
                semio_hub.record("client.first-frame-error");
                return;
            }
            None => {
                semio_hub.record("client.closed-before-hello");
                return;
            }
        };
        let (frontier, backlog) = {
            let log = semio_hub.log.lock().await;
            let ordinal = log.last().map_or(0, |(ordinal, _)| *ordinal);
            let backlog: Vec<MutationEnvelope> = log.iter().filter(|(ordinal, _)| *ordinal > requested_ordinal).map(|(_, envelope)| envelope.clone()).collect();
            (mock_frontier(ordinal).await, backlog)
        };
        if let Some(gate) = &semio_hub.session_gate {
            semio_hub.record("server.session-gate-waiting");
            gate.acquire().await.expect("mock session gate remains live").forget();
            semio_hub.record("server.session-gate-released");
        }
        let welcome = ServerFrame::Welcome { session_id: "mock-session".to_string(), resume_token: "mock-resume".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail };
        if write.send(WsMessage::Binary(encode_server_frame(&welcome, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.welcome-send-failed");
            return;
        }
        semio_hub.record("server.welcome-sent");
        let session = ServerFrame::Session { actor, color: 1 };
        if write.send(WsMessage::Binary(encode_server_frame(&session, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.session-send-failed");
            return;
        }
        semio_hub.record("server.session-sent");
        let commands = ServerFrame::Commands { envelopes: backlog, origin: ActorId("semio_hub-backlog".to_string()), frontier: frontier.clone() };
        if write.send(WsMessage::Binary(encode_server_frame(&commands, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.commands-send-failed");
            return;
        }
        semio_hub.record("server.commands-sent");
        let mut broadcast_rx = semio_hub.broadcast.subscribe();
        loop {
            tokio::select! {
                incoming = read.next() => {
                    match incoming {
                        Some(Ok(WsMessage::Binary(bytes))) => {
                            match decode_client_frame(&bytes).await {
                                Ok((_, ClientFrame::Commands { batch_id, envelopes })) => {
                                    semio_hub.record("client.commands");
                                    let mut assigned_frontier = frontier.clone();
                                    for envelope in envelopes {
                                        let (ordinal, origin) = {
                                            let mut log = semio_hub.log.lock().await;
                                            let next = log.last().map_or(0, |(ordinal, _)| *ordinal) + 1;
                                            log.push((next, envelope.clone()));
                                            (next, envelope.actor.clone())
                                        };
                                        assigned_frontier = mock_frontier(ordinal).await;
                                        let _ = semio_hub.broadcast.send(ServerFrame::Commands { envelopes: vec![envelope], origin, frontier: assigned_frontier.clone() });
                                    }
                                    let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: assigned_frontier };
                                    let _ = write.send(WsMessage::Binary(encode_server_frame(&ack, Lane::Command).await.into())).await;
                                }
                                Ok((_, ClientFrame::PreviewPublish { key, seq, payload })) => {
                                    semio_hub.record("client.preview-publish");
                                    // 👻️ Best-effort fan-out on the uncredited preview lane — this mock
                                    // semio_hub doesn't track per-connection actor identity beyond `Hello`, so
                                    // it stamps a fixed sentinel origin (fine for the round-trip test
                                    // this drives, which only asserts the *other* peer receives it).
                                    let _ = semio_hub.broadcast.send(ServerFrame::Preview { actor: ActorId("mock-semio_hub-peer".to_string()), key, seq, payload });
                                }
                                Ok((_, ClientFrame::Bye)) => semio_hub.record("client.bye"),
                                Ok(_) => semio_hub.record("client.other"),
                                Err(_) => semio_hub.record("client.malformed-frame"),
                            }
                        }
                        Some(Ok(WsMessage::Close(_))) | None | Some(Err(_)) => break,
                        Some(Ok(_)) => {}
                    }
                }
                frame = broadcast_rx.recv() => {
                    match frame {
                        Ok(frame) => {
                            if write.send(WsMessage::Binary(encode_server_frame(&frame, Lane::Command).await.into())).await.is_err() {
                                break;
                            }
                        }
                        Err(tokio_broadcast::error::RecvError::Lagged(_)) => {}
                        Err(tokio_broadcast::error::RecvError::Closed) => break,
                    }
                }
            }
        }
    }
    //#endregion 🔖️MockHub

    // 🔬️ Two ArtifactHosts converge through a semio_hub: A's operation fans out to B, whose store materializes it.
    #[tokio::test]
    async fn two_hosts_converge_through_hub() {
        let (addr, _hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_a = ArtifactHost::new(test_pool());
        let channels_a = host_a
            .open(ArtifactActorConfig {
                document_id: "shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let mut store_a = ArtifactStore::new(demo_envelope("shared").await).await.expect("valid shared actor A fixture");
        let key_a = channels_a.document_key.clone();
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");

        let host_b = ArtifactHost::new(test_pool());
        let channels_b = host_b
            .open(ArtifactActorConfig {
                document_id: "shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "B".into(),
            })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        let mut store_b = ArtifactStore::new(demo_envelope("shared").await).await.expect("valid shared actor B fixture");
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");

        // Give both actors time to connect + Hello.
        tokio::time::sleep(Duration::from_millis(300)).await;

        store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], description: None }).await.expect("apply on a");
        channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        assert_eq!(document_backbone_event_envelopes(&event).expect("exact document backbone event").len(), 1);
        store_b.tick().await.expect("tick b");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 7, "B converged on A's operation");

        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    #[tokio::test]
    async fn raw_document_backbone_reaches_hub_once_and_returns_one_canonical_event() {
        ensure_demo_codec_registered().await;
        let (addr, hub) = spawn_mock_hub_with_session_gate(true).await;
        let base_url = format!("ws://{addr}");
        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a
            .open(ArtifactActorConfig {
                document_id: "raw-shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let key_a = channels_a.document_key.clone();
        let mut events_a = host_a.subscribe_key(&key_a).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        assert!(matches!(wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await, ArtifactEvent::Session { .. }));
        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig {
                document_id: "raw-shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "B".into(),
            })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        assert!(matches!(wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await, ArtifactEvent::Session { .. }));

        let original = document_backbone_envelope("raw-mutation", "raw-shared");
        channels_a.cmd_tx.send(ArtifactActorMsg::DocumentBackbone { message: document_backbone_message(std::slice::from_ref(&original)) }).expect("exact raw owner admitted");
        let event = wait_for_mock_hub_event("B DocumentBackbone", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        let delivered = document_backbone_event_envelopes(&event).expect("canonical raw event");
        assert_eq!(delivered.len(), 1);
        assert_eq!(delivered[0].mutation_id, original.mutation_id);
        assert_eq!(delivered[0].diff, original.diff);
        assert_eq!(delivered[0].inverse, original.inverse);
        assert_ne!(delivered[0].actor, original.actor, "Hub authority stamps its actor without rewriting opaque mutation fields");
        assert!(matches!(wait_for_mock_hub_event("A CommandOutcome", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await, ArtifactEvent::CommandOutcome { outcome: CommandAckOutcome::Accepted, .. }));
        while let Ok(next) = events_b.try_recv() {
            assert!(!matches!(next, ArtifactEvent::RemoteMutations { .. }), "one server Commands frame cannot emit a duplicate legacy mutation event");
        }

        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    // 🔬️ Reconnect with `since` catch-up: after A appends operations while B is offline, B reconnects and
    // its Welcome backlog carries only the operations it missed.
    #[tokio::test]
    async fn reconnect_since_catch_up_replays_backlog() {
        let (addr, _hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_a = ArtifactHost::new(test_pool());
        let channels_a = host_a
            .open(ArtifactActorConfig {
                document_id: "catchup".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let mut store_a = ArtifactStore::new(demo_envelope("catchup").await).await.expect("valid catchup actor A fixture");
        let key_a = channels_a.document_key.clone();
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
        tokio::time::sleep(Duration::from_millis(300)).await;

        // A applies two operations while nobody else is connected.
        for n in [3, 4] {
            store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n }], description: None }).await.expect("apply on a");
            channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");
            tokio::time::sleep(Duration::from_millis(80)).await;
        }

        // B connects fresh (since_version 0) and its Welcome backlog replays both operations.
        let host_b = ArtifactHost::new(test_pool());
        let channels_b = host_b
            .open(ArtifactActorConfig { document_id: "catchup".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "B".into() })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        let mut store_b = ArtifactStore::new(demo_envelope("catchup").await).await.expect("valid catchup actor B fixture");
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        assert_eq!(document_backbone_event_envelopes(&event).expect("exact backlog event").len(), 2, "backlog replays both missed operations");
        store_b.tick().await.expect("tick b");
        assert_eq!(store_b.envelope().vcs.edits.len(), 2, "B caught up on the full backlog");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 4);

        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    // 🔬️ Detach drains the outbox: an operation applied right before close still reaches the semio_hub (and B).
    #[tokio::test]
    async fn detach_drains_pending_outbound_operations() {
        let (addr, _hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        // Observer B stays connected to witness A's last operation.
        let host_b = ArtifactHost::new(test_pool());
        let channels_b = host_b
            .open(ArtifactActorConfig {
                document_id: "drain".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "B".into(),
            })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        let mut store_b = ArtifactStore::new(demo_envelope("drain").await).await.expect("valid drain actor B fixture");
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");

        let host_a = ArtifactHost::new(test_pool());
        let channels_a =
            host_a.open(ArtifactActorConfig { document_id: "drain".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "A".into() }).await;
        let mut store_a = ArtifactStore::new(demo_envelope("drain").await).await.expect("valid drain actor A fixture");
        let key_a = channels_a.document_key.clone();
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
        tokio::time::sleep(Duration::from_millis(300)).await;

        store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], description: None }).await.expect("apply on a");
        // Immediately close A without waiting for the poll tick: Detach must flush the outbox first.
        host_a.close_key(&key_a);

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        assert_eq!(document_backbone_event_envelopes(&event).expect("exact drain event").len(), 1, "the operation applied before detach was not lost");
        store_b.tick().await.expect("tick b");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 5);
        host_b.close_key(&key_b);
    }

    // 🔬️ The mock semio_hub always Acks `Accepted` — confirms the new `ServerFrame::Ack` ->
    // `ArtifactEvent::CommandOutcome` wiring actually fires (not just that it compiles).
    #[tokio::test]
    async fn command_outcome_accepted_fires_after_hub_ack() {
        let (addr, _hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");
        let host = ArtifactHost::new(test_pool());
        let channels =
            host.open(ArtifactActorConfig { document_id: "outcome".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "A".into() }).await;
        let key = channels.document_key.clone();
        let mut events = host.subscribe_key(&key).await;
        let mut store = ArtifactStore::new(demo_envelope("outcome").await).await.expect("valid outcome actor fixture");
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");
        tokio::time::sleep(Duration::from_millis(300)).await;

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply");
        channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

        let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await;
        match event {
            ArtifactEvent::CommandOutcome { outcome, .. } => assert_eq!(outcome, CommandAckOutcome::Accepted),
            other => panic!("expected CommandOutcome, got {other:?}"),
        }
        host.close_key(&key);
    }

    // 🔬️ `SyncSession::publish_preview` -> `ClientFrame::PreviewPublish` -> the mock semio_hub's
    // preview-lane fan-out -> `ServerFrame::Preview` -> `ArtifactEvent::Preview` on another peer.
    #[tokio::test]
    async fn publish_preview_round_trips_through_hub() {
        let (addr, _hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_a = ArtifactHost::new(test_pool());
        let channels_a = host_a
            .open(ArtifactActorConfig {
                document_id: "preview".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;

        let host_b = ArtifactHost::new(test_pool());
        let channels_b = host_b
            .open(ArtifactActorConfig { document_id: "preview".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "B".into() })
            .await;
        let key_a = channels_a.document_key.clone();
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        tokio::time::sleep(Duration::from_millis(300)).await;

        channels_a.cmd_tx.send(ArtifactActorMsg::PublishPreview { key: "cursor".into(), seq: 1, payload: vec![1, 2, 3] }).expect("publish preview");

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::Preview { .. })).await;
        match event {
            ArtifactEvent::Preview { key, seq, payload, .. } => {
                assert_eq!(key, "cursor");
                assert_eq!(seq, 1);
                assert_eq!(payload, vec![1, 2, 3]);
            }
            other => panic!("expected Preview, got {other:?}"),
        }
        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    // 🔬️ Shared fixtures replay: each fixture's inbound stimuli produce the expected ArtifactEvent
    // sequence and final timeline. The same fixtures drive WS-E's vitest harness against the TS twin.
    #[tokio::test]
    async fn fixtures_replay_matches_expected_events() {
        let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("🧫️fixtures");
        let fixtures = load_fixtures(&fixtures_dir).await;
        assert!(!fixtures.is_empty(), "expected fixtures in {fixtures_dir:?}");
        for fixture in fixtures {
            replay_fixture(&fixture).await;
        }
    }

    async fn replay_fixture(fixture: &ActorFixture) {
        ensure_demo_codec_registered().await;
        let codec = crate::os_store::document_codec(&fixture.schema).await.expect("codec registry available").unwrap_or_else(|| panic!("no codec registered for fixture schema {:?}", fixture.schema));
        let dir = crate::os_store::test_support::tempdir().expect("tempdir");
        let host = ArtifactHost::new(test_pool());
        let channels =
            host.open(ArtifactActorConfig { document_id: fixture.document_id.clone(), schema: fixture.schema.clone(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() }).await;
        let mut events = host.subscribe(&fixture.document_id).await;
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>(&fixture.schema, &fixture.document_id, DemoSnapshot { n: 0 }, None)).await.expect("valid fixture store");
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");
        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
        wait_until(&format!("seed snapshot for {} on disk", fixture.document_id), || async { storage.read(&fixture.document_id).await.expect("read").is_some() }).await;

        // Lockstep: apply each stimulus, then wait for its paired expected event before the next
        // (removes any write/poke race). Folder-replayable fixtures pair inbound 1:1 with events.
        assert_eq!(fixture.inbound.len(), fixture.expected_events.len(), "fixture {} must pair each inbound stimulus with one expected event", fixture.name);
        let mut observed: Vec<String> = Vec::new();
        for (inbound, expected) in fixture.inbound.iter().zip(fixture.expected_events.iter()) {
            match inbound {
                FixtureInbound::ExternalEdits { ops_text } => {
                    let (pack, spr) = storage.read(&fixture.document_id).await.expect("read").expect("some");
                    let parsed = crate::os_spr::parse_ops_text(ops_text).unwrap_or_else(|error| panic!("fixture {} parse_ops_text: {error}", fixture.name));
                    let mut new_edits: Vec<crate::os_spr::HistoryEdit> = Vec::new();
                    for edit in parsed.edits {
                        let mut ops: Vec<crate::os_spr::OpPayload> = Vec::new();
                        for op in &edit.ops {
                            let text = op.text.as_deref().unwrap_or_else(|| panic!("fixture {} op line has no text", fixture.name));
                            let concrete = DemoMutation::parse_op(text).unwrap_or_else(|error| panic!("fixture {} parse_op {text:?}: {error}", fixture.name));
                            ops.push(crate::os_spr::OpPayload { text: None, binary: Some(concrete.encode_op().expect("encode demo op")) });
                        }
                        new_edits.push(crate::os_spr::HistoryEdit { ops, meta: None, ..edit });
                    }
                    let new_spr = crate::os_store::append_history_edits_to_spr(&spr, &new_edits).await.expect("append fixture edits");
                    storage.write(&fixture.document_id, &fixture.schema, &pack, &new_spr).await.expect("write");
                    channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                }
                FixtureInbound::ReplaceDocument { dsl_text, ops_text } => {
                    let (pack_files, _dsl_mirror) = (codec.compile_dsl)(dsl_text, ops_text).await.unwrap_or_else(|error| panic!("fixture {} compile_dsl: {error}", fixture.name));
                    storage.write(&fixture.document_id, &fixture.schema, &pack_files.pack, &pack_files.spr).await.expect("replace write");
                    channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                }
                FixtureInbound::HubFrame { .. } => {
                    panic!("fixture {} uses a HubFrame stimulus not supported by the Rust harness", fixture.name);
                }
            }
            let event = wait_for_event(&mut events, |event| document_event_tag(event) == expected.as_str()).await;
            observed.push(document_event_tag(&event).to_string());
            store.tick().await.expect("tick");
        }
        assert_eq!(&observed, &fixture.expected_events, "fixture {} event sequence", fixture.name);
        let timeline_ids: Vec<String> = store.envelope().vcs.edits.iter().map(|edit| edit.id.clone()).collect();
        for expected_id in &fixture.expected_edit_ids {
            assert!(timeline_ids.contains(expected_id), "fixture {} expected edit id {expected_id} in timeline {timeline_ids:?}", fixture.name);
        }
        host.close(&fixture.document_id);
    }

    // 🚫️async: E1-adjacent — pure match with no suspension point, consumed by
    // `wait_for_event`'s sync `FnMut(&ArtifactEvent) -> bool` predicate bound — see R9.
    fn document_event_tag(event: &ArtifactEvent) -> &'static str {
        match event {
            ArtifactEvent::RemoteMutations { .. } => "remoteMutations",
            ArtifactEvent::DocumentBackbone { .. } => "documentBackbone",
            ArtifactEvent::SnapshotReplaced { .. } => "snapshotReplaced",
            ArtifactEvent::BootstrapProgress { .. } => "bootstrapProgress",
            ArtifactEvent::Status(_) => "status",
            ArtifactEvent::Presence { .. } => "presence",
            ArtifactEvent::Session { .. } => "session",
            ArtifactEvent::Preview { .. } => "preview",
            ArtifactEvent::CommandOutcome { .. } => "commandOutcome",
            ArtifactEvent::Conflict(_) => "conflict",
        }
    }
}
//#endregion 🧪️Actor

/// @emoji 🎯️ `FolderEventLogStorage` is a pure `(pack, spr)` event store — schema-agnostic,
/// no JSON/codec involvement at this layer (that lives one level up, in `FolderEndpoint`, tested
/// via `folder_external_edit_delivers_remote_operations`). This test exercises exactly the
/// storage mechanics: per-id folding, append-only replacement, and the folder-wide index.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_event_log_storage_round_trips_by_document_id() {
    let dir = crate::os_store::test_support::tempdir().expect("tempdir");
    let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
    assert_eq!(storage.read("doc-a").await.expect("read empty"), None, "absent document reads as None");

    storage.write("doc-a", "demo/v1", b"pack-a", b"spr-a").await.expect("write a");
    storage.write("doc-b", "demo/v1", b"pack-b", b"spr-b").await.expect("write b");
    assert_eq!(storage.read("doc-a").await.expect("read a").expect("some a"), (b"pack-a".to_vec(), b"spr-a".to_vec()), "documents are keyed independently");
    assert_eq!(storage.read("doc-b").await.expect("read b").expect("some b"), (b"pack-b".to_vec(), b"spr-b".to_vec()));

    storage.write("doc-a", "demo/v1", b"pack-a2", b"spr-a2").await.expect("upsert a");
    assert_eq!(storage.read("doc-a").await.expect("reread a").expect("some a2"), (b"pack-a2".to_vec(), b"spr-a2".to_vec()), "the latest snapshot event replaces the projection");

    let mut ids = storage.document_ids().await.expect("document ids");
    ids.sort();
    assert_eq!(ids, vec!["doc-a".to_string(), "doc-b".to_string()], "folder indexes every document");
}

/// @emoji 🔐️ The endpoint-level save→load→undo proof: a store's undo/redo position survives a
/// full write/read cycle through the ACTUAL `FolderEventLogStorage` byte storage (`store`'s own
/// `save_load_undo_proof_pack_spr_round_trip_preserves_undo_redo_position` proves the pure
/// in-memory pack/spr encoding; this proves the folder persistence layer built on top of it).
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_event_log_storage_round_trips_undo_position_through_pack_spr() {
    let dir = crate::os_store::test_support::tempdir().expect("tempdir");
    let storage = FolderEventLogStorage::new(dir.path().to_path_buf());

    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "doc-a", DemoSnapshot { n: 0 }, None)).await.expect("valid folder store");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply e1");
    let post_e1 = store.snapshot().expect("post-e1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).await.expect("apply e2");
    store.dispatch(ArtifactCommand::Undo).await.expect("undo e2");
    assert_eq!(store.snapshot().expect("live"), post_e1, "precondition: live store is back at post-e1");

    let files = print_document_pack(store.envelope()).await.expect("print document pack");
    storage.write("doc-a", "demo/v1", &files.pack, &files.spr).await.expect("write");

    let (pack, spr) = storage.read("doc-a").await.expect("read").expect("some");
    let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&pack, &spr).await.unwrap_or_else(|error| panic!("parse: {error}"));
    assert_eq!(parsed.snapshot, post_e1, "loaded snapshot must equal post-e1 through the folder storage layer");
    let mut reloaded = ArtifactStore::new(parsed.envelope).await.expect("valid reloaded history");
    assert_eq!(reloaded.snapshot().expect("reloaded"), post_e1);

    reloaded.dispatch(ArtifactCommand::Redo).await.expect("redo e2 after folder reload");
    assert_eq!(reloaded.snapshot().expect("post-redo"), DemoSnapshot { n: 2 });
}

/// @emoji 🎯️ Seeds the write from a ZERO-edit envelope (no cursor line — a cursor is only
/// synced once an edit is dispatched, see `ArtifactStore::sync_cursor`) so both edits are then
/// added purely via the raw `append_ops` hot path with no cursor line ever written; a cursor
/// pinned to an earlier edit count would otherwise cap the reconstructed snapshot at that
/// edit (see `document_text_round_trips_a_cursor_after_undo_then_apply_interleaving` in
/// `store`'s own test suite for that law, exercised correctly there).
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_text_storage_round_trips_dsl_and_appends_ops() {
    let dir = crate::os_store::test_support::tempdir().expect("tempdir");
    let storage = FolderTextStorage::new(dir.path().to_path_buf()).await;
    assert_eq!(storage.read("demo", "demo").await.expect("read empty"), None, "absent document reads as None");

    let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid zero-edit text fixture");
    let files = print_document_text(seed.envelope()).await.expect("print document text");
    storage.write("demo", "demo", &files).await.expect("write");

    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid text fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply 1");
    let first_edit = store.envelope().vcs.edits.last().expect("first edit");
    storage.append_ops("demo", "demo", &print_edit_lines(first_edit).await.expect("print edit lines")).await.expect("append ops 1");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).await.expect("apply 2");
    let second_edit = store.envelope().vcs.edits.last().expect("second edit");
    storage.append_ops("demo", "demo", &print_edit_lines(second_edit).await.expect("print edit lines")).await.expect("append ops 2");

    let reloaded = storage.read("demo", "demo").await.expect("read").expect("some");
    let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_text(&reloaded.dsl, &reloaded.ops).await.unwrap_or_else(|error| panic!("parse: {error}"));
    assert_eq!(parsed.snapshot.n, 2, "write + append reconstructs every edit in order");

    assert_eq!(storage.document_ids("demo").await.expect("document ids"), vec!["demo".to_string()]);
}

/// @emoji 🎯️ Unlike the `.ops`-text hot path (`append_ops`, tested above), `.pack`+`.spr` have
/// no incremental-append primitive wired up yet (that's `crate::os_spr::HistoryAppender`, a future
/// wave's job to thread through this storage layer) — `write_pack` is the whole-file cold path
/// for the AUTHORITATIVE pair, called again after every edit. `append_ops` still keeps the
/// `.ops` TEXT MIRROR current independently (it is never read by the pack+spr-first
/// `parse_document_pack`/`read_pack` path — see `ArtifactPackFiles`'s doc — only by
/// `parse_document_text`), which this test verifies explicitly: appending ops text alone,
/// without an accompanying `write_pack`, does NOT change what `read_pack`/`parse_document_pack`
/// reconstructs, because pack+spr (not ops text) are authoritative for that path.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_text_storage_round_trips_pack() {
    let dir = crate::os_store::test_support::tempdir().expect("tempdir");
    let storage = FolderTextStorage::new(dir.path().to_path_buf()).await;
    assert_eq!(storage.read_pack("demo", "demo").await.expect("read empty"), None, "absent pack reads as None");

    let seed = ArtifactStore::<DemoSnapshot, DemoMutation>::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid pack fixture");
    let files = print_document_pack(seed.envelope()).await.expect("print document pack");
    let dsl_mirror = seed.envelope().vcs.initial_snapshot.print_dsl();
    storage.write_pack("demo", "demo", &files, &dsl_mirror).await.expect("write pack");

    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None)).await.expect("valid pack append fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], description: None }).await.expect("apply 1");
    let first_edit = store.envelope().vcs.edits.last().expect("first edit");
    storage.append_ops("demo", "demo", &print_edit_lines(first_edit).await.expect("print edit lines")).await.expect("append ops 1");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], description: None }).await.expect("apply 2");
    let second_edit = store.envelope().vcs.edits.last().expect("second edit");
    storage.append_ops("demo", "demo", &print_edit_lines(second_edit).await.expect("print edit lines")).await.expect("append ops 2");

    // Text mirror is current (both edits landed via append_ops).
    let reloaded_text = storage.read("demo", "demo").await.expect("read text").expect("some text");
    let parsed_text: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_text(&reloaded_text.dsl, &reloaded_text.ops).await.unwrap_or_else(|error| panic!("parse text: {error}"));
    assert_eq!(parsed_text.snapshot.n, 2, "the .ops text mirror reflects every appended edit");

    // pack+spr are unaffected by ops-text-only appends — still the zero-edit snapshot from
    // the initial write_pack, proving read_pack/parse_document_pack never reads .ops.
    let reloaded_pack = storage.read_pack("demo", "demo").await.expect("read pack").expect("some pack");
    let parsed_pack: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&reloaded_pack.pack, &reloaded_pack.spr).await.unwrap_or_else(|error| panic!("parse pack: {error}"));
    assert_eq!(parsed_pack.snapshot.n, 0, "pack+spr are authoritative and independent of ops-text-only appends");

    // A fresh whole-file write_pack (the actual cold-path persistence flow) brings pack+spr
    // current with the live store.
    let files2 = print_document_pack(store.envelope()).await.expect("print document pack 2");
    let dsl_mirror2 = store.envelope().vcs.initial_snapshot.print_dsl();
    storage.write_pack("demo", "demo", &files2, &dsl_mirror2).await.expect("write pack 2");
    let reloaded_pack2 = storage.read_pack("demo", "demo").await.expect("read pack 2").expect("some pack 2");
    let parsed_pack2: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&reloaded_pack2.pack, &reloaded_pack2.spr).await.unwrap_or_else(|error| panic!("parse pack 2: {error}"));
    assert_eq!(parsed_pack2.snapshot.n, 2, "a fresh write_pack brings pack+spr current with the live store");

    // The always-written DSL mirror must also be on disk and agree with the initial-snapshot.
    let mirror = std::fs::read_to_string(storage.pack_path("demo", "demo").await.with_extension("")).expect("dsl mirror on disk");
    assert_eq!(DemoSnapshot::parse_dsl(&mirror).expect("parse mirror").n, 0, "mirror captures the initial snapshot, not later edits");
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn blob_store_put_get_dedupes_idempotently() {
    let dir = crate::os_store::test_support::tempdir().expect("tempdir");
    let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
    let bytes = b"hello content-addressed world";
    assert!(!storage.has("not-a-real-hash").await.expect("has on empty store"));

    let first = storage.put(bytes, "text/plain").await.expect("first put");
    let second = storage.put(bytes, "text/plain").await.expect("second put");
    assert_eq!(first, second, "putting identical bytes twice is idempotent and dedupes by hash");
    assert_eq!(first.size, bytes.len() as u64);
    assert_eq!(first.media_type, "text/plain");

    assert!(storage.has(&first.hash).await.expect("has after put"));
    let fetched = storage.get(&first.hash).await.expect("get").expect("blob present");
    assert_eq!(fetched, bytes);

    let other = storage.put(b"different content", "text/plain").await.expect("put other");
    assert_ne!(other.hash, first.hash, "different bytes hash differently");

    storage.delete(&first.hash).await.expect("delete");
    assert!(!storage.has(&first.hash).await.expect("has after delete"));
    assert_eq!(storage.get(&first.hash).await.expect("get after delete"), None);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_event_log_ignores_an_incomplete_tail_and_coordinates_handles() {
    use std::io::Write;

    let dir = crate::os_store::test_support::tempdir().expect("tempdir");
    let first = FolderEventLogStorage::new(dir.path().to_path_buf());
    let second = FolderEventLogStorage::new(dir.path().to_path_buf());
    first.write("doc-a", "demo/v1", b"pack-a", b"spr-a").await.expect("first handle write");
    second.write("doc-b", "demo/v1", b"pack-b", b"spr-b").await.expect("second handle write");

    let mut file = std::fs::OpenOptions::new().append(true).open(first.event_path()).expect("open event log");
    file.write_all(&FOLDER_EVENT_MAGIC[..3]).expect("partial crash tail");
    file.sync_data().expect("sync crash tail");

    assert_eq!(first.read("doc-a").await.expect("read a").expect("doc a"), (b"pack-a".to_vec(), b"spr-a".to_vec()));
    assert_eq!(second.read("doc-b").await.expect("read b").expect("doc b"), (b"pack-b".to_vec(), b"spr-b".to_vec()));
}
