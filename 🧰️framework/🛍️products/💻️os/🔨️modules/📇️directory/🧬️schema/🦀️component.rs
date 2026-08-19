//! 📇️ Directory event log wire contract (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-
//! STUDIOS, contract C1): `DirectoryEvent`/`DirectoryEventBody` (persisted, backend-assigned dense
//! `seq`), `DirectoryCommand` (client intent posted to `/directory/commands`), `DirectoryStreamMessage`
//! (the `/directory/ws` wire envelope), and the read DTOs (`SpaceView`/`MemberView`/`UserView`/
//! `ConnectionView`/`DocumentView`/`InviteView`) the hub's REST surface returns. Pure data, no fold
//! logic — see the module root `../🦀️component.rs`'s `DirectoryReadModel`/`fold`. `DirectorySpaceKind`/
//! `DirectorySpaceVisibility`/`DirectorySpaceRole` mirror `🪐️space`'s `SpaceKind`/`SpaceVisibility`/
//! `SpaceRole` vocabulary (atelier/studio/archive, private/public, author/spectator) string-identically;
//! this wasm-safe kernel crate does not mount that module (`📦️glue.rs`'s header note: unwired pending
//! dep-DAG cleanup), so the enums are re-declared here, same convention `🌎️hub/📇️directory`'s
//! `SpaceRole` already uses for the same reason.
//!
//! 🧭️ `space.created`'s and `create-space`'s space-kind fields are named `space_kind`
//! (`spaceKind` on the wire), not contract-freeze.md's bare `kind` — both bodies are internally
//! tagged (`#[serde(tag = "kind")]`), so a same-named payload field would collide with the
//! discriminator. Flagged as a `sharedFileRequest` in lane 0-A's report.

use serde::{Deserialize, Serialize};

//#region 🔖️Vocabulary
/// 🏛️ Mirrors `🪐️space::SpaceKind` string-identically (see this file's header).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectorySpaceKind {
    Atelier,
    Studio,
    Archive,
}

/// 👁️ Mirrors `🪐️space::SpaceVisibility` string-identically.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectorySpaceVisibility {
    Private,
    Public,
}

/// 🧑️‍🤝️‍🧑️ Mirrors `🪐️space::SpaceRole` string-identically.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectorySpaceRole {
    Author,
    Spectator,
}
//#endregion 🔖️Vocabulary

//#region 🔖️Actor
/// 🎭️ Who issued a directory command / recorded a directory event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectoryActorKind {
    User,
    Admin,
    System,
}

/// 🎭️ `{ kind, id }` — the actor id grammar is `user:{user_id}#{shell_session_id}` for `User`
/// (contract-freeze.md §C0), opaque for `Admin`/`System`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryActor {
    pub kind: DirectoryActorKind,
    pub id: String,
}

/// 🕰️ Hybrid logical clock stamp: physical wall time plus a same-millisecond tiebreak counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hlc {
    pub physical_ms: i64,
    pub logical: u32,
}
//#endregion 🔖️Actor

//#region 🔖️Event
/// ⚡️ One directory event body. Every variant's `kind` tag is the contract's own dotted string
/// (e.g. `"space.created"`) — not a `rename_all` casing of the variant name — so every variant
/// carries an explicit `#[serde(rename = "…")]`. `rename_all_fields = "camelCase"` casings each
/// variant's own fields independently of the tag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all_fields = "camelCase")]
pub enum DirectoryEventBody {
    #[serde(rename = "user.created")]
    UserCreated { user_id: String, email: String, display_name: String },
    #[serde(rename = "space.created")]
    SpaceCreated { space_id: String, name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility, owner_user_id: String },
    #[serde(rename = "space.renamed")]
    SpaceRenamed { space_id: String, name: String },
    #[serde(rename = "space.visibility-changed")]
    SpaceVisibilityChanged { space_id: String, visibility: DirectorySpaceVisibility },
    #[serde(rename = "space.archived")]
    SpaceArchived { space_id: String },
    #[serde(rename = "space.deleted")]
    SpaceDeleted { space_id: String },
    #[serde(rename = "member.upserted")]
    MemberUpserted { space_id: String, user_id: String, role: DirectorySpaceRole },
    #[serde(rename = "member.removed")]
    MemberRemoved { space_id: String, user_id: String },
    #[serde(rename = "invite.redeemed")]
    InviteRedeemed { space_id: String, user_id: String, invite_id: String, role: DirectorySpaceRole },
}

/// 📜️ One persisted, backend-assigned directory event. `seq` is dense and 1-based; `space_id`/
/// `user_id` are denormalized indexing hints (redundant with `body`'s own fields) for cheap
/// `?since=`/visibility filtering without decoding `body`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryEvent {
    pub seq: u64,
    pub id: String,
    pub hlc: Hlc,
    pub actor: DirectoryActor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub body: DirectoryEventBody,
    pub recorded_at_ms: i64,
}
//#endregion 🔖️Event

//#region 🔖️Command
/// 🎮️ One client-issued directory command, posted to `POST /directory/commands` (contract C2).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub enum DirectoryCommand {
    CreateSpace { name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility },
    RenameSpace { space_id: String, name: String },
    SetVisibility { space_id: String, visibility: DirectorySpaceVisibility },
    ArchiveSpace { space_id: String },
    DeleteSpace { space_id: String },
    UpsertMember { space_id: String, email: String, role: DirectorySpaceRole },
    RemoveMember { space_id: String, user_id: String },
    CreateInvite { space_id: String, role: DirectorySpaceRole, ttl_secs: u64 },
    RevokeInvite { space_id: String, invite_id: String },
}
//#endregion 🔖️Command

//#region 🔖️Views
/// 🏠️ One space, as the hub's REST/read surface renders it. `role` is the CALLING user's
/// membership role (server-filled per request), never derived by the pure fold.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceView {
    pub id: String,
    pub name: String,
    pub kind: DirectorySpaceKind,
    pub visibility: DirectorySpaceVisibility,
    pub owner_user_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<DirectorySpaceRole>,
    pub member_count: u32,
    pub document_count: u32,
    pub active_connections: u32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
}

/// 🧑️ One space member, display-ready (`email`/`display_name` joined from the user directory).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberView {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub role: DirectorySpaceRole,
}

/// 🙋️ One platform user.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserView {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub created_at_ms: i64,
}

/// 🔴️ One realtime document connection (admin overview / presence roster).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionView {
    pub sync_session_id: String,
    pub space_id: String,
    pub document_id: String,
    pub surface: String,
    pub actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub role: DirectorySpaceRole,
    pub connected_at_ms: i64,
    pub presence_known: bool,
}

/// 🧾️ One document inside a space's artifact index (headSeq/commitSeq/epoch — sync bookkeeping).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentView {
    pub id: String,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
}

/// 🔗️ One outstanding (or revoked) space invite. Not event-sourced itself (secret token lives
/// outside the log) — only its `invite.redeemed` outcome is a `DirectoryEvent`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteView {
    pub id: String,
    pub space_id: String,
    pub role: DirectorySpaceRole,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revoked: bool,
}
//#endregion 🔖️Views

//#region 🔖️Stream
/// 🔌️ `connection` stream message phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectoryConnectionPhase {
    Opened,
    Closed,
}

/// 👥️ One live presence actor in a document's roster (Amendment 3 to C1) — the hub knows all four
/// fields without ever decoding the actor's opaque `PresencePeer` bytes: `surface`/`color` are
/// stamped at hub-handshake time (`?surface=`, `HubState.session_colors`), `user_id` from auth.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryPresenceActor {
    pub actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub surface: String,
    pub color: u8,
}

/// 📡️ One `/directory/ws` text frame (contract C1/C2) — subscribe, then gap-free replay.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase", rename_all_fields = "camelCase")]
pub enum DirectoryStreamMessage {
    Event { event: DirectoryEvent },
    Connection { phase: DirectoryConnectionPhase, connection: ConnectionView },
    /// 👥️ Amendment 3 to C1: the document-wide roster, published on every roster change.
    Presence { space_id: String, document_id: String, actors: Vec<DirectoryPresenceActor> },
    Heartbeat { head_seq: u64 },
}
//#endregion 🔖️Stream

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn event_body_kind_is_the_dotted_wire_string() {
        let body = DirectoryEventBody::SpaceCreated {
            space_id: "sp-1".into(),
            name: "Studio".into(),
            space_kind: DirectorySpaceKind::Studio,
            visibility: DirectorySpaceVisibility::Private,
            owner_user_id: "u-1".into(),
        };
        let json = serde_json::to_value(&body).expect("serialize");
        assert_eq!(json["kind"], "space.created");
        assert_eq!(json["spaceKind"], "studio");
        assert_eq!(json["visibility"], "private");
        let round: DirectoryEventBody = serde_json::from_value(json).expect("deserialize");
        assert_eq!(round, body);
    }

    #[test]
    async fn command_kind_is_kebab_case() {
        let command = DirectoryCommand::CreateSpace { name: "Atelier".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private };
        let json = serde_json::to_value(&command).expect("serialize");
        assert_eq!(json["kind"], "create-space");
        assert_eq!(json["spaceKind"], "atelier");
    }

    #[test]
    async fn stream_message_kinds_round_trip() {
        let heartbeat = DirectoryStreamMessage::Heartbeat { head_seq: 42 };
        let json = serde_json::to_value(&heartbeat).expect("serialize");
        assert_eq!(json["kind"], "heartbeat");
        assert_eq!(json["headSeq"], 42);
        let round: DirectoryStreamMessage = serde_json::from_value(json).expect("deserialize");
        assert_eq!(round, heartbeat);
    }
}
//#endregion 🧪️Tests
