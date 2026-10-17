//! 🔌️ `document_socket` — the document WebSocket and its wire projection.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory::access_policy::directory_access_permits;
use axum::extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use directory::{os_directory, DslValue, ToValue};
use directory::os_directory::{DirectoryConnectionPhase, DirectoryStreamMessage, DocumentOpenPlanErrorCodeV1};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use protocol::{decode_client_frame, encode_server_frame, AckStage, ActorId, ApplyOutcome, ClientFrame, Lane, MutationEnvelope, RuntimeFrontierSummary, ServerFrame};
use semio_framework_trace::record::TraceOutcome;
use semio_hub_artifact_creation::ArtifactCreationCommitAuthorityV1;
use semio_hub_artifact_authority::AuthorityOperationControl;
use semio_hub_auth::rate_limit::{RateLimitClassV1, RateLimitDecisionV1, RateLimitSubjectV1};
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use directory::os_directory::DocumentScope;
use semio_hub_directory::{DocumentDirectory, EventLog, SpaceDirectory, SyncSessionDirectory, UserDirectory};
use semio_hub_directory_contract::{IdentityVerificationControl, LocalBootstrapTransport, AUTH_TEXT_MAX_BYTES};
use semio_hub_inference_runtime_contract::{GisMapApprovalCheckpointPublisherV1, GisMapApprovalIngressAuthorityV1};
use semio_hub_lag_rebootstrap::{RebootstrapContext, RebootstrapTransferControl, REBOOTSTRAP_DEADLINE_MS};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::broadcast;
use semio_hub_runtime::startup::DocumentOpenCatalogAuthorityV1;
use semio_hub_runtime::support::now_ms;
use semio_hub_runtime::presence::{document_scope_key_v1, db_artifact_id, db_core_document_id, PresenceLeaseSlot, PresenceLeaseTransition, PRESENCE_LEASE_TTL_MS};
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::state::{DocumentPlanRefusalV1, record_document_plan_refusal};
use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SESSION_PROTOCOL_V1, SocketAudienceV1, SocketSubjectV1, SocketBindingValidityV1, SocketGrantRecordV1, SocketGrantAdmissionV1, SocketLiveLeaseV1};
use crate::socket_grants::socket_live_authority;
use semio_hub_ledgers::guard::AuthOutcome;
use semio_hub_ledgers::access_grants::{socket_text_bounded, document_open_checkpoint};
use semio_hub_session_routes::access_grants::{socket_binding_validity, authenticate_document_credential};
use crate::check_in::observe_checkpoint_policy;
use semio_hub_session_routes::directory_commands::connection_view;
use semio_hub_session_routes::session_agent::agent_delegation_for_session;
use semio_hub_runtime::lifecycle::hub_shutdown_close_frame;
use db::storage::PayloadStorage as _;
use protocol::ArtifactId as ProtocolArtifactId;
use semio_hub_route_models::document_socket::{AGENT_COMMAND_PATIENCE_MS, AdmittedCommandsV1, ClientFrameStepV1, DOCUMENT_SOCKET_FRAME_DEADLINE, DocumentPlanBootstrapV1, DocumentWsV1Query, HUB_CATCH_UP_ORIGIN, SocketRebootstrapControl, admit_writes, best_effort_frontier, close_socket, credential_from_protocol_header, document_socket_gate, encode, error_frame, messages_for_error, plan_seeded_hello_frontier, socket_grant_from_protocol_header, submit_commands, transient_apply_refusal_messages, undeclared_batch_refusal, wire_frontier_to_db, wire_rebootstrap};

//#region 🔖️WebSocket

/// 🧭️ The authority a document-open plan's socket holds for its whole life, re-proven before every
/// frame it admits or sends: the sealed plan, its subject binding, the document's descriptor, the
/// catalog's selection for that descriptor and surface, and a directory revision the directory has
/// reached. The pinned revision is a witness of issue order, not a freeze on the whole directory:
/// demanding equality made every outstanding plan die on the next directory append anywhere on
/// the hub — another space's invite, a rename, an unrelated membership edit — so a member's live
/// socket was closed 4401 by an event that had nothing to do with it. What a plan may never do is
/// claim a revision the directory has not reached: that pin is forged and stays refused. Whether
/// this caller still holds this scope is decided by membership and session revocation on their own
/// paths, which is what closes a removed member's socket in the very same exchange. The plan's
/// bootstrap facts are no authority: [`document_plan_bootstrap_current`] checks them once, at admission.
pub async fn document_plan_socket_validity(state: &HubState, record: &SocketGrantRecordV1, surface: Option<&str>) -> SocketBindingValidityV1 {
    let Some(authority) = record.document_plan.as_deref() else { return SocketBindingValidityV1::Active };
    if !state.readiness.features.open_plan || !state.readiness.features.open_plan_exchange {
        return SocketBindingValidityV1::Unavailable;
    }
    if authority.validate().is_err() {
        #[cfg(test)]
        return record_document_plan_refusal(DocumentPlanRefusalV1::PlanInvalid);
        #[cfg(not(test))]
        return SocketBindingValidityV1::Unauthorized;
    }
    if record.subject != authority.subject || record.actor_id != authority.server_actor_id || record.audience != SocketAudienceV1::Document(authority.scope.clone()) || surface.is_some_and(|surface| surface != authority.surface.surface_id) {
        #[cfg(test)]
        return record_document_plan_refusal(DocumentPlanRefusalV1::BindingMismatch);
        #[cfg(not(test))]
        return SocketBindingValidityV1::Unauthorized;
    }
    let descriptor = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_document_descriptor(&authority.scope)).await {
        Ok(Ok(Some(descriptor))) => descriptor,
        Ok(Ok(None)) => {
            #[cfg(test)]
            return record_document_plan_refusal(DocumentPlanRefusalV1::DescriptorMissing);
            #[cfg(not(test))]
            return SocketBindingValidityV1::Unauthorized;
        }
        Ok(Err(_)) | Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    if descriptor != authority.descriptor {
        #[cfg(test)]
        return record_document_plan_refusal(DocumentPlanRefusalV1::DescriptorDiffers);
        #[cfg(not(test))]
        return SocketBindingValidityV1::Unauthorized;
    }
    let Some(catalog) = state.openable_catalog.as_ref() else { return SocketBindingValidityV1::Unavailable };
    if catalog.resolve_document_open(&descriptor, Some(&authority.surface.surface_id), authority.grant.write).is_none_or(|selection| {
        selection.package != authority.package
            || selection.artifact != authority.artifact
            || selection.parent_dialect != authority.parent_dialect
            || selection.surface != authority.surface
            || selection.browser_actor != authority.browser_actor
            || selection.grant != authority.grant
    }) {
        #[cfg(test)]
        return record_document_plan_refusal(DocumentPlanRefusalV1::CatalogSelectionDiffers);
        #[cfg(not(test))]
        return SocketBindingValidityV1::Unauthorized;
    }
    let directory_revision = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.head_seq()).await {
        Ok(Ok(revision)) => revision,
        Ok(Err(_)) | Err(_) => return SocketBindingValidityV1::Unavailable,
    };
    if authority.revalidation.directory_revision > directory_revision || authority.revalidation.membership_generation > directory_revision {
        #[cfg(test)]
        return record_document_plan_refusal(DocumentPlanRefusalV1::DirectoryRevisionDiffers);
        #[cfg(not(test))]
        return SocketBindingValidityV1::Unauthorized;
    }
    SocketBindingValidityV1::Active
}

/// 🌱️ Whether a document-open plan still names the catalog generation it was resolved in and the
/// active checkpoint its client seeds from. Checked once, when its socket grant is consumed, because
/// the client bootstraps from exactly that checkpoint pair. A Check In — a member's or the declared
/// checkpoint policy's — and a catalog publication advance them without changing anyone's authority,
/// so a live socket never re-checks them (ticket 26/09/23 session 14c: every Check In closed every
/// other open socket of its document `4401` within one authorization tick), and a plan they outdated
/// since its exchange is answered stale, never as a revocation.
pub async fn document_plan_bootstrap_current(state: &HubState, record: &SocketGrantRecordV1) -> DocumentPlanBootstrapV1 {
    let Some(authority) = record.document_plan.as_deref() else { return DocumentPlanBootstrapV1::Current };
    let Some(catalog) = state.openable_catalog.as_ref() else { return DocumentPlanBootstrapV1::Unavailable };
    if catalog.generation_id() != authority.catalog.generation_id {
        return DocumentPlanBootstrapV1::Stale;
    }
    let checkpoint = match tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_active_artifact_checkpoint(&authority.scope)).await {
        Ok(Ok(checkpoint)) => checkpoint.map(document_open_checkpoint),
        Ok(Err(_)) | Err(_) => return DocumentPlanBootstrapV1::Unavailable,
    };
    if checkpoint.as_ref() == Some(&authority.checkpoint) {
        DocumentPlanBootstrapV1::Current
    } else {
        DocumentPlanBootstrapV1::Stale
    }
}

pub(crate) async fn consume_scoped_directory_socket_grant(state: &HubState, headers: &HeaderMap, scope: DocumentScope) -> Result<SocketGrantAdmissionV1, StatusCode> {
    let capability = socket_grant_from_protocol_header(headers)?;
    let candidate = state.socket_grants.pending(&capability, &SocketAudienceV1::DirectoryScoped(scope), now_ms()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let _binding_gates = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_record(&candidate.subject, &candidate.audience)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = socket_binding_validity(state, &candidate.subject, &candidate.audience).await;
    match validity {
        SocketBindingValidityV1::Active => state.socket_grants.consume(&candidate, now_ms()).map(|record| SocketGrantAdmissionV1 { record }).map_err(|_| StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unauthorized => Err(StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unavailable => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// 🧭️ Admits a document upgrade whose credential resolved to `subject`: the oldest pending
/// document grant of that binding and audience, revalidated against its sealed plan, consumed once.
/// A plan whose bootstrap a Check In or a catalog publication outdated since its exchange is
/// answered `409` and its grant rejected: nobody's access changed, the client re-plans.
pub async fn consume_document_socket_grant(state: &HubState, subject: &SocketSubjectV1, audience: SocketAudienceV1, surface: Option<&str>) -> Result<SocketGrantAdmissionV1, (StatusCode, &'static str)> {
    let candidate = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).map_err(|_| (StatusCode::UNAUTHORIZED, "socket-grant-pending"))?;
    match document_plan_socket_validity(state, &candidate, surface).await {
        SocketBindingValidityV1::Active => match document_plan_bootstrap_current(state, &candidate).await {
            DocumentPlanBootstrapV1::Current => state.socket_grants.consume(&candidate, now_ms()).map(|record| SocketGrantAdmissionV1 { record }).map_err(|_| (StatusCode::UNAUTHORIZED, "socket-grant-consume")),
            DocumentPlanBootstrapV1::Stale => {
                state.socket_grants.reject_pending(&candidate.selector);
                Err((StatusCode::CONFLICT, "socket-grant-stale"))
            }
            DocumentPlanBootstrapV1::Unavailable => Err((StatusCode::SERVICE_UNAVAILABLE, "socket-grant-503")),
        },
        SocketBindingValidityV1::Unauthorized => {
            state.socket_grants.reject_pending(&candidate.selector);
            Err((StatusCode::UNAUTHORIZED, "socket-grant-401"))
        }
        SocketBindingValidityV1::Unavailable => Err((StatusCode::SERVICE_UNAVAILABLE, "socket-grant-503")),
    }
}

pub(crate) async fn consume_directory_socket_grant(state: &HubState, headers: &HeaderMap) -> Result<SocketGrantAdmissionV1, StatusCode> {
    let capability = socket_grant_from_protocol_header(headers)?;
    let candidate = state.socket_grants.pending_directory(&capability, now_ms()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let _binding_gates = tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_record(&candidate.subject, &candidate.audience)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let validity = socket_binding_validity(state, &candidate.subject, &candidate.audience).await;
    match validity {
        SocketBindingValidityV1::Active => state.socket_grants.consume(&candidate, now_ms()).map(|record| SocketGrantAdmissionV1 { record }).map_err(|_| StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unauthorized => Err(StatusCode::UNAUTHORIZED),
        SocketBindingValidityV1::Unavailable => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// 📝️ The one document socket, `/scopes/{space}%2F{document}/document/ws?surface=`. The
/// credential — a session or a share token — rides `Sec-WebSocket-Protocol: semio.session.v1, <token>`;
/// it resolves to its subject exactly as the open-plan issuer did, and the upgrade consumes the pending
/// plan grant issued to that binding, whose actor the socket is then bound to. Nothing about identity
/// is read from the URL: an unknown query field is refused.
pub async fn document_ws_v1(ws: WebSocketUpgrade, Path(scope_path): Path<String>, Query(query): Query<DocumentWsV1Query>, headers: HeaderMap, State(state): State<HubState>) -> Response {
    let Some((space_id, document_id)) = scope_path.split_once('/').map(|(space_id, document_id)| (space_id.to_string(), document_id.to_string())).filter(|(space_id, document_id)| !space_id.is_empty() && !document_id.is_empty() && !space_id.contains('/')) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let span = state.span("server.document.socket").space(space_id.clone()).artifact(document_id.clone());
    let credential = match credential_from_protocol_header(&headers) {
        Ok(credential) => credential,
        Err(status) => {
            span.refused("credential-protocol");
            return status.into_response();
        }
    };
    let surface = query.surface.unwrap_or_default();
    if !socket_text_bounded(&space_id) || !socket_text_bounded(&document_id) || surface.len() > AUTH_TEXT_MAX_BYTES {
        span.refused("bounds");
        return StatusCode::BAD_REQUEST.into_response();
    }
    let scope = DocumentScope::new(&space_id, &document_id);
    let (subject, _) = match authenticate_document_credential(&state, &scope, &credential).await {
        Ok(resolved) => resolved,
        Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded) => {
            span.refused("credential-unavailable");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
        Err(_) => {
            span.refused("credential");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
    let admission = match consume_document_socket_grant(&state, &subject, SocketAudienceV1::Document(scope), Some(&surface)).await {
        Ok(admission) => admission,
        Err((status, refusal)) => {
            span.refused(refusal);
            return status.into_response();
        }
    };
    let span = span.principal(admission.record.subject.trace_principal());
    let session_span = span.fork();
    span.finish(TraceOutcome::Ok, Some("upgrade".to_string()));
    ws.protocols([SESSION_PROTOCOL_V1])
        .on_upgrade(move |socket| async move {
            handle_ws(socket, space_id, document_id, surface, state, admission).await;
            session_span.cancelled("closed");
        })
        .into_response()
}

/// ⏱️ The frame deadline this socket applies: [`DOCUMENT_SOCKET_FRAME_DEADLINE`], or a law's shorter one.
fn document_socket_frame_deadline(state: &HubState) -> std::time::Duration {
    #[cfg(test)]
    if let Some(deadline) = state.live_gate.as_ref().and_then(|gate| *gate.socket_frame_deadline.lock().unwrap_or_else(std::sync::PoisonError::into_inner)) {
        return deadline;
    }
    let _ = state;
    DOCUMENT_SOCKET_FRAME_DEADLINE
}

/// 📨️ Handles one decoded `ClientFrame` for an already-authenticated v1 socket session within the frame
/// deadline: every frame but an admitted `Commands` batch completes here; an admitted batch leaves holding its
/// document's write gate as [`ClientFrameStepV1::Commit`] for [`commit_admitted_commands`].
///
/// 🪙️ Command-lane credit-based flow control: no server-side congestion control implemented
/// this wave (matches `framework/sync`'s client, which also accepts and ignores this frame).
#[allow(clippy::too_many_arguments)]
async fn handle_client_frame(
    state: &HubState,
    handle: &db::ArtifactHandle,
    db_id: &ProtocolArtifactId,
    key: &str,
    space_id: &str,
    document_id: &str,
    fanout: &broadcast::Sender<ServerFrame>,
    actor: &ActorId,
    socket_live_id: &str,
    gate: &db::security::SecurityGate,
    principal: &db::security::Principal,
    tenant: &db::security::TenantId,
    frame: ClientFrame,
    sender: &mut SplitSink<WebSocket, Message>,
) -> ClientFrameStepV1 {
    match frame {
        ClientFrame::Commands { batch_id, mut envelopes } => {
            if envelopes.iter().any(|envelope| &envelope.actor != actor) {
                let frontier = best_effort_frontier(handle).await;
                let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "socket subject actor mismatch".into(), messages: Vec::new() }) }], frontier };
                return ClientFrameStepV1::after_send(sender.send(encode(&ack, document_id).await).await.is_ok());
            }
            if envelopes.iter().any(|envelope| envelope.document_id.0 != document_id) {
                let frontier = best_effort_frontier(handle).await;
                let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "envelope document does not match this socket".into(), messages: Vec::new() }) }], frontier };
                return ClientFrameStepV1::after_send(sender.send(encode(&ack, document_id).await).await.is_ok());
            }
            if let Some(outcome) = undeclared_batch_refusal(&envelopes) {
                let frontier = best_effort_frontier(handle).await;
                let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(outcome) }], frontier };
                return ClientFrameStepV1::after_send(sender.send(encode(&ack, document_id).await).await.is_ok());
            }
            for envelope in &mut envelopes {
                envelope.document_id = db_id.clone();
            }
            if let Err(error) = admit_writes(gate, principal, tenant, db_id, &envelopes, now_ms().max(0) as u64).await {
                let frontier = best_effort_frontier(handle).await;
                let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: error.to_string(), messages: messages_for_error(&error) }) }], frontier };
                return ClientFrameStepV1::after_send(sender.send(encode(&ack, document_id).await).await.is_ok());
            }
            let document_write = state.socket_binding_gates.document_write(&DocumentScope::new(space_id, document_id)).lock_owned().await;
            ClientFrameStepV1::Commit(AdmittedCommandsV1 { batch_id, envelopes, _document_write: document_write })
        }
        ClientFrame::FrontierAdvertise { mut frontier } => {
            if !wire_frontier_to_db(&mut frontier, document_id, db_id) {
                return ClientFrameStepV1::after_send(sender.send(error_frame("frontier-document-mismatch", "advertised frontier names a different document than this socket").await).await.is_ok());
            }
            let core_document = db_core_document_id(db_id);
            match db::sync::handle_frontier_advertise(&state.db.storage().await.wal().await, core_document, &frontier, ActorId(HUB_CATCH_UP_ORIGIN.into())).await {
                Ok(Some(catch_up)) => ClientFrameStepV1::after_send(sender.send(encode(&catch_up, document_id).await).await.is_ok()),
                Ok(None) | Err(_) => ClientFrameStepV1::Continue,
            }
        }
        ClientFrame::PreviewPublish { key: preview_key, seq, payload } => {
            let _ = fanout.send(ServerFrame::Preview { actor: actor.clone(), key: preview_key, seq, payload });
            ClientFrameStepV1::Continue
        }
        ClientFrame::Presence { peer } => {
            let _ = state.refresh_document_presence(key, space_id, document_id, &actor.0, socket_live_id, peer, state.presence_now()).await;
            ClientFrameStepV1::Continue
        }
        ClientFrame::CreditGrant { .. } => ClientFrameStepV1::Continue,
        ClientFrame::Bye => ClientFrameStepV1::End,
        ClientFrame::SocketHelloV1 { .. } => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            ClientFrameStepV1::End
        }
    }
}

/// 🧾️ Commits one admitted batch and answers it: the engine's receipt (or refusal) always reaches the socket as
/// the batch's `Ack` — never cut by the frame deadline, the engine's own bounds end the wait — and an advanced
/// frontier's relay reaches every peer; a committed batch is tallied for the checkpoint policy. Answers the socket's
/// next step: `End` when the Ack could not be sent, `Continue` otherwise — a refusal included.
#[allow(clippy::too_many_arguments)]
async fn commit_admitted_commands(state: &HubState, handle: &db::ArtifactHandle, scope: &DocumentScope, subject: &SocketSubjectV1, fanout: &broadcast::Sender<ServerFrame>, actor: &ActorId, gate: &db::security::SecurityGate, admitted: AdmittedCommandsV1, sender: &mut SplitSink<WebSocket, Message>) -> ClientFrameStepV1 {
    let document_id = scope.document_id.as_str();
    let AdmittedCommandsV1 { batch_id, envelopes, _document_write } = admitted;
    let (ack, relay) = submit_commands(handle, gate, actor, batch_id, envelopes, state.merge_policy).await;
    if let Some(ServerFrame::Commands { envelopes, .. }) = relay.as_ref() {
        observe_checkpoint_policy(state, scope, subject, handle, envelopes);
    }
    #[cfg(test)]
    if let Some(live_gate) = state.live_gate.as_ref().filter(|gate| gate.socket_commit_pause_enabled.load(std::sync::atomic::Ordering::Acquire)) {
        live_gate.socket_commit_admitted.add_permits(1);
        live_gate.socket_commit_release.acquire().await.expect("socket commit test release").forget();
    }
    drop(_document_write);
    if let Some(commands_frame) = relay {
        let _ = fanout.send(commands_frame);
    }
    ClientFrameStepV1::after_send(sender.send(encode(&ack, document_id).await).await.is_ok())
}

/// 🤖️ ticket 26/09/18 slice M6b — M6 §4's remaining step. The actor id stays the opaque,
/// per-session `hub.v1.<sha256>` the hub minted, which is what keeps per-actor undo separating an
/// agent's edits from the delegating human's; only the NAME a collaborator reads changes, from
/// that human's display name to the agent's own delegation label.
///
/// 🎨️ Contract §C7.3: acquired after successful SocketHelloV1 admission and before `Welcome`, released at
/// handler exit (every early-return path below releases it explicitly; the loop-exit cleanup
/// releases it on a clean disconnect).
///
/// 🔒️ Per-connection `SecurityGate` compiled from the hub's one declared access policy: this
/// connection's roles read and write exactly what `document.read`/`document.write` permit in
/// this space's kind (an archive denies every write). A space whose kind cannot be read compiles
/// to no grant at all. `TenantId` reuses the space id: every scope this gate ever evaluates
/// belongs to exactly this one space/document connection.
///
/// 🧭️ The client advertises its frontier by the document id it opened; the db layer compares it
/// against this hub's internal key, so it is re-keyed here — and a frontier that names any other
/// document is refused outright rather than silently overwritten with the one we wanted.
///
/// 🎨️ Contract §C7.3: sent exactly once per connection, after `Welcome` (and its follow-up
/// bootstrap frames) and before any `Presence` frame.
///
/// 🔁️ Join replay: a roster delta is only ever published when SOME peer's bytes change, so a
/// socket that attaches after the roster settled would stay blind to every peer already on it
/// until one of them moved. Measured (ticket 26/09/18 slice PR1,
/// `🗑️generated/pr1-before-hub-socket.txt` run A): two seconds after the late joiner's socket
/// opened it had received ZERO presence frames while the first human's roster already listed a
/// peer — C3 §3.4's asymmetry, from the hub's own side. The snapshot is taken under the same
/// publication gate every delta holds, so this replay can never be older than the first delta
/// this socket receives, and it is the ONE non-delta roster frame the wire carries.
///
/// 🦵️ Admin kick: only a session the directory actually recorded gets a live `Notify` registered
/// under its `syncSessionId` (see `session_kicks`' own doc) — a session that failed to record
/// (e.g. directory hiccup) falls back to a `Notify` nobody can ever reach, i.e. un-kickable, which
/// matches this crate's generally forgiving stance on directory-write failures elsewhere in this
/// handler.
/// 🔌️ One document socket, whatever ends it: the session, then a close frame (a no-op when the session already
/// sent its own typed close) and the closing handshake, so no exit — a refused hello, a storage fault after an error
/// frame, a revocation — drops the connection under the client's unread frames.
async fn handle_ws(socket: WebSocket, space_id: String, document_id: String, surface: String, state: HubState, socket_admission: SocketGrantAdmissionV1) {
    let (mut sender, mut receiver) = socket.split();
    serve_document_socket(&mut sender, &mut receiver, space_id, document_id, surface, state, socket_admission).await;
    close_socket(&mut sender, &mut receiver, Message::Close(Some(CloseFrame { code: 1000, reason: "session-ended".into() }))).await;
}

async fn serve_document_socket(sender: &mut SplitSink<WebSocket, Message>, receiver: &mut SplitStream<WebSocket>, space_id: String, document_id: String, surface: String, state: HubState, socket_admission: SocketGrantAdmissionV1) {
    let Some(mut drain) = state.socket_drain.admit() else {
        let _ = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(hub_shutdown_close_frame())).await;
        return;
    };
    let socket_grant = socket_admission.record;

    let hello = match tokio::time::timeout(std::time::Duration::from_secs(2), receiver.next()).await {
        Ok(Some(Ok(Message::Binary(bytes)))) => decode_client_frame(&bytes).await.ok().map(|(_lane, frame)| frame),
        _ => None,
    };
    let (schema, pack_schema_hash, actor, frontier, auth) = match hello {
        Some(ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema, pack_schema_hash, resume_token, frontier }) if socket_text_bounded(&schema) && resume_token.as_ref().is_none_or(|value| value.len() <= AUTH_TEXT_MAX_BYTES) => {
            let actor = ActorId(socket_grant.actor_id.clone());
            let auth = match &socket_grant.subject {
                SocketSubjectV1::Session { session_id, user_id, authorization_generation, role: Some(role), session_kind, .. } => {
                    AuthOutcome::Session { user_id: user_id.clone(), role: *role, session_id: session_id.clone(), authorization_generation: *authorization_generation, session_kind: *session_kind }
                }
                SocketSubjectV1::Share { .. } => AuthOutcome::ShareToken,
                SocketSubjectV1::Session { role: None, .. } => AuthOutcome::Denied,
            };
            (schema, pack_schema_hash, actor, frontier, auth)
        }
        _ => {
            let _ = sender.send(error_frame("protocol", "expected socket hello").await).await;
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            return;
        }
    };

    let agent_command_subject = match &auth {
        AuthOutcome::Session { session_id, session_kind, .. } if session_kind.is_agent() => Some(RateLimitSubjectV1::principal(session_id)),
        _ => None,
    };
    let (user_id, role, auth_session_id, authorization_generation, principal_kind) = match &auth {
        AuthOutcome::Session { user_id, role, session_id, authorization_generation, session_kind } => (
            Some(user_id.clone()),
            Some(*role),
            Some(session_id.as_str()),
            *authorization_generation,
            if session_kind.is_agent() { protocol::PresencePrincipalKind::Agent } else { protocol::PresencePrincipalKind::Human },
        ),
        AuthOutcome::ShareToken => (None, None, None, 0, protocol::PresencePrincipalKind::Human),
        AuthOutcome::Denied => {
            let _ = sender.send(error_frame("unauthorized", "unauthorized").await).await;
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            return;
        }
    };
    let connected_at_ms = now_ms();
    let authenticated_user = match user_id.as_deref() {
        Some(user_id) => tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_user(user_id)).await.ok().and_then(Result::ok).flatten(),
        None => None,
    };
    let agent_delegation = match (&socket_grant.subject, principal_kind) {
        (SocketSubjectV1::Session { device_instance_id, user_id, .. }, protocol::PresencePrincipalKind::Agent) => agent_delegation_for_session(&state, &space_id, user_id, device_instance_id).await,
        _ => None,
    };
    let agent_label = agent_delegation.as_ref().map(|row| row.agent_label.clone());
    let connection_label = match &agent_delegation {
        Some(row) => semio_hub_auth::agent::agent_actor_display(&row.agent_label),
        None => actor.0.clone(),
    };
    let scope = DocumentScope::new(&space_id, &document_id);
    let descriptor = match state.directory.get_document_descriptor(&scope).await {
        Ok(Some(descriptor)) => descriptor,
        Ok(None) => {
            let _ = sender.send(error_frame("document-not-announced", "document has no durable descriptor").await).await;
            return;
        }
        Err(error) => {
            let _ = sender.send(error_frame("directory", error.to_string()).await).await;
            return;
        }
    };
    let announced_hash = pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    if schema != descriptor.artifact_schema || announced_hash != descriptor.pack_schema_hash {
        let _ = sender.send(error_frame("schema-hash-mismatch", "hello codec identity does not match the durable document descriptor").await).await;
        return;
    }
    let key = document_scope_key_v1(&scope);
    let color = state.acquire_color(&space_id, &actor.0);

    let space_kind = state.directory.get_space(&space_id).await.ok().flatten().map(|space| space.kind);
    let access_roles = auth.access_roles();
    let granted: Vec<db::security::Action> = [(DirectoryAccessActionV1::DocumentRead, db::security::Action::Read), (DirectoryAccessActionV1::DocumentWrite, db::security::Action::Write)]
        .into_iter()
        .filter(|(action, _)| space_kind.as_deref().is_some_and(|kind| directory_access_permits(&access_roles, *action, Some(kind))))
        .map(|(_, granted)| granted)
        .collect();
    let tenant = db::security::TenantId::from(space_id.clone());
    let role_str = match &auth {
        AuthOutcome::Session { role, .. } => role.as_str().to_string(),
        AuthOutcome::ShareToken => "share".to_string(),
        AuthOutcome::Denied => unreachable!("Denied already returned above"),
    };
    let policy = db::security::RoleBasedPolicy::new().with_grant(db::security::Grant::allow(role_str.clone(), &["db", "document", "*", "**"], &granted));
    let gate = document_socket_gate(policy);
    let principal = db::security::Principal::new(actor.clone(), tenant.clone(), vec![role_str]);

    let db_id = db_artifact_id(&scope);
    let handle = match state.ensure_document(&db_id).await {
        Ok(handle) => handle,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string()).await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };

    let mut frontier = frontier;
    if let Some(advertised) = frontier.as_mut() {
        if !wire_frontier_to_db(advertised, &document_id, &db_id) {
            let _ = sender.send(error_frame("frontier-document-mismatch", "hello frontier names a different document than this socket").await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    }
    let frontier = frontier.or_else(|| plan_seeded_hello_frontier(socket_grant.document_plan.as_deref(), &db_id));

    let session_id = directory::os_identity::time_ordered_id();
    let mut hello_session = match state.db.hello(db_id.clone(), frontier, session_id, ActorId(HUB_CATCH_UP_ORIGIN.into()), 64 * 1024).await {
        Ok(session) => session,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string()).await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    let welcome = match hello_session.take_welcome() {
        Ok(welcome) => welcome,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string()).await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    let welcome_bytes = match welcome.frame() {
        Ok(frame) => encode(frame, &document_id).await,
        Err(error) => {
            let _ = sender.send(error_frame("storage", error.to_string()).await).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    let socket_binding_gates = match tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.share_record(&socket_grant.subject, &socket_grant.audience)).await {
        Ok(admission) => admission,
        Err(_) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    let socket_live = {
        let (id, notify) = match state.socket_grants.register_live(&socket_grant) {
            Ok(live) => live,
            Err(_) => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
        };
        let lease = SocketLiveLeaseV1 { ledger: state.socket_grants.clone(), record: socket_grant.clone(), id, notify };
        let validity = tokio::time::timeout(std::time::Duration::from_secs(2), socket_grant.subject.revalidate(state.directory.as_ref(), &socket_grant.audience, now_ms())).await.unwrap_or(SocketBindingValidityV1::Unavailable);
        let validity = if validity == SocketBindingValidityV1::Active { document_plan_socket_validity(&state, &socket_grant, Some(&surface)).await } else { validity };
        match validity {
            SocketBindingValidityV1::Active => lease,
            SocketBindingValidityV1::Unauthorized => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
            SocketBindingValidityV1::Unavailable => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
        }
    };
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_before_welcome.add_permits(1);
        gate.socket_welcome_release.acquire().await.expect("socket welcome test release").forget();
    }
    let welcome_sent = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(welcome_bytes)).await;
    let welcome_acknowledged = welcome.acknowledge();
    if !matches!(welcome_sent, Ok(Ok(()))) || welcome_acknowledged.is_err() {
        hello_session.cancel();
        state.release_color(&space_id, &actor.0);
        return;
    }
    drop(socket_binding_gates);
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_after_welcome.add_permits(1);
        gate.socket_bootstrap_release.acquire().await.expect("socket bootstrap test release").forget();
    }
    loop {
        match hello_session.next_frame().await {
            Ok(Some(frame)) => {
                let frame_bytes = match frame.frame() {
                    Ok(owner) => encode(owner, &document_id).await,
                    Err(error) => {
                        let _ = sender.send(error_frame("storage", error.to_string()).await).await;
                        hello_session.cancel();
                        state.release_color(&space_id, &actor.0);
                        return;
                    }
                };
                let _authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                    Ok(admission) => admission,
                    Err(SocketBindingValidityV1::Unauthorized) => {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                        hello_session.cancel();
                        state.release_color(&space_id, &actor.0);
                        return;
                    }
                    Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                        hello_session.cancel();
                        state.release_color(&space_id, &actor.0);
                        return;
                    }
                };
                let sent = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame_bytes)).await;
                let acknowledged = frame.acknowledge();
                if !matches!(sent, Ok(Ok(()))) || acknowledged.is_err() {
                    hello_session.cancel();
                    state.release_color(&space_id, &actor.0);
                    return;
                }
            }
            Ok(None) => break,
            Err(error) => {
                let _ = sender.send(error_frame("storage", error.to_string()).await).await;
                state.release_color(&space_id, &actor.0);
                return;
            }
        }
    }
    let session_frame = encode(&ServerFrame::Session { actor: actor.0.clone(), color }, &document_id).await;
    let _session_authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
        Ok(admission) => admission,
        Err(SocketBindingValidityV1::Unauthorized) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
        Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
            let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    };
    if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(session_frame)).await, Ok(Ok(()))) {
        state.release_color(&space_id, &actor.0);
        return;
    }
    let slot = PresenceLeaseSlot {
        socket_live_id: socket_live.id.clone(),
        expires_at: state.presence_now() + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS),
        user_id: user_id.clone(),
        connected_at_ms,
        label: agent_label.clone().or_else(|| authenticated_user.as_ref().map(|user| user.display_name.clone())),
        role: role.map(|role| role.as_str().to_string()),
        document_surface: socket_grant.document_plan.as_ref().map(|plan| plan.surface.surface_id.clone()),
        color,
        principal_kind,
        peer: None,
    };
    if matches!(state.install_presence_slot(&key, &space_id, &document_id, &actor.0, slot).await, PresenceLeaseTransition::Unavailable | PresenceLeaseTransition::Rejected) {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "presence-unavailable".into() }))).await;
        state.release_color(&space_id, &actor.0);
        return;
    }
    drop(_session_authority);

    let fanout = state.fanout_for(&key);
    let Some((mut broadcast_rx, replay)) = state.subscribe_with_presence_replay(&key).await else {
        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "presence-unavailable".into() }))).await;
        let _ = state.close_presence_for_live(&key, &space_id, &document_id, &actor.0, &socket_live.id).await;
        state.release_color(&space_id, &actor.0);
        return;
    };
    if !replay.peers.is_empty() {
        let replay_frame = encode(&ServerFrame::Presence { peers: replay.peers }, &document_id).await;
        if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(replay_frame)).await, Ok(Ok(()))) {
            let _ = state.close_presence_for_live(&key, &space_id, &document_id, &actor.0, &socket_live.id).await;
            state.release_color(&space_id, &actor.0);
            return;
        }
    }
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.document_subscribed.add_permits(1);
        gate.document_release.acquire().await.expect("document subscription test release").forget();
    }

    let authenticated_email = authenticated_user.as_ref().map(|user| user.email.as_str());
    let sync_session = state.directory.record_sync_session_open(auth_session_id, authorization_generation, &actor.0, &space_id, &document_id, &surface, user_id.as_deref(), authenticated_email, role, &connection_label).await.ok();
    if let Some(session) = &sync_session {
        let view = connection_view(&state, session).await;
        state.directory_service.publish(DirectoryStreamMessage::Connection { phase: DirectoryConnectionPhase::Opened, connection: view });
    }
    let kick = match &sync_session {
        Some(session) => {
            let notify = Arc::new(tokio::sync::Notify::new());
            state.session_kicks.insert(session.id.clone(), notify.clone());
            notify
        }
        None => Arc::new(tokio::sync::Notify::new()),
    };
    let mut authorization_tick = tokio::time::interval(std::time::Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = authorization_tick.tick() => {
                match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                    Ok(_) => {
                        #[cfg(test)]
                        let gated_clock = state.presence_clock.as_ref().filter(|clock| clock.gate_ticks.load(std::sync::atomic::Ordering::Acquire));
                        #[cfg(test)]
                        if let Some(clock) = gated_clock {
                            clock.tick_admitted.add_permits(1);
                            clock.tick_release.acquire().await.expect("presence tick release").forget();
                        }
                        let _ = state.expire_presence_for_live(&key, &space_id, &document_id, &actor.0, &socket_live.id, state.presence_now()).await;
                        #[cfg(test)]
                        if let Some(clock) = gated_clock { clock.tick_evaluated.add_permits(1); }
                    }
                    Err(SocketBindingValidityV1::Unauthorized) => {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                        break;
                    }
                    Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                        let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                        break;
                    }
                }
            }
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(bytes))) => {
                        let Ok((_lane, frame)) = decode_client_frame(&bytes).await else {
                            let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                            break;
                        };
                        if let (ClientFrame::Commands { batch_id, .. }, Some(subject)) = (&frame, agent_command_subject) {
                            let paced = state.rate_limits.admit_paced(RateLimitClassV1::AgentCommand, &[subject], AGENT_COMMAND_PATIENCE_MS, |ms| tokio::time::sleep(std::time::Duration::from_millis(ms))).await;
                            if let RateLimitDecisionV1::Refused { retry_after_ms } = paced {
                                let frontier = best_effort_frontier(&handle).await;
                                let reason = format!("rate-limited: retry after {retry_after_ms} ms");
                                let messages = transient_apply_refusal_messages(&reason);
                                let ack = ServerFrame::Ack { batch_id: *batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason, messages }) }], frontier };
                                if sender.send(encode(&ack, &document_id).await).await.is_err() {
                                    break;
                                }
                                continue;
                            }
                        }
                            #[cfg(test)]
                            if let Some(live_gate) = &state.live_gate {
                                live_gate.socket_command_received.add_permits(1);
                                live_gate.socket_command_release.acquire().await.expect("socket command test release").forget();
                            }
                            let _authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                                Ok(admission) => admission,
                                Err(SocketBindingValidityV1::Unauthorized) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                                    break;
                                }
                                Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                                    break;
                                }
                            };
                            match tokio::time::timeout(
                                document_socket_frame_deadline(&state),
                                handle_client_frame(&state, &handle, &db_id, &key, &space_id, &document_id, &fanout, &actor, &socket_live.id, &gate, &principal, &tenant, frame, sender),
                            )
                            .await
                            {
                                Ok(ClientFrameStepV1::Continue) => {}
                                Ok(ClientFrameStepV1::End) => break,
                                Ok(ClientFrameStepV1::Commit(admitted)) => match commit_admitted_commands(&state, &handle, &DocumentScope::new(space_id.as_str(), document_id.as_str()), &socket_grant.subject, &fanout, &actor, &gate, admitted, sender).await {
                                    ClientFrameStepV1::Continue | ClientFrameStepV1::Commit(_) => {}
                                    ClientFrameStepV1::End => break,
                                },
                                Err(_) => {
                                    let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "frame-deadline".into() }))).await;
                                    break;
                                }
                            }
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            event = broadcast_rx.recv() => {
                match event {
                    Ok(frame) => {
                        #[cfg(test)]
                        if let Some(live_gate) = &state.live_gate {
                            if live_gate.socket_broadcast_pause_enabled.load(std::sync::atomic::Ordering::Acquire) {
                                live_gate.socket_broadcast_received.add_permits(1);
                                live_gate.socket_broadcast_release.acquire().await.expect("socket broadcast test release").forget();
                            }
                        }
                        let frame = encode(&frame, &document_id).await;
                        let _authority = match socket_live_authority(&state, &socket_grant, &socket_live.id).await {
                            Ok(admission) => admission,
                            Err(SocketBindingValidityV1::Unauthorized) => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                                break;
                            }
                            Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => {
                                let _ = sender.send(Message::Close(Some(CloseFrame { code: 1013, reason: "authorization-unavailable".into() }))).await;
                                break;
                            }
                        };
                        if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame)).await, Ok(Ok(()))) {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        #[cfg(test)]
                        if let Some(live_gate) = &state.live_gate {
                            live_gate.socket_lag_received.add_permits(1);
                            live_gate.socket_lag_release.acquire().await.expect("socket lag test release").forget();
                        }
                        rebootstrap_document_socket(sender, &state, &socket_grant, &socket_live.id, &scope).await;
                        break;
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            _ = kick.notified() => break,
            () = drain.closing() => {
                let _ = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(hub_shutdown_close_frame())).await;
                break;
            }
            _ = async {
                socket_live.notify.notified().await
            } => {
                let _ = sender.send(Message::Close(Some(CloseFrame { code: 4401, reason: "unauthorized".into() }))).await;
                break;
            }
        }
    }

    if let Some(session) = sync_session {
        let view = connection_view(&state, &session).await;
        let _ = state.directory.record_sync_session_close(&session.id).await;
        state.session_kicks.remove(&session.id);
        state.directory_service.publish(DirectoryStreamMessage::Connection { phase: DirectoryConnectionPhase::Closed, connection: view });
    }
    let _ = state.close_presence_for_live(&key, &space_id, &document_id, &actor.0, &socket_live.id).await;
    state.release_color(&space_id, &actor.0);
}

//#endregion 🔖️WebSocket

//#region 🔖️Directory

pub(crate) async fn verified_rebootstrap_control(state: &HubState, scope: &DocumentScope) -> Option<os_directory::RebootstrapRequired> {
    let control = SocketRebootstrapControl;
    let deadline = control.now_ms().saturating_add(REBOOTSTRAP_DEADLINE_MS);
    state.rebootstrap.control(scope, &RebootstrapContext::new(deadline, &control)).await.ok()
}

/// 🛟️ Makes a lagged document socket's client — it lost frames — rebuild from the canonical checkpoint pair: the
/// `RebootstrapRequired` control when the document has one, then `1013 rebootstrap-required`; a revoked or unreadable
/// authority closes as it always does.
async fn rebootstrap_document_socket(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, record: &SocketGrantRecordV1, live_id: &str, scope: &DocumentScope) {
    let close = match send_socket_document_rebootstrap(sender, state, record, live_id, scope).await {
        Ok(_) => CloseFrame { code: 1013, reason: "rebootstrap-required".into() },
        Err(SocketBindingValidityV1::Unauthorized) => CloseFrame { code: 4401, reason: "unauthorized".into() },
        Err(SocketBindingValidityV1::Unavailable | SocketBindingValidityV1::Active) => CloseFrame { code: 1013, reason: "authorization-unavailable".into() },
    };
    let _ = tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(Message::Close(Some(close)))).await;
}

/// 📨️ Sends the document's `RebootstrapRequired` control under the socket's live authority: `Ok(true)` when it was sent,
/// `Ok(false)` when the document has no canonical checkpoint, the refused validity otherwise.
async fn send_socket_document_rebootstrap(sender: &mut SplitSink<WebSocket, Message>, state: &HubState, record: &SocketGrantRecordV1, live_id: &str, scope: &DocumentScope) -> Result<bool, SocketBindingValidityV1> {
    let _admission = socket_live_authority(state, record, live_id).await?;
    #[cfg(test)]
    if let Some(gate) = &state.live_gate {
        gate.socket_rebootstrap_read.add_permits(1);
    }
    let Some(control) = tokio::time::timeout(std::time::Duration::from_secs(2), verified_rebootstrap_control(state, scope)).await.map_err(|_| SocketBindingValidityV1::Unavailable)? else {
        return Ok(false);
    };
    let frame = encode(&ServerFrame::RebootstrapRequired { control: wire_rebootstrap(&control) }, &scope.document_id).await;
    if !matches!(tokio::time::timeout(std::time::Duration::from_secs(2), sender.send(frame)).await, Ok(Ok(()))) {
        return Err(SocketBindingValidityV1::Unavailable);
    }
    Ok(true)
}

//#endregion 🔖️Directory
