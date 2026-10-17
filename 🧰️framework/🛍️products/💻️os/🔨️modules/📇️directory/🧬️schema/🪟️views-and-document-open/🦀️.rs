//! 🪟️ Views and document opening of the directory schema: the space, member, user, connection and document views, the administration pages and the document open intent, package, surface and plan with their validation. See [`super`].

use super::*;

//#region 🔖️Views
/// 🏠️ One space, as the hub's REST/read surface renders it. `role` is the CALLING user's
/// membership role (server-filled per request), never derived by the pure fold.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SpaceView {
    pub id: String,
    pub name: String,
    pub kind: DirectorySpaceKind,
    pub visibility: DirectorySpaceVisibility,
    pub owner_user_id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<DirectorySpaceRole>,
    pub member_count: u32,
    pub document_count: u32,
    pub active_connections: u32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// 🌐️ Discoverable space metadata. Account identity, caller role, and live activity are
/// structurally absent rather than redacted from [`SpaceView`].
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicSpaceViewV1 {
    pub id: String,
    pub name: String,
    pub kind: DirectorySpaceKind,
    pub visibility: DirectorySpaceVisibility,
    pub member_count: u32,
    pub document_count: u32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// 🔐️ Membership-qualified space metadata. Its required role makes accidental use for
/// anonymous discovery a type error.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberSpaceViewV1 {
    pub id: String,
    pub name: String,
    pub kind: DirectorySpaceKind,
    pub visibility: DirectorySpaceVisibility,
    pub owner_user_id: String,
    pub role: DirectorySpaceRole,
    pub member_count: u32,
    pub document_count: u32,
    pub active_connections: u32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// 📖️ Public document catalog identity. Replication frontier/currentness and bootstrap
/// checkpoint state remain private to the authenticated D1 open authority.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PublicDocumentCatalogEntryV1 {
    pub document_id: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub owner: DocumentOwner,
    pub pack_schema_hash: String,
}

/// 🔎️ One list entry with an explicit public/member/author authority discriminator.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "access", rename_all = "lowercase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum DirectorySpaceListEntryV1 {
    Public { space: PublicSpaceViewV1 },
    Member { space: MemberSpaceViewV1 },
    Author { space: MemberSpaceViewV1 },
}

impl DirectorySpaceListEntryV1 {
    /// 🧭️ Checks discriminator-to-role/visibility correlation after wire decoding.
    pub fn validate(&self) -> bool {
        match self {
            Self::Public { space } => space.visibility == DirectorySpaceVisibility::Public,
            Self::Member { space } => space.role == DirectorySpaceRole::Spectator,
            Self::Author { space } => space.role == DirectorySpaceRole::Author,
        }
    }
}

/// 📏️ Maximum rows one administration-page window may carry.
pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS: usize = 64;

/// 📦️ Maximum canonical response bytes of one administration page.
pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES: usize = 48 * 1024;

/// 🔑️ Maximum UTF-8 bytes of one opaque administration cursor.
pub const DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES: usize = 1024;

/// 🏷️ Canonical schema identifier of the bounded space administration page.
pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA: &str = "semio.directory.space-administration-page.v1";

/// 🗂️ The one independently paged window a cursor may advance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectorySpaceAdministrationSectionV1 {
    Members,
    Invites,
    Documents,
}

impl DirectorySpaceAdministrationSectionV1 {
    /// 🔤️ Wire spelling shared by cursor payloads and client requests.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Members => "members",
            Self::Invites => "invites",
            Self::Documents => "documents",
        }
    }

    /// 🔍️ Parses exactly the three closed section names.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "members" => Some(Self::Members),
            "invites" => Some(Self::Invites),
            "documents" => Some(Self::Documents),
            _ => None,
        }
    }
}

/// 🧑️ One administration-page member row; never carries a credential, session, or provider column.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationMemberRowV1 {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub role: DirectorySpaceRole,
    pub owner: bool,
}

/// 🎟️ One administration-page invite row; never carries the selector, secret digest, or capability.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationInviteRowV1 {
    pub invite_id: String,
    pub role: DirectorySpaceRole,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revoked: bool,
    pub accepted: bool,
}

/// 🪟️ One bounded member window; `next_cursor` is present exactly when more rows remain.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationMemberWindowV1 {
    pub rows: Vec<DirectorySpaceAdministrationMemberRowV1>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// 🪟️ One bounded invite window; author-only by construction.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationInviteWindowV1 {
    pub rows: Vec<DirectorySpaceAdministrationInviteRowV1>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// 🪟️ One bounded membership-qualified document window.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationDocumentWindowV1 {
    pub rows: Vec<DocumentView>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// 🪟️ One bounded public document-catalog window.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationPublicDocumentWindowV1 {
    pub rows: Vec<PublicDocumentCatalogEntryV1>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// 🛂️ Server-decided administration affordances; the only authority a renderer may consult.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySpaceAdministrationCapabilitiesV1 {
    pub rename_space: bool,
    pub set_visibility: bool,
    pub delete_space: bool,
    pub upsert_member: bool,
    pub remove_member: bool,
    pub create_invite: bool,
    pub revoke_invite: bool,
}

/// 🏛️ One authenticated, receipt-bound bounded administration projection of exactly one space.
/// Only the `author` shape carries invites and capability flags; `member`/`public` omit them
/// structurally rather than sending empty placeholders.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "access", rename_all = "lowercase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum DirectorySpaceAdministrationPageV1 {
    Public {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: PublicSpaceViewV1,
        documents: DirectorySpaceAdministrationPublicDocumentWindowV1,
        receipt_sha256: String,
    },
    Member {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: MemberSpaceViewV1,
        members: DirectorySpaceAdministrationMemberWindowV1,
        documents: DirectorySpaceAdministrationDocumentWindowV1,
        receipt_sha256: String,
    },
    Author {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: MemberSpaceViewV1,
        members: DirectorySpaceAdministrationMemberWindowV1,
        documents: DirectorySpaceAdministrationDocumentWindowV1,
        invites: DirectorySpaceAdministrationInviteWindowV1,
        capabilities: DirectorySpaceAdministrationCapabilitiesV1,
        receipt_sha256: String,
    },
}

/// 🚫️ Stable bounded administration-page denial classes shared by hub and clients.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectorySpaceAdministrationPageErrorV1 {
    Invalid,
    TooLarge,
    ReceiptMismatch,
}

pub(crate) fn directory_space_administration_cursor_valid(cursor: &Option<String>) -> bool {
    match cursor {
        None => true,
        Some(cursor) => !cursor.is_empty() && cursor.len() <= DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES && cursor.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')),
    }
}

pub(crate) fn directory_space_administration_text_valid(value: &str) -> bool {
    !value.is_empty() && value.len() <= DOCUMENT_OPEN_ID_MAX_BYTES && !value.chars().any(char::is_control)
}

fn directory_space_administration_time_valid(value: i64) -> bool {
    value >= 0 && (value as u64) <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
}

impl DirectorySpaceAdministrationPageV1 {
    

    

    /// 🧾️ The receipt digest of whichever access shape this page carries.
    pub fn receipt_sha256(&self) -> &str {
        match self {
            Self::Public { receipt_sha256, .. } | Self::Member { receipt_sha256, .. } | Self::Author { receipt_sha256, .. } => receipt_sha256,
        }
    }

    /// 🆔️ The exact space this page projects.
    pub fn space_id(&self) -> &str {
        match self {
            Self::Public { space_id, .. } | Self::Member { space_id, .. } | Self::Author { space_id, .. } => space_id,
        }
    }

    /// 🛂️ Author capabilities, absent for every non-author shape.
    pub fn capabilities(&self) -> Option<DirectorySpaceAdministrationCapabilitiesV1> {
        match self {
            Self::Author { capabilities, .. } => Some(*capabilities),
            _ => None,
        }
    }

    

    
}

pub(crate) fn directory_space_administration_members_valid(window: &DirectorySpaceAdministrationMemberWindowV1) -> bool {
    if window.rows.len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS || !directory_space_administration_cursor_valid(&window.next_cursor) {
        return false;
    }
    let mut previous: Option<&str> = None;
    for row in &window.rows {
        if !directory_space_administration_text_valid(&row.user_id)
            || row.email.chars().any(char::is_control)
            || row.display_name.chars().any(char::is_control)
            || row.email.len() > DOCUMENT_OPEN_ID_MAX_BYTES
            || row.display_name.len() > DOCUMENT_OPEN_ID_MAX_BYTES
            || previous.is_some_and(|previous| previous >= row.user_id.as_str())
        {
            return false;
        }
        previous = Some(row.user_id.as_str());
    }
    true
}

pub(crate) fn directory_space_administration_invites_valid(window: &DirectorySpaceAdministrationInviteWindowV1) -> bool {
    if window.rows.len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS || !directory_space_administration_cursor_valid(&window.next_cursor) {
        return false;
    }
    let mut previous: Option<(i64, &str)> = None;
    for row in &window.rows {
        if !directory_space_administration_text_valid(&row.invite_id)
            || !directory_space_administration_time_valid(row.created_at_ms)
            || !directory_space_administration_time_valid(row.expires_at_ms)
            || previous.is_some_and(|previous| previous <= (row.created_at_ms, row.invite_id.as_str()))
        {
            return false;
        }
        previous = Some((row.created_at_ms, row.invite_id.as_str()));
    }
    true
}

/// 🧑️ One space member, display-ready (`email`/`display_name` joined from the user directory).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct MemberView {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub role: DirectorySpaceRole,
}

/// 🙋️ One platform user.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UserView {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub created_at_ms: i64,
}

/// 🔴️ One realtime document connection (admin overview / presence roster).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ConnectionView {
    pub sync_session_id: String,
    pub space_id: String,
    pub document_id: String,
    pub surface: String,
    pub actor: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub role: DirectorySpaceRole,
    pub connected_at_ms: i64,
    pub presence_known: bool,
}

/// 📦️ Immutable identity of the plugin package that owns a document codec.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOwner {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub package_hash: String,
}

/// 🏁️ One authoritative replication frontier bound to a canonical bootstrap snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentFrontier {
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
}

/// 🧬️ Durable, space-qualified codec and initial-bootstrap identity for one document.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentDescriptor {
    pub space_id: String,
    pub document_id: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub owner: DocumentOwner,
    pub pack_schema_hash: String,
    pub bootstrap_version: u32,
    pub bootstrap_frontier: DocumentFrontier,
    pub bootstrap_snapshot_hash: String,
}

/// 🧯️ Maximum UTF-8 byte length for one public document-open identity.
pub const DOCUMENT_OPEN_ID_MAX_BYTES: usize = 256;

/// 🧯️ Maximum UTF-8 byte length for one client-instance identity.
pub const DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES: usize = 128;

/// ⏳ Maximum lifetime of a document-open plan.
pub const DOCUMENT_OPEN_PLAN_MAX_TTL_MS: u64 = 30_000;

/// 🔢 Largest integer that has an exact representation in every v1 implementation.
pub const DOCUMENT_OPEN_MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// 📨 Structural, non-authoritative preference submitted to the protected open-plan command.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenIntentV1 {
    pub schema: String,
    pub version: u32,
    pub scope: DocumentScope,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub requested_surface_id: Option<String>,
    pub client_instance_id: String,
}

/// 🖼️ Renderer implementation selected by the verified server catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DocumentOpenRendererTargetV1 {
    React,
    Wgpu,
    Wasm,
}

impl DocumentOpenRendererTargetV1 {
    /// 🪞️ Canonical renderer identity for actor validation and transport projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::React => "react",
            Self::Wgpu => "wgpu",
            Self::Wasm => "wasm",
        }
    }
}

/// 👁️ Server-selected document surface authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "lowercase")]
pub enum DocumentOpenSurfaceRoleV1 {
    Viewer,
    Editor,
}

/// 📡️ Exact application-channel ABI claimed by the compiled package descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentExecutionProtocolV1 {
    pub app_channel_version: u32,
}

/// 📦️ Exact verified package projection required by one open plan: its identity, the byte identity of its component and
/// semantic descriptor, and the shipped `ComponentReceiptV1` binding the two (ticket 26/09/23 trusted-catalog receipts).
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenPackageV1 {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub component_sha256: String,
    pub component_blake3: String,
    pub descriptor_byte_sha256: String,
    pub execution_protocol: DocumentExecutionProtocolV1,
    pub component_receipt: semio_framework_plugin_component_receipt::ComponentReceiptV1,
}

impl DocumentOpenPackageV1 {
    /// 🔗️ Whether the package's receipt is a valid shipped receipt naming exactly this package, its component digest and
    /// its semantic descriptor digest — the one law every plan, lease and catalog binding holds its package to.
    pub fn receipt_binds(&self) -> bool {
        self.component_receipt.binds(semio_framework_plugin_component_receipt::ComponentProfileV1::Release, &self.plugin_id, &self.package_id, &self.component_sha256, &self.descriptor_byte_sha256).is_ok()
    }
}

/// 🗂️ Immutable verified-catalog generation selected for one plan.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenCatalogV1 {
    pub generation_id: String,
}

/// 🧬️ Exact immutable artifact projection required by one open plan.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenArtifactV1 {
    pub kind: String,
    pub schema: String,
    pub pack_schema_hash: String,
}

/// 🧭️ Complete parent dialect selected from the verified application declaration.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenParentDialectV1 {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}

/// 🪟️ One server-selected declared surface.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenSurfaceV1 {
    pub surface_id: String,
    pub app_id: String,
    pub window_kind_id: String,
    pub role: DocumentOpenSurfaceRoleV1,
    pub renderer_target: DocumentOpenRendererTargetV1,
}

/// 🔐️ Effective document operations after catalog and subject policy intersection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenGrantV1 {
    pub read: bool,
    pub write: bool,
    pub observe: bool,
}

/// 🏔️ Public immutable bootstrap identity selected for this plan.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenCheckpointV1 {
    pub checkpoint_id: String,
    pub descriptor_digest_v1: String,
    pub baseline_frontier: ArtifactFrontier,
    pub aggregate_sha256: String,
}

/// 🔁️ Durable generations that must remain exact until admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenRevalidationV1 {
    pub directory_revision: u64,
    pub membership_generation: u64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub session_generation: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub share_generation: Option<u64>,
}

/// 🎫️ Short-lived server-owned open decision. The receipt is exchanged once over protected HTTP.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenPlanV1 {
    pub schema: String,
    pub version: u32,
    pub receipt: String,
    pub expires_at_unix_ms: u64,
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub catalog: DocumentOpenCatalogV1,
    pub package: DocumentOpenPackageV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: DocumentOpenParentDialectV1,
    pub surface: DocumentOpenSurfaceV1,
    pub browser_actor: DocumentOpenBrowserActorV1,
    pub grant: DocumentOpenGrantV1,
    pub checkpoint: DocumentOpenCheckpointV1,
    pub revalidation: DocumentOpenRevalidationV1,
}

/// 🔄️ Protected command that exchanges one plan receipt for one document socket grant.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentPlanSocketGrantIntentV1 {
    pub schema: String,
    pub version: u32,
    pub plan_receipt: String,
}

/// 🚫️ Stable redacted open-plan failure vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum DocumentOpenPlanErrorCodeV1 {
    Denied,
    NotFound,
    CatalogUnavailable,
    ComponentUnavailable,
    Stale,
    Expired,
    AlreadyConsumed,
    Cancelled,
    DeadlineExceeded,
}

/// 🚨️ Public bounded open-plan failure without authority or catalog detail.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOpenPlanErrorV1 {
    pub schema: String,
    pub code: DocumentOpenPlanErrorCodeV1,
}

pub(super) fn valid_document_open_text(value: &str, max_bytes: usize) -> bool {
    !value.is_empty() && value.len() <= max_bytes && !value.chars().any(char::is_control)
}

pub fn valid_document_open_hash(value: &str) -> bool {
    value.len() == 64 && !value.as_bytes().iter().all(|byte| *byte == b'0') && value.as_bytes().iter().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn valid_document_open_receipt(value: &str) -> bool {
    value.strip_prefix("open.v1.").is_some_and(|secret| {
        let base64_value = |byte| match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'-' => Some(62),
            b'_' => Some(63),
            _ => None,
        };
        secret.len() == 43 && secret.bytes().all(|byte| base64_value(byte).is_some()) && secret.as_bytes().last().and_then(|byte| base64_value(*byte)).is_some_and(|tail| tail & 0b11 == 0)
    })
}

impl DocumentOpenIntentV1 {
    /// ✅ Validates the strict public intent without interpreting its fields as authority.
    pub fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        if self.schema != "semio.hub.document-open-intent/v1"
            || self.version != 1
            || !valid_document_open_text(&self.scope.space_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            || !valid_document_open_text(&self.scope.document_id, DOCUMENT_OPEN_ID_MAX_BYTES)
            || !valid_document_open_text(&self.client_instance_id, DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES)
            || self.requested_surface_id.as_deref().is_some_and(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES))
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        Ok(())
    }
}

impl DocumentOpenPlanV1 {
    /// ✅ Validates a complete receipt-free authority projection at a caller-supplied wall time.
    ///
    /// 🪢 `artifact.kind` and `parent_dialect.artifact_kind` are two DIFFERENT id spaces and are
    /// bounded separately, never against each other: `artifact.kind` is a manifest
    /// `ArtifactKindSpec::id` (`2d.note`, `text.document`, `stdio.json`) and `parent_dialect` is the
    /// owning app's `Dialect` (`s.note.note`, `s.writer.writer`, `s.stdio.json`). Only `gis` spells
    /// them alike, so an equality here is a plan that no plugin but `gis` can ever be issued. Both
    /// are pinned to the trusted catalog's own selection by `validate_descriptor_open_target`, which
    /// requires a manifest kind with exactly `artifact.kind` and an app whose dialect is exactly
    /// `parent_dialect`, so the binding is declared there and not re-derivable from two strings.
    pub fn validate(&self, now_ms: u64) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        self.browser_actor
            .validate(DocumentBrowserActorSourceV1 { component_sha256: &self.package.component_sha256, descriptor_byte_sha256: &self.package.descriptor_byte_sha256 }, self.surface.renderer_target.as_str())
            .map_err(|_| DocumentOpenPlanErrorCodeV1::Denied)?;
        let ids = [
            self.scope.space_id.as_str(),
            self.scope.document_id.as_str(),
            self.package.plugin_id.as_str(),
            self.package.package_id.as_str(),
            self.package.version.as_str(),
            self.artifact.kind.as_str(),
            self.artifact.schema.as_str(),
            self.surface.surface_id.as_str(),
            self.surface.app_id.as_str(),
            self.surface.window_kind_id.as_str(),
        ];
        if self.schema != "semio.hub.document-open-plan/v1"
            || self.version != 1
            || !valid_document_open_receipt(&self.receipt)
            || self.expires_at_unix_ms > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.expires_at_unix_ms <= now_ms
            || self.expires_at_unix_ms.checked_sub(now_ms).is_none_or(|ttl| ttl > DOCUMENT_OPEN_PLAN_MAX_TTL_MS)
            || ids.iter().any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES))
            || [&self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset].into_iter().any(|value| !valid_document_open_text(value, DOCUMENT_OPEN_ID_MAX_BYTES) || value.trim() != value.as_str())
            || !valid_document_open_hash(&self.descriptor_digest_v1)
            || !valid_document_open_hash(&self.catalog.generation_id)
            || !valid_document_open_hash(&self.package.component_sha256)
            || !valid_document_open_hash(&self.package.component_blake3)
            || !valid_document_open_hash(&self.package.descriptor_byte_sha256)
            || !self.package.receipt_binds()
            || self.package.execution_protocol.app_channel_version != semio_framework_os_channel::CHANNEL_VERSION
            || !valid_document_open_hash(&self.artifact.pack_schema_hash)
            || !self.grant.read
            || !self.grant.observe
            || self.grant.write != matches!(self.surface.role, DocumentOpenSurfaceRoleV1::Editor)
            || self.revalidation.directory_revision == 0
            || self.revalidation.directory_revision > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.revalidation.membership_generation == 0
            || self.revalidation.membership_generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || (self.revalidation.session_generation.is_some() == self.revalidation.share_generation.is_some())
            || self.revalidation.session_generation == Some(0)
            || self.revalidation.session_generation.is_some_and(|generation| generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER)
            || self.revalidation.share_generation == Some(0)
            || self.revalidation.share_generation.is_some_and(|generation| generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER)
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        let checkpoint = &self.checkpoint;
        if !checkpoint.baseline_frontier.is_genesis_for(&self.scope)
            && (!valid_document_open_text(&checkpoint.baseline_frontier.head_edit_id, DOCUMENT_OPEN_ID_MAX_BYTES)
                || checkpoint.baseline_frontier.head_edit_ordinal > DOCUMENT_OPEN_MAX_SAFE_INTEGER
                || checkpoint.baseline_frontier.last_commit_seq > DOCUMENT_OPEN_MAX_SAFE_INTEGER)
        {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        if !valid_document_open_hash(&checkpoint.checkpoint_id)
            || checkpoint.descriptor_digest_v1 != self.descriptor_digest_v1
            || !valid_document_open_hash(&checkpoint.aggregate_sha256)
            || checkpoint.baseline_frontier.document_id != self.scope.document_id
            || checkpoint.baseline_frontier.head_edit_ordinal < checkpoint.baseline_frontier.last_commit_seq
            || !(checkpoint.baseline_frontier.is_genesis_for(&self.scope) || checkpoint.baseline_frontier.is_edited_for(&self.scope))
        {
            return Err(DocumentOpenPlanErrorCodeV1::Stale);
        }
        Ok(())
    }
}

impl DocumentPlanSocketGrantIntentV1 {
    /// ✅ Validates the exact one-use receipt exchange command shape.
    pub fn validate(&self) -> Result<(), DocumentOpenPlanErrorCodeV1> {
        if self.schema != "semio.hub.document-plan-socket-grant-intent/v1" || self.version != 1 || !valid_document_open_receipt(&self.plan_receipt) {
            return Err(DocumentOpenPlanErrorCodeV1::Denied);
        }
        Ok(())
    }
}
//#endregion 🔖️Views
