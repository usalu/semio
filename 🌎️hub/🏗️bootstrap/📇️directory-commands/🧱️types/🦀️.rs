//! 🧱️ The `directory_commands` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use semio_hub_directory_contract::model::{AuthSessionKind, DirectoryPrincipalV1, SpaceRole};
use directory::os_directory::{DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES, DirectoryCommand, DirectorySpaceAdministrationInviteRowV1, DirectorySpaceAdministrationMemberRowV1, DirectorySpaceAdministrationSectionV1, DirectorySpaceRole, DocumentView, MemberSpaceViewV1, PublicDocumentCatalogEntryV1, PublicSpaceViewV1, SpaceView};
use directory::os_directory::access_policy::DirectoryAccessActionV1;
use semio_hub_directory_contract::error::DirectoryError;
use axum::response::{IntoResponse, Response};
use axum::Json;
use semio_hub_directory_contract::SessionCapability;
use semio_framework_hash::Sha256;
use axum::http::StatusCode;
use directory::os_directory;

//#region 🔖️Directory
/// 🙋️ A bearer token resolved to a live, unexpired `AuthSessionRecord`'s user — every
/// `/directory/*`/`/auth/sessions/me` route that needs a caller identity resolves through this
/// (distinct from `AuthOutcome`, which can also carry an exact document share; the directory
/// control plane has no such fallback — a command with no valid session is unauthenticated).
#[derive(Clone)]
pub struct AuthedUser {
    pub user_id: String,
    pub session_id: String,
    pub expires_at: i64,
    pub authorization_generation: u64,
    pub capability: SessionCapability,
    pub session_kind: AuthSessionKind,
    pub device_instance_id: String,
}

impl AuthedUser {
    /// 🎚️ Who this caller asks every space-role question as.
    pub fn principal(&self) -> DirectoryPrincipalV1<'_> {
        DirectoryPrincipalV1 { user_id: &self.user_id, session_kind: self.session_kind, device_instance_id: &self.device_instance_id }
    }
}

/// 🚪️ The refusal of a route that also answers anonymously ([`resolve_optional_bearer_user`]): its `401` means a
/// PRESENTED credential did not authenticate, so it answers the typed, localized `HubCredentialRefusalV1` body with the
/// RFC 6750 `invalid_token` challenge (audit s13 os-frontend §4.17) — the client learns its session ended instead of
/// reading the anonymous view. Every other status stays a bare, signed refusal.
pub struct CredentialOptionalRefusalV1(pub StatusCode);

impl From<StatusCode> for CredentialOptionalRefusalV1 {
    fn from(status: StatusCode) -> Self {
        Self(status)
    }
}

impl IntoResponse for CredentialOptionalRefusalV1 {
    fn into_response(self) -> Response {
        if self.0 != StatusCode::UNAUTHORIZED {
            return self.0.into_response();
        }
        (StatusCode::UNAUTHORIZED, [(axum::http::header::WWW_AUTHENTICATE, semio_hub_refusal::HUB_CREDENTIAL_REFUSAL_CHALLENGE)], Json(semio_hub_refusal::HUB_CREDENTIAL_REFUSAL)).into_response()
    }
}

pub fn role_wire(role: SpaceRole) -> DirectorySpaceRole {
    match role {
        SpaceRole::Author => DirectorySpaceRole::Author,
        SpaceRole::Spectator => DirectorySpaceRole::Spectator,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectorySpaceAccessDecisionV1 {
    Hidden,
    Public,
    Member,
    Author,
}

impl DirectorySpaceAccessDecisionV1 {
    pub fn is_member(self) -> bool {
        matches!(self, Self::Member | Self::Author)
    }
}

/// 🛡️ The single public/member/author decision used by REST reads and both stream paths.
pub fn directory_space_access_decision(public: bool, role: Option<DirectorySpaceRole>) -> DirectorySpaceAccessDecisionV1 {
    match role {
        Some(DirectorySpaceRole::Author) => DirectorySpaceAccessDecisionV1::Author,
        Some(DirectorySpaceRole::Spectator) => DirectorySpaceAccessDecisionV1::Member,
        None if public => DirectorySpaceAccessDecisionV1::Public,
        None => DirectorySpaceAccessDecisionV1::Hidden,
    }
}

pub fn public_space_view(space: SpaceView) -> PublicSpaceViewV1 {
    PublicSpaceViewV1 {
        id: space.id,
        name: space.name,
        kind: space.kind,
        visibility: space.visibility,
        member_count: space.member_count,
        document_count: space.document_count,
        created_at_ms: space.created_at_ms,
        updated_at_ms: space.updated_at_ms,
    }
}

pub fn member_space_view(space: SpaceView, role: DirectorySpaceRole) -> MemberSpaceViewV1 {
    MemberSpaceViewV1 {
        id: space.id,
        name: space.name,
        kind: space.kind,
        visibility: space.visibility,
        owner_user_id: space.owner_user_id,
        role,
        member_count: space.member_count,
        document_count: space.document_count,
        active_connections: space.active_connections,
        created_at_ms: space.created_at_ms,
        updated_at_ms: space.updated_at_ms,
    }
}

/// ⚖️ Contract §C2's command authorization matrix: `create-space` any session; `delete-space`/
/// `archive-space` owner or admin; everything else any AUTHOR of the named space or admin. `decide`
/// itself performs zero authorization (its own doc) — this is that check, run before `execute`.
/// 🎬️ The declared-policy action one directory command asks for.
pub fn directory_command_access_action(command: &DirectoryCommand) -> DirectoryAccessActionV1 {
    match command {
        DirectoryCommand::CreateSpace { .. } => DirectoryAccessActionV1::SpaceCreate,
        DirectoryCommand::RenameSpace { .. } => DirectoryAccessActionV1::SpaceRename,
        DirectoryCommand::SetVisibility { .. } => DirectoryAccessActionV1::SpaceVisibility,
        DirectoryCommand::ArchiveSpace { .. } => DirectoryAccessActionV1::SpaceArchive,
        DirectoryCommand::DeleteSpace { .. } => DirectoryAccessActionV1::SpaceDelete,
        DirectoryCommand::UpsertMember { .. } => DirectoryAccessActionV1::MemberUpsert,
        DirectoryCommand::RemoveMember { .. } => DirectoryAccessActionV1::MemberRemove,
        DirectoryCommand::CreateInvite { .. } => DirectoryAccessActionV1::InviteCreate,
        DirectoryCommand::RevokeInvite { .. } => DirectoryAccessActionV1::InviteRevoke,
        DirectoryCommand::AnnounceDocument { .. } => DirectoryAccessActionV1::DocumentAnnounce,
        DirectoryCommand::RecordUserPreference { .. } => DirectoryAccessActionV1::PreferenceRecord,
    }
}

#[derive(Debug)]
pub enum FencedDirectoryCommandErrorV1 {
    Directory(DirectoryError),
    Denied(StatusCode),
    Unavailable,
}

/// 🛡️ Command scope is derived from the closed command, never from a second client claim.
pub fn directory_command_space(command: &DirectoryCommand) -> Option<&str> {
    match command {
        DirectoryCommand::CreateSpace { .. } => None,
        DirectoryCommand::RenameSpace { space_id, .. }
        | DirectoryCommand::SetVisibility { space_id, .. }
        | DirectoryCommand::ArchiveSpace { space_id }
        | DirectoryCommand::DeleteSpace { space_id }
        | DirectoryCommand::UpsertMember { space_id, .. }
        | DirectoryCommand::RemoveMember { space_id, .. }
        | DirectoryCommand::CreateInvite { space_id, .. }
        | DirectoryCommand::RevokeInvite { space_id, .. } => Some(space_id),
        DirectoryCommand::AnnounceDocument { descriptor } => Some(&descriptor.space_id),
        DirectoryCommand::RecordUserPreference { .. } => None,
    }
}

/// 🧾️ `POST /directory/commands` — the closed request/receipt command wire. Authentication and the
/// full §C2 authorization matrix run BEFORE any stored completion is returned, so knowing a request
/// id never resurrects a result for an expired, revoked, or differently-scoped session. The one-shot
/// invite capability is returned to this live call alone; every later resolution of the same id is
/// redacted, proving no duplicate invitation was minted.
/// 🏷️ The stable label one directory command is reported under. The variant name only —
/// never a field, because every payload field of this enum is user data.
pub fn directory_command_trace_kind(command: &DirectoryCommand) -> &'static str {
    match command {
        DirectoryCommand::CreateSpace { .. } => "create-space",
        DirectoryCommand::RenameSpace { .. } => "rename-space",
        DirectoryCommand::SetVisibility { .. } => "set-visibility",
        DirectoryCommand::ArchiveSpace { .. } => "archive-space",
        DirectoryCommand::DeleteSpace { .. } => "delete-space",
        DirectoryCommand::UpsertMember { .. } => "upsert-member",
        DirectoryCommand::RemoveMember { .. } => "remove-member",
        DirectoryCommand::CreateInvite { .. } => "create-invite",
        DirectoryCommand::RevokeInvite { .. } => "revoke-invite",
        DirectoryCommand::AnnounceDocument { .. } => "announce-document",
        DirectoryCommand::RecordUserPreference { .. } => "record-user-preference",
    }
}

/// 🪐️ The space a directory command acts in, when it names one. `CreateSpace` has none yet
/// — the id is minted by the decider — and `AnnounceDocument` carries its own descriptor's.
pub fn directory_command_trace_space(command: &DirectoryCommand) -> Option<String> {
    match command {
        DirectoryCommand::CreateSpace { .. } => None,
        DirectoryCommand::RenameSpace { space_id, .. }
        | DirectoryCommand::SetVisibility { space_id, .. }
        | DirectoryCommand::ArchiveSpace { space_id }
        | DirectoryCommand::DeleteSpace { space_id }
        | DirectoryCommand::UpsertMember { space_id, .. }
        | DirectoryCommand::RemoveMember { space_id, .. }
        | DirectoryCommand::CreateInvite { space_id, .. }
        | DirectoryCommand::RevokeInvite { space_id, .. } => Some(space_id.clone()),
        DirectoryCommand::AnnounceDocument { descriptor } => Some(descriptor.space_id.clone()),
        DirectoryCommand::RecordUserPreference { .. } => None,
    }
}

pub const DIRECTORY_SPACE_ADMINISTRATION_DEADLINE_MS: u64 = 5_000;

/// 🛂️ Strict query admission: nothing, or exactly one opaque `cursor` token.
pub fn space_administration_request_admission(uri: &axum::http::Uri) -> Result<Option<String>, StatusCode> {
    let Some(query) = uri.query() else { return Ok(None) };
    if query.is_empty() {
        return Ok(None);
    }
    if query.contains('&') || query.contains('%') || query.contains('+') {
        return Err(StatusCode::BAD_REQUEST);
    }
    let (name, value) = query.split_once('=').ok_or(StatusCode::BAD_REQUEST)?;
    if name != "cursor" || value.is_empty() || value.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_')) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Some(value.to_owned()))
}

/// 🔐️ Session binding of one administration page — the client only ever receives this digest.
pub fn space_administration_session_binding_v1(caller: Option<&AuthedUser>, space_id: &str) -> Result<[u8; 32], StatusCode> {
    let Some(caller) = caller else { return Ok([0u8; 32]) };
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/directory-space-administration/session-binding/v1\0");
    hash.update(&u32::try_from(caller.session_id.len()).map_err(|_| StatusCode::UNAUTHORIZED)?.to_be_bytes());
    hash.update(caller.session_id.as_bytes());
    hash.update(&u32::try_from(caller.user_id.len()).map_err(|_| StatusCode::UNAUTHORIZED)?.to_be_bytes());
    hash.update(caller.user_id.as_bytes());
    hash.update(&caller.authorization_generation.to_be_bytes());
    hash.update(&caller.expires_at.to_be_bytes());
    hash.update(&u32::try_from(space_id.len()).map_err(|_| StatusCode::BAD_REQUEST)?.to_be_bytes());
    hash.update(space_id.as_bytes());
    Ok(hash.finalize().into())
}

fn space_administration_cursor_mac(key: &[u8; 32], caller: Option<&AuthedUser>, space_id: &str, section: DirectorySpaceAdministrationSectionV1, payload: &[u8]) -> [u8; 32] {
    let (session_id, user_id, generation) = caller.map_or(("", "", 0u64), |caller| (caller.session_id.as_str(), caller.user_id.as_str(), caller.authorization_generation));
    let mut hash = Sha256::new();
    hash.update(b"semio/hub/space-administration-cursor/v1\0");
    hash.update(key);
    hash.update(&(session_id.len() as u32).to_be_bytes());
    hash.update(session_id.as_bytes());
    hash.update(&(user_id.len() as u32).to_be_bytes());
    hash.update(user_id.as_bytes());
    hash.update(&generation.to_be_bytes());
    hash.update(&(space_id.len() as u32).to_be_bytes());
    hash.update(space_id.as_bytes());
    hash.update(section.as_str().as_bytes());
    hash.update(&(payload.len() as u32).to_be_bytes());
    hash.update(payload);
    hash.finalize().into()
}

pub fn space_administration_cursor_encode(key: &[u8; 32], caller: Option<&AuthedUser>, space_id: &str, section: DirectorySpaceAdministrationSectionV1, payload: &[u8]) -> Result<String, StatusCode> {
    let cursor = format!("{}.{}.{}", section.as_str(), os_directory::io::binary::artifact_hash::hex_lower(payload), os_directory::io::binary::artifact_hash::hex_lower(&space_administration_cursor_mac(key, caller, space_id, section, payload)));
    if cursor.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    Ok(cursor)
}

pub fn space_administration_cursor_decode(key: &[u8; 32], caller: Option<&AuthedUser>, space_id: &str, cursor: &str) -> Result<(DirectorySpaceAdministrationSectionV1, Vec<u8>), StatusCode> {
    let mut parts = cursor.split('.');
    let (Some(section), Some(payload), Some(mac), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let section = DirectorySpaceAdministrationSectionV1::parse(section).ok_or(StatusCode::BAD_REQUEST)?;
    let payload = decode_lower_hex(payload).ok_or(StatusCode::BAD_REQUEST)?;
    let mac: [u8; 32] = decode_lower_hex(mac).ok_or(StatusCode::BAD_REQUEST)?.try_into().map_err(|_| StatusCode::BAD_REQUEST)?;
    if !semio_hub_directory_contract::constant_time_digest_eq(&space_administration_cursor_mac(key, caller, space_id, section, &payload), &mac) {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok((section, payload))
}

fn decode_lower_hex(value: &str) -> Option<Vec<u8>> {
    if value.len() % 2 != 0 || !value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')) {
        return None;
    }
    (0..value.len() / 2).map(|index| u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).ok()).collect()
}

pub fn space_administration_member_key(payload: &[u8]) -> Result<String, StatusCode> {
    String::from_utf8(payload.to_vec()).map_err(|_| StatusCode::BAD_REQUEST)
}

pub fn space_administration_invite_key(payload: &[u8]) -> Result<(i64, String), StatusCode> {
    if payload.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let created_at = i64::from_be_bytes(payload[..8].try_into().map_err(|_| StatusCode::BAD_REQUEST)?);
    Ok((created_at, String::from_utf8(payload[8..].to_vec()).map_err(|_| StatusCode::BAD_REQUEST)?))
}

pub fn space_administration_document_key(payload: &[u8]) -> Result<usize, StatusCode> {
    let bytes: [u8; 8] = payload.try_into().map_err(|_| StatusCode::BAD_REQUEST)?;
    usize::try_from(u64::from_be_bytes(bytes)).map_err(|_| StatusCode::BAD_REQUEST)
}

pub fn space_administration_invite_payload(row: &DirectorySpaceAdministrationInviteRowV1) -> Vec<u8> {
    let mut payload = row.created_at_ms.to_be_bytes().to_vec();
    payload.extend_from_slice(row.invite_id.as_bytes());
    payload
}

pub struct SpaceAdministrationWindows {
    pub members: Vec<DirectorySpaceAdministrationMemberRowV1>,
    pub member_storage_more: bool,
    pub invites: Vec<DirectorySpaceAdministrationInviteRowV1>,
    pub invite_storage_more: bool,
    pub documents: Vec<DocumentView>,
    pub public_documents: Vec<PublicDocumentCatalogEntryV1>,
    pub document_storage_more: bool,
    pub document_offset: usize,
}
//#endregion 🔖️Directory
