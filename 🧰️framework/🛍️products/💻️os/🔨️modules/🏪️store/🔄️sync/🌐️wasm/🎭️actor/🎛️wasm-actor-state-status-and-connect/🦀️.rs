//! 🎛️ Wasm actor state, status and connect: the socket and admission limits, the hub and bootstrap state, the browser clock and sleep, the actor struct, its status and the socket connect, reconnect and link operations. See [`super`].

use super::*;

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

pub(super) type SharedDialer = Arc<RwLock<Option<Arc<dyn DocumentSocketDialer>>>>;

/// 🔐️ What the host lends one browser actor for its hub binding: the late-bound credential, grant
/// source and dialer (sign-in may come after the open), the verified lease, and the document's cancel.
pub(in super::super) struct WasmActorHub {
    pub credential: SharedCredential,
    pub socket_grant_source: SharedGrantSource,
    pub(in super::super) dialer: SharedDialer,
    pub lease: Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    pub seed: Option<semio_framework_os_directory::CanonicalCheckpointPairV1>,
    pub cancel: semio_framework_async::CancelToken,
}

pub(super) struct PendingWasmArtifactBootstrap {
    pub assembler: ArtifactBootstrapAssembler,
    pub(super) started_ms: u64,
    pub resume_token: String,
    pub baseline_frontier: RuntimeFrontierSummary,
    pub required_tail_frontier: RuntimeFrontierSummary,
    pub pack_schema_hash: [u8; 32],
}

pub(super) struct WasmBootstrapControl {
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
pub(super) fn wall_ms() -> u64 {
    js_sys::Date::now() as u64
}

/// 💤️ Resolves after `ms` on the global `setTimeout`, which a page and a dedicated Worker both own —
/// the actor's own cadence, independent of whoever pumps the renderer's cooperative pool.
pub(super) async fn sleep_ms(ms: u64) {
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

pub(super) struct WasmActor {
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
    pub(super) dialer: SharedDialer,
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
    pub(super) async fn connect(&mut self) {
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

    pub(super) fn close_socket(&mut self) {
        if let Some(mut socket) = self.socket.take() {
            socket.close();
        }
        self.clear_socket_epoch();
    }

    /// 🔁️ A socket that closed, lost frames or broke the protocol: keep unacknowledged work, drop the
    /// epoch, and back off before a fresh admission (a catch-up resumes from the last frontier).
    pub(super) fn disconnect(&mut self) {
        self.abort_artifact_bootstrap();
        self.requeue_pending_batches();
        self.close_socket();
        self.schedule_reconnect();
    }

    /// ⏳️ How long the loop may sleep before this actor has work of its own.
    pub(super) fn next_wake_ms(&self) -> u64 {
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
    pub(super) async fn pump_socket(&mut self) {
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

    pub(super) async fn send_frame(&mut self, frame: &ClientFrame, lane: Lane) {
        let bytes = encode_client_frame(frame, lane).await;
        self.send_encoded(bytes).await;
    }

    pub(super) async fn send_encoded(&mut self, bytes: Vec<u8>) {
        if self.hello.is_some() || !self.socket.as_ref().is_some_and(|socket| socket.is_open()) {
            return;
        }
        let Some(socket) = self.socket.as_mut() else { return };
        if socket.send_binary(bytes).is_err() {
            self.disconnect();
        }
    }
    //#endregion 🔖️Connect

}
