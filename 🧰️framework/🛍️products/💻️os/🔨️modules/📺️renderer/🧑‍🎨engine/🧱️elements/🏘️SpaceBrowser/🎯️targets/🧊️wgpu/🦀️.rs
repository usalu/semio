//! 🏘️ wgpu twin of the `🏘️SpaceBrowser` surface and of the pure contract underneath it
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts`).
//!
//! 🧩️ The end-user half of the directory — browse my spaces, switch, create, see who is here,
//! invite, redeem — which `🛂️SpaceAdministration` (an ADMIN pane for exactly one space) never
//! covered on either renderer. Pure projection plus command construction: every mutation leaves as
//! a closed [`DirectoryCommand`] for `POST /directory/commands` (CQRS — this module never issues a
//! CRUD write), and redemption leaves as the hub's own `POST /directory/invites/{token}/redeem`.
//!
//! ⚖️ Ordering and filtering are LOCALE-INDEPENDENT by construction. A locale-aware collation would
//! make two devices in one space disagree on row order, which is exactly the class of divergence
//! the shared fixture (`📇️directory/🏘️spaces/🔣️.json`) exists to catch: its `accessOrder`,
//! `commandFieldOrder`, `inviteTokens` and `redemptionStatusCodes` tables drive both this module's
//! tests and the React twin's.

use semio_framework_os_kernel::os_directory::{DirectoryCommand, DirectorySpaceAdministrationMemberRowV1, DirectorySpaceKind, DirectorySpaceListEntryV1, DirectorySpaceRole, DirectorySpaceVisibility};
use ui_wgpu::wgpu::Locale;

//#region 🔖️Routes
pub const DIRECTORY_COMMANDS_PATH_V1: &str = "/directory/commands";
pub const DIRECTORY_SPACES_PATH_V1: &str = "/directory/spaces";
pub const INVITE_TOKEN_MAX_BYTES: usize = 256;
pub const SPACE_NAME_MAX_BYTES: usize = 128;
/// ⏳️ The three lifetimes an invitation may be issued for — one hour, one day, one week.
pub const INVITE_TTL_CHOICES_SECS_V1: [u64; 3] = [3600, 86400, 604800];
/// 🔗️ The capability lives in the link's FRAGMENT, which browsers never put on the wire, so a
/// clicked invitation cannot reach a server log.
pub const INVITE_LINK_FRAGMENT_V1: &str = "#semio-invite=";

/// 🎟️ The hub's redemption route for one invite capability. The token is a capability, so it goes in
/// the path of a POST and never into a query string, a log line or the connection book.
pub fn invite_redeem_path(token: &str) -> Option<String> {
    let token = parse_invite_token(token)?;
    Some(format!("/directory/invites/{token}/redeem"))
}
//#endregion 🔖️Routes

//#region 🏠️Rows
/// 🔎️ How the caller reaches one space — the hub's own list discriminator, hoisted out of the union
/// so a row needs no narrowing to render.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpaceAccess {
    Author,
    Member,
    Public,
}

impl SpaceAccess {
    /// 🏷️ The wire spelling the React twin stamps on its row.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Author => "author",
            Self::Member => "member",
            Self::Public => "public",
        }
    }
}

/// 🏠️ One space as the end-user browser renders it. `role` is `None` for a public space the caller
/// is not a member of — absent rather than defaulted, because "no role" and "spectator" grant
/// different things.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceRow {
    pub id: String,
    pub name: String,
    pub kind: DirectorySpaceKind,
    pub visibility: DirectorySpaceVisibility,
    pub access: SpaceAccess,
    pub role: Option<DirectorySpaceRole>,
    pub member_count: u32,
    pub document_count: u32,
    pub active_connections: u32,
    pub updated_at_ms: i64,
}

fn space_row(entry: &DirectorySpaceListEntryV1) -> SpaceRow {
    match entry {
        DirectorySpaceListEntryV1::Public { space } => SpaceRow {
            id: space.id.clone(),
            name: space.name.clone(),
            kind: space.kind,
            visibility: space.visibility,
            access: SpaceAccess::Public,
            role: None,
            member_count: space.member_count,
            document_count: space.document_count,
            active_connections: 0,
            updated_at_ms: space.updated_at_ms,
        },
        DirectorySpaceListEntryV1::Member { space } | DirectorySpaceListEntryV1::Author { space } => SpaceRow {
            id: space.id.clone(),
            name: space.name.clone(),
            kind: space.kind,
            visibility: space.visibility,
            access: if matches!(entry, DirectorySpaceListEntryV1::Author { .. }) { SpaceAccess::Author } else { SpaceAccess::Member },
            role: Some(space.role),
            member_count: space.member_count,
            document_count: space.document_count,
            active_connections: space.active_connections,
            updated_at_ms: space.updated_at_ms,
        },
    }
}

/// 📇️ Projects the hub's space list into render rows: my spaces first (author, then member, then
/// public), each group most-recently-updated first, ties broken by id. Total and deterministic, so
/// two renderers and two devices agree byte for byte.
pub fn space_rows(entries: &[DirectorySpaceListEntryV1]) -> Vec<SpaceRow> {
    let mut rows: Vec<SpaceRow> = entries.iter().map(space_row).collect();
    rows.sort_by(|left, right| left.access.cmp(&right.access).then(right.updated_at_ms.cmp(&left.updated_at_ms)).then(left.id.cmp(&right.id)));
    rows
}

/// 🔎️ Case-insensitive substring filter over name and id. An empty query keeps every row.
pub fn filter_space_rows(rows: &[SpaceRow], query: &str) -> Vec<SpaceRow> {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return rows.to_vec();
    }
    rows.iter().filter(|row| row.name.to_lowercase().contains(&needle) || row.id.to_lowercase().contains(&needle)).cloned().collect()
}

/// ✍️ Whether the caller may create documents in this space — the only authority a row carries.
pub fn space_row_writable(row: &SpaceRow) -> bool {
    row.access == SpaceAccess::Author || row.role == Some(DirectorySpaceRole::Author)
}

/// 🎟️ Whether the caller may issue invitations for this space. Membership alone is never enough.
pub fn space_row_invitable(row: &SpaceRow) -> bool {
    row.access == SpaceAccess::Author
}
//#endregion 🏠️Rows

//#region 👥️Presence
/// 👥️ One member of a space, joined with whether they are connected right now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceMemberPresence {
    pub user_id: String,
    pub display_name: String,
    pub role: DirectorySpaceRole,
    pub owner: bool,
    pub online: bool,
}

/// 👥️ Joins the hub's member roster with the user ids the presence lane reports as connected.
/// Owners first, then authors, then spectators, each ascending by user id — a stable order, so the
/// roster does not reshuffle on every presence tick. A member with no display name falls back to
/// their email rather than rendering a blank row.
pub fn space_member_presence(members: &[DirectorySpaceAdministrationMemberRowV1], online_user_ids: &[String]) -> Vec<SpaceMemberPresence> {
    let mut rows: Vec<SpaceMemberPresence> = members
        .iter()
        .map(|member| SpaceMemberPresence {
            user_id: member.user_id.clone(),
            display_name: if member.display_name.is_empty() { member.email.clone() } else { member.display_name.clone() },
            role: member.role,
            owner: member.owner,
            online: online_user_ids.iter().any(|candidate| candidate == &member.user_id),
        })
        .collect();
    rows.sort_by(|left, right| right.owner.cmp(&left.owner).then(left.role.cmp(&right.role)).then(left.user_id.cmp(&right.user_id)));
    rows
}
//#endregion 👥️Presence

//#region 🎮️Commands
/// 🏗️ Builds the `create-space` command. The canonical field order
/// (`kind, name, spaceKind, visibility`) is the enum's own declaration order in
/// `📇️directory/🧬️schema/🦀️.rs`, which the sealer re-serializes and byte-compares — a divergent
/// order is a `noncanonical-command` refusal at seal time in this repo, never a silent hub error.
pub fn create_space_command(name: &str, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility) -> Option<DirectoryCommand> {
    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed.len() > SPACE_NAME_MAX_BYTES || trimmed.chars().any(char::is_control) {
        return None;
    }
    Some(DirectoryCommand::CreateSpace { name: trimmed.to_string(), space_kind, visibility })
}

/// 🎟️ Builds the `create-invite` command. The hub answers with a one-shot capability inside the
/// command receipt's `result`; this module never sees it, so it cannot leak it.
pub fn create_invite_command(space_id: &str, role: DirectorySpaceRole, ttl_secs: u64) -> Option<DirectoryCommand> {
    if space_id.is_empty() || space_id.chars().any(char::is_control) || ttl_secs == 0 {
        return None;
    }
    Some(DirectoryCommand::CreateInvite { space_id: space_id.to_string(), role, ttl_secs })
}

/// 🚪️ Builds the `archive-space` command — the end-user "leave this behind" verb. Deletion stays in
/// the admin pane, which gates it on the server-declared `deleteSpace` capability.
pub fn archive_space_command(space_id: &str) -> Option<DirectoryCommand> {
    if space_id.is_empty() || space_id.chars().any(char::is_control) {
        return None;
    }
    Some(DirectoryCommand::ArchiveSpace { space_id: space_id.to_string() })
}
//#endregion 🎮️Commands

//#region 🎟️Invites
/// 🎟️ Accepts either a bare capability or a whole invitation link and yields the bare capability.
/// Pasting a link is what humans actually do, so the fragment form is parsed here rather than
/// refused with a lecture.
pub fn parse_invite_token(text: &str) -> Option<String> {
    let trimmed = text.trim();
    let token = match trimmed.find(INVITE_LINK_FRAGMENT_V1) {
        Some(index) => &trimmed[index + INVITE_LINK_FRAGMENT_V1.len()..],
        None => trimmed,
    };
    if token.is_empty() || token.len() > INVITE_TOKEN_MAX_BYTES {
        return None;
    }
    if !token.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'~' | b'-')) {
        return None;
    }
    Some(token.to_string())
}

/// 🔗️ Renders one invitation link for a hub origin and capability, for the human to send through a
/// channel of their own choosing.
pub fn invite_link(origin: &str, token: &str) -> Option<String> {
    let token = parse_invite_token(token)?;
    Some(format!("{origin}/{INVITE_LINK_FRAGMENT_V1}{token}"))
}

/// 🚫️ Closed redemption denial classes, so the surface names a cause instead of a status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InviteRedemptionErrorCode {
    InvalidInvite,
    ExpiredInvite,
    AlreadyMember,
    Unauthorized,
    Unreachable,
    HubRefused,
    Cancelled,
}

impl InviteRedemptionErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInvite => "invalid-invite",
            Self::ExpiredInvite => "expired-invite",
            Self::AlreadyMember => "already-member",
            Self::Unauthorized => "unauthorized",
            Self::Unreachable => "unreachable",
            Self::HubRefused => "hub-refused",
            Self::Cancelled => "cancelled",
        }
    }

    /// 🗣️ What the human reads, in both languages this product owns.
    pub fn text(self, locale: Locale) -> &'static str {
        match (self, locale) {
            (Self::InvalidInvite, Locale::En) => "That invitation is not readable. Ask for a new link.",
            (Self::InvalidInvite, Locale::De) => "Diese Einladung ist nicht lesbar. Bitte um einen neuen Link.",
            (Self::ExpiredInvite, Locale::En) => "That invitation has expired. Ask for a new one.",
            (Self::ExpiredInvite, Locale::De) => "Diese Einladung ist abgelaufen. Bitte um eine neue.",
            (Self::AlreadyMember, Locale::En) => "You are already a member of this space.",
            (Self::AlreadyMember, Locale::De) => "Du bist bereits Mitglied dieses Spaces.",
            (Self::Unauthorized, Locale::En) => "Sign in to this hub before redeeming an invitation.",
            (Self::Unauthorized, Locale::De) => "Melde dich bei diesem Hub an, bevor du eine Einladung einlöst.",
            (Self::Unreachable, Locale::En) => "This hub cannot be reached right now. Your invitation is still valid.",
            (Self::Unreachable, Locale::De) => "Dieser Hub ist gerade nicht erreichbar. Deine Einladung bleibt gültig.",
            (Self::HubRefused, Locale::En) => "This hub refused the invitation.",
            (Self::HubRefused, Locale::De) => "Dieser Hub hat die Einladung abgelehnt.",
            (Self::Cancelled, Locale::En) => "Redemption was cancelled.",
            (Self::Cancelled, Locale::De) => "Das Einlösen wurde abgebrochen.",
        }
    }
}

/// 🌐️ Maps one redemption status to its closed code. A `429` is deliberately `Unreachable`, not a
/// denial: the invitation is still good and the human should try again, which is what that text says.
pub fn invite_redemption_error_from_status(status: u16) -> InviteRedemptionErrorCode {
    match status {
        400 | 404 => InviteRedemptionErrorCode::InvalidInvite,
        410 => InviteRedemptionErrorCode::ExpiredInvite,
        409 => InviteRedemptionErrorCode::AlreadyMember,
        401 | 403 => InviteRedemptionErrorCode::Unauthorized,
        408 | 429 => InviteRedemptionErrorCode::Unreachable,
        other if other >= 500 => InviteRedemptionErrorCode::Unreachable,
        _ => InviteRedemptionErrorCode::HubRefused,
    }
}
//#endregion 🎟️Invites

//#region 🔄️Phase
/// 🔄️ The spaces surface's own load/mutate phase. `Stale` is the local-first state: rows keep
/// rendering from the last projection while the hub is unreachable, and the app stays usable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpaceBrowserPhase {
    #[default]
    Loading,
    Ready,
    Stale,
    Submitting,
    Failed,
}

impl SpaceBrowserPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Ready => "ready",
            Self::Stale => "stale",
            Self::Submitting => "submitting",
            Self::Failed => "failed",
        }
    }

    /// 🗣️ The live region's text, in both languages.
    pub fn text(self, locale: Locale) -> &'static str {
        match (self, locale) {
            (Self::Loading, Locale::En) => "Loading your spaces…",
            (Self::Loading, Locale::De) => "Deine Spaces werden geladen…",
            (Self::Ready, Locale::En) => "Your spaces are current.",
            (Self::Ready, Locale::De) => "Deine Spaces sind aktuell.",
            (Self::Stale, Locale::En) => "Showing the last known spaces; this hub is not answering.",
            (Self::Stale, Locale::De) => "Es werden die zuletzt bekannten Spaces gezeigt; dieser Hub antwortet nicht.",
            (Self::Submitting, Locale::En) => "Sending…",
            (Self::Submitting, Locale::De) => "Wird gesendet…",
            (Self::Failed, Locale::En) => "This hub refused the last request.",
            (Self::Failed, Locale::De) => "Dieser Hub hat die letzte Anfrage abgelehnt.",
        }
    }
}

/// 🏠️ Whether the rows on screen may still be opened. Only an empty first load blocks.
pub fn space_browser_rows_usable(phase: SpaceBrowserPhase, row_count: usize) -> bool {
    row_count > 0 && phase != SpaceBrowserPhase::Loading
}
//#endregion 🔄️Phase

//#region 🌐️Text
/// 🏷️ The closed set of chrome labels the spaces surface names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpaceBrowserLabel {
    Title,
    Search,
    Open,
    Create,
    CreateName,
    Invite,
    InviteLink,
    CopyLink,
    DiscardLink,
    Redeem,
    RedeemPlaceholder,
    Members,
    Empty,
    Online,
    Offline,
    Owner,
    RoleAuthor,
    RoleSpectator,
}

/// 🏷️ One chrome label, in both languages this product owns. Both columns are written in the same
/// match, so a half-translated key cannot compile.
pub fn space_browser_label(key: SpaceBrowserLabel, locale: Locale) -> &'static str {
    match (key, locale) {
        (SpaceBrowserLabel::Title, Locale::En) => "Spaces",
        (SpaceBrowserLabel::Title, Locale::De) => "Spaces",
        (SpaceBrowserLabel::Search, Locale::En) => "Search spaces",
        (SpaceBrowserLabel::Search, Locale::De) => "Spaces durchsuchen",
        (SpaceBrowserLabel::Open, Locale::En) => "Open",
        (SpaceBrowserLabel::Open, Locale::De) => "Öffnen",
        (SpaceBrowserLabel::Create, Locale::En) => "Create a space",
        (SpaceBrowserLabel::Create, Locale::De) => "Space erstellen",
        (SpaceBrowserLabel::CreateName, Locale::En) => "Name",
        (SpaceBrowserLabel::CreateName, Locale::De) => "Name",
        (SpaceBrowserLabel::Invite, Locale::En) => "Invite someone",
        (SpaceBrowserLabel::Invite, Locale::De) => "Jemanden einladen",
        (SpaceBrowserLabel::InviteLink, Locale::En) => "Invitation link",
        (SpaceBrowserLabel::InviteLink, Locale::De) => "Einladungslink",
        (SpaceBrowserLabel::CopyLink, Locale::En) => "Copy link",
        (SpaceBrowserLabel::CopyLink, Locale::De) => "Link kopieren",
        (SpaceBrowserLabel::DiscardLink, Locale::En) => "Discard link",
        (SpaceBrowserLabel::DiscardLink, Locale::De) => "Link verwerfen",
        (SpaceBrowserLabel::Redeem, Locale::En) => "Redeem an invitation",
        (SpaceBrowserLabel::Redeem, Locale::De) => "Einladung einlösen",
        (SpaceBrowserLabel::RedeemPlaceholder, Locale::En) => "Paste an invitation link",
        (SpaceBrowserLabel::RedeemPlaceholder, Locale::De) => "Einladungslink einfügen",
        (SpaceBrowserLabel::Members, Locale::En) => "Members",
        (SpaceBrowserLabel::Members, Locale::De) => "Mitglieder",
        (SpaceBrowserLabel::Empty, Locale::En) => "No spaces yet.",
        (SpaceBrowserLabel::Empty, Locale::De) => "Noch keine Spaces.",
        (SpaceBrowserLabel::Online, Locale::En) => "here now",
        (SpaceBrowserLabel::Online, Locale::De) => "gerade hier",
        (SpaceBrowserLabel::Offline, Locale::En) => "away",
        (SpaceBrowserLabel::Offline, Locale::De) => "abwesend",
        (SpaceBrowserLabel::Owner, Locale::En) => "owner",
        (SpaceBrowserLabel::Owner, Locale::De) => "Eigentümer",
        (SpaceBrowserLabel::RoleAuthor, Locale::En) => "author",
        (SpaceBrowserLabel::RoleAuthor, Locale::De) => "Autor",
        (SpaceBrowserLabel::RoleSpectator, Locale::En) => "spectator",
        (SpaceBrowserLabel::RoleSpectator, Locale::De) => "Zuschauer",
    }
}

/// 🏷️ One row's secondary line: access class, member count and how many people are connected.
pub fn space_row_summary(row: &SpaceRow, locale: Locale) -> String {
    let access = match (row.access, locale) {
        (SpaceAccess::Author, Locale::En) => "yours",
        (SpaceAccess::Author, Locale::De) => "deiner",
        (SpaceAccess::Member, Locale::En) => "shared with you",
        (SpaceAccess::Member, Locale::De) => "mit dir geteilt",
        (SpaceAccess::Public, Locale::En) => "public",
        (SpaceAccess::Public, Locale::De) => "öffentlich",
    };
    let members = space_browser_label(SpaceBrowserLabel::Members, locale);
    format!("{access} · {} {members} · {} {}", row.member_count, row.active_connections, space_browser_label(SpaceBrowserLabel::Online, locale))
}
//#endregion 🌐️Text

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
