//! 🧠️ `state` — the shared `HubState` and its test gates.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::http::StatusCode;
use directory::os_directory::{DirectoryPresenceActor, DirectoryStreamMessage, DocumentDescriptor};
use directory::os_spr::channel::{PRESENCE_ROSTER_MAXIMUM_BYTES, PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES, PRESENCE_ROSTER_MAXIMUM_ITEMS};
use futures::{SinkExt, StreamExt};
use protocol::ServerFrame;
use semio_framework_async_slots::ShardedMap;
use semio_framework_trace::record::{Span, TraceOutcome, TraceRecord, Tracer};
use semio_hub_artifact_authority::chunk_cas::{ArtifactChunkCasStorage, ArtifactChunkCasStores};
use semio_hub_artifact_creation_service::ArtifactCreationServiceV1;
use semio_hub_trusted_catalog::VerifiedTrustedCatalog;
use semio_hub_observability::HubRouteMetricsV1;
use semio_hub_artifact_checkin::DocumentCheckInJobs;
use semio_hub_auth::rate_limit::{HubRateLimiterV1, HubStreamLimiterV1};
use semio_hub_auth::CredentialSignInPolicyV1;
use semio_hub_directory_contract::error::DirectoryError;
use semio_hub_directory_service::DirectoryService;
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_inference_runtime_contract::{GisMapApprovalCheckpointPublisherV1, GisMapApprovalIngressAuthorityV1};
use semio_hub_inference_runtime::HubInferenceRuntimeV1;
use semio_hub_inference_catalog::VerifiedGisMapArtifactBindingV1;
use semio_hub_lag_rebootstrap::VerifiedRebootstrapSource;
use semio_hub_stores::HubInstance;
use server::contract::{Principal, SessionId};
use server::gateway::ServerState;
use server::storage::SessionRecord;
use std::collections::BTreeSet;
use std::sync::Arc;
#[cfg(any(test, feature = "testkit"))]
use std::sync::Mutex;
use tokio::sync::broadcast;
use semio_hub_runtime::startup::{HubArtifactAuthority, HubArtifactPublication, ArtifactCasMaintenanceSupervisor, DocumentOpenCatalogAuthorityV1};
use semio_hub_runtime::support::{now_ms, directory_error_status};
use semio_hub_runtime::presence::{PresenceLeaseSlot, PresenceIdentityV1, PresenceLeaseTransition, PresenceSnapshot, PresenceDirectoryProjections, ColorLease, SpaceColors, PRESENCE_LEASE_TTL_MS};
#[cfg(any(test,feature="testkit"))]
use semio_hub_runtime::presence::TestPresenceClock;
use semio_hub_ledgers::socket_grants::{SocketGrantLedgerV1, SocketBindingGatesV1};
use semio_hub_ledgers::open_plan::DocumentOpenPlanLedgerV1;
use semio_hub_ledgers::guard::AdminSubject;
use semio_hub_runtime::readiness::{HubReadinessV1, CheckpointPolicyLedgerV1};
#[cfg(any(test,feature="testkit"))]
use semio_hub_ledgers::pair_blobs::TestCanonicalPairRequestGate;
use semio_hub_route_models::artifact_creation::{HubArtifactCreationCommitAuthorityV1, ArtifactCreationHttpTaskOwnerV1};
use semio_hub_route_models::admin::{AdminOperationRuntime, AdminOperationTaskOwner};
use semio_hub_runtime::lifecycle::HubSocketDrainV1;
use db::storage::PayloadStorage as _;
use protocol::ArtifactId as ProtocolArtifactId;
use server::storage::SessionStore as _;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::state::{TestDocumentOpenPlanIssueGate, TestLiveGate};

//#region 🔖️State

#[derive(Clone)]
pub struct HubState {
    pub db: Arc<db::Database>,
    pub artifact_cas: Arc<ArtifactChunkCasStores>,
    pub directory: Arc<HubDirectories>,
    pub rebootstrap: Arc<VerifiedRebootstrapSource>,
    pub artifact_authority: Option<Arc<HubArtifactAuthority>>,
    pub verified_catalog: Option<Arc<VerifiedTrustedCatalog>>,
    pub artifact_creation: Option<Arc<ArtifactCreationServiceV1>>,
    pub artifact_creation_commit_authority: Arc<HubArtifactCreationCommitAuthorityV1>,
    pub artifact_creation_tasks: Arc<ArtifactCreationHttpTaskOwnerV1>,
    pub gis_map_binding: Option<Arc<VerifiedGisMapArtifactBindingV1>>,
    /// 💡️ The sole GIS Map proposal authority — present only when a verified trusted profile
    /// froze a writable Map editor selection; otherwise every inference route fails closed with
    /// `inference.unavailable` and readiness publishes `inference: false`.
    pub inference_runtime: Option<Arc<HubInferenceRuntimeV1>>,
    pub openable_catalog: Option<Arc<dyn DocumentOpenCatalogAuthorityV1>>,
    pub artifact_publication: Arc<HubArtifactPublication>,
    pub artifact_maintenance: Arc<ArtifactCasMaintenanceSupervisor>,
    /// 📌️ Every Check In this process accepted, keyed by author, document and request id.
    pub check_ins: Arc<DocumentCheckInJobs>,
    /// 🏭️ Wave 1.B: the single serialized directory writer (contract §C1's decider laws +
    /// dense event `seq`) built once over `directory` at startup — see `semio_hub::directory::
    /// DirectoryService`'s own doc. `/directory/commands` and `/directory/invites/{token}/redeem`
    /// go through this; every other `/directory/*` route reads `directory` directly.
    pub directory_service: Arc<DirectoryService>,
    /// 🔐️ Whether this deployment mints sessions from password credentials at
    /// `POST /auth/sessions`, and under which lifetime/cost — fail-closed, so a development hub
    /// keeps issuing only through the local-bootstrap pipe (see `semio_hub::auth`).
    pub credential_sign_in: CredentialSignInPolicyV1,
    /// 🚦️ Per-principal and per-remote-address token buckets in front of credential
    /// sign-in, directory commands, invite redemption and socket-grant issuance.
    pub rate_limits: Arc<HubRateLimiterV1>,
    /// 🚰️ The stream rule of the execution-target asset routes (component, browser actor): immutable, content-addressed
    /// bodies no request bucket meters, bounded by how many stream at once hub-wide and per principal.
    pub asset_streams: Arc<HubStreamLimiterV1>,
    pub admin_subjects: Arc<[AdminSubject]>,
    pub admin_cursor_key: [u8; 32],
    /// 🏛️ Process-local MAC key authenticating one space-administration keyset cursor.
    pub space_administration_cursor_key: [u8; 32],
    pub admin_operations: Arc<ShardedMap<String, Arc<AdminOperationRuntime>>>,
    pub admin_operation_slots: Arc<tokio::sync::Semaphore>,
    pub admin_operation_tasks: Arc<AdminOperationTaskOwner>,
    pub readiness: Arc<HubReadinessV1>,
    /// 🛡️ Contract §C0 `OS_HUB_ADMIN_DIR`: the admin SPA's static asset root. Lane 2-E owns
    /// the actual `/admin` file-serving handler (and its 503-if-missing stub) — this lane only
    /// carries the resolved path through `HubState` so that handler has something to read.
    /// 🌵️ Unread until 2-E's handler lands and calls `state.admin_dir` — not dead code, just not
    /// wired to a route yet (explicitly out of this lane's scope, see the doc above).
    #[allow(dead_code)]
    pub admin_dir: std::path::PathBuf,
    /// 📡️ Command-lane + preview-lane fan-out, one `broadcast::Sender` per v1 scope key —
    /// `db::Database`'s own `ArtifactHandle` exposes no live-subscription seam yet (see
    /// `db_engine`'s module doc: `subscribe`/`preview` are honest `Unimplemented` extension seams),
    /// so relaying newly-committed commands / preview blobs / presence updates to other connected
    /// sessions on the same document is this crate's own, deliberately thin responsibility — it
    /// never itself decides ordering or durability, only re-broadcasts what `db` already committed
    /// or what a preview/presence frame carries verbatim.
    pub fanout: Arc<ShardedMap<String, broadcast::Sender<ServerFrame>>>,
    pub fanout_capacity: usize,
    #[cfg(any(test, feature = "testkit"))]
    pub live_gate: Option<Arc<TestLiveGate>>,
    #[cfg(any(test, feature = "testkit"))]
    pub canonical_pair_authorization_gate: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    #[cfg(any(test, feature = "testkit"))]
    pub canonical_pair_request_gate: Option<Arc<TestCanonicalPairRequestGate>>,
    #[cfg(any(test, feature = "testkit"))]
    pub canonical_pair_deadline_ms: Option<u64>,
    #[cfg(any(test, feature = "testkit"))]
    pub document_open_plan_issue_gate: Option<Arc<TestDocumentOpenPlanIssueGate>>,
    #[cfg(any(test, feature = "testkit"))]
    pub document_open_plan_deadline_ms: Option<u64>,
    /// 👥️ `(document_scope_key_v1, actor)` -> that actor's presence session (contract §C7.3) — ephemeral,
    /// never durable (mirrors the preview lane's own law), rebuilt from nothing on hub restart. The
    /// roster is document-wide now (contract §C7.0): `ServerFrame::Presence` fans out on `fanout`, not
    /// a surface-scoped channel; identity and the plan-bound surface are reconstructed by Hub
    /// before canonical peer bytes are stored or published.
    pub presence: Arc<ShardedMap<(String, String), PresenceLeaseSlot>>,
    pub presence_publication_gate: Arc<tokio::sync::Mutex<PresenceDirectoryProjections>>,
    #[cfg(any(test, feature = "testkit"))]
    pub presence_clock: Option<Arc<TestPresenceClock>>,
    /// 🎨️ Contract §C7.3 session colors: `space_id` -> that space's live `(actor -> palette
    /// index)` leases. `acquire_color`/`release_color` below are the only mutators. Never persisted.
    pub session_colors: Arc<ShardedMap<String, SpaceColors>>,
    /// 🦵️ Wave 1.B admin kick: `syncSessionId` (the `SyncSessionRecord.id`/`ConnectionView.
    /// syncSessionId` the directory hands out on connect) -> a `Notify` the WS loop `select!`s on
    /// alongside its socket/broadcast reads. `POST /admin/api/connections/{syncSessionId}/close`
    /// fires it; the loop observes the wake-up and closes the connection on its own next tick —
    /// this map never itself closes a socket, only signals the session that owns it to.
    pub session_kicks: Arc<ShardedMap<String, Arc<tokio::sync::Notify>>>,
    /// 🚪️ Every upgraded socket (document and directory) holds one admission from here for
    /// its whole life. `axum`'s graceful shutdown never waits for an upgraded connection, so `main`
    /// closes them through this owner and waits for the last admission before it shuts the
    /// `Database` down and closes its storage.
    pub socket_drain: Arc<HubSocketDrainV1>,
    pub socket_grants: Arc<SocketGrantLedgerV1>,
    pub document_open_plans: Arc<DocumentOpenPlanLedgerV1>,
    pub socket_binding_gates: Arc<SocketBindingGatesV1>,
    /// 🧩️ Installed runtime extensions mirrored from dev `/🧩️extension-modules` —
    /// populated by hub deploy copy / sideload; `GET /🧩️extension-modules` lists `install.json` rows.
    pub extensions_root: std::path::PathBuf,
    /// ⚖️ Contract §C9 ("hub `🏗️bootstrap/🦀️.rs`: policy from config → `SubmitOptions.policy`"): the
    /// authority-local `protocol::MergePolicy` every `submit_commands` call on this hub instance
    /// judges a batch's worst graded conflict/message level against — read once at startup from
    /// `OS_HUB_MERGE_POLICY` (see `merge_policy_from_env`'s doc), never per-connection/per-space,
    /// matching `protocol::MergePolicy`'s own "local/authority state, never on the wire" law.
    pub merge_policy: protocol::MergePolicy,
    /// 📌️ The checkpoint policy's per-document tallies ([`HubCheckpointPolicyV1`], declared in `readiness`).
    pub checkpoint_policy: Arc<CheckpointPolicyLedgerV1>,
    /// 🛣️ Every matched route's answers by class and latency, keyed by method and route template —
    /// the per-route table `GET /admin/api/observability` reads.
    pub route_metrics: Arc<HubRouteMetricsV1>,
    /// 📝️ The structured observer every route reports through — a level, a sink and the
    /// per-event counter table `GET /admin/api/observability` reads. Configured from
    /// `SEMIO_TRACE_LEVEL`/`SEMIO_TRACE_SINK` at boot; a test injects a capturing one and asserts
    /// on the exact records its own request produced.
    pub tracer: Tracer,
    /// 🗄️ Hub as instance #1 of the generic server product (`semio_hub::stores`): the four
    /// durable storage roles plus the command bus and saga runner over them, opened in the same
    /// `OS_HUB_DATA` root the directory and the artifact CAS live in. `sessions` is the role a
    /// production route writes today — see `record_instance_session`.
    pub instance: ServerState<HubInstance>,
}

/// 🧩️ Every production field of [`HubState`]; the test seams are filled in by [`From`] only where a test build asks for them.
pub struct HubStateParts {
    pub db: Arc<db::Database>,
    pub artifact_cas: Arc<ArtifactChunkCasStores>,
    pub directory: Arc<HubDirectories>,
    pub rebootstrap: Arc<VerifiedRebootstrapSource>,
    pub artifact_authority: Option<Arc<HubArtifactAuthority>>,
    pub verified_catalog: Option<Arc<VerifiedTrustedCatalog>>,
    pub artifact_creation: Option<Arc<ArtifactCreationServiceV1>>,
    pub artifact_creation_commit_authority: Arc<HubArtifactCreationCommitAuthorityV1>,
    pub artifact_creation_tasks: Arc<ArtifactCreationHttpTaskOwnerV1>,
    pub gis_map_binding: Option<Arc<VerifiedGisMapArtifactBindingV1>>,
    pub inference_runtime: Option<Arc<HubInferenceRuntimeV1>>,
    pub openable_catalog: Option<Arc<dyn DocumentOpenCatalogAuthorityV1>>,
    pub artifact_publication: Arc<HubArtifactPublication>,
    pub artifact_maintenance: Arc<ArtifactCasMaintenanceSupervisor>,
    pub check_ins: Arc<DocumentCheckInJobs>,
    pub directory_service: Arc<DirectoryService>,
    pub credential_sign_in: CredentialSignInPolicyV1,
    pub rate_limits: Arc<HubRateLimiterV1>,
    pub asset_streams: Arc<HubStreamLimiterV1>,
    pub admin_subjects: Arc<[AdminSubject]>,
    pub admin_cursor_key: [u8; 32],
    pub space_administration_cursor_key: [u8; 32],
    pub admin_operations: Arc<ShardedMap<String, Arc<AdminOperationRuntime>>>,
    pub admin_operation_slots: Arc<tokio::sync::Semaphore>,
    pub admin_operation_tasks: Arc<AdminOperationTaskOwner>,
    pub readiness: Arc<HubReadinessV1>,
    pub admin_dir: std::path::PathBuf,
    pub fanout: Arc<ShardedMap<String, broadcast::Sender<ServerFrame>>>,
    pub fanout_capacity: usize,
    pub presence: Arc<ShardedMap<(String, String), PresenceLeaseSlot>>,
    pub presence_publication_gate: Arc<tokio::sync::Mutex<PresenceDirectoryProjections>>,
    pub session_colors: Arc<ShardedMap<String, SpaceColors>>,
    pub session_kicks: Arc<ShardedMap<String, Arc<tokio::sync::Notify>>>,
    pub socket_drain: Arc<HubSocketDrainV1>,
    pub socket_grants: Arc<SocketGrantLedgerV1>,
    pub document_open_plans: Arc<DocumentOpenPlanLedgerV1>,
    pub socket_binding_gates: Arc<SocketBindingGatesV1>,
    pub extensions_root: std::path::PathBuf,
    pub merge_policy: protocol::MergePolicy,
    pub checkpoint_policy: Arc<CheckpointPolicyLedgerV1>,
    pub route_metrics: Arc<HubRouteMetricsV1>,
    pub tracer: Tracer,
    pub instance: ServerState<HubInstance>,
}

impl From<HubStateParts> for HubState {
    fn from(parts: HubStateParts) -> Self {
        Self {
            db: parts.db,
            artifact_cas: parts.artifact_cas,
            directory: parts.directory,
            rebootstrap: parts.rebootstrap,
            artifact_authority: parts.artifact_authority,
            verified_catalog: parts.verified_catalog,
            artifact_creation: parts.artifact_creation,
            artifact_creation_commit_authority: parts.artifact_creation_commit_authority,
            artifact_creation_tasks: parts.artifact_creation_tasks,
            gis_map_binding: parts.gis_map_binding,
            inference_runtime: parts.inference_runtime,
            openable_catalog: parts.openable_catalog,
            artifact_publication: parts.artifact_publication,
            artifact_maintenance: parts.artifact_maintenance,
            check_ins: parts.check_ins,
            directory_service: parts.directory_service,
            credential_sign_in: parts.credential_sign_in,
            rate_limits: parts.rate_limits,
            asset_streams: parts.asset_streams,
            admin_subjects: parts.admin_subjects,
            admin_cursor_key: parts.admin_cursor_key,
            space_administration_cursor_key: parts.space_administration_cursor_key,
            admin_operations: parts.admin_operations,
            admin_operation_slots: parts.admin_operation_slots,
            admin_operation_tasks: parts.admin_operation_tasks,
            readiness: parts.readiness,
            admin_dir: parts.admin_dir,
            fanout: parts.fanout,
            fanout_capacity: parts.fanout_capacity,
            presence: parts.presence,
            presence_publication_gate: parts.presence_publication_gate,
            session_colors: parts.session_colors,
            session_kicks: parts.session_kicks,
            socket_drain: parts.socket_drain,
            socket_grants: parts.socket_grants,
            document_open_plans: parts.document_open_plans,
            socket_binding_gates: parts.socket_binding_gates,
            extensions_root: parts.extensions_root,
            merge_policy: parts.merge_policy,
            checkpoint_policy: parts.checkpoint_policy,
            route_metrics: parts.route_metrics,
            tracer: parts.tracer,
            instance: parts.instance,
            #[cfg(any(test, feature = "testkit"))]
            live_gate: None,
            #[cfg(any(test, feature = "testkit"))]
            canonical_pair_authorization_gate: None,
            #[cfg(any(test, feature = "testkit"))]
            canonical_pair_request_gate: None,
            #[cfg(any(test, feature = "testkit"))]
            canonical_pair_deadline_ms: None,
            #[cfg(any(test, feature = "testkit"))]
            document_open_plan_issue_gate: None,
            #[cfg(any(test, feature = "testkit"))]
            document_open_plan_deadline_ms: None,
            #[cfg(any(test, feature = "testkit"))]
            presence_clock: None,
        }
    }
}

impl HubState {
    /// 📝️ Opens one span on this hub's tracer with a fresh request id already bound, so
    /// every route reports under the same identity shape without restating it.
    pub fn span(&self, event: &str) -> Span {
        self.tracer.span(event).request(self.tracer.allocate_request_id())
    }

    /// 📝️ Reports one fact that has no duration — a refusal taken before any work started,
    /// or a lifecycle event such as boot.
    pub fn note(&self, event: &str, outcome: TraceOutcome, detail: &str) {
        let mut record = TraceRecord::new(event, outcome);
        record.detail = Some(detail.to_string());
        self.tracer.emit(record);
    }

    /// 👥️ Reports one presence transition — a peer becoming visible (`join`), its lease lapsing
    /// back to identity (`expiry`) or its socket leaving the roster (`leave`) — under the socket actor.
    fn note_presence(&self, event: &str, space_id: &str, document_id: &str, actor: &str) {
        let mut record = TraceRecord::new(event, TraceOutcome::Ok);
        record.principal = Some(actor.to_string());
        record.space = Some(space_id.to_string());
        record.artifact = Some(document_id.to_string());
        self.tracer.emit(record);
    }

    /// 🚨️ Maps a directory fault to its status the way [`directory_error_status`] does, and
    /// **records the backend's own message first**.
    ///
    /// `DirectoryError::Backend` is the one arm whose payload the status code cannot carry: the
    /// caller gets a bare `500` and the operator gets nothing at all, so a driver-level fault on a
    /// read route is indistinguishable from a bug in the handler. That silence cost this ticket a
    /// whole bisection (26/09/18 slice DB3, the Neo4j space-administration page). Every other arm
    /// already names itself through its status, so only this one is reported.
    pub fn directory_fault(&self, route: &str, error: DirectoryError) -> StatusCode {
        if let DirectoryError::Backend(ref detail) = error {
            self.note("server.directory.backend", TraceOutcome::Refused, &format!("route={route} {detail}"));
        }
        directory_error_status(error)
    }

    /// 🎫️ Records one live session in the server-product instance's [`SessionStore`].
    ///
    /// The hub's session **authority** is and stays the directory: it owns the capability digest,
    /// the expiry, the authorization generation and the revocation audit fact, and every
    /// authentication decision is taken there. What this store holds is the *instance's* live
    /// session set — the port `ServerState` hands to the framework's own routes and, once hub's
    /// auth moves behind `HubResolvers`, the set the principal ladder resolves a bearer against.
    ///
    /// Writing it here is what makes that move possible instead of theoretical, and it is the
    /// first production write into any [`HubInstance`] store. It is also, until the move happens,
    /// two records of one fact: `📓️ob1-hub-observability-and-store-wiring.md` §5 names the
    /// direction (the directory's auth-session rows *become* this store; this store is never
    /// dropped in favour of them) and `instance_sessions_survive_a_hub_restart` holds the two in
    /// lockstep across a reopen.
    pub async fn record_instance_session(&self, session_id: &str, user_id: &str, device: Option<&str>) -> Result<(), server::storage::StorageError> {
        let record = SessionRecord {
            id: SessionId(session_id.to_string()),
            principal: Principal::User { id: user_id.to_string() },
            device: device.map(|device| server::contract::DeviceId(device.to_string())),
            issued_at_millis: now_ms().max(0) as u64,
        };
        self.instance.sessions.lock().await.create(record).await
    }

    /// 🗑️ Removes one session from the instance's live set, leaving no replayable trace.
    pub async fn forget_instance_session(&self, session_id: &str) -> Result<(), server::storage::StorageError> {
        self.instance.sessions.lock().await.delete(&SessionId(session_id.to_string())).await
    }

    /// 🚪️ Removes every session of one principal — the instance-side half of "signed out
    /// everywhere", which `POST /auth/credentials` performs on the directory.
    pub async fn revoke_instance_principal(&self, user_id: &str) -> Result<usize, server::storage::StorageError> {
        self.instance.sessions.lock().await.revoke_principal(&Principal::User { id: user_id.to_string() }).await
    }

    pub fn fanout_for(&self, key: &str) -> broadcast::Sender<ServerFrame> {
        self.fanout.get_or_insert_with_cloned(key.to_string(), || broadcast::channel(self.fanout_capacity).0)
    }

    pub fn presence_now(&self) -> tokio::time::Instant {
        #[cfg(any(test, feature = "testkit"))]
        if let Some(clock) = &self.presence_clock {
            return clock.now();
        }
        tokio::time::Instant::now()
    }

    /// 👥️ Produces one actor-sorted, defensively bounded roster snapshot from one map traversal.
    pub fn presence_snapshot(&self, key: &str) -> PresenceSnapshot {
        let mut rows = Vec::new();
        self.presence.for_each(|(scope, actor), slot| {
            if scope == key {
                if let Some(peer) = &slot.peer {
                    let directory_actor = slot.document_surface.as_ref().map(|surface| DirectoryPresenceActor { actor: actor.clone(), user_id: slot.user_id.clone(), surface: surface.clone(), color: slot.color });
                    rows.push((actor.clone(), peer.clone(), directory_actor));
                }
            }
        });
        rows.sort_by(|left, right| left.0.cmp(&right.0));
        let mut peers = Vec::with_capacity(rows.len().min(PRESENCE_ROSTER_MAXIMUM_ITEMS));
        let mut actors = Vec::with_capacity(peers.capacity());
        let mut bytes = 0usize;
        for (_, peer, actor) in rows {
            if peers.len() == PRESENCE_ROSTER_MAXIMUM_ITEMS || peer.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES || bytes.checked_add(peer.len()).is_none_or(|next| next > PRESENCE_ROSTER_MAXIMUM_BYTES) {
                continue;
            }
            bytes += peer.len();
            peers.push(peer);
            if let Some(actor) = actor {
                actors.push(actor);
            }
        }
        PresenceSnapshot { peers, actors }
    }

    /// 📡️ Publishes one document roster, then its member-directory projection when that differs from the one this
    /// document last published ([`PresenceDirectoryProjections`]); `projections` is the held publication gate.
    fn publish_presence_delta(&self, projections: &mut PresenceDirectoryProjections, key: &str, space_id: &str, document_id: &str, snapshot: PresenceSnapshot) {
        let _ = self.fanout_for(key).send(ServerFrame::Presence { peers: snapshot.peers });
        if projections.published.get(key).map_or(snapshot.actors.is_empty(), |published| published == &snapshot.actors) {
            return;
        }
        if snapshot.actors.is_empty() {
            projections.published.remove(key);
        } else {
            projections.published.insert(key.to_string(), snapshot.actors.clone());
        }
        self.directory_service.publish(DirectoryStreamMessage::Presence { space_id: space_id.to_string(), document_id: document_id.to_string(), actors: snapshot.actors });
    }

    /// 🔁️ Subscribes one joining socket to the document fanout and reads the roster it must start
    /// from, both under the publication gate `publish_presence_delta`'s callers hold — so the
    /// replayed roster is never older than the first delta the same socket goes on to receive, and
    /// a join can never interleave with a delta into a roster neither side ever held.
    pub async fn subscribe_with_presence_replay(&self, key: &str) -> Option<(broadcast::Receiver<ServerFrame>, PresenceSnapshot)> {
        let _publication = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await.ok()?;
        let receiver = self.fanout_for(key).subscribe();
        Some((receiver, self.presence_snapshot(key)))
    }

    /// 🆕️ Selects one live socket as the actor's current owner without making it visible.
    pub async fn install_presence_slot(&self, key: &str, space_id: &str, document_id: &str, actor: &str, slot: PresenceLeaseSlot) -> PresenceLeaseTransition {
        let Ok(mut publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let replaced_visible = self.presence.with(&map_key, |slot| slot.is_some_and(|slot| slot.peer.is_some()));
        self.presence.insert(map_key, slot);
        if replaced_visible {
            self.publish_presence_delta(&mut publication, key, space_id, document_id, self.presence_snapshot(key));
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// 🪪️ Retains only canonical ephemerals under the matching socket's admitted identity.
    pub async fn refresh_document_presence(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str, peer: Vec<u8>, now: tokio::time::Instant) -> PresenceLeaseTransition {
        let Ok(input) = protocol::decode_presence_peer(&peer).await else { return PresenceLeaseTransition::Rejected };
        let normalized = self.presence.with(&(key.to_string(), actor.to_string()), |slot| {
            slot.filter(|slot| slot.socket_live_id == socket_live_id).map(|slot| protocol::PresencePeer {
                actor: actor.to_string(),
                connected_at_ms: slot.connected_at_ms,
                label: slot.label.clone(),
                user_id: slot.user_id.clone(),
                role: slot.role.clone(),
                color: Some(slot.color),
                surface: slot.document_surface.clone(),
                presence_pack: input.presence_pack,
                drag_ghost_json: input.drag_ghost_json,
                interaction: input.interaction,
                views: input.views,
                ui: input.ui,
                tool_run: input.tool_run,
                principal_kind: Some(slot.principal_kind),
                active_tool: input.active_tool,
                history_edit: input.history_edit,
                typing: Vec::new(),
            })
        });
        let Some(normalized) = normalized else { return PresenceLeaseTransition::NoChange };
        let peer = protocol::encode_presence_peer(&normalized).await;
        if peer.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES || protocol::decode_presence_peer(&peer).await.is_err() {
            return PresenceLeaseTransition::Rejected;
        }
        self.refresh_presence(key, space_id, document_id, actor, socket_live_id, peer, now).await
    }

    /// 💓️ Refreshes only the matching live owner and admits only a bounded visible roster.
    pub async fn refresh_presence(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str, peer: Vec<u8>, now: tokio::time::Instant) -> PresenceLeaseTransition {
        if peer.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES {
            return PresenceLeaseTransition::Rejected;
        }
        let Ok(mut publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let Some((was_visible, old_len, changed)) =
            self.presence.with(&map_key, |slot| slot.filter(|slot| slot.socket_live_id == socket_live_id).map(|slot| (slot.peer.is_some(), slot.peer.as_ref().map_or(0, Vec::len), slot.peer.as_ref() != Some(&peer))))
        else {
            return PresenceLeaseTransition::NoChange;
        };
        let mut visible = 0usize;
        let mut bytes = 0usize;
        self.presence.for_each(|(scope, _), slot| {
            if scope == key {
                if let Some(peer) = &slot.peer {
                    visible += 1;
                    bytes = bytes.saturating_add(peer.len());
                }
            }
        });
        if (!was_visible && visible >= PRESENCE_ROSTER_MAXIMUM_ITEMS) || bytes.checked_sub(old_len).and_then(|base| base.checked_add(peer.len())).is_none_or(|next| next > PRESENCE_ROSTER_MAXIMUM_BYTES) {
            return PresenceLeaseTransition::Rejected;
        }
        self.presence.with_mut(&map_key, |slot| {
            if let Some(slot) = slot.filter(|slot| slot.socket_live_id == socket_live_id) {
                slot.expires_at = now + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS);
                if changed {
                    slot.peer = Some(peer);
                }
            }
        });
        if changed {
            self.publish_presence_delta(&mut publication, key, space_id, document_id, self.presence_snapshot(key));
            if !was_visible {
                self.note_presence("server.presence.join", space_id, document_id, actor);
            }
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// ⏳️ Strips a due peer back to the identity its socket was ADMITTED with, and re-arms the lease.
    /// Presence is a property of the open socket, not of the client's willingness to beat: a lapsed
    /// lease means the app-owned ephemerals (`presence_pack`, `drag_ghost_json`, `interaction`,
    /// `views`, `ui`, `tool_run`) are stale, never that the human left. Only
    /// `close_presence_for_live` removes a row. Measured before this law existed (ticket
    /// 26/09/18 slice PR1, `🗑️generated/pr1-before-hub-socket.txt` run B): with two document sockets
    /// held open and their clients silent, BOTH rosters were empty 15 s later while neither socket
    /// had closed — the shape C3 §3.4 saw in two browsers.
    pub async fn expire_presence_for_live(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str, now: tokio::time::Instant) -> PresenceLeaseTransition {
        let Some(identity) = self.presence.with(&(key.to_string(), actor.to_string()), |slot| {
            slot.filter(|slot| slot.socket_live_id == socket_live_id && slot.peer.is_some() && now >= slot.expires_at).map(PresenceIdentityV1::of)
        }) else {
            return PresenceLeaseTransition::NoChange;
        };
        let stripped_peer = identity.encode(actor).await;
        let Ok(mut publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let expired = self.presence.with_mut(&map_key, |slot| {
            let Some(slot) = slot.filter(|slot| slot.socket_live_id == socket_live_id && slot.peer.is_some() && now >= slot.expires_at) else { return false };
            slot.expires_at = now + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS);
            if slot.peer.as_ref() == Some(&stripped_peer) {
                return false;
            }
            slot.peer = Some(stripped_peer);
            true
        });
        if expired {
            self.publish_presence_delta(&mut publication, key, space_id, document_id, self.presence_snapshot(key));
            self.note_presence("server.presence.expiry", space_id, document_id, actor);
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// 🧹️ Removes only the matching live owner; a stale handler cannot erase its replacement.
    pub async fn close_presence_for_live(&self, key: &str, space_id: &str, document_id: &str, actor: &str, socket_live_id: &str) -> PresenceLeaseTransition {
        let Ok(mut publication) = tokio::time::timeout(std::time::Duration::from_secs(2), self.presence_publication_gate.lock()).await else { return PresenceLeaseTransition::Unavailable };
        let map_key = (key.to_string(), actor.to_string());
        let visible = self.presence.with(&map_key, |slot| slot.filter(|slot| slot.socket_live_id == socket_live_id).is_some_and(|slot| slot.peer.is_some()));
        if !self.presence.remove_if(&map_key, |slot| slot.socket_live_id == socket_live_id) {
            return PresenceLeaseTransition::NoChange;
        }
        if visible {
            self.publish_presence_delta(&mut publication, key, space_id, document_id, self.presence_snapshot(key));
            self.note_presence("server.presence.leave", space_id, document_id, actor);
            PresenceLeaseTransition::Published
        } else {
            PresenceLeaseTransition::NoChange
        }
    }

    /// 🎨️ Contract §C7.3: an existing lease for `actor` in `space` is ref-counted and its
    /// index reused; otherwise the lowest index in `0..=255` not currently held by any live actor of
    /// `space`, wrapping `n % 256` once all 256 are taken.
    pub fn acquire_color(&self, space: &str, actor: &str) -> u8 {
        self.session_colors.mutate_or_default(space.to_string(), |colors| {
            if let Some(lease) = colors.by_actor.get_mut(actor) {
                lease.refs += 1;
                return lease.index;
            }
            let used: BTreeSet<u8> = colors.by_actor.values().map(|lease| lease.index).collect();
            let index = (0..=255u8).find(|candidate| !used.contains(candidate)).unwrap_or((colors.by_actor.len() as u32 % 256) as u8);
            colors.by_actor.insert(actor.to_string(), ColorLease { index, refs: 1 });
            index
        })
    }

    /// 🎨️ `refs -= 1`, dropping the lease at 0 — freed on the last disconnect of that actor's
    /// shell session across all of its document sockets in `space`.
    pub fn release_color(&self, space: &str, actor: &str) {
        self.session_colors.with_mut(space, |colors| {
            let Some(colors) = colors else { return };
            let drop_lease = match colors.by_actor.get_mut(actor) {
                Some(lease) => {
                    lease.refs = lease.refs.saturating_sub(1);
                    lease.refs == 0
                }
                None => false,
            };
            if drop_lease {
                colors.by_actor.remove(actor);
            }
        });
    }

    /// 🗂️ Get-or-create: after the caller has authenticated and validated the durable
    /// descriptor, a document is lazily minted in `db`'s catalog on its first open. Concurrent
    /// opens resolve to the same live handle.
    pub async fn ensure_document(&self, id: &ProtocolArtifactId) -> Result<db::ArtifactHandle, db::DbError> {
        match self.db.ensure_document(id).await {
            Ok(handle) => Ok(handle),
            Err(mut rejected) => loop {
                match rejected.retry_close().await {
                    Ok(error) => return Err(error),
                    Err(retained) => {
                        rejected = retained;
                        semio_framework_async::yield_once().await;
                    }
                }
            },
        }
    }
}
//#endregion 🔖️State
