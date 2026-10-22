//! 🔌️ Actor socket types, bootstrap control and folder endpoint: the hub socket aliases and connection, the readiness wake, the connect failure, the pending bootstrap and its control, the drive phases, the folder endpoint and the artifact actor struct. See [`super`].

use super::*;

type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

type WsSink = futures::stream::SplitSink<WsStream, Message>;

type WsRead = futures::stream::SplitStream<WsStream>;

pub(super) struct ConnectedDocumentSocket {
    pub(super) stream: WsStream,
    pub socket_actor: semio_framework_value::SharedUtf8,
    pub authority: semio_framework_os_directory_client::DocumentSocketAuthorityV1,
    pub(super) identity_progress: semio_framework_value::RetainedCloneProgress,
}

pub(super) struct ArtifactReadinessWake(pub(super) Arc<dyn Fn() + Send + Sync>);

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
pub(super) enum DocumentConnectFailure {
    Refused,
    Short,
}

/// ⏰️ The wall clock the link state machine reads.
pub(super) fn wall_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

/// ⏰️ The `Instant` of a wall-clock deadline, for the actor's timer-driven drive.
pub(super) fn instant_at(at_ms: u64, now_ms: u64) -> Instant {
    Instant::now() + Duration::from_millis(at_ms.saturating_sub(now_ms))
}

pub(super) struct HubConn {
    pub write: WsSink,
    pub read: WsRead,
}

pub(super) struct PendingArtifactBootstrap {
    pub assembler: ArtifactBootstrapAssembler,
    pub started_at: Instant,
    pub resume_token: String,
    pub baseline_frontier: RuntimeFrontierSummary,
    pub required_tail_frontier: RuntimeFrontierSummary,
    pub pack_schema_hash: [u8; 32],
}

pub(super) struct NativeBootstrapControl {
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
pub(super) enum ArtifactDrivePhase {
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
pub(super) enum ArtifactDrive {
    MoreWork,
    Idle { deadline: Option<Instant> },
    Terminal,
}

/// 📁️ A folder/file binding's storage driver, keyed for multi-document sqlite or single
/// pack-backed blob. Both variants move real `pack`+`spr` bytes end to end — this actor never
/// touches JSON. `Sqlite` is fully codec-free (its row IS the pack+spr pair, no bridging
/// needed). `Pack` requires the schema codec for both authoritative DSL fallback validation
/// and synchronized text mirrors.
pub(super) enum FolderEndpoint {
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
    pub(super) folder: Option<FolderEndpoint>,
    pub(super) folder_watch_path: Option<PathBuf>,
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
    pub(super) connect_future: Option<ConnectFuture>,
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
    pub(super) reconnect_at: Option<Instant>,
    pub(super) link_expires_at: Option<Instant>,
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
    pub(super) hlc_counter: u64,
    pub(super) current_pack: Option<Vec<u8>>,
    pub(super) current_spr: Option<Vec<u8>>,
    pub(super) current_archive: Option<Vec<u8>>,
    pub(super) known_op_ids: HashSet<String>,
    /// 🔁️ Every operation id this replica authored or applied — the echo filter ([`admit_remote_envelopes`]).
    pub applied_op_ids: HashSet<String>,
    pub(super) last_written_hash: Option<String>,
    /// 🪞️ The hash of the archive the folder is known to hold — the last one read or written, none after a read that found
    /// the folder empty: an archive with these bytes is not written again.
    pub(super) folder_held_hash: Option<String>,
    /// 🪹️ Whether this open's first folder read was answered, or an archive was read or written: no later read announces an
    /// absent archive ([`ArtifactEvent::DocumentArchiveAbsent`]).
    pub(super) folder_bootstrapped: bool,
    #[cfg(any(test, feature = "testing"))]
    pub(super) folder_writes: usize,
    pub remote_state: RemoteState,
    pub last_status: Option<ArtifactSyncStatus>,
    pub(super) watcher: Option<semio_framework_os_services::OwnedFileChangeWatcher>,
    pub(super) fs_deadline: Option<Instant>,
    pub started: bool,
    pub(super) command_turns: u8,
    pub(super) drive_phase: ArtifactDrivePhase,
    pub closing: bool,
    pub(super) close_drain_deadline: Option<Instant>,
    pub(super) readiness: Option<Arc<dyn Fn() + Send + Sync>>,
    pub(super) readiness_requested: Arc<std::sync::atomic::AtomicBool>,
    #[cfg(any(test, feature = "testing"))]
    pub(super) fail_bootstrap_local_replay_once: bool,
}

/// ⏳️ How long a closing actor keeps reading its hub socket so a catch-up in flight can complete
/// and flush operations the store handed over before the close; after it the outbox is dropped.
pub(super) const ARTIFACT_CLOSE_OUTBOX_DRAIN: Duration = Duration::from_secs(2);
