//! 🎭️ The native document actor: one finite-turn actor per open document on the process WorkerPool, its runner, and the native `ArtifactHost` that owns them.

use semio_framework_os_store_sync::{ARTIFACT_BOOTSTRAP_DEADLINE_MS, ArtifactActorConfig, ArtifactActorMsg, ArtifactChannelsOf, ArtifactDocumentKey, ArtifactEvent, ArtifactHostCore, ArtifactHostState, ArtifactMailboxClose, ArtifactMailboxReceiver, ArtifactSyncStatus, CommandAckOutcome, CommandRejectionCode, CommandRejectionDetail, DOCUMENT_LINK_SHORTAGE_POLICY, DocumentBackboneRetentionV1, DocumentLink, DocumentLinkEvent, OpenDocument, PersistenceBinding, PresenceHeartbeatProducer, RemoteState, actor_identity_admission_admits, actor_seed, add_actor_identity_progress, admit_remote_envelopes, artifact_mailbox_pair, canonical_pair_baseline, decode_document_backbone_message_exact, document_admission_refuses_access, document_archive_hash, document_link_terminal_message, document_pack_schema_hash, frontier_reaches, history_edit_from_envelope, hub_ws_url, next_timestamp, note_authored_envelopes, now_ms, presence_from_bytes, presence_to_bytes, rebootstrap_control, rollback_envelope, settle_committed_envelopes, spr_events, spr_op_ids, stamp_session, transition_refusal, validate_artifact_bootstrap_identity, wire_envelopes_with_original_actor};
use semio_framework_os_spr::{AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationMessage, PresencePeer, PresenceToolRun, RuntimeFrontierSummary, ServerFrame, decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes};
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_backbone::{BackboneMessage, ChannelBackbone, ChannelBackboneRemote};
use crate::lifted_sync::{FolderEventLogStorage, FolderTextStorage};
use tokio::sync::broadcast;
use futures::{SinkExt, StreamExt};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};

type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
type WsSink = futures::stream::SplitSink<WsStream, Message>;
type WsRead = futures::stream::SplitStream<WsStream>;
struct ConnectedDocumentSocket {
    stream: WsStream,
    pub socket_actor: semio_framework_value::SharedUtf8,
    pub authority: semio_framework_os_directory_client::DocumentSocketAuthorityV1,
    identity_progress: semio_framework_value::RetainedCloneProgress,
}

struct ArtifactReadinessWake(Arc<dyn Fn() + Send + Sync>);

impl std::task::Wake for ArtifactReadinessWake {
    fn wake(self: Arc<Self>) {
        (self.0)();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        (self.0)();
    }
}

struct WipeSocketHeader(String);

impl Drop for WipeSocketHeader {
    fn drop(&mut self) {
        unsafe { self.0.as_mut_vec().fill(0) };
    }
}

type ConnectFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Result<ConnectedDocumentSocket, DocumentConnectFailure>> + Send>>;

/// 🔌️ Why a hub dial did not produce a socket: the hub withdrew access (the link is revoked), or the
/// link is short (it backs off along [`DOCUMENT_LINK_SHORTAGE_POLICY`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocumentConnectFailure {
    Refused,
    Short,
}

/// ⏰️ The wall clock the link state machine reads.
fn wall_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

/// ⏰️ The `Instant` of a wall-clock deadline, for the actor's timer-driven drive.
fn instant_at(at_ms: u64, now_ms: u64) -> Instant {
    Instant::now() + Duration::from_millis(at_ms.saturating_sub(now_ms))
}

struct HubConn {
    pub write: WsSink,
    pub read: WsRead,
}

struct PendingArtifactBootstrap {
    pub assembler: ArtifactBootstrapAssembler,
    pub started_at: Instant,
    pub resume_token: String,
    pub baseline_frontier: RuntimeFrontierSummary,
    pub required_tail_frontier: RuntimeFrontierSummary,
    pub pack_schema_hash: [u8; 32],
}

struct NativeBootstrapControl {
    pub cancelled: bool,
    pub now_ms: u64,
    pub events: broadcast::Sender<ArtifactEvent>,
}

impl ArtifactBootstrapControl for NativeBootstrapControl {
    fn is_cancelled(&mut self) -> bool {
        self.cancelled
    }

    fn now_ms(&mut self) -> u64 {
        self.now_ms
    }

    fn on_progress(&mut self, progress: ArtifactBootstrapProgress) {
        let _ = self.events.send(ArtifactEvent::BootstrapProgress { received_bytes: progress.received_bytes, total_bytes: progress.total_bytes, received_chunks: progress.received_chunks, total_chunks: progress.total_chunks });
    }
}

#[derive(Clone, Copy)]
enum ArtifactDrivePhase {
    Connect,
    Hub,
    ConnectResult,
    Watch,
    Reconnect,
    Folder,
    Backbone,
    Status,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArtifactDrive {
    MoreWork,
    Idle { deadline: Option<Instant> },
    Terminal,
}

/// 📁️ A folder/file binding's storage driver, keyed for multi-document sqlite or single
/// pack-backed blob. Both variants move real `pack`+`spr` bytes end to end — this actor never
/// touches JSON. `Sqlite` is fully codec-free (its row IS the pack+spr pair, no bridging
/// needed). `Pack` requires the schema codec for both authoritative DSL fallback validation
/// and synchronized text mirrors.
enum FolderEndpoint {
    EventLog { storage: FolderEventLogStorage, document_id: String, schema: String },
    Pack { storage: FolderTextStorage, document_id: String, extension: String, schema: String },
}

impl FolderEndpoint {
    /// 🗃️ Reads one exact versioned recursive-document archive.
    pub async fn read_archive(&self) -> Result<Option<Vec<u8>>, String> {
        match self {
            FolderEndpoint::EventLog { storage, document_id, .. } => storage.read_archive(document_id).await.map_err(|error| error.to_string()),
            FolderEndpoint::Pack { storage, document_id, extension, schema } => {
                if let Some(archive) = storage.read_archive(document_id, extension).await.map_err(|error| error.to_string())? {
                    return Ok(Some(archive));
                }
                let pair = match storage.read_pack(document_id, extension).await.map_err(|error| error.to_string())? {
                    Some(files) => Some((files.pack, files.spr)),
                    None => match storage.read(document_id, extension).await.map_err(|error| error.to_string())? {
                        Some(files) => {
                            let codec = semio_framework_os_store_codec_registry::document_codec(schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec registered for schema {schema:?} — cannot compile the DSL-only source"))?;
                            let (pack, _) = (codec.compile_dsl)(&files.dsl, &files.ops).await.map_err(|error| error.to_string())?;
                            Some((pack.pack, pack.spr))
                        }
                        None => None,
                    },
                };
                pair.map(|(parent_pack, parent_spr)| {
                    semio_framework_os_spr::encode_document_archive_bytes(&semio_framework_os_spr::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).map_err(|error| error.to_string())
                })
                .transpose()
            }
        }
    }

    /// 🗃️ Persists one exact versioned recursive-document archive and its root text mirrors.
    pub async fn write_archive(&self, archive_bytes: &[u8]) -> Result<(), String> {
        let archive = semio_framework_os_spr::decode_document_archive_bytes(archive_bytes).await.map_err(|error| error.to_string())?;
        match self {
            FolderEndpoint::EventLog { storage, document_id, schema } => storage.write_archive(document_id, schema, archive_bytes).await.map_err(|error| error.to_string()),
            FolderEndpoint::Pack { storage, document_id, extension, schema } => {
                let codec = semio_framework_os_store_codec_registry::document_kind_codec(schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec resolves schema {schema:?} — cannot persist synchronized archive mirrors"))?;
                let mirror = codec.print_mirror(&archive.parent_pack, &archive.parent_spr).await.map_err(|error| error.to_string())?;
                let pack_files = ArtifactPackFiles { pack: archive.parent_pack, spr: archive.parent_spr, ops: mirror.ops };
                storage.write_archive(document_id, extension, archive_bytes, &pack_files, &mirror.dsl).await.map_err(|error| error.to_string())
            }
        }
    }

}

/// 🎭️ One document's backbone actor: drains the store's outbound queue to persist + relay,
/// ingests semio_hub/file changes back into the store, and keeps subscribers current with status/events.
pub struct ArtifactActor {
    pub pool: Arc<semio_framework_async::WorkerPool>,
    pub document_id: String,
    pub schema: String,
    pub actor: String,
    pub actor_identity_grant: semio_framework_value::RetainedCloneGrant,
    pub actor_identity_progress: semio_framework_value::RetainedCloneProgress,
    pub remote: ChannelBackboneRemote,
    pub events: broadcast::Sender<ArtifactEvent>,
    pub cmd_rx: ArtifactMailboxReceiver,
    folder: Option<FolderEndpoint>,
    folder_watch_path: Option<PathBuf>,
    pub watch_external: bool,
    pub hub_base_url: Option<String>,
    pub hub_space_id: Option<String>,
    pub credential: Arc<std::sync::RwLock<Option<Arc<semio_framework_os_directory_client::LocalHubCredential>>>>,
    pub socket_grant_source: Arc<std::sync::RwLock<Option<Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>>>>,
    pub document_execution_target_lease: Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    pub operation_cancel: semio_framework_async::CancelToken,
    pub hub_surface: Option<String>,
    pub socket_actor: Option<semio_framework_value::SharedUtf8>,
    pub socket_actor_confirmed: bool,
    /// 🎨️ This connection's hub-assigned session color (`ServerFrame::Session.color`) —
    /// `None` until the hub sends it (or for a folder-only document, which never connects to a
    /// hub). Stamped onto every outbound `PresenceHeartbeat` via {@link stamp_session}.
    pub session_color: Option<u8>,
    pub semio_hub: Option<HubConn>,
    connect_future: Option<ConnectFuture>,
    /// 🏔️ Last frontier the semio_hub reported (`Welcome.server_frontier` / `Commands.frontier` /
    /// `Ack.frontier`) — the wire-v2 replacement for the old `hub_version: i64` counter.
    pub server_frontier: Option<RuntimeFrontierSummary>,
    /// 🎟️ The semio_hub's last `Welcome.resume_token`, echoed in the next `SocketHelloV1` after a
    /// reconnect so the semio_hub can resume rather than replay from scratch.
    pub resume_token: Option<String>,
    pub pending_resume_token: Option<String>,
    pub required_tail_frontier: Option<RuntimeFrontierSummary>,
    pub artifact_rebootstrap_required: bool,
    pub artifact_bootstrap: Option<PendingArtifactBootstrap>,
    /// 🔌️ The hub link ([`DocumentLink`]): `reconnect_at` and `link_expires_at` are its two deadlines as instants.
    pub link: DocumentLink,
    reconnect_at: Option<Instant>,
    link_expires_at: Option<Instant>,
    /// 🧺️ Outbound `Commands` batches awaiting an `Ack`, keyed by `batch_id`, so `Rejected`/
    /// `Transformed` can roll back exactly the envelopes that batch sent.
    pub pending_batches: std::collections::HashMap<u64, Vec<MutationEnvelope>>,
    pub outbox: Vec<MutationEnvelope>,
    pub document_backbone_retention: DocumentBackboneRetentionV1,
    pub next_batch_id: u64,
    pub next_local_rejection_batch_id: u64,
    /// ⏰️ This actor's `HybridLogicalTimestamp` seed (derived from `actor`) + logical tick
    /// counter, for {@link next_timestamp} on an external folder envelope without a clock.
    pub hlc_seed: u64,
    hlc_counter: u64,
    current_pack: Option<Vec<u8>>,
    current_spr: Option<Vec<u8>>,
    current_archive: Option<Vec<u8>>,
    known_op_ids: HashSet<String>,
    /// 🔁️ Every operation id this replica authored or applied — the echo filter ([`admit_remote_envelopes`]).
    pub applied_op_ids: HashSet<String>,
    last_written_hash: Option<String>,
    /// 🪞️ The hash of the archive the folder is known to hold — the last one read or written, none after a read that found
    /// the folder empty: an archive with these bytes is not written again.
    folder_held_hash: Option<String>,
    /// 🪹️ Whether this open's first folder read was answered, or an archive was read or written: no later read announces an
    /// absent archive ([`ArtifactEvent::DocumentArchiveAbsent`]).
    folder_bootstrapped: bool,
    #[cfg(test)]
    folder_writes: usize,
    pub remote_state: RemoteState,
    pub last_status: Option<ArtifactSyncStatus>,
    watcher: Option<semio_framework_os_services::OwnedFileChangeWatcher>,
    fs_deadline: Option<Instant>,
    pub started: bool,
    command_turns: u8,
    drive_phase: ArtifactDrivePhase,
    pub closing: bool,
    close_drain_deadline: Option<Instant>,
    readiness: Option<Arc<dyn Fn() + Send + Sync>>,
    readiness_requested: Arc<std::sync::atomic::AtomicBool>,
    #[cfg(test)]
    fail_bootstrap_local_replay_once: bool,
}

/// ⏳️ How long a closing actor keeps reading its hub socket so a catch-up in flight can complete
/// and flush operations the store handed over before the close; after it the outbox is dropped.
const ARTIFACT_CLOSE_OUTBOX_DRAIN: Duration = Duration::from_secs(2);

impl ArtifactActor {
    pub(super) async fn new(
        pool: Arc<semio_framework_async::WorkerPool>,
        config: ArtifactActorConfig,
        remote: ChannelBackboneRemote,
        cmd_rx: ArtifactMailboxReceiver,
        events: broadcast::Sender<ArtifactEvent>,
        credential: Arc<std::sync::RwLock<Option<Arc<semio_framework_os_directory_client::LocalHubCredential>>>>,
        socket_grant_source: Arc<std::sync::RwLock<Option<Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>>>>,
        document_execution_target_lease: Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
        operation_cancel: semio_framework_async::CancelToken,
    ) -> Self {
        let mut folder = None;
        let mut folder_watch_path = None;
        let mut hub_base_url = None;
        let mut hub_space_id = None;
        let mut hub_surface = None;
        for binding in &config.bindings {
            match binding {
                PersistenceBinding::Folder { path } => {
                    if folder.is_none() {
                        folder = Some(build_folder_endpoint(path, &config.document_id, &config.schema).await);
                        folder_watch_path = Some(folder_watch_path_for(path).await);
                    }
                }
                PersistenceBinding::Hub { base_url, space_id, surface } => {
                    if hub_base_url.is_none() {
                        hub_base_url = Some(base_url.clone());
                        hub_space_id = Some(space_id.clone());
                        hub_surface = surface.clone();
                    }
                }
            }
        }
        let hlc_seed = actor_seed(&config.actor).await;
        Self {
            pool,
            document_id: config.document_id,
            schema: config.schema,
            actor: config.actor,
            actor_identity_grant: config.actor_identity_grant,
            actor_identity_progress: Default::default(),
            remote,
            events,
            cmd_rx,
            folder,
            folder_watch_path,
            watch_external: config.watch_external,
            hub_base_url,
            hub_space_id,
            credential,
            socket_grant_source,
            document_execution_target_lease,
            operation_cancel,
            hub_surface,
            socket_actor: None,
            socket_actor_confirmed: false,
            session_color: None,
            semio_hub: None,
            connect_future: None,
            server_frontier: None,
            resume_token: None,
            pending_resume_token: None,
            required_tail_frontier: None,
            artifact_bootstrap: None,
            artifact_rebootstrap_required: false,
            link: DocumentLink::opened(wall_ms()),
            reconnect_at: None,
            link_expires_at: None,
            pending_batches: std::collections::HashMap::new(),
            outbox: Vec::new(),
            document_backbone_retention: DocumentBackboneRetentionV1::default(),
            next_batch_id: 0,
            next_local_rejection_batch_id: u64::MAX,
            hlc_seed,
            hlc_counter: 0,
            current_pack: None,
            current_spr: None,
            current_archive: None,
            known_op_ids: HashSet::new(),
            applied_op_ids: HashSet::new(),
            last_written_hash: None,
            folder_held_hash: None,
            folder_bootstrapped: false,
            #[cfg(test)]
            folder_writes: 0,
            remote_state: RemoteState::Detached,
            last_status: None,
            watcher: None,
            fs_deadline: None,
            started: false,
            command_turns: 0,
            drive_phase: ArtifactDrivePhase::Connect,
            closing: false,
            close_drain_deadline: None,
            readiness: None,
            readiness_requested: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            #[cfg(test)]
            fail_bootstrap_local_replay_once: false,
        }
    }

    fn set_readiness(&mut self, readiness: Arc<dyn Fn() + Send + Sync>) {
        self.readiness = Some(readiness);
    }

    /// 🪢️ Starts this hub document at the canonical checkpoint pair its shell verified and already loaded into the
    /// guest: the pair's history is known (a tail never re-applies it), a folder binding persists it, and the first
    /// `SocketHelloV1` names its baseline, so the hub sends only what followed it. A pair whose SPR cannot be read
    /// is reported as a conflict and the document starts unseeded.
    pub async fn seed_hub_document(&mut self, pair: semio_framework_os_directory::CanonicalCheckpointPairV1) {
        self.install_hub_seed(pair.pack_bytes, pair.spr_bytes, &pair.baseline_frontier).await;
    }

    /// 🛟️ Restarts a rebootstrapping hub document at the pair the host verified and loaded into the guest
    /// ([`ArtifactActorMsg::Reseed`]): the pair becomes the baseline, the unacked local operations the pair does not already
    /// hold go back to the guest, and the actor dials at once — no outage backoff, Hello at the baseline.
    pub async fn reseed_hub_document(&mut self, pack: Vec<u8>, spr: Vec<u8>, baseline: semio_framework_os_directory::ArtifactFrontier) {
        if !self.artifact_rebootstrap_required {
            return;
        }
        if !self.install_hub_seed(pack, spr, &baseline).await {
            return;
        }
        let known = &self.known_op_ids;
        self.outbox.retain(|envelope| !known.contains(&envelope.mutation_id.0));
        if !self.outbox.is_empty() {
            self.emit(ArtifactEvent::RemoteMutations { envelopes: self.outbox.clone() });
        }
        self.artifact_rebootstrap_required = false;
        self.reconnect_at = None;
        self.start_connect_hub().await;
    }

    /// 🪢️ Adopts one verified pair as this document's baseline; `false` (and one conflict) when its SPR cannot be read.
    async fn install_hub_seed(&mut self, pack: Vec<u8>, spr: Vec<u8>, baseline: &semio_framework_os_directory::ArtifactFrontier) -> bool {
        let seeded = match spr_op_ids(&spr).await {
            Ok(op_ids) => semio_framework_os_spr::encode_document_archive_bytes(&semio_framework_os_spr::DocumentArchivePack { parent_pack: pack.clone(), parent_spr: spr.clone(), members: Vec::new() }).map(|archive| (op_ids, archive)).map_err(|error| error.to_string()),
            Err(error) => Err(error),
        };
        let (op_ids, archive) = match seeded {
            Ok(seeded) => seeded,
            Err(error) => {
                self.emit(ArtifactEvent::Conflict(MutationMessage {
                    level: semio_framework_diagnostic::Severity::Error,
                    code: semio_framework_diagnostic::FaultCode::new("hubDocumentSeed"),
                    message: format!("the canonical checkpoint pair could not seed the document: {error}"),
                    target: vec![self.document_id.clone()],
                    op_index: None,
                }));
                return false;
            }
        };
        if let Some(folder) = self.folder.as_ref() {
            if folder.write_archive(&archive).await.is_ok() {
                self.last_written_hash = Some(document_archive_hash(&archive));
            }
        }
        self.known_op_ids = op_ids;
        self.server_frontier = Some(canonical_pair_baseline(baseline));
        self.current_pack = Some(pack);
        self.current_spr = Some(spr);
        self.current_archive = Some(archive);
        true
    }

    /// 🏃️ Advances exactly one command, readiness source, timer, backbone owner, or status turn.
    /// It idles only after observing an empty mailbox and an empty store outbound in the same turn:
    /// a command that relayed one backbone owner consumes that owner's wake, so a second queued
    /// owner has no wake left (ticket 26/09/23 `📓️wp-h6.md`).
    async fn drive_one(&mut self) -> ArtifactDrive {
        if !self.started {
            self.started = true;
            self.setup().await;
            return ArtifactDrive::MoreWork;
        }
        if self.closing {
            self.operation_cancel.cancel_now();
            self.document_backbone_retention.clear();
            if let Some(mut pending) = self.artifact_bootstrap.take() {
                pending.assembler.abort();
            }
            if self.cmd_rx.close_one() {
                return ArtifactDrive::MoreWork;
            }
            match self.relay_one_backbone().await {
                Ok(true) | Err(_) => return ArtifactDrive::MoreWork,
                Ok(false) => {}
            }
            if !self.outbox.is_empty() && self.semio_hub.is_some() {
                let deadline = *self.close_drain_deadline.get_or_insert_with(|| Instant::now() + ARTIFACT_CLOSE_OUTBOX_DRAIN);
                if Instant::now() < deadline {
                    return match self.poll_hub_message() {
                        Some(message) => {
                            self.on_hub_message(message).await;
                            ArtifactDrive::MoreWork
                        }
                        None => ArtifactDrive::Idle { deadline: Some(deadline) },
                    };
                }
            }
            self.connect_future = None;
            return ArtifactDrive::Terminal;
        }
        if self.command_turns < 32 {
            if let Some(message) = self.cmd_rx.try_recv() {
                self.command_turns += 1;
                if self.handle_cmd(message).await {
                    self.closing = true;
                    self.cmd_rx.close();
                }
                return ArtifactDrive::MoreWork;
            }
        }
        self.command_turns = 0;
        if self.readiness_requested.swap(false, std::sync::atomic::Ordering::AcqRel) {
            if self.connect_future.is_some() {
                self.drive_phase = ArtifactDrivePhase::ConnectResult;
            } else if self.semio_hub.is_some() {
                self.drive_phase = ArtifactDrivePhase::Hub;
            }
        }
        let phase = self.drive_phase;
        self.drive_phase = match phase {
            ArtifactDrivePhase::Connect => ArtifactDrivePhase::Hub,
            ArtifactDrivePhase::Hub => ArtifactDrivePhase::ConnectResult,
            ArtifactDrivePhase::ConnectResult => ArtifactDrivePhase::Watch,
            ArtifactDrivePhase::Watch => ArtifactDrivePhase::Reconnect,
            ArtifactDrivePhase::Reconnect => ArtifactDrivePhase::Folder,
            ArtifactDrivePhase::Folder => ArtifactDrivePhase::Backbone,
            ArtifactDrivePhase::Backbone => ArtifactDrivePhase::Status,
            ArtifactDrivePhase::Status => ArtifactDrivePhase::Connect,
        };
        match phase {
            ArtifactDrivePhase::Connect => self.start_connect_hub().await,
            ArtifactDrivePhase::Hub => {
                if let Some(message) = self.poll_hub_message() {
                    self.drive_phase = ArtifactDrivePhase::Hub;
                    self.on_hub_message(message).await;
                }
            }
            ArtifactDrivePhase::ConnectResult => {
                let connection = self.readiness.clone().and_then(|readiness| {
                    let waker = std::task::Waker::from(Arc::new(ArtifactReadinessWake(readiness)));
                    let mut context = std::task::Context::from_waker(&waker);
                    match self.connect_future.as_mut().map(|future| future.as_mut().poll(&mut context)) {
                        Some(std::task::Poll::Ready(connection)) => Some(connection),
                        Some(std::task::Poll::Pending) | None => None,
                    }
                });
                if let Some(connection) = connection {
                    self.connect_future = None;
                    self.finish_connect_hub(connection).await;
                    if self.semio_hub.is_some() {
                        self.drive_phase = ArtifactDrivePhase::Hub;
                    }
                }
            }
            ArtifactDrivePhase::Watch => {
                if self.watcher.as_mut().is_some_and(|watcher| watcher.poll_changed()) {
                    self.fs_deadline = Some(Instant::now() + Duration::from_millis(200));
                }
            }
            ArtifactDrivePhase::Reconnect => {
                if self.reconnect_at.is_some_and(|deadline| deadline <= Instant::now()) {
                    self.reconnect_at = None;
                    self.start_connect_hub().await;
                }
            }
            ArtifactDrivePhase::Folder => {
                if self.fs_deadline.is_some_and(|deadline| deadline <= Instant::now()) {
                    self.fs_deadline = None;
                    self.handle_external_change().await;
                }
            }
            ArtifactDrivePhase::Backbone => {
                if !matches!(self.relay_one_backbone().await, Ok(false)) {
                    self.drive_phase = ArtifactDrivePhase::Backbone;
                }
            }
            ArtifactDrivePhase::Status => {
                self.tick_link().await;
                self.emit_status_if_changed().await;
                if self.cmd_rx.close_handle().has_pending() {
                    return ArtifactDrive::MoreWork;
                }
                if !matches!(self.relay_one_backbone().await, Ok(false)) {
                    self.drive_phase = ArtifactDrivePhase::Backbone;
                    return ArtifactDrive::MoreWork;
                }
                return ArtifactDrive::Idle { deadline: [self.reconnect_at, self.link_expires_at, self.fs_deadline].into_iter().flatten().min() };
            }
        }
        ArtifactDrive::MoreWork
    }

    /// 📡️ Polls one hub frame under the actor's readiness waker; `None` parks until the socket wakes it.
    fn poll_hub_message(&mut self) -> Option<Option<Result<Message, tokio_tungstenite::tungstenite::Error>>> {
        let readiness = self.readiness.clone()?;
        let waker = std::task::Waker::from(Arc::new(ArtifactReadinessWake(readiness)));
        let mut context = std::task::Context::from_waker(&waker);
        match self.semio_hub.as_mut().map(|connection| connection.read.poll_next_unpin(&mut context)) {
            Some(std::task::Poll::Ready(message)) => Some(message),
            Some(std::task::Poll::Pending) | None => None,
        }
    }

    /// 🌱️ Seeds persistence state from any already-stored recursive archive and installs the file watcher. A bound folder
    /// whose bootstrap read finds no archive is announced once ([`ArtifactEvent::DocumentArchiveAbsent`]).
    async fn setup(&mut self) {
        let read = match self.folder.as_ref().filter(|_| self.current_pack.is_none()) {
            Some(folder) => Some(folder.read_archive().await),
            None => None,
        };
        if matches!(read, Some(Ok(None))) && !self.folder_bootstrapped {
            self.emit(ArtifactEvent::DocumentArchiveAbsent);
        }
        self.folder_bootstrapped |= matches!(read, Some(Ok(_)));
        let seeded = read.and_then(|read| read.ok().flatten());
        self.folder_held_hash = seeded.as_deref().map(document_archive_hash).or(self.folder_held_hash.take());
        if let Some(bytes) = seeded {
            if let Ok(archive) = semio_framework_os_spr::decode_document_archive_bytes(&bytes).await {
                let pack = archive.parent_pack;
                let spr = archive.parent_spr;
                if let Ok(op_ids) = spr_op_ids(&spr).await {
                self.known_op_ids = op_ids;
                self.last_written_hash = Some(document_archive_hash(&bytes));
                self.current_pack = Some(pack);
                self.current_spr = Some(spr);
                self.current_archive = Some(bytes);
                }
            }
        }
        if self.watch_external {
            if let Some(watch_path) = self.folder_watch_path.clone() {
                if let Some(readiness) = self.readiness.clone() {
                    self.watcher = Some(install_watcher(&watch_path, self.pool.clone(), readiness));
                }
            }
        }
    }

    /// 📨️ Handles a caller control message. Returns `true` when the actor should stop.
    pub async fn handle_cmd(&mut self, message: ArtifactActorMsg) -> bool {
        match message {
            ArtifactActorMsg::LocalMutations { envelopes } => {
                let drained = self.relay_one_backbone().await.unwrap_or(false);
                if !drained && !envelopes.is_empty() {
                    self.persist_operations(&envelopes).await;
                    self.relay_operations_to_hub(&envelopes).await;
                }
                false
            }
            ArtifactActorMsg::DocumentBackbone { message } => {
                let envelopes = match decode_document_backbone_message_exact(&message) {
                    Ok(envelopes) if envelopes.iter().all(|envelope| envelope.document_id.0 == self.document_id) => envelopes,
                    Ok(envelopes) => {
                        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneScopeMismatch, "document backbone scope mismatch", CommandRejectionDetail { envelopes: Some(envelopes.len() as u64), ..CommandRejectionDetail::default() }));
                        return false;
                    }
                    Err(_) => {
                        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneMalformed, "document backbone malformed", CommandRejectionDetail { bytes: Some(message.len() as u64), ..CommandRejectionDetail::default() }));
                        return false;
                    }
                };
                if let Err(refusal) = self.document_backbone_retention.retain(message.len(), &envelopes) {
                    self.reject_document_backbone(refusal);
                    return false;
                }
                self.persist_operations(&envelopes).await;
                if self.hub_space_id.is_some() {
                    self.relay_operations_to_hub(&envelopes).await;
                } else {
                    self.document_backbone_retention.release(&envelopes);
                }
                false
            }
            ArtifactActorMsg::LocalDocumentArchive { archive } => {
                if let Ok(decoded) = semio_framework_os_spr::decode_document_archive_bytes(&archive).await {
                    self.persist_archive(archive, decoded).await;
                }
                false
            }
            ArtifactActorMsg::PresenceHeartbeat { mut peer } => {
                stamp_session(&mut peer, self.session_color, self.hub_surface.as_deref()).await;
                self.send_client_frame(ClientFrame::Presence { peer: presence_to_bytes(&peer).await }, Lane::Preview).await;
                false
            }
            ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                self.send_client_frame(ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
                false
            }
            ArtifactActorMsg::ExternalChanged => {
                self.handle_external_change().await;
                false
            }
            ArtifactActorMsg::Reseed { pack, spr, baseline } => {
                self.reseed_hub_document(pack, spr, baseline).await;
                false
            }
            ArtifactActorMsg::Detach => true,
        }
    }

    pub fn reject_document_backbone(&mut self, outcome: CommandAckOutcome) {
        let batch_id = self.next_local_rejection_batch_id;
        self.next_local_rejection_batch_id = self.next_local_rejection_batch_id.wrapping_sub(1);
        self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome });
    }

    /// 📤️ Pops and advances exactly one store-to-actor FIFO owner.
    pub async fn relay_one_backbone(&mut self) -> Result<bool, semio_framework_os_vcs::VcsError> {
        let Some(message) = self.remote.try_pop_front()? else { return Ok(false) };
        match message {
            BackboneMessage::Mutations { envelopes } => {
                let envelopes = decode_envelopes(&envelopes).unwrap_or_default();
                self.persist_operations(&envelopes).await;
                self.relay_operations_to_hub(&envelopes).await;
            }
            BackboneMessage::Genesis { pack } => self.persist_genesis(pack).await,
            BackboneMessage::Ack { .. } => {}
            BackboneMessage::Member { .. } => return Err(semio_framework_os_vcs::VcsError::Backbone("a composed member requires its exact member transport lane".into())),
            BackboneMessage::Retract { .. } => return Err(semio_framework_os_vcs::VcsError::Backbone("a retraction flows from the actor to its store, never back".into())),
        }
        Ok(true)
    }

    //#region 🔖️Folder
    /// ✍️ Persists the current recursive archive to the folder binding and records the
    /// content hash for self-write suppression. A write failure (e.g. no `crate::os_store::serialization::ArtifactCodec`
    /// registered for this document's schema on the `Pack` endpoint — see `FolderEndpoint::write_archive`)
    /// is swallowed here the same way every other best-effort path in this actor already is, but
    /// deliberately does NOT record `last_written_hash` on failure — a false "persisted" mark
    /// would make `handle_external_change` mistake the still-stale on-disk content for a
    /// self-write and ignore a real external change.
    ///
    /// Bytes the folder is known to hold already — the last archive read or written — are not written again.
    async fn persist_write_archive(&mut self, archive: &[u8]) {
        let Some(folder) = self.folder.as_ref() else { return };
        let hash = document_archive_hash(archive);
        if self.folder_held_hash.as_deref() == Some(hash.as_str()) {
            return;
        }
        if folder.write_archive(archive).await.is_ok() {
            self.last_written_hash = Some(hash.clone());
            self.folder_held_hash = Some(hash);
            self.folder_bootstrapped = true;
            #[cfg(test)]
            {
                self.folder_writes += 1;
            }
        }
    }

    async fn persist_archive(&mut self, bytes: Vec<u8>, archive: semio_framework_os_spr::DocumentArchivePack) {
        if let Ok(op_ids) = spr_op_ids(&archive.parent_spr).await {
            self.known_op_ids = op_ids;
        }
        self.persist_write_archive(&bytes).await;
        self.current_pack = Some(archive.parent_pack);
        self.current_spr = Some(archive.parent_spr);
        self.current_archive = Some(bytes);
    }

    /// 🧬️ Adopts the store's genesis: an empty binding persists it as the document's
    /// initial snapshot with an event-free history; a bound document must share it, otherwise
    /// the store names a different document and the mismatch is reported, never merged.
    async fn persist_genesis(&mut self, pack: Vec<u8>) {
        if let Some(current) = self.current_pack.as_ref() {
            if *current != pack {
                self.emit(ArtifactEvent::Conflict(MutationMessage {
                    level: semio_framework_diagnostic::Severity::Error,
                    code: semio_framework_diagnostic::FaultCode::new("genesisMismatch"),
                    message: "the attached store names a different initial snapshot than the persisted document".into(),
                    target: vec![format!("folder://{}", self.document_id)],
                    op_index: None,
                }));
            }
            return;
        }
        let spr = semio_framework_os_store_text_io::text_format::empty_document_spr(&self.document_id, &self.schema).await;
        let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() };
        if let Ok(bytes) = semio_framework_os_spr::encode_document_archive_bytes(&archive) {
            self.persist_archive(bytes, archive).await;
        }
    }

    /// ➕️ Appends locally-applied operations to the persisted spr log (append-only),
    /// keeping the on-disk copy coherent so self-writes are never mistaken for external edits.
    async fn persist_operations(&mut self, envelopes: &[MutationEnvelope]) {
        if self.folder.is_none() {
            return;
        }
        let (Some(pack), Some(spr)) = (self.current_pack.clone(), self.current_spr.clone()) else { return };
        let mut new_edits: Vec<semio_framework_os_spr::HistoryEdit> = Vec::new();
        let mut new_transitions: Vec<semio_framework_os_spr::HistoryTransitionRecord> = Vec::new();
        for envelope in envelopes.iter().filter(|envelope| self.known_op_ids.insert(envelope.mutation_id.0.clone())) {
            if semio_framework_os_spr::is_history_transition(envelope) {
                new_transitions.push(semio_framework_os_spr::HistoryTransitionRecord::from_envelope(envelope));
            } else {
                new_edits.push(history_edit_from_envelope(envelope).await);
            }
        }
        if new_edits.is_empty() && new_transitions.is_empty() {
            return;
        }
        let Ok(new_spr) = semio_framework_os_store_text_io::text_format::append_history_events_to_spr(&spr, &new_edits, &new_transitions).await else { return };
        self.persist_spr(pack, new_spr).await;
    }

    /// 🪒️ Drops from the persisted log the transitions `refused` retracts — the same ones the store drops
    /// ([`crate::os_store::transition_retraction_closure`]) — so a reload never resurrects what the hub refused.
    async fn persist_retraction(&mut self, refused: &[String]) {
        if self.folder.is_none() {
            return;
        }
        let (Some(pack), Some(spr)) = (self.current_pack.clone(), self.current_spr.clone()) else { return };
        let Ok((spr, retracted)) = semio_framework_os_store_engine::lifted_store::retract_history_transitions_from_spr(&spr, refused).await else { return };
        for id in &retracted {
            self.known_op_ids.remove(id);
        }
        self.persist_spr(pack, spr).await;
    }

    /// 💾️ Persists `spr` as the recursive archive's parent log beside `pack`.
    async fn persist_spr(&mut self, pack: Vec<u8>, spr: Vec<u8>) {
        let mut archive = match self.current_archive.as_deref() {
            Some(bytes) => semio_framework_os_spr::decode_document_archive_bytes(bytes).await.ok(),
            None => None,
        }
        .unwrap_or(semio_framework_os_spr::DocumentArchivePack { parent_pack: pack.clone(), parent_spr: spr.clone(), members: Vec::new() });
        archive.parent_spr = spr.clone();
        if let Ok(bytes) = semio_framework_os_spr::encode_document_archive_bytes(&archive) {
            self.persist_write_archive(&bytes).await;
            self.current_archive = Some(bytes);
        }
        self.current_pack = Some(pack);
        self.current_spr = Some(spr);
    }

    /// 👁️ Re-reads the folder binding and classifies the change: append-only → `RemoteMutations`,
    /// divergence → `DocumentArchiveReplaced`, divergence with local pending operations → `Conflict`. Self-writes
    /// (content hash match) are ignored.
    ///
    /// Append-only spr events decoded without HistoryOpMeta carry HLT(0,0). Ingest merges
    /// by HLC, so a zero stamp would reorder under already-applied local edits and leave
    /// the live snapshot at the older head. Stamp each new envelope with this actor clock.
    ///
    /// The first answered read of an open that finds no archive, with none read or written before, announces
    /// [`ArtifactEvent::DocumentArchiveAbsent`] once; a later empty answer only forgets the archive the folder held.
    async fn handle_external_change(&mut self) {
        let read = match self.folder.as_ref() {
            Some(folder) => folder.read_archive().await,
            None => return,
        };
        let bytes = match read {
            Ok(Some(bytes)) => bytes,
            Ok(None) => {
                if !self.folder_bootstrapped {
                    self.emit(ArtifactEvent::DocumentArchiveAbsent);
                }
                self.folder_bootstrapped = true;
                self.folder_held_hash = None;
                return;
            }
            Err(_) => return,
        };
        self.folder_bootstrapped = true;
        let Ok(archive) = semio_framework_os_spr::decode_document_archive_bytes(&bytes).await else { return };
        let pack = archive.parent_pack.clone();
        let spr = archive.parent_spr.clone();
        let hash = document_archive_hash(&bytes);
        self.folder_held_hash = Some(hash.clone());
        if self.last_written_hash.as_deref() == Some(hash.as_str()) {
            return;
        }
        let Ok(file_ids) = spr_op_ids(&spr).await else { return };
        let lost: Vec<String> = self.known_op_ids.difference(&file_ids).cloned().collect();
        let new_ids: HashSet<String> = file_ids.difference(&self.known_op_ids).cloned().collect();

        if lost.is_empty() && !new_ids.is_empty() {
            let Ok(events) = spr_events(&spr, &self.document_id, &self.schema,self.actor_identity_grant).await else { return };
            let mut appended: Vec<MutationEnvelope> = events.into_iter().filter(|event| new_ids.contains(&event.mutation_id.0)).collect();
            for envelope in &mut appended {
                if envelope.timestamp == semio_framework_os_spr::HybridLogicalTimestamp::new(0, 0) {
                    envelope.timestamp = next_timestamp(self.hlc_seed, &mut self.hlc_counter).await;
                }
            }
            self.known_op_ids.extend(new_ids);
            self.current_pack = Some(pack);
            self.current_spr = Some(spr);
            self.current_archive = Some(bytes);
            self.last_written_hash = Some(hash);
            let _ = self.deliver_remote_operations(appended).await;
        } else if !lost.is_empty() {
            if !self.pending_batches.is_empty() {
                self.emit(ArtifactEvent::Conflict(MutationMessage {
                    level: semio_framework_diagnostic::Severity::Error,
                    code: semio_framework_diagnostic::FaultCode::new("externalDivergence"),
                    message: "external history diverged while local operations are pending".into(),
                    target: vec![format!("folder://{}", self.document_id)],
                    op_index: None,
                }));
            } else {
                self.known_op_ids = file_ids;
                self.current_pack = Some(pack.clone());
                self.current_spr = Some(spr.clone());
                self.current_archive = Some(bytes.clone());
                self.last_written_hash = Some(hash);
                self.deliver_archive(bytes).await;
            }
        }
    }
    //#endregion 🔖️Folder

    //#region 🔖️Hub
    pub fn bootstrap_control(&self, started_at: Instant) -> NativeBootstrapControl {
        NativeBootstrapControl { cancelled: self.closing, now_ms: started_at.elapsed().as_millis().min(u128::from(u64::MAX)) as u64, events: self.events.clone() }
    }

    pub fn abort_artifact_bootstrap(&mut self) {
        if let Some(mut pending) = self.artifact_bootstrap.take() {
            pending.assembler.abort();
        }
        self.pending_resume_token = None;
        self.required_tail_frontier = None;
    }

    pub fn requeue_pending_batches(&mut self) {
        let mut batches: Vec<(u64, Vec<MutationEnvelope>)> = self.pending_batches.drain().collect();
        batches.sort_by_key(|(batch_id, _)| *batch_id);
        for (_, envelopes) in batches {
            self.queue_outbox(envelopes);
        }
    }

    pub fn queue_outbox(&mut self, envelopes: impl IntoIterator<Item = MutationEnvelope>) {
        let mut queued: HashSet<String> = self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
        for envelope in envelopes {
            if queued.insert(envelope.mutation_id.0.clone()) {
                self.outbox.push(envelope);
            }
        }
    }

    pub fn clear_socket_epoch(&mut self) {
        self.socket_actor = None;
        self.socket_actor_confirmed = false;
        self.session_color = None;
    }

    pub async fn fail_artifact_bootstrap(&mut self, detail: impl Into<String>) {
        self.abort_artifact_bootstrap();
        self.requeue_pending_batches();
        self.emit(ArtifactEvent::Conflict(MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new("artifactBootstrap"), message: detail.into(), target: vec![self.document_id.clone()], op_index: None }));
        self.semio_hub = None;
        self.clear_socket_epoch();
        self.schedule_reconnect().await;
    }

    /// ♻️ Hub lag forces a canonical pair refresh: clear the live projection tokens, keep unacked work queued,
    /// drop the socket and ask the host for the pair ([`ArtifactEvent::RebootstrapRequired`]); the actor dials again only
    /// once the host re-seeded it ([`Self::reseed_hub_document`]) — never into a Welcome it would have to refuse, and never
    /// after an outage backoff (the rebuild's own close is not a shortage).
    pub async fn require_artifact_rebootstrap(&mut self, control: semio_framework_os_directory::RebootstrapRequired) {
        self.requeue_pending_batches();
        self.abort_artifact_bootstrap();
        self.current_pack = None;
        self.current_spr = None;
        self.current_archive = None;
        self.server_frontier = None;
        self.resume_token = None;
        self.pending_resume_token = None;
        self.required_tail_frontier = None;
        self.artifact_rebootstrap_required = true;
        self.known_op_ids.clear();
        self.semio_hub = None;
        self.clear_socket_epoch();
        self.reconnect_at = None;
        self.set_remote_state(RemoteState::Connecting).await;
        self.emit(ArtifactEvent::RebootstrapRequired { control });
    }

    pub async fn flush_outbox(&mut self) {
        if self.outbox.is_empty() || self.semio_hub.is_none() {
            return;
        }
        let envelopes = std::mem::take(&mut self.outbox);
        self.relay_operations_to_hub(&envelopes).await;
    }

    pub async fn finish_catchup_if_ready(&mut self) {
        let Some(required) = self.required_tail_frontier.clone() else { return };
        let Some(actual) = self.server_frontier.as_ref() else { return };
        if !frontier_reaches(actual, &required) {
            return;
        }
        self.required_tail_frontier = None;
        if let Some(resume_token) = self.pending_resume_token.take() {
            self.resume_token = Some(resume_token);
        }
        self.set_remote_state(RemoteState::Live { peer_count: 0 }).await;
        self.flush_outbox().await;
    }

    async fn start_connect_hub(&mut self) {
        let Some(base_url) = self.hub_base_url.clone() else { return };
        if self.artifact_rebootstrap_required || self.semio_hub.is_some() || self.connect_future.is_some() || !self.link.admits_local_edits() || self.reconnect_at.is_some_and(|deadline| deadline > Instant::now()) {
            return;
        }
        let Some(credential) = self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone() else {
            self.schedule_reconnect().await;
            return;
        };
        let Some(source) = self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone() else {
            self.schedule_reconnect().await;
            return;
        };
        let space_id = self.hub_space_id.clone().unwrap_or_default();
        let document_id = self.document_id.clone();
        let surface = self.hub_surface.clone();
        let schema = self.schema.clone();
        let Some(pack_schema_hash) = document_pack_schema_hash(&schema, self.document_execution_target_lease.as_ref()).await else {
            self.schedule_reconnect().await;
            return;
        };
        let expectation = semio_framework_os_directory_client::DocumentSocketExpectationV1 { artifact_schema: schema, pack_schema_hash, requested_surface_id: surface, lease: self.document_execution_target_lease.clone() };
        let client_instance_id = format!("native-document-{:016x}", self.hlc_seed);
        let actor_identity_grant=self.actor_identity_grant;
        if !actor_identity_admission_admits(actor_identity_grant){self.schedule_reconnect().await;return;}
        let operation_cancel = self.operation_cancel.child_now();
        let pool = self.pool.clone();
        self.set_remote_state(RemoteState::Connecting).await;
        self.connect_future = Some(Box::pin(async move {
            let ctx = semio_framework_async::OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 1, deadline_ms: None, cancel: operation_cancel, capability: None };
            let admission_ctx = ctx.clone();
            let (admission_sender, admission_receiver) = semio_framework_async::oneshot::channel();
            let admission_job = Box::new(move || {
                let admission = source.admit_document_socket(&admission_ctx, &space_id, &document_id, &expectation, &client_instance_id, 5_000);
                let _ = admission_sender.send(admission);
            });
            pool.submit_at(pool.now_ms(), semio_framework_async::Lane::Io, admission_job);
            let admission = admission_receiver.await.map_err(|_| DocumentConnectFailure::Short)?.map_err(|error| if document_admission_refuses_access(&error) { DocumentConnectFailure::Refused } else { DocumentConnectFailure::Short })?;
            if ctx.cancel.is_cancelled_now() || admission.authority.expires_at_unix_ms <= now_ms().await {
                return Err(DocumentConnectFailure::Short);
            }
            if base_url.trim_end_matches('/') != admission.authority.hub_origin.trim_end_matches('/') {
                return Err(DocumentConnectFailure::Short);
            }
            let semio_framework_os_directory_client::DocumentSocketAdmissionV1 { mut socket, authority } = admission;
            let url = hub_ws_url(
                &authority.hub_origin,
                &authority.scope.space_id,
                &authority.scope.document_id,
                Some(&authority.surface.surface_id),
            )
            .await;
            let mut request = url.into_client_request().map_err(|_| DocumentConnectFailure::Short)?;
            let session = credential.capability().map_err(|_| DocumentConnectFailure::Short)?;
            let protocol_header = format!("{}, {session}", socket.protocol);
            request.headers_mut().insert(
                tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL,
                protocol_header.parse().map_err(|_| DocumentConnectFailure::Short)?,
            );
            let (mut stream, _response) = tokio::time::timeout(Duration::from_secs(5), tokio_tungstenite::connect_async(request)).await.map_err(|_| DocumentConnectFailure::Short)?.map_err(|_| DocumentConnectFailure::Short)?;
            if ctx.cancel.is_cancelled_now() || authority.expires_at_unix_ms <= now_ms().await {
                let _ = stream.close(None).await;
                return Err(DocumentConnectFailure::Short);
            }
            let(socket_actor,identity_progress)=semio_framework_value::SharedUtf8::admit(std::mem::take(&mut socket.actor_id),actor_identity_grant).unwrap_or_else(|(error,_)|panic!("original pre-admitted socket actor invariant: {error}"));
            Ok(ConnectedDocumentSocket { stream, socket_actor, authority,identity_progress })
        }));
    }

    async fn finish_connect_hub(&mut self, connection: Result<ConnectedDocumentSocket, DocumentConnectFailure>) {
        match connection {
            Ok(ConnectedDocumentSocket { mut stream, socket_actor, authority,identity_progress }) => {
                add_actor_identity_progress(&mut self.actor_identity_progress,identity_progress);
                let local_schema_hash = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await;
                let now = now_ms().await;
                if self.operation_cancel.is_cancelled_now()
                    || authority.expires_at_unix_ms <= now
                    || authority.hub_origin.trim_end_matches('/') != self.hub_base_url.as_deref().unwrap_or_default().trim_end_matches('/')
                    || authority.scope.space_id != self.hub_space_id.as_deref().unwrap_or_default()
                    || authority.scope.document_id != self.document_id
                    || authority.artifact.schema != self.schema
                    || local_schema_hash != Some(authority.pack_schema_hash)
                    || self.hub_surface.as_ref().is_some_and(|surface| surface != &authority.surface.surface_id)
                    || self.document_execution_target_lease.as_ref().is_some_and(|lease| !authority.matches_lease_fields(lease))
                {
                    let _ = stream.close(None).await;
                    self.clear_socket_epoch();
                    self.schedule_reconnect().await;
                    return;
                }
                let pack_schema_hash = authority.pack_schema_hash;
                let (write, read) = stream.split();
                self.semio_hub = Some(HubConn { write, read });
                self.socket_actor = Some(socket_actor);
                self.socket_actor_confirmed = false;
                self.hub_surface = Some(authority.surface.surface_id.clone());
                self.session_color = None;
                let hello = ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: self.schema.clone(), pack_schema_hash, resume_token: self.resume_token.clone(), frontier: self.server_frontier.clone() };
                self.send_client_frame(hello, Lane::Command).await;
            }
            Err(DocumentConnectFailure::Refused) => {
                self.clear_socket_epoch();
                self.link_event(DocumentLinkEvent::Refused { now_ms: wall_ms() }).await;
            }
            Err(DocumentConnectFailure::Short) => {
                self.clear_socket_epoch();
                self.schedule_reconnect().await;
            }
        }
    }

    /// 🔁️ A failed attempt or a lost socket: the link backs off along [`DOCUMENT_LINK_SHORTAGE_POLICY`] — or
    /// expires, once the shortage outlasts its bound.
    pub async fn schedule_reconnect(&mut self) {
        if let Some(retry_in_ms) = self.fail_link().await {
            self.set_remote_state(RemoteState::Backoff { retry_in_ms }).await;
        }
    }

    /// 🔌️ Applies `Failed` to the link and arms its two deadlines; the retry delay while it stays unlinked.
    async fn fail_link(&mut self) -> Option<u64> {
        let now_ms = wall_ms();
        self.link_event(DocumentLinkEvent::Failed { now_ms }).await;
        let retry_at_ms = self.link.retry_at_ms()?;
        self.reconnect_at = Some(instant_at(retry_at_ms, now_ms));
        self.link_expires_at = self.link.expires_at_ms(&DOCUMENT_LINK_SHORTAGE_POLICY).map(|at_ms| instant_at(at_ms, now_ms));
        Some(retry_at_ms.saturating_sub(now_ms))
    }

    /// 🔌️ Applies one link event; a link that turns terminal drops its socket and deadlines, reports
    /// `Detached` and emits the coded terminal message a shell localizes ([`document_link_terminal_message`]).
    pub async fn link_event(&mut self, event: DocumentLinkEvent) {
        let was_open = self.link.admits_local_edits();
        self.link = self.link.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, event);
        if was_open && !self.link.admits_local_edits() {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            self.connect_future = None;
            if let Some(mut connection) = self.semio_hub.take() {
                let _ = tokio::time::timeout(Duration::from_millis(4), connection.write.close()).await;
            }
            self.clear_socket_epoch();
            self.reconnect_at = None;
            self.link_expires_at = None;
            self.set_remote_state(RemoteState::Detached).await;
            let _ = self.events.send(ArtifactEvent::Conflict(document_link_terminal_message(&self.document_id, self.link.status())));
        }
    }

    /// ⏳️ Lets time pass for an unlinked hub link: past the shortage bound it expires.
    pub async fn tick_link(&mut self) {
        if self.hub_base_url.is_some() && matches!(self.link, DocumentLink::Unlinked { .. }) {
            self.link_event(DocumentLinkEvent::Tick { now_ms: wall_ms() }).await;
        }
    }

    async fn on_hub_message(&mut self, message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>) {
        match message {
            Some(Ok(Message::Binary(bytes))) => match decode_server_frame(&bytes).await {
                Ok((_lane, frame)) => self.on_hub_frame(frame).await,
                Err(error) => self.fail_artifact_bootstrap(format!("malformed hub frame: {error}")).await,
            },
            Some(Ok(Message::Ping(payload))) => {
                self.send_raw(Message::Pong(payload)).await;
            }
            Some(Ok(_)) => {}
            Some(Err(_)) | None => {
                self.abort_artifact_bootstrap();
                self.requeue_pending_batches();
                self.semio_hub = None;
                self.clear_socket_epoch();
                self.schedule_reconnect().await;
            }
        }
    }

    pub async fn start_artifact_bootstrap(&mut self, bootstrap: ArtifactBootstrap, resume_token: String, server_frontier: RuntimeFrontierSummary) {
        self.abort_artifact_bootstrap();
        let Some(local_schema_hash) = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await else {
            self.fail_artifact_bootstrap(format!("no pack schema identity resolves schema {:?}", self.schema)).await;
            return;
        };
        if let Err(error) = validate_artifact_bootstrap_identity(&bootstrap, &self.document_id, &self.schema, local_schema_hash, &server_frontier) {
            self.fail_artifact_bootstrap(error).await;
            return;
        }
        let inline = bootstrap.inline.is_some();
        let started_at = Instant::now();
        let baseline_frontier = bootstrap.baseline_frontier.clone();
        let required_tail_frontier = bootstrap.required_tail_frontier.clone();
        let pack_schema_hash = bootstrap.pack_schema_hash;
        let mut control = self.bootstrap_control(started_at);
        let assembler = match ArtifactBootstrapAssembler::new(bootstrap.clone(), bootstrap.descriptor_hash, ArtifactBootstrapLimits::default(), Some(ARTIFACT_BOOTSTRAP_DEADLINE_MS), &mut control) {
            Ok(assembler) => assembler,
            Err(error) => {
                self.fail_artifact_bootstrap(error.to_string()).await;
                return;
            }
        };
        let mut pending = PendingArtifactBootstrap { assembler, started_at, resume_token, baseline_frontier, required_tail_frontier, pack_schema_hash };
        if !inline {
            self.artifact_bootstrap = Some(pending);
            return;
        }
        let mut control = self.bootstrap_control(started_at);
        match pending.assembler.finish(None, &mut control) {
            Ok(pair) => {
                if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                    self.fail_artifact_bootstrap(error).await;
                }
            }
            Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
        }
    }

    pub async fn install_artifact_bootstrap(&mut self, pending: PendingArtifactBootstrap, pair: ArtifactBootstrapPair) -> Result<(), String> {
        let codec = semio_framework_os_store_codec_registry::document_kind_codec(&self.schema).await.map_err(|error| error.to_string())?.ok_or_else(|| format!("no document codec resolves schema {:?}", self.schema))?;
        if codec.pack_schema_hash().await.map_err(|error| error.to_string())? != pending.pack_schema_hash {
            return Err("artifact bootstrap codec changed during transfer".into());
        }
        codec.print_mirror(&pair.pack, &pair.spr).await.map_err(|error| format!("artifact bootstrap decode failed: {error}"))?;
        let op_ids = spr_op_ids(&pair.spr).await.map_err(|error| format!("artifact bootstrap SPR failed: {error}"))?;
        let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: pair.pack.clone(), parent_spr: pair.spr.clone(), members: Vec::new() };
        let archive_bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).map_err(|error| format!("artifact bootstrap archive failed: {error}"))?;
        let previous = self.current_archive.clone();
        if let Some(folder) = self.folder.as_ref() {
            folder.write_archive(&archive_bytes).await.map_err(|error| format!("artifact bootstrap persistence failed: {error}"))?;
        }
        let events = spr_events(&pair.spr, &self.document_id, &self.schema,self.actor_identity_grant).await.map_err(|error| format!("artifact bootstrap SPR failed: {error}"))?;
        let pushed = match self.remote.push(BackboneMessage::Genesis { pack: pair.pack.clone() }).await {
            Ok(()) if !events.is_empty() => self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&events) }).await,
            result => result,
        };
        if let Err(error) = pushed {
            if let (Some(folder), Some(previous)) = (self.folder.as_ref(), previous) {
                let _ = folder.write_archive(&previous).await;
            }
            return Err(format!("artifact bootstrap event delivery failed: {error}"));
        }
        self.known_op_ids = op_ids;
        self.current_pack = Some(pair.pack.clone());
        self.current_spr = Some(pair.spr.clone());
        self.current_archive = Some(archive_bytes.clone());
        if self.folder.is_some() {
            self.last_written_hash = Some(document_archive_hash(&archive_bytes));
        }
        self.emit(ArtifactEvent::DocumentArchiveReplaced { archive: archive_bytes });
        self.server_frontier = Some(pending.baseline_frontier);
        self.pending_resume_token = Some(pending.resume_token);
        self.required_tail_frontier = Some(pending.required_tail_frontier);
        if !self.outbox.is_empty() && self.replay_local_outbox_after_bootstrap().await.is_err() {
            self.emit(ArtifactEvent::Conflict(MutationMessage {
                level: semio_framework_diagnostic::Severity::Error,
                code: semio_framework_diagnostic::FaultCode::new("artifactBootstrapLocalReplay"),
                message: "artifact baseline committed; pending local replay will retry after reconnect".into(),
                target: vec![self.document_id.clone()],
                op_index: None,
            }));
            self.semio_hub = None;
            self.clear_socket_epoch();
            self.schedule_reconnect().await;
            return Ok(());
        }
        self.finish_catchup_if_ready().await;
        Ok(())
    }

    async fn replay_local_outbox_after_bootstrap(&mut self) -> Result<(), semio_framework_os_vcs::VcsError> {
        #[cfg(test)]
        if std::mem::take(&mut self.fail_bootstrap_local_replay_once) {
            return Err(semio_framework_os_vcs::VcsError::Backbone("injected bootstrap local replay failure".into()));
        }
        self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&self.outbox) }).await
    }

    #[cfg(test)]
    pub async fn inject_hub_frame(&mut self, frame: ServerFrame) {
        self.on_hub_frame(frame).await;
    }

    #[cfg(test)]
    pub async fn handle_test_cmd(&mut self, message: ArtifactActorMsg) -> bool {
        self.handle_cmd(message).await
    }

    #[cfg(test)]
    pub fn inject_bootstrap_local_replay_failure(&mut self) {
        self.fail_bootstrap_local_replay_once = true;
    }

    #[cfg(test)]
    pub fn cancel_test_bootstrap(&mut self) {
        self.abort_artifact_bootstrap();
    }

    #[cfg(test)]
    pub fn queue_test_outbox(&mut self, envelopes: Vec<MutationEnvelope>) {
        self.queue_outbox(envelopes);
    }

    #[cfg(test)]
    pub fn install_test_socket_actor(&mut self, actor: &str) {
        let(original,receipt)=semio_framework_value::SharedUtf8::admit(actor.to_owned(),self.actor_identity_grant).unwrap_or_else(|(error,_)|panic!("original fixture actor admission: {error}"));add_actor_identity_progress(&mut self.actor_identity_progress,receipt);self.socket_actor = Some(original);
        self.socket_actor_confirmed = false;
    }

    #[cfg(test)]
    pub async fn connect_test_socket(&mut self, url: &str, actor: &str) {
        let (stream, _) = tokio_tungstenite::connect_async(url).await.expect("test socket connects");
        let (write, read) = stream.split();
        self.semio_hub = Some(HubConn { write, read });
        let(original,receipt)=semio_framework_value::SharedUtf8::admit(actor.to_owned(),self.actor_identity_grant).unwrap_or_else(|(error,_)|panic!("original fixture actor admission: {error}"));add_actor_identity_progress(&mut self.actor_identity_progress,receipt);self.socket_actor = Some(original);
        self.socket_actor_confirmed = false;
        self.session_color = None;
        self.send_client_frame(
            ClientFrame::SocketHelloV1 {
                wire_version: 1,
                protocol_version: 1,
                schema: self.schema.clone(),
                pack_schema_hash: semio_framework_os_store_codec_registry::document_codec(&self.schema).await.ok().flatten().map_or([0u8; 32], |codec| codec.pack_schema_hash),
                resume_token: self.resume_token.clone(),
                frontier: self.server_frontier.clone(),
            },
            Lane::Command,
        )
        .await;
    }

    #[cfg(test)]
    pub fn socket_epoch_test_state(&self) -> (Option<String>, bool, usize, Vec<String>) {
        (self.socket_actor.as_ref().map(|actor|actor.as_str().to_owned()), self.socket_actor_confirmed, self.pending_batches.len(), self.outbox.iter().map(|envelope| envelope.actor.0.as_str().to_owned()).collect())
    }

    #[cfg(test)]
    pub async fn fail_test_connection(&mut self) {
        self.on_hub_message(None).await;
    }

    #[cfg(test)]
    pub async fn fail_test_bootstrap(&mut self) {
        self.fail_artifact_bootstrap("injected bootstrap failure").await;
    }

    #[cfg(test)]
    pub async fn run_test_connect_attempt(&mut self) {
        self.start_connect_hub().await;
        if let Some(future) = self.connect_future.take() {
            let outcome = future.await;
            self.finish_connect_hub(outcome).await;
        }
    }

    #[cfg(test)]
    pub async fn relay_test_envelope(&mut self, envelope: MutationEnvelope) {
        self.relay_operations_to_hub(std::slice::from_ref(&envelope)).await;
    }

    #[cfg(test)]
    pub async fn handle_test_document_archive(&mut self, archive: Vec<u8>) {
        let _ = self.handle_cmd(ArtifactActorMsg::LocalDocumentArchive { archive }).await;
    }

    #[cfg(test)]
    pub fn current_archive_test(&self) -> Option<Vec<u8>> {
        self.current_archive.clone()
    }

    #[cfg(test)]
    pub async fn setup_test(&mut self) {
        self.setup().await;
    }

    #[cfg(test)]
    pub fn folder_writes_test(&self) -> usize {
        self.folder_writes
    }

    #[cfg(test)]
    pub fn bootstrap_test_state(&self) -> (Option<Vec<u8>>, Option<Vec<u8>>, Option<RuntimeFrontierSummary>, Option<RuntimeFrontierSummary>, Option<String>, Option<String>, RemoteState, Vec<String>, bool) {
        (
            self.current_pack.clone(),
            self.current_spr.clone(),
            self.server_frontier.clone(),
            self.required_tail_frontier.clone(),
            self.resume_token.clone(),
            self.pending_resume_token.clone(),
            self.remote_state.clone(),
            self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect(),
            self.artifact_rebootstrap_required,
        )
    }

    #[cfg(test)]
    pub fn parity_test_state(&self) -> (Option<String>, Option<String>, RemoteState, Vec<String>, usize, bool, bool, Vec<String>) {
        (
            self.resume_token.clone(),
            self.server_frontier.as_ref().map(|frontier| frontier.head_edit_id.clone()),
            self.remote_state.clone(),
            self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect(),
            self.pending_batches.len(),
            self.socket_actor_confirmed,
            self.artifact_rebootstrap_required,
            self.known_op_ids.union(&self.applied_op_ids).cloned().collect(),
        )
    }

    /// 🪙️ Command-lane credit-based flow control: no client-side backpressure
    /// implemented this wave (scope is frame plumbing, not congestion control) —
    /// accepted and ignored.
    async fn on_hub_frame(&mut self, frame: ServerFrame) {
        match frame {
            ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                self.requeue_pending_batches();
                match bootstrap {
                    Bootstrap::None => {
                        self.abort_artifact_bootstrap();
                        self.resume_token = Some(resume_token);
                        self.server_frontier = Some(server_frontier);
                        self.set_remote_state(RemoteState::Live { peer_count: 0 }).await;
                        self.flush_outbox().await;
                    }
                    Bootstrap::Tail => {
                        self.abort_artifact_bootstrap();
                        self.pending_resume_token = Some(resume_token);
                        self.required_tail_frontier = Some(server_frontier);
                        self.finish_catchup_if_ready().await;
                    }
                    Bootstrap::Snapshot { .. } => {
                        self.fail_artifact_bootstrap("database-private snapshot cannot seed an artifact client").await;
                    }
                    Bootstrap::ArtifactBootstrap(bootstrap) => {
                        self.start_artifact_bootstrap(*bootstrap, resume_token, server_frontier).await;
                    }
                }
            }
            ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {
                self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client").await;
            }
            ServerFrame::RebootstrapRequired { control } => {
                if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                    self.fail_artifact_bootstrap("rebootstrap control scope mismatch").await;
                } else {
                    self.require_artifact_rebootstrap(rebootstrap_control(&control)).await;
                }
            }
            ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index, bytes } => {
                let Some(mut pending) = self.artifact_bootstrap.take() else {
                    self.fail_artifact_bootstrap("artifact bootstrap chunk arrived without an active transfer").await;
                    return;
                };
                let mut control = self.bootstrap_control(pending.started_at);
                match pending.assembler.push(descriptor_hash, index, bytes.as_slice(), &mut control) {
                    Ok(_) => self.artifact_bootstrap = Some(pending),
                    Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
                }
            }
            ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count } => {
                let Some(mut pending) = self.artifact_bootstrap.take() else {
                    self.fail_artifact_bootstrap("artifact bootstrap completion arrived without an active transfer").await;
                    return;
                };
                let mut control = self.bootstrap_control(pending.started_at);
                match pending.assembler.finish(Some((descriptor_hash, chunk_count)), &mut control) {
                    Ok(pair) => {
                        if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                            self.fail_artifact_bootstrap(error).await;
                        }
                    }
                    Err(error) => self.fail_artifact_bootstrap(error.to_string()).await,
                }
            }
            ServerFrame::Commands { envelopes, frontier, .. } => {
                if self.artifact_bootstrap.is_some() {
                    self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion").await;
                    return;
                }
                let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                self.document_backbone_retention.release(&settled);
                note_authored_envelopes(&mut self.applied_op_ids, &settled);
                let persisted = &self.known_op_ids;
                let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes.into_iter().filter(|envelope| !persisted.contains(&envelope.mutation_id.0)));
                if !fresh.is_empty() {
                    self.persist_operations(&fresh).await;
                    if !self.deliver_remote_operations(fresh).await {
                        self.fail_artifact_bootstrap("artifact tail could not be installed").await;
                        return;
                    }
                }
                self.server_frontier = Some(frontier);
                self.finish_catchup_if_ready().await;
            }
            ServerFrame::Ack { batch_id, stages, frontier } => {
                if self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                    self.fail_artifact_bootstrap("ack arrived before artifact catch-up completion").await;
                    return;
                }
                self.server_frontier = Some(frontier);
                self.handle_ack(batch_id, stages).await;
            }
            ServerFrame::Preview { actor, key, seq, payload } => {
                if self.socket_actor.as_deref() != Some(actor.0.as_str()) {
                    self.emit(ArtifactEvent::Preview { actor: actor.0, key, seq, payload });
                }
            }
            ServerFrame::Presence { peers } => {
                let mut decoded: Vec<PresencePeer> = Vec::new();
                for p in &peers {
                    if let Some(peer) = presence_from_bytes(p).await {
                        decoded.push(peer);
                    }
                }
                let peers = decoded;
                if matches!(self.remote_state, RemoteState::Live { .. }) && self.artifact_bootstrap.is_none() && self.required_tail_frontier.is_none() {
                    self.set_remote_state(RemoteState::Live { peer_count: peers.len() }).await;
                }
                self.emit(ArtifactEvent::Presence { peers });
            }
            ServerFrame::Session { actor, color } => {
                if self.socket_actor.as_deref() != Some(actor.as_str()) {
                    self.fail_artifact_bootstrap("socket receipt actor mismatch").await;
                    return;
                }
                self.socket_actor_confirmed = true;
                self.session_color = Some(color);
                self.emit(ArtifactEvent::Session { actor, color });
                self.flush_outbox().await;
            }
            ServerFrame::CreditGrant { .. } => {
            }
            ServerFrame::Error { code, message } => {
                self.emit(ArtifactEvent::Conflict(MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new(code), message, target: vec![self.hub_base_url.clone().unwrap_or_default()], op_index: None }));
            }
        }
    }

    /// 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage: `Accepted`
    /// just clears the pending batch; `Transformed`/`Rejected` both roll back the speculative
    /// local head first (via {@link rollback_envelope}, replayed as remote operations), and
    /// `Transformed` then delivers the semio_hub's replacement envelope the same way.
    pub async fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
        for stage in stages {
            let AckStage::Applied { outcome } = stage else { continue };
            let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
            self.document_backbone_retention.release(&sent);
            match *outcome {
                ApplyOutcome::Accepted => {
                    self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                }
                ApplyOutcome::Transformed { envelope } => {
                    let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                    for envelope in sent.iter().rev() {
                        rollbacks.extend(rollback_envelope(envelope).await);
                    }
                    self.persist_operations(&rollbacks).await;
                    let _ = self.deliver_remote_operations(rollbacks).await;
                    let refusal = self.retract_refused_transitions(&sent, "transformed").await;
                    let converted = *envelope;
                    self.persist_operations(std::slice::from_ref(&converted)).await;
                    let _ = self.deliver_remote_operations(vec![converted]).await;
                    self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    if let Some(refusal) = refusal {
                        self.emit(ArtifactEvent::Conflict(refusal));
                    }
                }
                ApplyOutcome::Rejected { reason, messages } => {
                    let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                    for envelope in sent.iter().rev() {
                        rollbacks.extend(rollback_envelope(envelope).await);
                    }
                    self.persist_operations(&rollbacks).await;
                    let _ = self.deliver_remote_operations(rollbacks).await;
                    let refusal = self.retract_refused_transitions(&sent, &reason).await;
                    self.emit(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::hub_rejected(reason, &messages) });
                    if let Some(refusal) = refusal {
                        self.emit(ArtifactEvent::Conflict(refusal));
                    }
                }
            }
        }
        self.emit_status_if_changed().await;
    }

    /// 🔙️ Retracts the history transitions of `refused`, a batch the hub did not take as sent: the persisted log and
    /// the store drop them ([`transition_refusal`]); answers the typed refusal naming them.
    pub async fn retract_refused_transitions(&mut self, refused: &[MutationEnvelope], reason: &str) -> Option<MutationMessage> {
        let refusal = transition_refusal(refused, reason)?;
        self.persist_retraction(&refusal.target).await;
        let _ = self.deliver_retraction(refusal.target.clone()).await;
        Some(refusal)
    }

    async fn relay_operations_to_hub(&mut self, envelopes: &[MutationEnvelope]) {
        if envelopes.is_empty() {
            return;
        }
        note_authored_envelopes(&mut self.applied_op_ids, envelopes);
        let Some(socket_actor) = self.socket_actor.as_ref() else {
            self.queue_outbox(envelopes.iter().cloned());
            return;
        };
        if !self.socket_actor_confirmed || self.semio_hub.is_none() || self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
            self.queue_outbox(envelopes.iter().cloned());
            return;
        }
        let batch_id = self.next_batch_id;
        self.next_batch_id = self.next_batch_id.wrapping_add(1);
        let(wire_envelopes,identity_receipt)=match wire_envelopes_with_original_actor(envelopes,socket_actor,self.actor_identity_grant){Ok(result)=>result,Err(_)=>{self.queue_outbox(envelopes.iter().cloned());return;}};
        add_actor_identity_progress(&mut self.actor_identity_progress,identity_receipt);
        self.pending_batches.insert(batch_id, envelopes.to_vec());
        self.send_client_frame(ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command).await;
        self.emit_status_if_changed().await;
    }

    async fn send_client_frame(&mut self, frame: ClientFrame, lane: Lane) {
        let bytes = encode_client_frame(&frame, lane).await;
        self.send_raw(Message::Binary(bytes.into())).await;
    }

    async fn send_raw(&mut self, message: Message) {
        let mut failed = false;
        if let Some(conn) = self.semio_hub.as_mut() {
            if conn.write.send(message).await.is_err() {
                failed = true;
            }
        }
        if failed {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            self.semio_hub = None;
            self.clear_socket_epoch();
            self.schedule_reconnect().await;
        }
    }
    //#endregion 🔖️Hub

    //#region 🔖️Deliver
    /// 🕸️ Pushes remote operations into the store's inbound queue and notifies subscribers.
    pub async fn deliver_remote_operations(&mut self, envelopes: Vec<MutationEnvelope>) -> bool {
        if envelopes.is_empty() {
            return true;
        }
        if self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) }).await.is_err() {
            return false;
        }
        if self.hub_space_id.is_some() {
            let message = match (BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) }).encode_op() {
                Ok(message) => message,
                Err(_) => return false,
            };
            self.emit(ArtifactEvent::DocumentBackbone { message });
        } else {
            self.emit(ArtifactEvent::RemoteMutations { envelopes });
        }
        true
    }

    /// 🛎️ Asks the store to retract `mutation_ids` (`BackboneMessage::Retract`), through the document backbone too when
    /// a hub space binds this document.
    pub async fn deliver_retraction(&mut self, mutation_ids: Vec<String>) -> bool {
        let message = BackboneMessage::Retract { mutation_ids };
        let encoded = message.encode_op();
        if self.remote.push(message).await.is_err() {
            return false;
        }
        if self.hub_space_id.is_some() {
            let Ok(message) = encoded else { return false };
            self.emit(ArtifactEvent::DocumentBackbone { message });
        }
        true
    }

    /// 🗃️ Publishes a replaced recursive archive. An external rewrite that LOST events cannot be
    /// expressed as new events, so it is a document replacement: the host reloads the store from
    /// the archive by replaying its events — nothing is pushed into (or merged by) the live store.
    async fn deliver_archive(&mut self, bytes: Vec<u8>) {
        self.emit(ArtifactEvent::DocumentArchiveReplaced { archive: bytes });
    }

    /// 🚫️async: E1 pure sync body — `broadcast::Sender::send` never suspends. Declaring this
    /// `async fn` forced `&ArtifactActor` to live across an await, which demanded
    /// `ArtifactActor: Sync` from every `!Sync` field it owns (the folder watcher's `Receiver`,
    /// `connect_future`, the codec table's `dyn Fn`) and made the whole actor turn non-`Send`.
    fn emit(&self, event: ArtifactEvent) {
        let _ = self.events.send(event);
    }

    /// 🚫️async: E1 pure sync body — see `emit` above; same `&self`-across-await mechanism.
    pub fn status(&self) -> ArtifactSyncStatus {
        let pending_mutations = self.pending_batches.values().map(Vec::len).sum();
        let acknowledged_head = if pending_mutations == 0 && self.outbox.is_empty() { self.server_frontier.as_ref().and_then(acknowledged_edited_head) } else { None };
        ArtifactSyncStatus { persisted: self.last_written_hash.is_some() || self.server_frontier.is_some(), pending_mutations, remote: self.remote_state.clone(), acknowledged_head }
    }

    pub async fn set_remote_state(&mut self, state: RemoteState) {
        if matches!(state, RemoteState::Live { .. }) {
            self.link = self.link.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, DocumentLinkEvent::Restored { now_ms: wall_ms() });
            self.link_expires_at = None;
        }
        self.remote_state = state;
        self.emit_status_if_changed().await;
    }

    pub async fn emit_status_if_changed(&mut self) {
        let status = self.status();
        if self.last_status.as_ref() != Some(&status) {
            self.last_status = Some(status.clone());
            self.emit(ArtifactEvent::Status(status));
        }
    }
    //#endregion 🔖️Deliver
}

/// 🔀️ A binding path with a file extension addresses one document's text blob directly
/// (`Text`, generalizing the deleted single-file `FileJsonStorage` beyond `.json`); an extensionless
/// directory path is the canonical multi-document append-only store (`EventLog`).
async fn build_folder_endpoint(path: &Path, document_id: &str, schema: &str) -> FolderEndpoint {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(extension) => {
            let folder = path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
            FolderEndpoint::Pack { storage: FolderTextStorage::new(folder).await, document_id: document_id.to_string(), extension: extension.to_string(), schema: schema.to_string() }
        }
        None => FolderEndpoint::EventLog { storage: FolderEventLogStorage::new(path.to_path_buf()), document_id: document_id.to_string(), schema: schema.to_string() },
    }
}

/// 📍️ The on-disk path a folder binding writes to: the `<document_id>.<extension>` text blob
/// itself, or the multi-document sqlite db under `<folder>/.semio/documents.db`.
async fn folder_watch_path_for(path: &Path) -> PathBuf {
    if path.extension().is_some() {
        path.to_path_buf()
    } else {
        path.join(".semio").join("documents.db")
    }
}

/// 👁️ Creates the owned non-recursive watcher; its first probe establishes a
/// baseline and later snapshots feed the actor's existing 200 ms debounce.
fn install_watcher(watch_path: &Path, pool: Arc<semio_framework_async::WorkerPool>, readiness: Arc<dyn Fn() + Send + Sync>) -> semio_framework_os_services::OwnedFileChangeWatcher {
    semio_framework_os_services::OwnedFileChangeWatcher::new(watch_path, pool, readiness)
}

const ACTOR_RUNNER_RETRY_MS: u64 = 1;
const ACTOR_RUNNER_RETRY_LIMIT: u8 = 8;

type ActorTurnFuture = std::pin::Pin<Box<dyn std::future::Future<Output = (Box<ArtifactActor>, ArtifactDrive)> + Send>>;

enum ActorTurnOwner {
    Actor(Box<ArtifactActor>),
    Future(ActorTurnFuture),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactActorTerminalReason {
    Detached,
    Cancelled,
    TurnFault,
    Pool(semio_framework_async::WorkerSubmitErrorKind),
}

struct ActorTerminalJob {
    pub reason: ArtifactActorTerminalReason,
    job: semio_framework_async::Job,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactActorTerminalState {
    Live,
    Closing(ArtifactActorTerminalReason),
    Complete(ArtifactActorTerminalReason),
}

#[derive(Clone)]
pub struct ArtifactActorRunnerHandle {
    generation: u64,
    runner: Arc<ActorRunner>,
}

pub struct ArtifactActorRunnerTicket {
    pub generation: u64,
    pub runner: std::sync::Weak<ActorRunner>,
    pub host: Option<Arc<std::sync::Mutex<ArtifactHostState<ArtifactActorRunnerHandle>>>>,
    pub returned: bool,
}

pub struct ArtifactActorTerminalJob {
    handle: ArtifactActorRunnerHandle,
    pub owner: Option<ActorTerminalJob>,
}

struct ActorRunner {
    pub pool: Arc<semio_framework_async::WorkerPool>,
    io_reactor: Option<tokio::runtime::Handle>,
    pub generation: u64,
    turn: std::sync::Mutex<Option<ActorTurnOwner>>,
    terminal_turn: std::sync::Mutex<Option<ActorTurnOwner>>,
    mailbox: ArtifactMailboxClose,
    scheduled: std::sync::atomic::AtomicBool,
    wake_requested: std::sync::atomic::AtomicBool,
    turn_generation: std::sync::atomic::AtomicU64,
    deadline_armed: std::sync::atomic::AtomicBool,
    deadline_generation: std::sync::atomic::AtomicU64,
    deadline_at_ms: std::sync::atomic::AtomicU64,
    retry_armed: std::sync::atomic::AtomicBool,
    retry_generation: std::sync::atomic::AtomicU64,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    terminal_job: std::sync::Mutex<Option<ActorTerminalJob>>,
    terminal_reason: std::sync::Mutex<Option<ArtifactActorTerminalReason>>,
    terminal_empty_callback: std::sync::Mutex<Option<Arc<dyn Fn(u64) + Send + Sync>>>,
    self_retained: std::sync::Mutex<Option<Arc<ActorRunner>>>,
    external_tickets: std::sync::atomic::AtomicUsize,
    close_requested: std::sync::atomic::AtomicBool,
    pub terminal: std::sync::atomic::AtomicBool,
    pub complete: std::sync::atomic::AtomicBool,
}

impl ArtifactActorRunnerHandle {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn cancel(&self) {
        self.runner.cancel();
    }

    pub fn terminal_state(&self) -> ArtifactActorTerminalState {
        let reason = *self.runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match reason {
            None => ArtifactActorTerminalState::Live,
            Some(reason) if self.runner.complete.load(std::sync::atomic::Ordering::Acquire) => ArtifactActorTerminalState::Complete(reason),
            Some(reason) => ArtifactActorTerminalState::Closing(reason),
        }
    }

    pub fn close_step(&self) -> bool {
        if !self.runner.terminal.load(std::sync::atomic::Ordering::Acquire) {
            self.runner.request_close();
            return false;
        }
        let advanced = self.runner.close_one_terminal_owner();
        if !advanced && self.runner.terminal_is_empty() {
            self.runner.finish_terminal();
        }
        advanced
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.runner.terminal_is_empty()
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactActorTerminalJob> {
        self.runner.take_terminal_job().map(|owner| ArtifactActorTerminalJob { handle: self.clone(), owner: Some(owner) })
    }

    pub(super) fn start(&self) {
        self.runner.schedule();
    }

    pub(super) fn request_close(&self) {
        self.runner.request_close();
    }

    pub(super) fn issue_ticket(&self, host: Arc<std::sync::Mutex<ArtifactHostState<ArtifactActorRunnerHandle>>>) -> ArtifactActorRunnerTicket {
        self.runner.external_tickets.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| count.checked_add(1)).expect("artifact actor ticket capacity exhausted");
        ArtifactActorRunnerTicket { generation: self.generation, runner: Arc::downgrade(&self.runner), host: Some(host), returned: false }
    }

    pub(super) fn set_terminal_empty_callback(&self, callback: Arc<dyn Fn(u64) + Send + Sync>) {
        *self.runner.terminal_empty_callback.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(callback);
        if self.runner.terminal_is_empty() {
            self.runner.finish_terminal();
        }
    }
}

impl ArtifactActorRunnerTicket {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn terminal_state(&self) -> Option<ArtifactActorTerminalState> {
        self.runner.upgrade().map(|runner| ArtifactActorRunnerHandle { generation: self.generation, runner }.terminal_state())
    }

    pub fn return_to_host(mut self) {
        self.return_once();
    }

    fn return_once(&mut self) {
        if self.returned {
            return;
        }
        self.returned = true;
        if let Some(runner) = self.runner.upgrade() {
            if runner.generation == self.generation {
                runner.return_ticket();
            }
        }
        self.host.take();
    }
}

impl Drop for ArtifactActorRunnerTicket {
    fn drop(&mut self) {
        self.return_once();
    }
}

impl ArtifactActorTerminalJob {
    pub fn reason(&self) -> ArtifactActorTerminalReason {
        self.owner.as_ref().expect("terminal job owner already resolved").reason
    }

    pub fn resume(mut self) {
        let owner = self.owner.take().expect("terminal job owner already resolved");
        self.handle.runner.resume_terminal_job(owner);
    }

    pub fn close(mut self) {
        let owner = self.owner.take().expect("terminal job owner already resolved");
        self.handle.runner.close_terminal_job(owner);
    }
}

impl Drop for ArtifactActorTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            self.handle.runner.return_terminal_job(owner);
        }
    }
}

struct ActorTurnWake {
    pub runner: std::sync::Weak<ActorRunner>,
    pub generation: u64,
}

impl std::task::Wake for ActorTurnWake {
    fn wake(self: Arc<Self>) {
        if let Some(runner) = self.runner.upgrade() {
            runner.request_wake(self.generation);
        }
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(runner) = self.runner.upgrade() {
            runner.request_wake(self.generation);
        }
    }
}

impl ActorRunner {
    /// 📬️ A mailbox, readiness or close wake targets whichever turn is current: it carries no
    /// generation, so a turn that completes between the sender's read and its request cannot
    /// turn it into a stale wake (the actor then idled with a queued command, ticket 26/09/23
    /// `📓️wp-h6.md`).
    fn schedule(self: &Arc<Self>) {
        if self.complete.load(std::sync::atomic::Ordering::Acquire) || self.terminal.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let _ = self.wake_requested.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
        self.enqueue(false);
    }

    /// ⏰️ A turn future's own waker, or its deadline: only the turn that armed it may wake it.
    fn request_wake(self: &Arc<Self>, generation: u64) {
        if generation != self.turn_generation.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.schedule();
    }

    fn enqueue(self: &Arc<Self>, terminal_close: bool) {
        if self.complete.load(std::sync::atomic::Ordering::Acquire) || (!terminal_close && self.terminal.load(std::sync::atomic::Ordering::Acquire)) {
            return;
        }
        if self.scheduled.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let runner = self.clone();
        self.submit_exact(Box::new(move || runner.run_job()), 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(semio_framework_async::Lane::UserVisible, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated if attempt < ACTOR_RUNNER_RETRY_LIMIT => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                    self.arm_retry();
                }
                kind => {
                    let job = error.into_job();
                    self.begin_terminal(ArtifactActorTerminalReason::Pool(kind));
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason: ArtifactActorTerminalReason::Pool(kind), job });
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        if self.retry_armed.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let generation = self.retry_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel).wrapping_add(1);
        let runner = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(ACTOR_RUNNER_RETRY_MS), move || {
            if generation != runner.retry_generation.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            runner.retry_armed.store(false, std::sync::atomic::Ordering::Release);
            let retry = runner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if runner.terminal.load(std::sync::atomic::Ordering::Acquire) {
                    let reason = runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner).unwrap_or(ArtifactActorTerminalReason::Cancelled);
                    *runner.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason, job });
                } else {
                    runner.submit_exact(job, attempt);
                }
            }
        });
    }

    /// 🏃️ TurnFault retains the future in `terminal_turn`. Do not `enqueue` a drain job:
    /// against a shut-down pool that would mint a second `Pool(Shutdown)` terminal_job
    /// grant and leak mailbox ownership (one-owner-per-grant). Host `close_one` drains.
    fn run_job(self: Arc<Self>) {
        if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
            self.close_one_terminal_owner();
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            if self.has_terminal_owner() {
                self.enqueue(true);
            } else {
                self.finish_terminal();
            }
            return;
        }
        self.wake_requested.store(false, std::sync::atomic::Ordering::Release);
        let owner = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let Some(owner) = owner else {
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            return;
        };
        let mut future = match owner {
            ActorTurnOwner::Actor(mut actor) => {
                if self.close_requested.load(std::sync::atomic::Ordering::Acquire) {
                    actor.closing = true;
                    actor.cmd_rx.close();
                }
                Box::pin(async move {
                    let outcome = actor.drive_one().await;
                    (actor, outcome)
                }) as ActorTurnFuture
            }
            ActorTurnOwner::Future(future) => future,
        };
        let generation = self.turn_generation.load(std::sync::atomic::Ordering::Acquire);
        let waker = std::task::Waker::from(Arc::new(ActorTurnWake { runner: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);
        let _io_reactor = self.io_reactor.as_ref().map(tokio::runtime::Handle::enter);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                    *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    self.enqueue(true);
                    return;
                }
                *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                if self.release_scheduled_after_turn_with(|| {}) {
                    self.enqueue(false);
                }
            }
            Ok(std::task::Poll::Ready((mut actor, mut outcome))) => {
                self.turn_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
                self.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
                if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                    *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    self.enqueue(true);
                    return;
                }
                if self.close_requested.load(std::sync::atomic::Ordering::Acquire) && !actor.closing && outcome != ArtifactDrive::Terminal {
                    actor.closing = true;
                    actor.cmd_rx.close();
                    outcome = ArtifactDrive::MoreWork;
                }
                match outcome {
                    ArtifactDrive::Terminal => {
                        *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                        self.begin_terminal(ArtifactActorTerminalReason::Detached);
                        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                        self.enqueue(true);
                    }
                    ArtifactDrive::MoreWork | ArtifactDrive::Idle { .. } => {
                        *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                        let woke = self.release_scheduled_after_turn_with(|| {});
                        if outcome == ArtifactDrive::MoreWork || woke {
                            self.schedule();
                        } else if let ArtifactDrive::Idle { deadline } = outcome {
                            self.arm_deadline(deadline);
                        }
                    }
                }
            }
            Err(_) => {
                *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                self.begin_terminal(ArtifactActorTerminalReason::TurnFault);
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            }
        }
    }

    fn release_scheduled_after_turn_with(&self, after_release: impl FnOnce()) -> bool {
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        after_release();
        self.wake_requested.swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    fn arm_deadline(self: &Arc<Self>, deadline: Option<Instant>) {
        let Some(deadline) = deadline else {
            self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
            self.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
            return;
        };
        if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let delay_ms = u64::try_from(deadline.saturating_duration_since(Instant::now()).as_millis()).unwrap_or(u64::MAX).max(1);
        let at_ms = self.pool.now_ms().saturating_add(delay_ms);
        if self.deadline_armed.load(std::sync::atomic::Ordering::Acquire) && self.deadline_at_ms.load(std::sync::atomic::Ordering::Acquire) <= at_ms {
            return;
        }
        self.deadline_at_ms.store(at_ms, std::sync::atomic::Ordering::Release);
        self.deadline_armed.store(true, std::sync::atomic::Ordering::Release);
        let generation = self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel).wrapping_add(1);
        let turn_generation = self.turn_generation.load(std::sync::atomic::Ordering::Acquire);
        let runner = self.clone();
        self.pool.callback_at(at_ms, move || {
            if generation != runner.deadline_generation.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            runner.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
            runner.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
            runner.request_wake(turn_generation);
        });
    }

    fn begin_terminal(&self, reason: ArtifactActorTerminalReason) {
        if self.terminal.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        self.mailbox.close();
        self.turn_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
        self.retry_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        *self.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(reason);
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason, job });
        }
        if self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let owner = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = owner;
        }
    }

    pub fn cancel(self: &Arc<Self>) {
        self.begin_terminal(ArtifactActorTerminalReason::Cancelled);
        self.enqueue(true);
    }

    fn request_close(self: &Arc<Self>) {
        if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.close_requested.store(true, std::sync::atomic::Ordering::Release);
        self.mailbox.close();
        self.schedule();
    }

    fn close_one_terminal_owner(&self) -> bool {
        if let Some(owner) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            drop(owner);
            return true;
        }
        if self.mailbox.close_one() {
            return true;
        }
        let owner = self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if owner.is_none() {
            return false;
        }
        drop(owner);
        true
    }

    fn has_terminal_owner(&self) -> bool {
        self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.mailbox.has_pending()
            || self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal.load(std::sync::atomic::Ordering::Acquire)
            && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
            && self.external_tickets.load(std::sync::atomic::Ordering::Acquire) == 0
            && !self.has_terminal_owner()
            && self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    fn finish_terminal(&self) {
        if !self.terminal_is_empty() {
            return;
        }
        if self.complete.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        if let Some(callback) = self.terminal_empty_callback.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            callback(self.generation);
        }
        self.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    }

    fn return_ticket(&self) {
        let previous = self.external_tickets.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        debug_assert!(previous > 0, "artifact actor ticket returned twice");
        if previous == 1 && self.terminal_is_empty() {
            self.finish_terminal();
        }
    }

    fn take_terminal_job(&self) -> Option<ActorTerminalJob> {
        self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }

    fn resume_terminal_job(self: &Arc<Self>, terminal: ActorTerminalJob) {
        self.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.submit_exact(terminal.job, 0);
    }

    fn return_terminal_job(&self, terminal: ActorTerminalJob) {
        let mut slot = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        debug_assert!(slot.is_none(), "terminal job handback slot already occupied");
        if slot.is_none() {
            *slot = Some(terminal);
        }
    }

    fn close_terminal_job(&self, terminal: ActorTerminalJob) {
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        drop(terminal);
        if self.terminal_is_empty() {
            self.finish_terminal();
        }
    }
}

/// 🔌️ The one platform I/O reactor every native document actor polls its hub socket against: a
/// current-thread Tokio runtime whose only thread (`semio-document-io`) parks on the I/O and time
/// drivers, created on first use and never shut down, like `db_storage_driver_runtime`. Actors keep
/// running on the `WorkerPool` and only enter this handle while polled. It used to be whatever
/// runtime the spawning thread happened to be inside; a native wgpu shell spawns from a plain
/// thread, so the first hub dial panicked with `there is no reactor running` and the document
/// socket never left `Connecting` (ticket 26/09/23 slice WG8, `hub-live-collaboration-check` run 1).
/// `None` only when the operating system refuses the reactor, which the dial then reports as a fault.
fn document_socket_io_reactor() -> Option<tokio::runtime::Handle> {
    static REACTOR: std::sync::OnceLock<Option<tokio::runtime::Handle>> = std::sync::OnceLock::new();
    REACTOR
        .get_or_init(|| {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_io().enable_time().build().ok()?;
            let handle = runtime.handle().clone();
            std::thread::Builder::new().name("semio-document-io".to_string()).stack_size(1 << 20).spawn(move || runtime.block_on(std::future::pending::<()>())).ok()?;
            Some(handle)
        })
        .clone()
}

/// 🚀️ Creates one finite-turn actor on the process WorkerPool. Tokio remains only the
/// platform I/O reactor ([`document_socket_io_reactor`]); it never owns an actor or timer thread.
pub(super) async fn spawn_actor(
    pool: Arc<semio_framework_async::WorkerPool>,
    generation: u64,
    config: ArtifactActorConfig,
    remote: ChannelBackboneRemote,
    cmd_rx: ArtifactMailboxReceiver,
    events: broadcast::Sender<ArtifactEvent>,
    credential: Arc<std::sync::RwLock<Option<Arc<semio_framework_os_directory_client::LocalHubCredential>>>>,
    socket_grant_source: Arc<std::sync::RwLock<Option<Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>>>>,
    document_execution_target_lease: Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    seed: Option<semio_framework_os_directory::CanonicalCheckpointPairV1>,
    operation_cancel: semio_framework_async::CancelToken,
) -> ArtifactActorRunnerHandle {
    let io_reactor = document_socket_io_reactor();
    let mailbox = cmd_rx.close_handle();
    let mut actor = ArtifactActor::new(pool.clone(), config, remote, cmd_rx, events, credential, socket_grant_source, document_execution_target_lease, operation_cancel).await;
    if let Some(pair) = seed {
        actor.seed_hub_document(pair).await;
    }
    let runner = Arc::new(ActorRunner {
        pool,
        io_reactor,
        generation,
        turn: std::sync::Mutex::new(Some(ActorTurnOwner::Actor(Box::new(actor)))),
        terminal_turn: std::sync::Mutex::new(None),
        mailbox,
        scheduled: std::sync::atomic::AtomicBool::new(false),
        wake_requested: std::sync::atomic::AtomicBool::new(false),
        turn_generation: std::sync::atomic::AtomicU64::new(1),
        deadline_armed: std::sync::atomic::AtomicBool::new(false),
        deadline_generation: std::sync::atomic::AtomicU64::new(1),
        deadline_at_ms: std::sync::atomic::AtomicU64::new(u64::MAX),
        retry_armed: std::sync::atomic::AtomicBool::new(false),
        retry_generation: std::sync::atomic::AtomicU64::new(1),
        retry_job: std::sync::Mutex::new(None),
        terminal_job: std::sync::Mutex::new(None),
        terminal_reason: std::sync::Mutex::new(None),
        terminal_empty_callback: std::sync::Mutex::new(None),
        self_retained: std::sync::Mutex::new(None),
        external_tickets: std::sync::atomic::AtomicUsize::new(0),
        close_requested: std::sync::atomic::AtomicBool::new(false),
        terminal: std::sync::atomic::AtomicBool::new(false),
        complete: std::sync::atomic::AtomicBool::new(false),
    });
    *runner.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(runner.clone());
    let weak = Arc::downgrade(&runner);
    let schedule: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        if let Some(runner) = weak.upgrade() {
            runner.schedule();
        }
    });
    if let Some(ActorTurnOwner::Actor(actor)) = runner.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut() {
        let readiness_requested = actor.readiness_requested.clone();
        let readiness_schedule = schedule.clone();
        let readiness: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
            readiness_requested.store(true, std::sync::atomic::Ordering::Release);
            readiness_schedule();
        });
        actor.remote.set_outbound_wake(schedule.clone());
        actor.cmd_rx.set_wake(schedule);
        actor.set_readiness(readiness);
    }
    ArtifactActorRunnerHandle { generation, runner }
}

/// 🎛️ The channels of a native document actor, carrying its runner ticket.
pub type ArtifactChannels = ArtifactChannelsOf<ArtifactActorRunnerTicket>;

/// 🏛️ Registry of open per-document actors. One `ArtifactHost` per host process (wgpu native
/// or tests) owns every open document's actor + event fan-out.
pub struct ArtifactHost {
    pub core: ArtifactHostCore<ArtifactActorRunnerHandle>,
}

impl Clone for ArtifactHost {
    fn clone(&self) -> Self {
        Self { core: self.core.retain() }
    }
}

impl ArtifactHost {
    /// 🧩️ Creates a host on the process WorkerPool; callers must inject the same pool
    /// used by their service and renderer runtimes.
    pub fn new(pool: Arc<semio_framework_async::WorkerPool>) -> Self {
        Self { core: ArtifactHostCore::new(pool) }
    }

    /// 🏛️ The platform-neutral registry this host composes with the native actor.
    pub fn core(&self) -> &ArtifactHostCore<ArtifactActorRunnerHandle> {
        &self.core
    }

    pub fn set_local_hub_credential(&self, credential: Arc<semio_framework_os_directory_client::LocalHubCredential>) {
        self.core.set_local_hub_credential(credential);
    }

    pub fn set_hub_socket_grant_source(&self, source: Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>) {
        self.core.set_hub_socket_grant_source(source);
    }

    pub fn set_document_execution_target_lease(&self, document_key: &ArtifactDocumentKey, surface: semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1) -> bool {
        self.core.set_document_execution_target_lease(document_key, surface)
    }

    pub fn set_document_seed(&self, document_key: &ArtifactDocumentKey, pair: semio_framework_os_directory::CanonicalCheckpointPairV1) -> bool {
        self.core.set_document_seed(document_key, pair)
    }

    /// 🟢️ Whether a hub document opened now can dial at once: a credential and a grant source.
    pub fn local_hub_ready(&self) -> bool {
        self.core.hub_binding_present()
    }

    /// 🚀️ Spawns (or replaces) the actor for `config.document_id` and returns the channels the
    /// caller wires into its store. Idempotent per id: opening an already-open id closes the old actor.
    pub async fn open(&self, config: ArtifactActorConfig) -> ArtifactChannels {
        let document_id = config.document_id.clone();
        let document_key = ArtifactDocumentKey::for_config(&config);
        let (document_execution_target_lease, seed) = self.core.take_open_inputs(&document_key);
        let _ = self.close_key(&document_key);
        let generation = self.core.state().claim_generation();
        let (channel_backbone, remote) = ChannelBackbone::pair(&format!("actor://{document_id}")).await;
        let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
        let (event_tx, _event_rx) = broadcast::channel(256);
        let document_cancel = self.core.cancel.child_now();
        let runner = spawn_actor(self.core.pool.clone(), generation, config, remote, cmd_rx, event_tx.clone(), self.core.credential.clone(), self.core.socket_grant_source.clone(), document_execution_target_lease, seed, document_cancel.clone()).await;
        let weak_host = Arc::downgrade(&self.core.inner);
        runner.set_terminal_empty_callback(Arc::new(move |generation| {
            if let Some(host) = weak_host.upgrade() {
                host.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing.remove(&generation);
            }
        }));
        let runner_ticket = runner.issue_ticket(self.core.inner.clone());
        let entry = OpenDocument {
            generation,
            cancel: document_cancel,
            cmd_tx: cmd_tx.clone(),
            events: event_tx,
            presence: PresenceHeartbeatProducer::default(),
            runner: runner.clone(),
        };
        self.core.insert_document(document_key.clone(), entry);
        runner.start();
        ArtifactChannelsOf { cmd_tx, document_key, runner: runner_ticket, channel_backbone }
    }

    pub async fn subscribe(&self, document_id: &str) -> broadcast::Receiver<ArtifactEvent> {
        self.core.subscribe(document_id).await
    }

    pub async fn subscribe_key(&self, document_key: &ArtifactDocumentKey) -> broadcast::Receiver<ArtifactEvent> {
        self.core.subscribe_key(document_key).await
    }

    pub async fn send(&self, document_id: &str, message: ArtifactActorMsg) {
        self.core.send(document_id, message).await;
    }

    pub async fn send_key(&self, document_key: &ArtifactDocumentKey, message: ArtifactActorMsg) {
        self.core.send_key(document_key, message).await;
    }

    pub fn presence_heartbeat(&self, document_id: &str, now_ms: u64, peer: PresencePeer) -> bool {
        self.core.presence_heartbeat(document_id, now_ms, peer)
    }

    pub fn observe_presence_tool_run_key(&self, document_key: &ArtifactDocumentKey, tool_run: Option<PresenceToolRun>) -> bool {
        self.core.observe_presence_tool_run_key(document_key, tool_run)
    }

    pub fn presence_heartbeat_key(&self, document_key: &ArtifactDocumentKey, now_ms: u64, peer: PresencePeer) -> bool {
        self.core.presence_heartbeat_key(document_key, now_ms, peer)
    }

    /// ✂️ Transfers a document into generation-keyed retained close ownership. The runner
    /// rejects later mailbox ingress and drains one mailbox, backbone, actor, or job owner per grant.
    /// 🚫️async: E1 pure lock/remove + sync channel send, no real suspension point — consumed by
    /// `impl Drop` (sync-only external trait); see R9.
    pub fn close(&self, document_id: &str) -> Option<u64> {
        self.close_key(&ArtifactDocumentKey::local(document_id))
    }

    pub fn close_key(&self, document_key: &ArtifactDocumentKey) -> Option<u64> {
        let document = self.core.remove_document(document_key)?;
        document.cancel.cancel_now();
        let generation = document.generation;
        let runner = document.runner;
        self.core.state().closing.insert(generation, runner.clone());
        if matches!(runner.terminal_state(), ArtifactActorTerminalState::Complete(_)) {
            self.core.state().closing.remove(&generation);
            return Some(generation);
        }
        runner.request_close();
        Some(generation)
    }

    pub fn closing_runner(&self, generation: u64) -> Option<ArtifactActorRunnerHandle> {
        self.core.state().closing.get(&generation).cloned()
    }

    pub fn open_artifacts(&self) -> Vec<ArtifactDocumentKey> {
        self.core.open_artifacts()
    }
}

impl Drop for ArtifactHost {
    fn drop(&mut self) {
        if !self.core.release() {
            return;
        }
        self.core.cancel.cancel_now();
        for document_key in self.core.open_artifacts() {
            self.close_key(&document_key);
        }
    }
}

#[cfg(any(test, feature = "testing"))]
include!("../🧪️tests/🔬️retained-turn-fixtures/🦀️.rs");

#[cfg(test)]
#[path = "../🧪️tests/🔬️echo-suppression-pin/🦀️.rs"]
mod echo_suppression_pin;
