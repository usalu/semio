use super::{
    artifact_mailbox_pair, ensure_demo_codec_registered, native_actor, presence_to_bytes, ActorId, ArtifactActorConfig, ArtifactEvent, Bootstrap, ChannelBackbone, ClientFrame, CommandAckOutcome, DemoMutation, DemoSnapshot, MutationEnvelope,
    PersistenceBinding, PresencePeer, RemoteState, RuntimeFrontierSummary, ServerFrame, AckStage, ApplyOutcome,
};
use crate::os_spr::wire::RebootstrapRequired;
use crate::os_spr::{ArtifactId, Edit};
use futures::StreamExt;
use serde_json::Value;
use std::collections::BTreeSet;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

fn parity_pool() -> Arc<semio_framework_async::WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<semio_framework_async::WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)))).clone()
}

fn parity_frontier(value: &Value) -> RuntimeFrontierSummary {
    let byte = value["chainHashByte"].as_u64().expect("chainHashByte") as u8;
    RuntimeFrontierSummary {
        document_id: ArtifactId(value["documentId"].as_str().expect("documentId").into()),
        head_edit_ordinal: value["headEditOrdinal"].as_u64().expect("ordinal"),
        head_edit_id: value["headEditId"].as_str().expect("editId").into(),
        last_commit_seq: value["lastCommitSeq"].as_u64().expect("commit"),
        chain_hash: [byte; 32],
    }
}

async fn parity_envelope(document_id: &str, mutation_id: &str, n: i32) -> MutationEnvelope {
    let edit = Edit {
        id: mutation_id.into(),
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
    let document_id = ArtifactId(document_id.into());
    let schema = crate::os_spr::SchemaId("demo/v1".to_string());
    let mut envelopes = crate::os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("envelope");
    let mut envelope = envelopes.pop().expect("one envelope");
    envelope.mutation_id = crate::os_spr::MutationId(mutation_id.into());
    envelope
}

fn event_kind(event: &ArtifactEvent) -> &'static str {
    match event {
        ArtifactEvent::RemoteMutations { .. } => "remoteMutations",
        ArtifactEvent::DocumentBackbone { .. } => "documentBackbone",
        ArtifactEvent::DocumentArchiveReplaced { .. } => "documentArchiveReplaced",
        ArtifactEvent::BootstrapProgress { .. } => "bootstrapProgress",
        ArtifactEvent::Status(_) => "status",
        ArtifactEvent::Presence { .. } => "presence",
        ArtifactEvent::Session { .. } => "session",
        ArtifactEvent::Preview { .. } => "preview",
        ArtifactEvent::CommandOutcome { .. } => "commandOutcome",
        ArtifactEvent::Conflict(_) => "conflict",
    }
}

fn remote_kind(state: &RemoteState) -> &'static str {
    match state {
        RemoteState::Detached => "detached",
        RemoteState::Connecting => "connecting",
        RemoteState::Live { .. } => "live",
        RemoteState::Backoff { .. } => "offline",
    }
}

fn peer_count(state: &RemoteState) -> Option<usize> {
    match state {
        RemoteState::Live { peer_count } => Some(*peer_count),
        _ => None,
    }
}

struct ParityHarness {
    actor: native_actor::ArtifactActor,
    events: broadcast::Receiver<ArtifactEvent>,
    observed_events: Vec<String>,
    outgoing: Vec<String>,
    last_outcome: Option<String>,
    socket: Option<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>,
    document_id: String,
}

impl ParityHarness {
    async fn open(document_id: &str, space_id: &str, actor: &str) -> Self {
        ensure_demo_codec_registered().await;
        let (_, remote) = ChannelBackbone::pair("backbone-parity").await;
        let (_, receiver) = artifact_mailbox_pair();
        let (events_tx, events) = broadcast::channel(64);
        let actor = native_actor::ArtifactActor::new(
            parity_pool(),
            ArtifactActorConfig {
                document_id: document_id.into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: "http://127.0.0.1:9".into(), space_id: space_id.into(), surface: None }],
                watch_external: false,
                actor: actor.into(),
            },
            remote,
            receiver,
            events_tx,
            Arc::new(std::sync::RwLock::new(None)),
            Arc::new(std::sync::RwLock::new(None)),
            None,
            semio_framework_async::CancelToken::root_now(),
        )
        .await;
        Self { actor, events, observed_events: Vec::new(), outgoing: Vec::new(), last_outcome: None, socket: None, document_id: document_id.into() }
    }

    fn drain_events(&mut self) {
        while let Ok(event) = self.events.try_recv() {
            if let ArtifactEvent::CommandOutcome { outcome, .. } = &event {
                self.last_outcome = Some(match outcome {
                    CommandAckOutcome::Accepted => "accepted".into(),
                    CommandAckOutcome::Rejected { .. } => "rejected".into(),
                    CommandAckOutcome::Transformed => "transformed".into(),
                });
            }
            let kind = event_kind(&event).to_string();
            if self.observed_events.last() != Some(&kind) || kind == "documentBackbone" || kind == "presence" || kind == "commandOutcome" {
                self.observed_events.push(kind);
            }
        }
    }

    async fn pump_outgoing(&mut self) {
        let Some(socket) = self.socket.as_mut() else { return };
        loop {
            match tokio::time::timeout(std::time::Duration::from_millis(20), socket.next()).await {
                Ok(Some(Ok(Message::Binary(bytes)))) => {
                    let frame = crate::os_spr::decode_client_frame(&bytes).await.expect("client frame").1;
                    let kind = match frame {
                        ClientFrame::SocketHelloV1 { .. } => "socketHelloV1",
                        ClientFrame::Commands { .. } => "commands",
                        ClientFrame::Presence { .. } => "presence",
                        ClientFrame::PreviewPublish { .. } => "previewPublish",
                        _ => "other",
                    };
                    self.outgoing.push(kind.into());
                }
                _ => break,
            }
        }
    }

    async fn connect_socket(&mut self, receipt_actor: &str) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let url = format!("ws://{}", listener.local_addr().expect("addr"));
        let accepted = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            tokio_tungstenite::accept_async(stream).await.expect("upgrade")
        });
        self.actor.connect_test_socket(&url, receipt_actor).await;
        self.socket = Some(accepted.await.expect("task"));
        self.pump_outgoing().await;
    }

    async fn apply_frame(&mut self, frame: &Value) {
        let kind = frame["kind"].as_str().expect("kind");
        let server = match kind {
            "welcome" => ServerFrame::Welcome {
                session_id: frame["sessionId"].as_str().unwrap_or("session").into(),
                resume_token: frame["resumeToken"].as_str().expect("resume").into(),
                server_frontier: parity_frontier(&frame["frontier"]),
                bootstrap: match frame["bootstrap"].as_str().unwrap_or("none") {
                    "tail" => Bootstrap::Tail,
                    _ => Bootstrap::None,
                },
            },
            "session" => ServerFrame::Session { actor: frame["actor"].as_str().expect("actor").into(), color: frame["color"].as_u64().unwrap_or(0) as u8 },
            "commands" => {
                let mut envelopes = Vec::new();
                for row in frame["envelopes"].as_array().expect("envelopes") {
                    envelopes.push(parity_envelope(&self.document_id, row["mutationId"].as_str().expect("id"), row["n"].as_i64().unwrap_or(0) as i32).await);
                }
                ServerFrame::Commands { envelopes, origin: ActorId(frame["origin"].as_str().expect("origin").into()), frontier: parity_frontier(&frame["frontier"]) }
            }
            "ack" => {
                let outcome = match frame["outcome"].as_str().unwrap_or("accepted") {
                    "rejected" => ApplyOutcome::Rejected { reason: frame["rejectReason"].as_str().unwrap_or("rejected").into(), messages: Vec::new() },
                    _ => ApplyOutcome::Accepted,
                };
                ServerFrame::Ack { batch_id: frame["batchId"].as_u64().unwrap_or(0), stages: vec![AckStage::Applied { outcome: Box::new(outcome) }], frontier: parity_frontier(&frame["frontier"]) }
            }
            "presence" => {
                let count = frame["peerCount"].as_u64().unwrap_or(0) as usize;
                let mut peers = Vec::new();
                for index in 0..count {
                    let peer = PresencePeer {
                        actor: format!("peer-{index}"),
                        connected_at_ms: 1,
                        label: None,
                        presence_pack: None,
                        user_id: None,
                        role: None,
                        drag_ghost_json: None,
                        interaction: None,
                        color: Some(index as u8),
                        surface: None,
                        views: Vec::new(),
                        ui: None,
                        tool_run: None,
                        principal_kind: None, active_tool: None,
                    };
                    peers.push(presence_to_bytes(&peer).await);
                }
                ServerFrame::Presence { peers }
            }
            "rebootstrapRequired" => ServerFrame::RebootstrapRequired {
                control: RebootstrapRequired {
                    space_id: frame["spaceId"].as_str().expect("space").into(),
                    document_id: frame["documentId"].as_str().expect("document").into(),
                    checkpoint_id: [1; 32],
                    descriptor_hash: [2; 32],
                    baseline_frontier: parity_frontier(&frame["frontier"]),
                },
            },
            other => panic!("unsupported frame kind {other}"),
        };
        self.actor.inject_hub_frame(server).await;
        self.drain_events();
        self.pump_outgoing().await;
    }

    async fn apply_dispatch(&mut self, dispatch: &Value) {
        match dispatch["kind"].as_str().expect("kind") {
            "queueMutation" => {
                let envelope = parity_envelope(&self.document_id, dispatch["mutationId"].as_str().expect("id"), dispatch["n"].as_i64().unwrap_or(0) as i32).await;
                self.actor.queue_test_outbox(vec![envelope]);
            }
            "installSocketActor" => {
                self.actor.install_test_socket_actor(dispatch["actor"].as_str().expect("actor"));
            }
            "connectSocket" => {
                self.connect_socket(dispatch["actor"].as_str().expect("actor")).await;
            }
            "failConnection" => {
                self.actor.fail_test_connection().await;
                self.socket = None;
            }
            other => panic!("unsupported dispatch {other}"),
        }
        self.drain_events();
        self.pump_outgoing().await;
    }

    fn assert_expect(&mut self, expect: &Value) {
        self.drain_events();
        let (resume, frontier_edit, remote, outbox, pending, confirmed, rebootstrap, ingested) = self.actor.parity_test_state();
        if expect.get("resumeToken").is_some() {
            let expected = expect.get("resumeToken").and_then(|value| value.as_str()).map(str::to_string);
            assert_eq!(resume, expected, "resumeToken");
        }
        if expect.get("frontierEditId").is_some() {
            let expected = expect.get("frontierEditId").and_then(|value| value.as_str()).map(str::to_string);
            assert_eq!(frontier_edit, expected, "frontierEditId");
        }
        if let Some(kind) = expect.get("remoteKind").and_then(|value| value.as_str()) {
            assert_eq!(remote_kind(&remote), kind, "remoteKind");
        }
        if let Some(count) = expect.get("peerCount").and_then(|value| value.as_u64()) {
            assert_eq!(peer_count(&remote), Some(count as usize), "peerCount");
        }
        if let Some(rows) = expect.get("outboxMutationIds").and_then(|value| value.as_array()) {
            let expected: Vec<String> = rows.iter().map(|row| row.as_str().expect("id").into()).collect();
            assert_eq!(outbox, expected, "outboxMutationIds");
        }
        if let Some(count) = expect.get("pendingBatchCount").and_then(|value| value.as_u64()) {
            assert_eq!(pending, count as usize, "pendingBatchCount");
        }
        if let Some(flag) = expect.get("socketActorConfirmed").and_then(|value| value.as_bool()) {
            assert_eq!(confirmed, flag, "socketActorConfirmed");
        }
        if let Some(flag) = expect.get("rebootstrapRequired").and_then(|value| value.as_bool()) {
            assert_eq!(rebootstrap, flag, "rebootstrapRequired");
        }
        if let Some(rows) = expect.get("eventKinds").and_then(|value| value.as_array()) {
            let expected: Vec<String> = rows.iter().map(|row| row.as_str().expect("kind").into()).collect();
            for kind in &expected {
                assert!(self.observed_events.iter().any(|seen| seen == kind), "missing event {kind} in {:?}", self.observed_events);
            }
        }
        if let Some(rows) = expect.get("outgoingClientFrameKinds").and_then(|value| value.as_array()) {
            let expected: Vec<String> = rows.iter().map(|row| row.as_str().expect("kind").into()).collect();
            assert_eq!(self.outgoing, expected, "outgoingClientFrameKinds");
        }
        if let Some(rows) = expect.get("ingestedMutationIds").and_then(|value| value.as_array()) {
            let expected: BTreeSet<String> = rows.iter().map(|row| row.as_str().expect("id").into()).collect();
            let actual: BTreeSet<String> = ingested.into_iter().collect();
            assert!(expected.is_subset(&actual), "ingested {expected:?} not in {actual:?}");
        }
        if let Some(outcome) = expect.get("commandOutcome").and_then(|value| value.as_str()) {
            assert_eq!(self.last_outcome.as_deref(), Some(outcome), "commandOutcome");
        }
    }
}

fn load_fixture() -> Value {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔨️modules/🏪️store/🔄️sync/⚖️parity");
    let fixtures = std::fs::read_dir(&root).expect("parity dir").filter_map(|entry| entry.ok()).find(|entry| entry.file_name().to_string_lossy().contains("fixture")).expect("fixtures dir");
    let text = std::fs::read_to_string(fixtures.path().join("🔣️.json")).expect("fixture json");
    serde_json::from_str(&text).expect("fixture parse")
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn backbone_parity_scenarios_match_neutral_fixture() {
    let fixture = load_fixture();
    assert_eq!(fixture["version"], 1);
    for scenario in fixture["scenarios"].as_array().expect("scenarios") {
        let id = scenario["id"].as_str().expect("id");
        let mut harness = ParityHarness::open(scenario["documentId"].as_str().expect("doc"), scenario["spaceId"].as_str().expect("space"), scenario["actor"].as_str().expect("actor")).await;
        for step in scenario["steps"].as_array().expect("steps") {
            match step["op"].as_str().expect("op") {
                "serverFrame" => harness.apply_frame(&step["frame"]).await,
                "localDispatch" => harness.apply_dispatch(&step["dispatch"]).await,
                "expect" => harness.assert_expect(&step["expect"]),
                other => panic!("scenario {id} unknown op {other}"),
            }
        }
    }
}
