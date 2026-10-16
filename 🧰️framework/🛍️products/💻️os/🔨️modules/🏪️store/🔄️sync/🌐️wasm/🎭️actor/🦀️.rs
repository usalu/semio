//! 🎬️ The browser document actor: one actor per open document on the page executor, its hub binding, and the browser `ArtifactHost` that owns them.

use semio_framework_os_store_sync::{ARTIFACT_BOOTSTRAP_DEADLINE_MS, ArtifactActorConfig, ArtifactActorMsg, ArtifactChannelsOf, ArtifactDocumentKey, ArtifactEvent, ArtifactHostCore, ArtifactMailboxReceiver, ArtifactSyncStatus, CommandAckOutcome, CommandRejectionCode, CommandRejectionDetail, DOCUMENT_LINK_SHORTAGE_POLICY, DocumentBackboneRetentionV1, DocumentLink, DocumentLinkEvent, DocumentSocket, DocumentSocketBinding, DocumentSocketDialer, DocumentSocketPoll, OpenDocument, PersistenceBinding, PresenceHeartbeatProducer, RemoteState, actor_identity_admission_admits, add_actor_identity_progress, admit_remote_envelopes, artifact_mailbox_pair, canonical_pair_baseline, commands_frames_within, decode_document_backbone_message_exact, document_admission_refuses_access, document_link_terminal_message, document_pack_schema_hash, document_socket_authority_admits, document_socket_expectation, document_socket_hello, document_socket_protocols, frontier_reaches, hub_ws_url, note_authored_envelopes, now_ms, presence_from_bytes, presence_to_bytes, rebootstrap_control, replica_hlc_seed, rollback_envelope, settle_committed_envelopes, spr_events, stamp_session, transition_refusal, validate_artifact_bootstrap_identity, wire_envelopes_with_original_actor};
use semio_framework_os_spr::{AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationMessage, PresencePeer, PresenceToolRun, RuntimeFrontierSummary, ServerFrame, decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes};
use semio_framework_os_store_backbone::{BackboneMessage, ChannelBackbone, ChannelBackboneRemote};
use tokio::sync::broadcast;
use std::sync::{Arc, RwLock};
use wasm_bindgen::JsCast;

/// ⏱️ How often a dialed socket's page-owned receive queue is read while it is connecting or live.
const DOCUMENT_SOCKET_POLL_MS: u64 = 16;
/// ⏱️ The longest the actor sleeps with nothing dialed and nothing scheduled.
const DOCUMENT_ACTOR_IDLE_MS: u64 = 1_000;
/// 📃️ Frames one socket turn hands to the actor before it yields to its mailbox again.
const DOCUMENT_SOCKET_FRAMES_PER_TURN: usize = 32;
/// ⏱️ The admission deadline, the native actor's own.
const DOCUMENT_ADMISSION_TIMEOUT_MS: u64 = 5_000;

type SharedCredential = Arc<RwLock<Option<Arc<semio_framework_os_directory_client::LocalHubCredential>>>>;
type SharedGrantSource = Arc<RwLock<Option<Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>>>>;
type SharedDialer = Arc<RwLock<Option<Arc<dyn DocumentSocketDialer>>>>;

/// 🔐️ What the host lends one browser actor for its hub binding: the late-bound credential, grant
/// source and dialer (sign-in may come after the open), the verified lease, and the document's cancel.
pub(super) struct WasmActorHub {
    pub credential: SharedCredential,
    pub socket_grant_source: SharedGrantSource,
    pub(super) dialer: SharedDialer,
    pub lease: Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    pub seed: Option<semio_framework_os_directory::CanonicalCheckpointPairV1>,
    pub cancel: semio_framework_async::CancelToken,
}

struct PendingWasmArtifactBootstrap {
    pub assembler: ArtifactBootstrapAssembler,
    started_ms: u64,
    pub resume_token: String,
    pub baseline_frontier: RuntimeFrontierSummary,
    pub required_tail_frontier: RuntimeFrontierSummary,
    pub pack_schema_hash: [u8; 32],
}

struct WasmBootstrapControl {
    pub cancelled: bool,
    pub now_ms: u64,
    pub events: broadcast::Sender<ArtifactEvent>,
}

impl ArtifactBootstrapControl for WasmBootstrapControl {
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

/// 🕰️ Wall-clock milliseconds; `std`'s `SystemTime` panics on this target.
fn wall_ms() -> u64 {
    js_sys::Date::now() as u64
}

/// 💤️ Resolves after `ms` on the global `setTimeout`, which a page and a dedicated Worker both own —
/// the actor's own cadence, independent of whoever pumps the renderer's cooperative pool.
async fn sleep_ms(ms: u64) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let global = js_sys::global();
        if let Ok(set_timeout) = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("setTimeout")).and_then(|value| value.dyn_into::<js_sys::Function>().map_err(Into::into)) {
            let _ = set_timeout.call2(&global, &resolve, &wasm_bindgen::JsValue::from_f64(ms as f64));
        }
    });
    let _ = semio_framework_async::browser::JsFuture::from(promise).await;
}

/// 🧯️ Overwrites a secret string's bytes before it is dropped.
fn wipe(value: &mut String) {
    unsafe { value.as_mut_vec().fill(0) };
}

struct WasmActor {
    pub actor_identity_grant: semio_framework_value::RetainedCloneGrant,
    pub actor_identity_progress: semio_framework_value::RetainedCloneProgress,
    pub document_id: String,
    pub schema: String,
    pub remote: ChannelBackboneRemote,
    pub events: broadcast::Sender<ArtifactEvent>,
    pub hub_base_url: Option<String>,
    pub hub_space_id: Option<String>,
    pub hub_surface: Option<String>,
    pub credential: SharedCredential,
    pub socket_grant_source: SharedGrantSource,
    dialer: SharedDialer,
    pub document_execution_target_lease: Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    pub operation_cancel: semio_framework_async::CancelToken,
    pub socket: Option<Box<dyn DocumentSocket>>,
    pub hello: Option<ClientFrame>,
    pub socket_actor: Option<semio_framework_value::SharedUtf8>,
    pub socket_actor_confirmed: bool,
    /// 🎨️ See the native actor's matching field — same role, browser side.
    pub session_color: Option<u8>,
    pub server_frontier: Option<RuntimeFrontierSummary>,
    pub resume_token: Option<String>,
    pub pending_resume_token: Option<String>,
    pub required_tail_frontier: Option<RuntimeFrontierSummary>,
    pub artifact_rebootstrap_required: bool,
    pub artifact_bootstrap: Option<PendingWasmArtifactBootstrap>,
    pub pending_batches: std::collections::HashMap<u64, Vec<MutationEnvelope>>,
    pub outbox: Vec<MutationEnvelope>,
    /// 🔁️ Every operation id this replica authored or applied — the echo filter ([`admit_remote_envelopes`]).
    pub applied_op_ids: std::collections::HashSet<String>,
    pub document_backbone_retention: DocumentBackboneRetentionV1,
    pub next_batch_id: u64,
    pub next_local_rejection_batch_id: u64,
    pub hlc_seed: Option<u64>,
    pub remote_state: RemoteState,
    pub last_status: Option<ArtifactSyncStatus>,
    pub link: DocumentLink,
}

impl WasmActor {
    //#region 🔖️Status
    pub fn status(&self) -> ArtifactSyncStatus {
        let pending_mutations = self.pending_batches.values().map(Vec::len).sum();
        let acknowledged_head = if pending_mutations == 0 && self.outbox.is_empty() { self.server_frontier.as_ref().and_then(acknowledged_edited_head) } else { None };
        ArtifactSyncStatus { persisted: self.server_frontier.is_some(), pending_mutations, remote: self.remote_state.clone(), acknowledged_head }
    }

    pub fn set_remote_state(&mut self, state: RemoteState) {
        if matches!(state, RemoteState::Live { .. }) {
            self.link = self.link.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, DocumentLinkEvent::Restored { now_ms: wall_ms() });
        }
        self.remote_state = state;
        self.emit_status_if_changed();
    }

    pub fn emit_status_if_changed(&mut self) {
        let status = self.status();
        if self.last_status.as_ref() != Some(&status) {
            self.last_status = Some(status.clone());
            let _ = self.events.send(ArtifactEvent::Status(status));
        }
    }
    //#endregion 🔖️Status

    //#region 🔖️Connect
    /// 🌐️ One hub admission and dial, the native actor's `start_connect_hub`/`finish_connect_hub`
    /// sequence on this isolate: every refusal backs off with the same doubling law and nothing
    /// is dialed until credential, grant source, dialer, replica seed and kind identity all exist.
    async fn connect(&mut self) {
        let Some(base_url) = self.hub_base_url.clone() else { return };
        if self.artifact_rebootstrap_required || self.socket.is_some() || self.operation_cancel.is_cancelled_now() || !self.link.retry_due(wall_ms()) {
            return;
        }
        let credential = self.credential.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let source = self.socket_grant_source.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let dialer = self.dialer.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let (Some(credential), Some(source), Some(dialer), Some(hlc_seed)) = (credential, source, dialer, self.hlc_seed) else {
            self.schedule_reconnect();
            return;
        };
        let Some(pack_schema_hash) = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await else {
            self.schedule_reconnect();
            return;
        };
        let space_id = self.hub_space_id.clone().unwrap_or_default();
        let expectation = document_socket_expectation(&self.schema, pack_schema_hash, self.hub_surface.as_deref(), self.document_execution_target_lease.as_ref());
        if !actor_identity_admission_admits(self.actor_identity_grant){self.schedule_reconnect();return;}
        let client_instance_id = format!("browser-document-{hlc_seed:016x}");
        self.set_remote_state(RemoteState::Connecting);
        let ctx = semio_framework_async::OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 1, deadline_ms: None, cancel: self.operation_cancel.child_now(), capability: None };
        let admission = source.admit_document_socket(&ctx, &space_id, &self.document_id, &expectation, &client_instance_id, DOCUMENT_ADMISSION_TIMEOUT_MS).await;
        let semio_framework_os_directory_client::DocumentSocketAdmissionV1 { mut socket, authority } = match admission {
            Ok(admission) => admission,
            Err(error) if document_admission_refuses_access(&error) => {
                self.link_event(DocumentLinkEvent::Refused { now_ms: wall_ms() });
                return;
            }
            Err(_) => {
                self.schedule_reconnect();
                return;
            }
        };
        let binding = DocumentSocketBinding {
            hub_base_url: &base_url,
            space_id: &space_id,
            document_id: &self.document_id,
            schema: &self.schema,
            pack_schema_hash,
            surface: self.hub_surface.as_deref(),
            lease: self.document_execution_target_lease.as_ref(),
        };
        if ctx.cancel.is_cancelled_now() || !document_socket_authority_admits(&authority, &binding, wall_ms()) {
            self.schedule_reconnect();
            return;
        }
        let url = hub_ws_url(&authority.hub_origin, &authority.scope.space_id, &authority.scope.document_id, Some(&authority.surface.surface_id)).await;
        let Ok(capability) = credential.capability() else {
            self.schedule_reconnect();
            return;
        };
        let mut protocols = document_socket_protocols(&socket.protocol, capability);
        let dialed = dialer.dial(&url, &protocols);
        wipe(&mut protocols[1]);
        let Ok(dialed) = dialed else {
            self.schedule_reconnect();
            return;
        };
        self.socket = Some(dialed);
        let(original_actor,receipt)=semio_framework_value::SharedUtf8::admit(std::mem::take(&mut socket.actor_id),self.actor_identity_grant).unwrap_or_else(|(error,_)|panic!("original pre-admitted browser actor invariant: {error}"));
        add_actor_identity_progress(&mut self.actor_identity_progress,receipt);
        self.socket_actor = Some(original_actor);
        self.socket_actor_confirmed = false;
        self.hub_surface = Some(authority.surface.surface_id.clone());
        self.hello = Some(document_socket_hello(&self.schema, pack_schema_hash, self.resume_token.clone(), self.server_frontier.clone()));
        self.session_color = None;
    }

    /// 🔁️ A failed attempt or a lost socket: the link backs off along [`DOCUMENT_LINK_SHORTAGE_POLICY`] — or
    /// expires, once the shortage outlasts its bound.
    pub fn schedule_reconnect(&mut self) {
        let now_ms = wall_ms();
        self.link_event(DocumentLinkEvent::Failed { now_ms });
        if let Some(retry_at_ms) = self.link.retry_at_ms() {
            self.set_remote_state(RemoteState::Backoff { retry_in_ms: retry_at_ms.saturating_sub(now_ms) });
        }
    }

    /// 🔌️ Applies one link event; a link that turns terminal closes its socket, reports `Detached` and emits the
    /// coded terminal message a shell localizes ([`document_link_terminal_message`]). Local work stays queued.
    pub fn link_event(&mut self, event: DocumentLinkEvent) {
        let was_open = self.link.admits_local_edits();
        self.link = self.link.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, event);
        if was_open && !self.link.admits_local_edits() {
            self.abort_artifact_bootstrap();
            self.requeue_pending_batches();
            self.close_socket();
            self.set_remote_state(RemoteState::Detached);
            let _ = self.events.send(ArtifactEvent::Conflict(document_link_terminal_message(&self.document_id, self.link.status())));
        }
    }

    /// ⏳️ Lets time pass for an unlinked hub link: past the shortage bound it expires.
    pub fn tick_link(&mut self) {
        if self.hub_base_url.is_some() && matches!(self.link, DocumentLink::Unlinked { .. }) {
            self.link_event(DocumentLinkEvent::Tick { now_ms: wall_ms() });
        }
    }

    pub fn clear_socket_epoch(&mut self) {
        self.socket_actor = None;
        self.socket_actor_confirmed = false;
        self.session_color = None;
        self.hello = None;
    }

    fn close_socket(&mut self) {
        if let Some(mut socket) = self.socket.take() {
            socket.close();
        }
        self.clear_socket_epoch();
    }

    /// 🔁️ A socket that closed, lost frames or broke the protocol: keep unacknowledged work, drop the
    /// epoch, and back off before a fresh admission (a catch-up resumes from the last frontier).
    fn disconnect(&mut self) {
        self.abort_artifact_bootstrap();
        self.requeue_pending_batches();
        self.close_socket();
        self.schedule_reconnect();
    }

    /// ⏳️ How long the loop may sleep before this actor has work of its own.
    fn next_wake_ms(&self) -> u64 {
        if self.socket.is_some() {
            return DOCUMENT_SOCKET_POLL_MS;
        }
        match (self.hub_base_url.as_ref(), self.link.next_deadline_ms(&DOCUMENT_LINK_SHORTAGE_POLICY)) {
            (Some(_), Some(at)) => at.saturating_sub(wall_ms()).clamp(1, DOCUMENT_ACTOR_IDLE_MS),
            _ => DOCUMENT_ACTOR_IDLE_MS,
        }
    }

    /// 📥️ One socket turn: greet the hub once the page reports the socket open, then hand at most one
    /// page of frames to the protocol. A close or any lost frame reconnects; the admission plan's
    /// expiry bounds when its grant may be exchanged, never the live socket (the hub re-proves the
    /// socket's authority every second).
    async fn pump_socket(&mut self) {
        let Some(socket) = self.socket.as_mut() else { return };
        if self.hello.is_some() && socket.is_open() {
            let hello = self.hello.take().expect("hello was just observed");
            let bytes = encode_client_frame(&hello, Lane::Command).await;
            let Some(socket) = self.socket.as_mut() else { return };
            if socket.send_binary(bytes).is_err() {
                self.disconnect();
                return;
            }
        }
        for _ in 0..DOCUMENT_SOCKET_FRAMES_PER_TURN {
            let Some(socket) = self.socket.as_mut() else { return };
            match socket.poll() {
                DocumentSocketPoll::Frame(bytes) => self.on_binary(&bytes).await,
                DocumentSocketPoll::Pending => return,
                DocumentSocketPoll::Closed(_) | DocumentSocketPoll::Lost(_) => {
                    self.disconnect();
                    return;
                }
            }
        }
    }

    async fn send_frame(&mut self, frame: &ClientFrame, lane: Lane) {
        let bytes = encode_client_frame(frame, lane).await;
        self.send_encoded(bytes).await;
    }

    async fn send_encoded(&mut self, bytes: Vec<u8>) {
        if self.hello.is_some() || !self.socket.as_ref().is_some_and(|socket| socket.is_open()) {
            return;
        }
        let Some(socket) = self.socket.as_mut() else { return };
        if socket.send_binary(bytes).is_err() {
            self.disconnect();
        }
    }
    //#endregion 🔖️Connect

    //#region 🔖️Relay
    /// 🧺️ Builds + sends `Commands` batches under the hub-issued socket actor, tracking each in
    /// `pending_batches` for {@link WasmActor::handle_ack}. Mirrors the native `relay_operations_to_hub`:
    /// nothing leaves before the hub confirmed the socket actor and every catch-up completed. A batch
    /// whose frame exceeds the socket's own ceiling is halved until it fits; one envelope that can never
    /// fit is rolled back locally and reported, never retried into a reconnect loop.
    async fn relay_operations(&mut self, envelopes: &[MutationEnvelope]) {
        if envelopes.is_empty() {
            return;
        }
        note_authored_envelopes(&mut self.applied_op_ids, envelopes);
        let Some(socket_actor) = self.socket_actor.as_ref() else {
            self.queue_outbox(envelopes.iter().cloned());
            return;
        };
        if !self.socket_actor_confirmed || self.socket.is_none() || self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
            self.queue_outbox(envelopes.iter().cloned());
            return;
        }
        let max_frame = self.socket.as_ref().map_or(usize::MAX, |socket| socket.max_frame_bytes());
        let(wire_envelopes,identity_receipt)=match wire_envelopes_with_original_actor(envelopes,socket_actor,self.actor_identity_grant){Ok(result)=>result,Err(_)=>{self.queue_outbox(envelopes.iter().cloned());return;}};
        add_actor_identity_progress(&mut self.actor_identity_progress,identity_receipt);
        let plan = commands_frames_within(max_frame, self.next_batch_id, envelopes.to_vec(), wire_envelopes).await;
        for frame in plan.frames {
            self.next_batch_id = frame.batch_id.wrapping_add(1);
            self.pending_batches.insert(frame.batch_id, frame.local);
            self.send_encoded(frame.bytes).await;
        }
        if !plan.oversized.is_empty() {
            self.refuse_oversized_batch(plan.oversized).await;
        }
        self.emit_status_if_changed();
    }

    /// 📏️ One envelope whose `Commands` frame exceeds the socket's ceiling on its own: the hub can never
    /// receive it, so its speculative local head is rolled back and the refusal is published.
    async fn refuse_oversized_batch(&mut self, local: Vec<MutationEnvelope>) {
        self.document_backbone_retention.release(&local);
        let mut rollbacks: Vec<MutationEnvelope> = Vec::with_capacity(local.len());
        for envelope in local.iter().rev() {
            rollbacks.extend(rollback_envelope(envelope).await);
        }
        let _ = self.deliver_remote_operations(rollbacks).await;
        let refusal = self.retract_refused_transitions(&local, "document socket frame exceeds the socket's ceiling").await;
        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalSocketFrameCeiling, "document socket frame exceeds the socket's ceiling", CommandRejectionDetail { envelopes: Some(local.len() as u64), ..CommandRejectionDetail::default() }));
        if let Some(refusal) = refusal {
            let _ = self.events.send(ArtifactEvent::Conflict(refusal));
        }
    }

    pub async fn relay_one_backbone(&mut self) -> Result<bool, semio_framework_os_vcs::VcsError> {
        let Some(message) = self.remote.try_pop_front()? else { return Ok(false) };
        match message {
            BackboneMessage::Mutations { envelopes } => {
                let envelopes = decode_envelopes(&envelopes).unwrap_or_default();
                self.relay_operations(&envelopes).await;
            }
            BackboneMessage::Genesis { .. } | BackboneMessage::Ack { .. } => {}
            BackboneMessage::Member { .. } => return Err(semio_framework_os_vcs::VcsError::Backbone("a composed member requires its exact member transport lane".into())),
            BackboneMessage::Retract { .. } => return Err(semio_framework_os_vcs::VcsError::Backbone("a retraction flows from the actor to its store, never back".into())),
        }
        Ok(true)
    }

    pub async fn handle_cmd(&mut self, message: ArtifactActorMsg) {
        match message {
            ArtifactActorMsg::LocalMutations { envelopes } => {
                let drained = self.relay_one_backbone().await.unwrap_or(false);
                if !drained && !envelopes.is_empty() {
                    self.relay_operations(&envelopes).await;
                }
            }
            ArtifactActorMsg::DocumentBackbone { message } => {
                let envelopes = match decode_document_backbone_message_exact(&message) {
                    Ok(envelopes) if envelopes.iter().all(|envelope| envelope.document_id.0 == self.document_id) => envelopes,
                    Ok(envelopes) => {
                        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneScopeMismatch, "document backbone scope mismatch", CommandRejectionDetail { envelopes: Some(envelopes.len() as u64), ..CommandRejectionDetail::default() }));
                        return;
                    }
                    Err(_) => {
                        self.reject_document_backbone(CommandAckOutcome::local_rejected(CommandRejectionCode::LocalBackboneMalformed, "document backbone malformed", CommandRejectionDetail { bytes: Some(message.len() as u64), ..CommandRejectionDetail::default() }));
                        return;
                    }
                };
                if let Err(refusal) = self.document_backbone_retention.retain(message.len(), &envelopes) {
                    self.reject_document_backbone(refusal);
                    return;
                }
                if self.hub_space_id.is_some() {
                    self.relay_operations(&envelopes).await;
                } else {
                    self.document_backbone_retention.release(&envelopes);
                }
            }
            ArtifactActorMsg::LocalDocumentArchive { archive } => {
                let _ = semio_framework_os_spr::decode_document_archive_bytes(&archive).await;
            }
            ArtifactActorMsg::PresenceHeartbeat { mut peer } => {
                stamp_session(&mut peer, self.session_color, self.hub_surface.as_deref()).await;
                self.send_frame(&ClientFrame::Presence { peer: presence_to_bytes(&peer).await }, Lane::Preview).await;
            }
            ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                self.send_frame(&ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
            }
            ArtifactActorMsg::Reseed { baseline, .. } => self.reseed_hub_document(baseline),
            ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
        }
    }

    pub fn reject_document_backbone(&mut self, outcome: CommandAckOutcome) {
        let batch_id = self.next_local_rejection_batch_id;
        self.next_local_rejection_batch_id = self.next_local_rejection_batch_id.wrapping_sub(1);
        let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome });
    }

    pub fn requeue_pending_batches(&mut self) {
        let mut batches: Vec<(u64, Vec<MutationEnvelope>)> = self.pending_batches.drain().collect();
        batches.sort_by_key(|(batch_id, _)| *batch_id);
        for (_, envelopes) in batches {
            self.queue_outbox(envelopes);
        }
    }

    pub fn queue_outbox(&mut self, envelopes: impl IntoIterator<Item = MutationEnvelope>) {
        let mut queued: std::collections::HashSet<String> = self.outbox.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
        for envelope in envelopes {
            if queued.insert(envelope.mutation_id.0.clone()) {
                self.outbox.push(envelope);
            }
        }
    }

    pub async fn flush_outbox(&mut self) {
        if self.outbox.is_empty() || self.socket.is_none() {
            return;
        }
        let envelopes = std::mem::take(&mut self.outbox);
        self.relay_operations(&envelopes).await;
    }
    //#endregion 🔖️Relay

    //#region 🔖️Bootstrap
    pub fn bootstrap_control(&self, cancelled: bool) -> WasmBootstrapControl {
        WasmBootstrapControl { cancelled, now_ms: wall_ms(), events: self.events.clone() }
    }

    pub fn abort_artifact_bootstrap(&mut self) {
        if let Some(mut pending) = self.artifact_bootstrap.take() {
            pending.assembler.abort();
        }
        self.pending_resume_token = None;
        self.required_tail_frontier = None;
    }

    /// ⚠️ A bootstrap or protocol fault is reported as the native actor reports it — one `Conflict` —
    /// and the socket reconnects for a fresh catch-up.
    pub fn fail_artifact_bootstrap(&mut self, detail: impl Into<String>) {
        let _ = self.events.send(ArtifactEvent::Conflict(MutationMessage { level: semio_framework_diagnostic::Severity::Error, code: semio_framework_diagnostic::FaultCode::new("artifactBootstrap"), message: detail.into(), target: vec![self.document_id.clone()], op_index: None }));
        self.disconnect();
    }

    /// ♻️ Hub lag forces a canonical pair refresh: keep unacked work queued, clear the projection tokens, drop the socket
    /// and ask the host for the pair — the native `require_artifact_rebootstrap`; no dial until the host's reseed.
    pub fn require_artifact_rebootstrap(&mut self, control: semio_framework_os_directory::RebootstrapRequired) {
        self.requeue_pending_batches();
        self.abort_artifact_bootstrap();
        self.server_frontier = None;
        self.resume_token = None;
        self.artifact_rebootstrap_required = true;
        self.close_socket();
        self.set_remote_state(RemoteState::Connecting);
        let _ = self.events.send(ArtifactEvent::RebootstrapRequired { control });
    }

    /// 🛟️ The browser twin of the native reseed: the pair's baseline is the next Hello's frontier, the unacked local
    /// operations go back to the guest, and the next turn dials at once.
    pub fn reseed_hub_document(&mut self, baseline: semio_framework_os_directory::ArtifactFrontier) {
        if !self.artifact_rebootstrap_required {
            return;
        }
        self.server_frontier = Some(canonical_pair_baseline(&baseline));
        if !self.outbox.is_empty() {
            let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes: self.outbox.clone() });
        }
        self.artifact_rebootstrap_required = false;
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
        self.set_remote_state(RemoteState::Live { peer_count: 0 });
        self.flush_outbox().await;
    }

    pub async fn start_artifact_bootstrap(&mut self, bootstrap: ArtifactBootstrap, resume_token: String, server_frontier: RuntimeFrontierSummary) {
        self.abort_artifact_bootstrap();
        let Some(pack_schema_hash) = document_pack_schema_hash(&self.schema, self.document_execution_target_lease.as_ref()).await else {
            self.fail_artifact_bootstrap(format!("no pack schema identity for schema {:?}", self.schema));
            return;
        };
        if let Err(error) = validate_artifact_bootstrap_identity(&bootstrap, &self.document_id, &self.schema, pack_schema_hash, &server_frontier) {
            self.fail_artifact_bootstrap(error);
            return;
        }
        let inline = bootstrap.inline.is_some();
        let started_ms = wall_ms();
        let baseline_frontier = bootstrap.baseline_frontier.clone();
        let required_tail_frontier = bootstrap.required_tail_frontier.clone();
        let mut control = self.bootstrap_control(false);
        let assembler = match ArtifactBootstrapAssembler::new(bootstrap.clone(), bootstrap.descriptor_hash, ArtifactBootstrapLimits::default(), Some(started_ms.saturating_add(ARTIFACT_BOOTSTRAP_DEADLINE_MS)), &mut control) {
            Ok(assembler) => assembler,
            Err(error) => {
                self.fail_artifact_bootstrap(error.to_string());
                return;
            }
        };
        let mut pending = PendingWasmArtifactBootstrap { assembler, started_ms, resume_token, baseline_frontier, required_tail_frontier, pack_schema_hash };
        if !inline {
            self.artifact_bootstrap = Some(pending);
            return;
        }
        let mut control = self.bootstrap_control(false);
        match pending.assembler.finish(None, &mut control) {
            Ok(pair) => {
                if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                    self.fail_artifact_bootstrap(error);
                }
            }
            Err(error) => self.fail_artifact_bootstrap(error.to_string()),
        }
    }

    /// 🗃️ Installs one assembled canonical pair. With a codec this process resolves for the kind
    /// (linked, or the mounted component's — [`crate::os_store::codec_registry::document_kind_codec`]) the pair is
    /// decoded here first; without one the pair's identity was already bound to the verified lease's
    /// pack schema hash, and the mounted guest that owns the kind validates the archive when the shell
    /// loads it.
    pub async fn install_artifact_bootstrap(&mut self, pending: PendingWasmArtifactBootstrap, pair: ArtifactBootstrapPair) -> Result<(), String> {
        if let Some(codec) = semio_framework_os_store_codec_registry::document_kind_codec(&self.schema).await.map_err(|error| error.to_string())? {
            if codec.pack_schema_hash().await.map_err(|error| error.to_string())? != pending.pack_schema_hash {
                return Err("artifact bootstrap codec changed during transfer".into());
            }
            codec.print_mirror(&pair.pack, &pair.spr).await.map_err(|error| format!("artifact bootstrap decode failed: {error}"))?;
        }
        let events = spr_events(&pair.spr, &self.document_id, &self.schema,self.actor_identity_grant).await.map_err(|error| format!("artifact bootstrap SPR failed: {error}"))?;
        self.remote.push(BackboneMessage::Genesis { pack: pair.pack.clone() }).await.map_err(|error| format!("artifact bootstrap event delivery failed: {error}"))?;
        if !events.is_empty() {
            self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&events) }).await.map_err(|error| format!("artifact bootstrap event delivery failed: {error}"))?;
        }
        let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: pair.pack, parent_spr: pair.spr, members: Vec::new() };
        let archive = semio_framework_os_spr::encode_document_archive_bytes(&archive).map_err(|error| format!("artifact bootstrap archive failed: {error}"))?;
        let _ = self.events.send(ArtifactEvent::DocumentArchiveReplaced { archive });
        self.server_frontier = Some(pending.baseline_frontier);
        self.pending_resume_token = Some(pending.resume_token);
        self.required_tail_frontier = Some(pending.required_tail_frontier);
        if !self.outbox.is_empty() && self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&self.outbox) }).await.is_err() {
            let _ = self.events.send(ArtifactEvent::Conflict(MutationMessage {
                level: semio_framework_diagnostic::Severity::Error,
                code: semio_framework_diagnostic::FaultCode::new("artifactBootstrapLocalReplay"),
                message: "artifact baseline committed; pending local replay will retry after reconnect".into(),
                target: vec![self.document_id.clone()],
                op_index: None,
            }));
            self.close_socket();
            self.schedule_reconnect();
            return Ok(());
        }
        self.finish_catchup_if_ready().await;
        Ok(())
    }
    //#endregion 🔖️Bootstrap

    //#region 🔖️Protocol
    /// 📡️ The native actor's `on_hub_frame`, frame for frame: the socket actor the grant named is the
    /// only origin this replica filters as its own, `Session` must confirm it, and `Live` is reached
    /// only after every catch-up.
    async fn on_binary(&mut self, bytes: &[u8]) {
        let frame = match decode_server_frame(bytes).await {
            Ok((_lane, frame)) => frame,
            Err(error) => {
                self.fail_artifact_bootstrap(format!("malformed hub frame: {error}"));
                return;
            }
        };
        match frame {
            ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                self.requeue_pending_batches();
                match bootstrap {
                    Bootstrap::None => {
                        self.abort_artifact_bootstrap();
                        self.resume_token = Some(resume_token);
                        self.server_frontier = Some(server_frontier);
                        self.set_remote_state(RemoteState::Live { peer_count: 0 });
                        self.flush_outbox().await;
                    }
                    Bootstrap::Tail => {
                        self.abort_artifact_bootstrap();
                        self.pending_resume_token = Some(resume_token);
                        self.required_tail_frontier = Some(server_frontier);
                        self.finish_catchup_if_ready().await;
                    }
                    Bootstrap::Snapshot { .. } => self.fail_artifact_bootstrap("database-private snapshot cannot seed an artifact client"),
                    Bootstrap::ArtifactBootstrap(bootstrap) => {
                        self.start_artifact_bootstrap(*bootstrap, resume_token, server_frontier).await;
                    }
                }
            }
            ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client"),
            ServerFrame::RebootstrapRequired { control } => {
                if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                    self.fail_artifact_bootstrap("rebootstrap control scope mismatch");
                } else {
                    self.require_artifact_rebootstrap(rebootstrap_control(&control));
                }
            }
            ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index, bytes } => {
                let Some(mut pending) = self.artifact_bootstrap.take() else {
                    self.fail_artifact_bootstrap("artifact bootstrap chunk arrived without an active transfer");
                    return;
                };
                let mut control = self.bootstrap_control(false);
                match pending.assembler.push(descriptor_hash, index, bytes.as_slice(), &mut control) {
                    Ok(_) => self.artifact_bootstrap = Some(pending),
                    Err(error) => self.fail_artifact_bootstrap(error.to_string()),
                }
            }
            ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count } => {
                let Some(mut pending) = self.artifact_bootstrap.take() else {
                    self.fail_artifact_bootstrap("artifact bootstrap completion arrived without an active transfer");
                    return;
                };
                let mut control = self.bootstrap_control(false);
                match pending.assembler.finish(Some((descriptor_hash, chunk_count)), &mut control) {
                    Ok(pair) => {
                        if let Err(error) = self.install_artifact_bootstrap(pending, pair).await {
                            self.fail_artifact_bootstrap(error);
                        }
                    }
                    Err(error) => self.fail_artifact_bootstrap(error.to_string()),
                }
            }
            ServerFrame::Commands { envelopes, frontier, .. } => {
                if self.artifact_bootstrap.is_some() {
                    self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion");
                    return;
                }
                let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                self.document_backbone_retention.release(&settled);
                note_authored_envelopes(&mut self.applied_op_ids, &settled);
                let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes);
                if !self.deliver_remote_operations(fresh).await {
                    self.fail_artifact_bootstrap("artifact tail could not be installed");
                    return;
                }
                self.server_frontier = Some(frontier);
                self.finish_catchup_if_ready().await;
            }
            ServerFrame::Ack { batch_id, stages, frontier } => {
                if self.artifact_bootstrap.is_some() || self.required_tail_frontier.is_some() {
                    self.fail_artifact_bootstrap("ack arrived before artifact catch-up completion");
                    return;
                }
                self.server_frontier = Some(frontier);
                self.handle_ack(batch_id, stages).await;
            }
            ServerFrame::Preview { actor, key, seq, payload } => {
                if self.socket_actor.as_deref() != Some(actor.0.as_str()) {
                    let _ = self.events.send(ArtifactEvent::Preview { actor: actor.0, key, seq, payload });
                }
            }
            ServerFrame::Presence { peers } => {
                let mut decoded: Vec<PresencePeer> = Vec::with_capacity(peers.len());
                for peer in &peers {
                    if let Some(peer) = presence_from_bytes(peer).await {
                        decoded.push(peer);
                    }
                }
                if matches!(self.remote_state, RemoteState::Live { .. }) && self.artifact_bootstrap.is_none() && self.required_tail_frontier.is_none() {
                    self.set_remote_state(RemoteState::Live { peer_count: decoded.len() });
                }
                let _ = self.events.send(ArtifactEvent::Presence { peers: decoded });
            }
            ServerFrame::Session { actor, color } => {
                if self.socket_actor.as_deref() != Some(actor.as_str()) {
                    self.fail_artifact_bootstrap("socket receipt actor mismatch");
                    return;
                }
                self.socket_actor_confirmed = true;
                self.session_color = Some(color);
                let _ = self.events.send(ArtifactEvent::Session { actor, color });
                self.flush_outbox().await;
            }
            ServerFrame::CreditGrant { .. } => {}
            ServerFrame::Error { code, message } => {
                let _ = self.events.send(ArtifactEvent::Conflict(MutationMessage {
                    level: semio_framework_diagnostic::Severity::Error,
                    code: semio_framework_diagnostic::FaultCode::new(code),
                    message,
                    target: vec![self.hub_base_url.clone().unwrap_or_default()],
                    op_index: None,
                }));
            }
        }
    }

    /// 📮️ Mirrors the native actor's `handle_ack` — see its doc comment.
    pub async fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
        for stage in stages {
            let AckStage::Applied { outcome } = stage else { continue };
            let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
            self.document_backbone_retention.release(&sent);
            match *outcome {
                ApplyOutcome::Accepted => {
                    let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                }
                ApplyOutcome::Transformed { envelope } => {
                    let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                    for envelope in sent.iter().rev() {
                        rollbacks.extend(rollback_envelope(envelope).await);
                    }
                    let _ = self.deliver_remote_operations(rollbacks).await;
                    let refusal = self.retract_refused_transitions(&sent, "transformed").await;
                    let _ = self.deliver_remote_operations(vec![*envelope]).await;
                    let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    if let Some(refusal) = refusal {
                        let _ = self.events.send(ArtifactEvent::Conflict(refusal));
                    }
                }
                ApplyOutcome::Rejected { reason, messages } => {
                    let mut rollbacks: Vec<MutationEnvelope> = Vec::new();
                    for envelope in sent.iter().rev() {
                        rollbacks.extend(rollback_envelope(envelope).await);
                    }
                    let _ = self.deliver_remote_operations(rollbacks).await;
                    let refusal = self.retract_refused_transitions(&sent, &reason).await;
                    let _ = self.events.send(ArtifactEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::hub_rejected(reason, &messages) });
                    if let Some(refusal) = refusal {
                        let _ = self.events.send(ArtifactEvent::Conflict(refusal));
                    }
                }
            }
        }
        self.emit_status_if_changed();
    }

    pub async fn deliver_remote_operations(&self, envelopes: Vec<MutationEnvelope>) -> bool {
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
            let _ = self.events.send(ArtifactEvent::DocumentBackbone { message });
        } else {
            let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes });
        }
        true
    }

    /// 🪞️ Mirrors the native actor's `retract_refused_transitions`; a browser actor persists no log of its own.
    pub async fn retract_refused_transitions(&self, refused: &[MutationEnvelope], reason: &str) -> Option<MutationMessage> {
        let refusal = transition_refusal(refused, reason)?;
        let _ = self.deliver_retraction(refusal.target.clone()).await;
        Some(refusal)
    }

    /// 🪩️ Mirrors the native actor's `deliver_retraction`.
    pub async fn deliver_retraction(&self, mutation_ids: Vec<String>) -> bool {
        let message = BackboneMessage::Retract { mutation_ids };
        let encoded = message.encode_op();
        if self.remote.push(message).await.is_err() {
            return false;
        }
        if self.hub_space_id.is_some() {
            let Ok(message) = encoded else { return false };
            let _ = self.events.send(ArtifactEvent::DocumentBackbone { message });
        }
        true
    }
    //#endregion 🔖️Protocol
}

/// 🚀️ Spawns one browser document actor on the isolate's executor. Its loop owns one cadence: the
/// mailbox, or — when the mailbox is quiet — the next socket poll or scheduled redial, whichever
/// comes first. A cancelled document (closed by the host) ends the loop and closes its socket.
pub(super) async fn spawn_actor(config: ArtifactActorConfig, remote: ChannelBackboneRemote, cmd_rx: ArtifactMailboxReceiver, events: broadcast::Sender<ArtifactEvent>, hub: WasmActorHub) {
    let mut hub_base_url = None;
    let mut hub_space_id = None;
    let mut hub_surface = None;
    for binding in &config.bindings {
        if let PersistenceBinding::Hub { base_url, space_id, surface } = binding {
            if hub_base_url.is_none() {
                hub_base_url = Some(base_url.clone());
                hub_space_id = Some(space_id.clone());
                hub_surface = surface.clone();
            }
        }
    }
    let mut actor = WasmActor {
        actor_identity_grant: config.actor_identity_grant,
        actor_identity_progress: Default::default(),
        document_id: config.document_id,
        schema: config.schema,
        remote,
        events,
        hub_base_url,
        hub_space_id,
        hub_surface,
        credential: hub.credential,
        socket_grant_source: hub.socket_grant_source,
        dialer: hub.dialer,
        document_execution_target_lease: hub.lease,
        operation_cancel: hub.cancel,
        socket: None,
        hello: None,
        socket_actor: None,
        socket_actor_confirmed: false,
        session_color: None,
        server_frontier: hub.seed.as_ref().map(|pair| canonical_pair_baseline(&pair.baseline_frontier)),
        resume_token: None,
        pending_resume_token: None,
        required_tail_frontier: None,
        artifact_rebootstrap_required: false,
        artifact_bootstrap: None,
        pending_batches: std::collections::HashMap::new(),
        outbox: Vec::new(),
        applied_op_ids: std::collections::HashSet::new(),
        document_backbone_retention: DocumentBackboneRetentionV1::default(),
        next_batch_id: 0,
        next_local_rejection_batch_id: u64::MAX,
        hlc_seed: replica_hlc_seed().ok(),
        remote_state: RemoteState::Detached,
        last_status: None,
        link: DocumentLink::opened(wall_ms()),
    };
    semio_framework_async::browser::spawn_local(async move {
        loop {
            actor.tick_link();
            actor.connect().await;
            let wake_ms = actor.next_wake_ms();
            tokio::select! {
                biased;
                cmd = cmd_rx.recv() => match cmd {
                    None => {
                        actor.document_backbone_retention.clear();
                        actor.close_socket();
                        break;
                    }
                    Some(ArtifactActorMsg::Detach) => {
                        actor.document_backbone_retention.clear();
                        let _ = actor.relay_one_backbone().await;
                        actor.close_socket();
                        break;
                    }
                    Some(message) => actor.handle_cmd(message).await,
                },
                _ = sleep_ms(wake_ms) => actor.pump_socket().await,
            }
            if actor.operation_cancel.is_cancelled_now() {
                actor.close_socket();
                break;
            }
        }
    });
}

/// 🎛️ The channels of a browser document actor, which has no runner.
pub type ArtifactChannels = ArtifactChannelsOf<()>;

/// 🏛️ Registry of open per-document actors of the browser wgpu build: one `ArtifactHost` per page
/// owns every open document's actor + event fan-out, and the page's document-socket dialer.
pub struct ArtifactHost {
    pub core: ArtifactHostCore<()>,
    document_socket_dialer: SharedDialer,
}

impl Clone for ArtifactHost {
    fn clone(&self) -> Self {
        Self { core: self.core.retain(), document_socket_dialer: self.document_socket_dialer.clone() }
    }
}

impl ArtifactHost {
    /// 🧩️ Creates a host on the process WorkerPool; callers must inject the same pool
    /// used by their service and renderer runtimes.
    pub fn new(pool: Arc<semio_framework_async::WorkerPool>) -> Self {
        Self { core: ArtifactHostCore::new(pool), document_socket_dialer: Arc::new(RwLock::new(None)) }
    }

    /// 🏛️ The platform-neutral registry this host composes with the browser actor.
    pub fn core(&self) -> &ArtifactHostCore<()> {
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

    /// ☎️ Installs the browser's document-socket dialer; every later hub document actor dials through it.
    pub fn set_document_socket_dialer(&self, dialer: Arc<dyn DocumentSocketDialer>) {
        *self.document_socket_dialer.write().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(dialer);
    }

    /// 🟢️ Whether a hub document opened now can dial at once: a credential, a grant source and
    /// the socket dialer the page owns.
    pub fn local_hub_ready(&self) -> bool {
        self.core.hub_binding_present() && self.document_socket_dialer.read().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
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
        spawn_actor(
            config,
            remote,
            cmd_rx,
            event_tx.clone(),
            WasmActorHub { credential: self.core.credential.clone(), socket_grant_source: self.core.socket_grant_source.clone(), dialer: self.document_socket_dialer.clone(), lease: document_execution_target_lease, seed, cancel: document_cancel.clone() },
        )
        .await;
        let entry = OpenDocument { generation, cancel: document_cancel, cmd_tx: cmd_tx.clone(), events: event_tx, presence: PresenceHeartbeatProducer::default(), runner: () };
        self.core.insert_document(document_key.clone(), entry);
        ArtifactChannelsOf { cmd_tx, document_key, runner: (), channel_backbone }
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

    /// ✂️ Detaches a document's actor and drops its registry entry.
    /// 🚫️async: E1 pure lock/remove + sync channel send, no real suspension point — consumed by
    /// `impl Drop` (sync-only external trait); see R9.
    pub fn close(&self, document_id: &str) -> Option<u64> {
        self.close_key(&ArtifactDocumentKey::local(document_id))
    }

    pub fn close_key(&self, document_key: &ArtifactDocumentKey) -> Option<u64> {
        let document = self.core.remove_document(document_key)?;
        document.cancel.cancel_now();
        let _ = document.cmd_tx.send(ArtifactActorMsg::Detach);
        Some(document.generation)
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

#[cfg(test)]
#[path = "../🧪️tests/🔬️echo-suppression-pin/🦀️.rs"]
mod echo_suppression_pin;
