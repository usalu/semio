//! 📇️ `directory_commands` — directory identity, commands, spaces, administration pages and invites.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use directory::os_directory::access_policy::directory_access_permits;
use directory::os_directory::io::text::directory_command_sha256;
use axum::body::Bytes;
use axum::extract::{OriginalUri, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use directory::os_directory;
use directory::os_directory::{ConnectionView, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryEvent, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationDocumentWindowV1, DirectorySpaceAdministrationInviteRowV1, DirectorySpaceAdministrationInviteWindowV1, DirectorySpaceAdministrationMemberRowV1, DirectorySpaceAdministrationMemberWindowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceAdministrationPublicDocumentWindowV1, DirectorySpaceAdministrationSectionV1, DirectorySpaceListEntryV1, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, DocumentView, MemberSpaceViewV1, PublicDocumentCatalogEntryV1, PublicSpaceViewV1, SpaceView, DIRECTORY_COMMAND_REQUEST_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA};
use futures::{SinkExt, StreamExt};
use semio_framework_hash::Sha256;
use semio_framework_trace::record::TraceOutcome;
use semio_hub_artifact_creation::ArtifactCreationCommitAuthorityV1;
use directory::os_directory::access_policy::{DirectoryAccessActionV1, DirectoryAccessRoleV1};
use semio_hub_directory_contract::model::{DirectoryPrincipalCeilingV1, DirectoryPrincipalV1, NewDirectoryCommandReceipt, SyncSessionRecord};
use directory::os_directory::DocumentScope;
use semio_hub_directory_service::{directory_command_result_kind, DirectoryCommandExecutionV1};
use semio_hub_directory::{AuthSessionDirectory, DocumentDirectory, InviteDirectory, PrincipalAuthority, SpaceDirectory, UserDirectory};
use semio_hub_directory_contract::{SPACE_ADMINISTRATION_PAGE_FETCH_MAX, SPACE_ADMINISTRATION_PAGE_MAX, InviteCapability, SessionCapability};
use semio_hub_inference_runtime_contract::GisMapApprovalIngressAuthorityV1;
use std::collections::BTreeSet;
use std::net::SocketAddr;
use semio_hub_runtime::support::{DirectoryJson, now_ms, bearer, directory_error_status};
use semio_hub_runtime::presence::{document_scope_key_v1, db_artifact_id};
use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SocketBindingKeyV1, SocketBindingModeV1, SocketBindingGuardV1};
use semio_hub_ledgers::guard::session_access_roles;
use crate::guard::is_admin;
#[cfg(any(test, feature = "testkit"))]
use crate::access_grants::{pause_directory_command_authority, pause_directory_command_membership_fence};
use semio_hub_route_models::admin::admin_space_summary_view;
use db::storage::PayloadStorage as _;
use semio_hub_route_models::directory_commands::{AuthedUser, CredentialOptionalRefusalV1, DIRECTORY_SPACE_ADMINISTRATION_DEADLINE_MS, DirectorySpaceAccessDecisionV1, FencedDirectoryCommandErrorV1, SpaceAdministrationWindows, directory_command_access_action, directory_command_space, directory_command_trace_kind, directory_command_trace_space, directory_space_access_decision, member_space_view, public_space_view, role_wire, space_administration_cursor_decode, space_administration_cursor_encode, space_administration_document_key, space_administration_invite_key, space_administration_invite_payload, space_administration_member_key, space_administration_request_admission, space_administration_session_binding_v1};

//#region 🔖️Directory

pub async fn resolve_bearer_user(state: &HubState, token: Option<&str>) -> Option<AuthedUser> {
    let capability = SessionCapability::parse(token?).ok()?;
    let session = state.directory.authenticate_session(&capability).await.ok().flatten()?;
    Some(AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability, session_kind: session.session_kind, device_instance_id: session.device_instance_id })
}

/// 🪪️ The caller of a route that also answers anonymously: no `Authorization` header is the anonymous
/// caller, and a presented credential must authenticate. A malformed, forged, expired or revoked
/// credential is refused `401` — never downgraded to the anonymous view — so a client whose session
/// ended learns it at once instead of silently reading as a stranger.
pub async fn resolve_optional_bearer_user(state: &HubState, headers: &HeaderMap) -> Result<Option<AuthedUser>, StatusCode> {
    if !headers.contains_key(axum::http::header::AUTHORIZATION) {
        return Ok(None);
    }
    let capability = bearer(headers).and_then(|token| SessionCapability::parse(&token).ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    let session = state.directory.authenticate_session(&capability).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(Some(AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability, session_kind: session.session_kind, device_instance_id: session.device_instance_id }))
}

/// 🪪️ An admitted command keeps the exact authenticated session, not a reusable user identity.
pub async fn revalidate_directory_caller(state: &HubState, caller: &AuthedUser) -> Result<(), StatusCode> {
    let session = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.authenticate_session(&caller.capability))
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if session.id != caller.session_id || session.user_id != caller.user_id || session.authorization_generation != caller.authorization_generation || session.expires_at != caller.expires_at || session.expires_at <= now_ms() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(())
}

/// 🔴️ `ConnectionView` for one live `SyncSessionRecord` — `presenceKnown` cross-references
/// `state.presence` (contract: "connections = `list_active_sync_sessions()` joined with the
/// in-memory presence map").
pub async fn connection_view(state: &HubState, session: &SyncSessionRecord) -> ConnectionView {
    let email = match &session.user_id {
        Some(id) => state.directory.get_user(id).await.ok().flatten().map(|user| user.email),
        None => None,
    };
    connection_view_with_email(state, session, email)
}

fn connection_view_with_email(state: &HubState, session: &SyncSessionRecord, email: Option<String>) -> ConnectionView {
    let scope = document_scope_key_v1(&DocumentScope::new(&session.space_id, &session.document_id));
    let presence_known = state.presence.with(&(scope, session.client_label.clone()), |entry| entry.is_some_and(|entry| entry.peer.is_some()));
    ConnectionView {
        sync_session_id: session.id.clone(),
        space_id: session.space_id.clone(),
        document_id: session.document_id.clone(),
        surface: session.surface.clone(),
        actor: session.client_label.clone(),
        user_id: session.user_id.clone(),
        email,
        role: session.space_role.map(role_wire).unwrap_or(DirectorySpaceRole::Spectator),
        connected_at_ms: session.connected_at,
        presence_known,
    }
}

/// 📄️ One durable directory descriptor enriched with its opened DB handle's current frontier; an unopened
/// document retains the descriptor's authoritative bootstrap frontier.
pub async fn document_view(state: &HubState, descriptor: DocumentDescriptor) -> DocumentView {
    let db_id = db_artifact_id(&DocumentScope::new(&descriptor.space_id, &descriptor.document_id));
    let frontier = match state.db.document(&db_id).await {
        Ok(handle) => handle.frontier().await.ok().map(|frontier| (frontier.head_seq, frontier.commit_seq, frontier.epoch)),
        Err(_) => None,
    }
    .unwrap_or((descriptor.bootstrap_frontier.head_seq, descriptor.bootstrap_frontier.commit_seq, descriptor.bootstrap_frontier.epoch));
    DocumentView { descriptor, head_seq: frontier.0, commit_seq: frontier.1, epoch: frontier.2 }
}

/// 🛡️ A directory command is admitted exactly when the declared access policy permits its action to
/// the actor's roles in the command's space: operator subject, space owner (a human session only — a
/// delegation never carries administration authority) and the session's own roles
/// ([`session_access_roles`] of its [`PrincipalAuthority::principal_role`]). A space-scoped command naming a
/// space that does not exist is `404`.
pub async fn authorize_directory_command(state: &HubState, principal: DirectoryPrincipalV1<'_>, admin: bool, command: &DirectoryCommand) -> Result<(), StatusCode> {
    let mut roles = if admin { vec![DirectoryAccessRoleV1::Admin] } else { Vec::new() };
    let space_kind = match directory_command_space(command) {
        None => {
            roles.extend(session_access_roles(principal.session_kind, None));
            None
        }
        Some(space_id) => {
            let space = state.directory.get_space(space_id).await.map_err(directory_error_status)?.ok_or(StatusCode::NOT_FOUND)?;
            if space.owner_user_id == principal.user_id && !principal.session_kind.is_agent() {
                roles.push(DirectoryAccessRoleV1::Owner);
            }
            roles.extend(session_access_roles(principal.session_kind, state.directory.principal_role(space_id, principal).await.map_err(directory_error_status)?));
            Some(space.kind)
        }
    };
    if directory_access_permits(&roles, directory_command_access_action(command), space_kind.as_deref()) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

/// 🔗️ Holds, before the directory writer, exactly the authority the command changes exclusively and
/// everything else it uses shared: the whole space for a visibility change, an archive or a delete;
/// the target's membership for a member upsert or removal; the caller's own principal and
/// membership, and the space for every other command, shared. A changed authority waits for every use
/// of it already admitted and fences new ones until the change has committed, while a space's
/// sockets, creations and unrelated commands never wait on each other.
pub(crate) async fn acquire_directory_command_fence(state: &HubState, caller: Vec<SocketBindingKeyV1>, command: &DirectoryCommand) -> Result<Vec<SocketBindingGuardV1>, FencedDirectoryCommandErrorV1> {
    let mut bindings: Vec<(SocketBindingKeyV1, SocketBindingModeV1)> = caller.into_iter().map(|binding| (binding, SocketBindingModeV1::Shared)).collect();
    if let Some(space_id) = directory_command_space(command) {
        let space_wide = matches!(command, DirectoryCommand::SetVisibility { .. } | DirectoryCommand::ArchiveSpace { .. } | DirectoryCommand::DeleteSpace { .. });
        bindings.push((SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() }, if space_wide { SocketBindingModeV1::Exclusive } else { SocketBindingModeV1::Shared }));
    }
    let target = match command {
        DirectoryCommand::RemoveMember { space_id, user_id } => Some((space_id.clone(), user_id.clone())),
        DirectoryCommand::UpsertMember { space_id, email, .. } => {
            let user = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.get_user_by_email(email)).await.map_err(|_| FencedDirectoryCommandErrorV1::Unavailable)?.map_err(|_| FencedDirectoryCommandErrorV1::Unavailable)?;
            user.map(|user| (space_id.clone(), user.id))
        }
        _ => None,
    };
    if let Some((space_id, user_id)) = target {
        bindings.push((SocketBindingKeyV1::Membership { user_id, space_id }, SocketBindingModeV1::Exclusive));
    }
    tokio::time::timeout(std::time::Duration::from_secs(2), state.socket_binding_gates.acquire(bindings)).await.map_err(|_| FencedDirectoryCommandErrorV1::Unavailable)
}

/// 🔒️ Durable role and space transitions retire only the authority they changed.
pub fn invalidate_directory_event_authority(state: &HubState, events: &[DirectoryEvent]) {
    let mut bindings = BTreeSet::new();
    for event in events {
        let binding = match &event.body {
            os_directory::DirectoryEventBody::MemberUpserted { space_id, user_id, .. } | os_directory::DirectoryEventBody::MemberRemoved { space_id, user_id } | os_directory::DirectoryEventBody::InviteRedeemed { space_id, user_id, .. } => {
                SocketBindingKeyV1::Membership { user_id: user_id.clone(), space_id: space_id.clone() }
            }
            os_directory::DirectoryEventBody::SpaceArchived { space_id } | os_directory::DirectoryEventBody::SpaceDeleted { space_id } => SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.clone() },
            _ => continue,
        };
        bindings.insert(binding);
    }
    for binding in bindings {
        state.socket_grants.invalidate_binding(binding.clone());
        state.document_open_plans.invalidate_binding(&binding);
    }
}

#[cfg(not(any(test, feature = "testkit")))]
pub async fn pause_directory_command_authority(_state: &HubState, _user_id: &str, _fenced: bool) {}

#[cfg(not(any(test, feature = "testkit")))]
pub async fn pause_directory_command_membership_fence(_state: &HubState) {}

#[cfg(not(test))]
pub async fn pause_admin_effect_started(_state: &HubState) {}

/// 🆔️ Reauthenticates inside the mutation fence before consulting any durable receipt.
async fn execute_directory_command_receipt_fenced(
    state: &HubState,
    user: &AuthedUser,
    headers: &HeaderMap,
    peer: SocketAddr,
    claim: NewDirectoryCommandReceipt,
    command: DirectoryCommand,
) -> Result<DirectoryCommandExecutionV1, FencedDirectoryCommandErrorV1> {
    let mut bindings = vec![SocketBindingKeyV1::User(user.user_id.clone()), SocketBindingKeyV1::Session(user.session_id.clone())];
    if let Some(space_id) = directory_command_space(&command) {
        bindings.push(SocketBindingKeyV1::Membership { user_id: user.user_id.clone(), space_id: space_id.to_owned() });
    }
    let _authority = acquire_directory_command_fence(state, bindings, &command).await?;
    revalidate_directory_caller(state, user).await.map_err(FencedDirectoryCommandErrorV1::Denied)?;
    let admin = is_admin(state, headers, Some(peer)).await;
    authorize_directory_command(state, user.principal(), admin, &command).await.map_err(FencedDirectoryCommandErrorV1::Denied)?;
    pause_directory_command_authority(state, &user.user_id, true).await;
    if matches!(&command, DirectoryCommand::RemoveMember { .. }) {
        pause_directory_command_membership_fence(state).await;
    }
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let execution = state.directory_service.execute_idempotent(actor, claim, command).await.map_err(FencedDirectoryCommandErrorV1::Directory)?;
    if let DirectoryCommandExecutionV1::Receipt(receipt) = &execution {
        if receipt.outcome == os_directory::DirectoryCommandOutcomeV1::Accepted {
            invalidate_directory_event_authority(state, &receipt.events);
        }
    }
    Ok(execution)
}

/// 📝️ The directory command path's span (`server.directory.command`). The record names the
/// command kind rather than its payload — a command body carries space names, invite addresses and
/// member ids, none of which belong in an operator's log.
pub async fn post_directory_commands(headers: HeaderMap, axum::extract::ConnectInfo(peer): axum::extract::ConnectInfo<SocketAddr>, State(state): State<HubState>, body: Bytes) -> Result<(StatusCode, DirectoryJson<DirectoryCommandReceiptV1>), StatusCode> {
    let span = state.span("server.directory.command");
    if body.len() > DIRECTORY_COMMAND_REQUEST_MAX_BYTES {
        span.refused("payload-too-large");
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    let Some(request) = std::str::from_utf8(&body).ok().and_then(|json| DirectoryCommandRequestV1::parse_canonical_json(json).ok()) else {
        span.refused("malformed-request");
        return Err(StatusCode::BAD_REQUEST);
    };
    let Some(user) = resolve_bearer_user(&state, bearer(&headers).as_deref()).await else {
        span.refused("unauthorized");
        return Err(StatusCode::UNAUTHORIZED);
    };
    let span = span.principal(format!("user:{}", user.user_id));
    let admin = is_admin(&state, &headers, Some(peer)).await;
    if let Err(status) = authorize_directory_command(&state, user.principal(), admin, &request.command).await {
        span.refused(&format!("forbidden-{}", status.as_u16()));
        return Err(status);
    }
    pause_directory_command_authority(&state, &user.user_id, false).await;
    let command_kind = directory_command_trace_kind(&request.command);
    let span = match directory_command_trace_space(&request.command) {
        Some(space_id) => span.space(space_id),
        None => span,
    };
    let claim = NewDirectoryCommandReceipt {
        actor_user_id: user.user_id.clone(),
        request_id: request.request_id.clone(),
        command_sha256: directory_command_sha256(&request.command),
        result_kind: directory_command_result_kind(&request.command),
        claimed_at: now_ms(),
    };
    let execution = match execute_directory_command_receipt_fenced(&state, &user, &headers, peer, claim, request.command).await {
        Ok(execution) => execution,
        Err(error) => {
            let status = match error {
                FencedDirectoryCommandErrorV1::Directory(error) => directory_error_status(error),
                FencedDirectoryCommandErrorV1::Denied(status) => status,
                FencedDirectoryCommandErrorV1::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            };
            span.failed(&format!("{command_kind}-{}", status.as_u16()));
            return Err(status);
        }
    };
    let receipt = match execution {
        DirectoryCommandExecutionV1::Conflict => {
            span.refused(&format!("{command_kind}-conflict"));
            return Err(StatusCode::CONFLICT);
        }
        DirectoryCommandExecutionV1::Receipt(receipt) => receipt,
    };
    if receipt.validate().is_err() {
        span.failed(&format!("{command_kind}-invalid-receipt"));
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    span.finish(TraceOutcome::Ok, Some(command_kind.to_string()));
    Ok((StatusCode::ACCEPTED, DirectoryJson(receipt)))
}

/// 🏘️ The caller's space list: every public space and every space it belongs to, ordered by id, from ONE
/// directory query ([`SpaceDirectory::list_visible_space_summaries`]) whatever the number of spaces, documents or
/// events. It used to fold the whole directory event log and then, per visible space, list its documents (mounting
/// each one for its frontier just to count them) and its sessions — 25–58 s for a member of 82 spaces on hub 7800
/// (ticket 26/09/23 WG8). Counts and `updatedAtMs` are the space administration route's own (the same summary row).
pub async fn get_directory_spaces(headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<DirectorySpaceListEntryV1>>, CredentialOptionalRefusalV1> {
    let caller = resolve_optional_bearer_user(&state, &headers).await?;
    let summaries = state.directory.list_visible_space_summaries(caller.as_ref().map(|caller| caller.user_id.as_str())).await.map_err(directory_error_status)?;
    let ceiling = match &caller {
        Some(caller) => state.directory.principal_ceiling(caller.principal()).await.map_err(directory_error_status)?,
        None => DirectoryPrincipalCeilingV1::Account,
    };
    let mut views = Vec::with_capacity(summaries.len());
    for (summary, role) in summaries {
        let space = admin_space_summary_view(summary)?;
        match directory_space_access_decision(space.visibility == DirectorySpaceVisibility::Public, ceiling.cap(&space.id, role).map(role_wire)) {
            DirectorySpaceAccessDecisionV1::Hidden => {}
            DirectorySpaceAccessDecisionV1::Public => views.push(DirectorySpaceListEntryV1::Public { space: public_space_view(space) }),
            DirectorySpaceAccessDecisionV1::Member => views.push(DirectorySpaceListEntryV1::Member { space: member_space_view(space, DirectorySpaceRole::Spectator) }),
            DirectorySpaceAccessDecisionV1::Author => views.push(DirectorySpaceListEntryV1::Author { space: member_space_view(space, DirectorySpaceRole::Author) }),
        }
    }
    Ok(DirectoryJson(views))
}

/// 🧾️ Seals one candidate page over the retained window prefixes and stamps its canonical receipt.
fn seal_space_administration_page_v1(
    state: &HubState,
    caller: Option<&AuthedUser>,
    space_id: &str,
    binding: [u8; 32],
    generation: u64,
    access: DirectorySpaceAccessDecisionV1,
    space: &SpaceView,
    windows: &SpaceAdministrationWindows,
    members: usize,
    invites: usize,
    documents: usize,
) -> Result<DirectorySpaceAdministrationPageV1, StatusCode> {
    let key = &state.space_administration_cursor_key;
    let member_rows = windows.members[..members].to_vec();
    let member_more = windows.member_storage_more || members < windows.members.len();
    let member_window = DirectorySpaceAdministrationMemberWindowV1 {
        next_cursor: match (member_more, member_rows.last()) {
            (true, Some(last)) => Some(space_administration_cursor_encode(key, caller, space_id, DirectorySpaceAdministrationSectionV1::Members, last.user_id.as_bytes())?),
            (true, None) => return Err(StatusCode::PAYLOAD_TOO_LARGE),
            (false, _) => None,
        },
        rows: member_rows,
    };
    let invite_rows = windows.invites[..invites].to_vec();
    let invite_more = windows.invite_storage_more || invites < windows.invites.len();
    let invite_window = DirectorySpaceAdministrationInviteWindowV1 {
        next_cursor: match (invite_more, invite_rows.last()) {
            (true, Some(last)) => Some(space_administration_cursor_encode(key, caller, space_id, DirectorySpaceAdministrationSectionV1::Invites, &space_administration_invite_payload(last))?),
            (true, None) => return Err(StatusCode::PAYLOAD_TOO_LARGE),
            (false, _) => None,
        },
        rows: invite_rows,
    };
    let document_more = windows.document_storage_more || documents < windows.documents.len().max(windows.public_documents.len());
    let document_cursor = if document_more {
        if documents == 0 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
        Some(space_administration_cursor_encode(key, caller, space_id, DirectorySpaceAdministrationSectionV1::Documents, &u64::try_from(windows.document_offset + documents).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.to_be_bytes())?)
    } else {
        None
    };
    let binding_hex = os_directory::io::binary::artifact_hash::hex_lower(&binding);
    let mut page = match access {
        DirectorySpaceAccessDecisionV1::Hidden => return Err(StatusCode::NOT_FOUND),
        DirectorySpaceAccessDecisionV1::Public => DirectorySpaceAdministrationPageV1::Public {
            schema: DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
            session_binding_sha256: binding_hex,
            authorization_generation: generation,
            space_id: space_id.to_owned(),
            space: PublicSpaceViewV1 {
                id: space.id.clone(),
                name: space.name.clone(),
                kind: space.kind,
                visibility: space.visibility,
                member_count: space.member_count,
                document_count: space.document_count,
                created_at_ms: space.created_at_ms,
                updated_at_ms: space.updated_at_ms,
            },
            documents: DirectorySpaceAdministrationPublicDocumentWindowV1 { rows: windows.public_documents[..documents].to_vec(), next_cursor: document_cursor },
            receipt_sha256: String::new(),
        },
        DirectorySpaceAccessDecisionV1::Member | DirectorySpaceAccessDecisionV1::Author => {
            let role = if matches!(access, DirectorySpaceAccessDecisionV1::Author) { DirectorySpaceRole::Author } else { DirectorySpaceRole::Spectator };
            let member_space = MemberSpaceViewV1 {
                id: space.id.clone(),
                name: space.name.clone(),
                kind: space.kind,
                visibility: space.visibility,
                owner_user_id: space.owner_user_id.clone(),
                role,
                member_count: space.member_count,
                document_count: space.document_count,
                active_connections: space.active_connections,
                created_at_ms: space.created_at_ms,
                updated_at_ms: space.updated_at_ms,
            };
            let document_window = DirectorySpaceAdministrationDocumentWindowV1 { rows: windows.documents[..documents].to_vec(), next_cursor: document_cursor };
            if matches!(access, DirectorySpaceAccessDecisionV1::Author) {
                DirectorySpaceAdministrationPageV1::Author {
                    schema: DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
                    session_binding_sha256: binding_hex,
                    authorization_generation: generation,
                    space_id: space_id.to_owned(),
                    space: member_space,
                    members: member_window,
                    documents: document_window,
                    invites: invite_window,
                    capabilities: DirectorySpaceAdministrationCapabilitiesV1 {
                        rename_space: true,
                        set_visibility: true,
                        delete_space: caller.is_some_and(|caller| caller.user_id == space.owner_user_id),
                        upsert_member: true,
                        remove_member: true,
                        create_invite: true,
                        revoke_invite: true,
                    },
                    receipt_sha256: String::new(),
                }
            } else {
                DirectorySpaceAdministrationPageV1::Member {
                    schema: DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
                    session_binding_sha256: binding_hex,
                    authorization_generation: generation,
                    space_id: space_id.to_owned(),
                    space: member_space,
                    members: member_window,
                    documents: document_window,
                    receipt_sha256: String::new(),
                }
            }
        }
    };
    let receipt = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(page.canonical_unsigned_json().as_bytes()));
    match &mut page {
        DirectorySpaceAdministrationPageV1::Public { receipt_sha256, .. } | DirectorySpaceAdministrationPageV1::Member { receipt_sha256, .. } | DirectorySpaceAdministrationPageV1::Author { receipt_sha256, .. } => *receipt_sha256 = receipt,
    }
    Ok(page)
}

/// 🏛️ One bounded, receipt-bound administration projection of exactly one space. Every row is read
/// through a safe backend projection; the caller's role is re-read after the page reads and before
/// the response is handed out, so a revocation or downgrade answers 401/403 instead of a stale page.
pub async fn get_directory_space(Path(space_id): Path<String>, OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectorySpaceAdministrationPageV1>, CredentialOptionalRefusalV1> {
    let cursor = space_administration_request_admission(&uri)?;
    let operation = build_directory_space_administration_page_v1(&state, &space_id, cursor.as_deref(), &headers);
    match tokio::time::timeout(std::time::Duration::from_millis(DIRECTORY_SPACE_ADMINISTRATION_DEADLINE_MS), operation).await {
        Ok(result) => result.map(DirectoryJson).map_err(CredentialOptionalRefusalV1),
        Err(_) => Err(CredentialOptionalRefusalV1(StatusCode::GATEWAY_TIMEOUT)),
    }
}

async fn build_directory_space_administration_page_v1(state: &HubState, space_id: &str, cursor: Option<&str>, headers: &HeaderMap) -> Result<DirectorySpaceAdministrationPageV1, StatusCode> {
    let caller = resolve_optional_bearer_user(state, headers).await?;
    let binding = space_administration_session_binding_v1(caller.as_ref(), space_id)?;
    let generation = caller.as_ref().map_or(0, |caller| caller.authorization_generation);
    let section = match cursor {
        Some(cursor) => Some(space_administration_cursor_decode(&state.space_administration_cursor_key, caller.as_ref(), space_id, cursor)?),
        None => None,
    };
    let summary = state.directory.list_admin_space_summaries_page(Some(space_id), 0, 1).await.map_err(|error| state.directory_fault("directory.space-administration", error))?.into_iter().next().ok_or(StatusCode::NOT_FOUND)?;
    let space = admin_space_summary_view(summary)?;
    let role = match caller.as_ref() {
        Some(caller) => state.directory.principal_role(space_id, caller.principal()).await.map_err(|error| state.directory_fault("directory.space-administration", error))?.map(role_wire),
        None => None,
    };
    let access = directory_space_access_decision(space.visibility == DirectorySpaceVisibility::Public, role);
    if matches!(access, DirectorySpaceAccessDecisionV1::Hidden) {
        return Err(StatusCode::NOT_FOUND);
    }
    let member_after = match section {
        Some((DirectorySpaceAdministrationSectionV1::Members, ref payload)) => Some(space_administration_member_key(payload)?),
        _ => None,
    };
    let invite_after = match section {
        Some((DirectorySpaceAdministrationSectionV1::Invites, ref payload)) => Some(space_administration_invite_key(payload)?),
        _ => None,
    };
    let document_offset = match section {
        Some((DirectorySpaceAdministrationSectionV1::Documents, ref payload)) => space_administration_document_key(payload)?,
        _ => 0,
    };
    let mut windows = SpaceAdministrationWindows { members: Vec::new(), member_storage_more: false, invites: Vec::new(), invite_storage_more: false, documents: Vec::new(), public_documents: Vec::new(), document_storage_more: false, document_offset };
    if access.is_member() {
        let mut rows = state.directory.list_space_administration_members_page(space_id, member_after.as_deref(), SPACE_ADMINISTRATION_PAGE_FETCH_MAX).await.map_err(|error| state.directory_fault("directory.space-administration", error))?;
        windows.member_storage_more = rows.len() > SPACE_ADMINISTRATION_PAGE_MAX;
        rows.truncate(SPACE_ADMINISTRATION_PAGE_MAX);
        windows.members = rows.into_iter().map(|row| DirectorySpaceAdministrationMemberRowV1 { owner: row.user_id == space.owner_user_id, user_id: row.user_id, email: row.email, display_name: row.display_name, role: role_wire(row.role) }).collect();
    }
    if matches!(access, DirectorySpaceAccessDecisionV1::Author) {
        let mut rows =
            state.directory.list_space_administration_invites_page(space_id, invite_after.as_ref().map(|(created_at, invite_id)| (*created_at, invite_id.as_str())), SPACE_ADMINISTRATION_PAGE_FETCH_MAX).await.map_err(|error| state.directory_fault("directory.space-administration", error))?;
        windows.invite_storage_more = rows.len() > SPACE_ADMINISTRATION_PAGE_MAX;
        rows.truncate(SPACE_ADMINISTRATION_PAGE_MAX);
        windows.invites = rows
            .into_iter()
            .map(|row| DirectorySpaceAdministrationInviteRowV1 { invite_id: row.invite_id, role: role_wire(row.role), created_at_ms: row.created_at_ms, expires_at_ms: row.expires_at_ms, revoked: row.revoked, accepted: row.accepted })
            .collect();
    }
    let mut descriptors = state.directory.list_document_descriptors_page(Some(space_id), document_offset, SPACE_ADMINISTRATION_PAGE_FETCH_MAX).await.map_err(|error| state.directory_fault("directory.space-administration", error))?;
    windows.document_storage_more = descriptors.len() > SPACE_ADMINISTRATION_PAGE_MAX;
    descriptors.truncate(SPACE_ADMINISTRATION_PAGE_MAX);
    if access.is_member() {
        for descriptor in descriptors {
            windows.documents.push(document_view(state, descriptor).await);
        }
    } else {
        windows.public_documents = descriptors
            .into_iter()
            .map(|descriptor| PublicDocumentCatalogEntryV1 {
                document_id: descriptor.document_id,
                artifact_kind: descriptor.artifact_kind,
                artifact_schema: descriptor.artifact_schema,
                owner: descriptor.owner,
                pack_schema_hash: descriptor.pack_schema_hash,
            })
            .collect();
    }
    let access = revalidate_space_administration_caller(state, caller.as_ref(), space_id, binding, &space, access).await?;
    let mut members = windows.members.len();
    let mut invites = windows.invites.len();
    let mut documents = windows.documents.len().max(windows.public_documents.len());
    loop {
        let page = seal_space_administration_page_v1(state, caller.as_ref(), space_id, binding, generation, access, &space, &windows, members, invites, documents)?;
        if semio_framework_pack_json::to_json_string(&page).len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES {
            page.validate().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            return Ok(page);
        }
        if documents >= members && documents >= invites && documents > 1 {
            documents -= 1;
        } else if members >= invites && members > 1 {
            members -= 1;
        } else if invites > 1 {
            invites -= 1;
        } else {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
    }
}

/// 🛡️ Re-reads the exact session and current membership after the page reads: a revoked session
/// answers 401 and a role downgrade or removal answers 403/404 instead of a stale administration page.
async fn revalidate_space_administration_caller(state: &HubState, caller: Option<&AuthedUser>, space_id: &str, binding: [u8; 32], space: &SpaceView, observed: DirectorySpaceAccessDecisionV1) -> Result<DirectorySpaceAccessDecisionV1, StatusCode> {
    let Some(caller) = caller else { return Ok(observed) };
    let session = state.directory.authenticate_session(&caller.capability).await.map_err(|_| StatusCode::UNAUTHORIZED)?.ok_or(StatusCode::UNAUTHORIZED)?;
    let current = AuthedUser { user_id: session.user_id, session_id: session.id, expires_at: session.expires_at, authorization_generation: session.authorization_generation, capability: caller.capability.clone(), session_kind: session.session_kind, device_instance_id: session.device_instance_id };
    if current.session_id != caller.session_id || current.user_id != caller.user_id || current.authorization_generation != caller.authorization_generation || space_administration_session_binding_v1(Some(&current), space_id)? != binding {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let role = state.directory.principal_role(space_id, current.principal()).await.map_err(directory_error_status)?.map(role_wire);
    let access = directory_space_access_decision(space.visibility == DirectorySpaceVisibility::Public, role);
    match (observed, access) {
        (DirectorySpaceAccessDecisionV1::Hidden, _) | (_, DirectorySpaceAccessDecisionV1::Hidden) => Err(StatusCode::NOT_FOUND),
        (observed, current) if observed == current => Ok(current),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn post_redeem_invite(Path(token): Path<String>, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<Vec<DirectoryEvent>>, StatusCode> {
    let user = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
    let capability = InviteCapability::parse(&token).map_err(directory_error_status)?;
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hub-rest", user.user_id) };
    let hint = tokio::time::timeout(std::time::Duration::from_secs(2), state.directory.invite_redemption_scope_hint(&capability, &actor, &user.user_id)).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?.map_err(directory_error_status)?;
    pause_directory_command_authority(&state, &user.user_id, false).await;
    let _authority = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        state.socket_binding_gates.acquire(vec![
            (SocketBindingKeyV1::User(user.user_id.clone()), SocketBindingModeV1::Shared),
            (SocketBindingKeyV1::Session(user.session_id.clone()), SocketBindingModeV1::Shared),
            (SocketBindingKeyV1::DirectorySpaceAuthority { space_id: hint.space_id().to_owned() }, SocketBindingModeV1::Shared),
            (SocketBindingKeyV1::Membership { user_id: user.user_id.clone(), space_id: hint.space_id().to_owned() }, SocketBindingModeV1::Exclusive),
        ]),
    )
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    revalidate_directory_caller(&state, &user).await?;
    pause_directory_command_authority(&state, &user.user_id, true).await;
    let committed = state.directory_service.redeem_invite(actor, &capability, &user.user_id).await.map_err(directory_error_status)?;
    if matches!(committed, semio_hub_directory_contract::model::InviteRedemptionCommit::NewlyCommitted { .. }) {
        invalidate_directory_event_authority(&state, std::slice::from_ref(committed.event()));
    }
    revalidate_directory_caller(&state, &user).await?;
    Ok(DirectoryJson(vec![committed.into_event()]))
}
//#endregion 🔖️Directory
