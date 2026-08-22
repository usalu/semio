//! 🗄️ Backend-agnostic os-hub identity/tenancy directory. `HubDirectory` is the single seam every
//! persistence backend (sqlite/postgres/neo4j — see the sibling `🪶️sqlite`/`🐘️postgres`/`🌐️neo4j`
//! component folders, each `#[cfg(feature = "…")]`-gated) implements; `bin.rs` never sees a driver
//! type (sqlx/neo4rs/rusqlite), only this trait and the DTOs in `model` — satisfies the "external
//! libraries stay behind an interface" rule for a trait three backends must share.
//!
//! 🎯️ Design choice (split from the pre-CW6 `HubStorage`): document persistence (snapshots,
//! operations) and content-addressed blobs are no longer this crate's concern — `db::Database`
//! (server-side document authority) and `db`'s own `PayloadStorage` own that now (see `bin.rs`).
//! This module keeps exactly the identity/tenancy surface that has no `db` counterpart: users,
//! spaces, memberships, auth sessions, share tokens, and realtime sync sessions. The former VFS
//! tree (`NodeRecord`/`list_nodes`/`create_node`) was deleted in the space/collection/artifact
//! unification wave — the collection document now replaces the hub-side tree (see
//! `.claude/plans/the-final-goal-for-jolly-spindle.md`'s "Roles/kinds/visibility" design ruling).

//#region 🔖️Error
pub mod error {
    /// @emoji 🧯️ Opaque directory error — never wraps a backend driver's error type, so no `sqlx`/
    /// `neo4rs`/`rusqlite` type ever crosses this crate's public API.
    #[derive(Debug)]
    pub enum DirectoryError {
        NotFound(String),
        Conflict(String),
        Unauthorized,
        Backend(String),
    }

    impl std::fmt::Display for DirectoryError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::NotFound(detail) => write!(formatter, "not found: {detail}"),
                Self::Conflict(detail) => write!(formatter, "conflict: {detail}"),
                Self::Unauthorized => formatter.write_str("unauthorized"),
                Self::Backend(detail) => write!(formatter, "backend error: {detail}"),
            }
        }
    }

    impl std::error::Error for DirectoryError {}

    pub type DirectoryResult<T> = Result<T, DirectoryError>;
}
//#endregion 🔖️Error

//#region 🔖️Model
pub mod model {
    use serde::{Deserialize, Serialize};

    /// @emoji 🔗️ An anonymous per-document bearer token (existing auth-lite scheme, kept as-is).
    /// `document_id` is opaque here — the directory has no FK relationship to a `db::Database`
    /// document; it never persists document content itself.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ShareTokenRecord {
        pub token: String,
        pub document_id: String,
        pub created_at: i64,
    }

    /// @emoji 🙋️ A platform user — local password login and/or one linked SSO identity. Also the
    /// projection `user.created` folds into (see the module root's `//#region 🔖️Projections` on
    /// each backend).
    #[derive(Clone, Debug, PartialEq)]
    pub struct UserRecord {
        pub id: String,
        pub email: String,
        pub display_name: String,
        pub password_hash: Option<String>,
        pub sso_subject: Option<String>,
        pub sso_provider: Option<String>,
        pub created_at: i64,
    }

    /// @emoji 🏛️ A space: the tenant/workspace unit that owns documents and memberships. `kind`
    /// (`"atelier"|"studio"|"archive"`) and `visibility` (`"private"|"public"`) mirror the
    /// wasm-facing `space` crate's `SpaceKind`/`SpaceVisibility` string-identically — this crate
    /// cannot depend on that crate (server-side binary vs wasm-facing kernel), so the two are kept
    /// in lockstep by hand, same as `SpaceRole` below. Also the projection `space.created`/
    /// `space.renamed`/`space.visibility-changed`/`space.archived`/`space.deleted` fold into.
    #[derive(Clone, Debug, PartialEq)]
    pub struct SpaceRecord {
        pub id: String,
        pub name: String,
        pub owner_user_id: String,
        pub created_at: i64,
        pub kind: String,
        pub visibility: String,
    }

    /// @emoji 🧑️‍🤝️‍🧑️ A space member's permission level, string-identical to the `space` crate's
    /// `SpaceRole { Author, Spectator }` (`"author"`/`"spectator"`) — see `SpaceRecord`'s doc for
    /// why this crate re-declares rather than depends. Distinct from the wire-facing
    /// `directory::os_directory::DirectorySpaceRole` events/commands carry (see this module root's
    /// `//#region 🔖️Wire` for the conversion between the two).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum SpaceRole {
        Author,
        Spectator,
    }

    impl SpaceRole {
        pub fn as_str(&self) -> &'static str {
            match self {
                SpaceRole::Author => "author",
                SpaceRole::Spectator => "spectator",
            }
        }

        pub fn parse(value: &str) -> Option<Self> {
            match value {
                "author" => Some(SpaceRole::Author),
                "spectator" => Some(SpaceRole::Spectator),
                _ => None,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct SpaceMembershipRecord {
        pub space_id: String,
        pub user_id: String,
        pub role: SpaceRole,
        pub created_at: i64,
    }

    /// @emoji 🍪️ A browser login session (distinct from {@link SyncSessionRecord}'s realtime connection).
    #[derive(Clone, Debug, PartialEq)]
    pub struct AuthSessionRecord {
        pub id: String,
        pub user_id: String,
        pub created_at: i64,
        pub expires_at: i64,
        pub sso_provider: Option<String>,
    }

    /// @emoji 🔴️ A realtime document connection — the "session as live-features backend" record;
    /// written by `bin.rs`'s wire-v2 WS handler on Hello/disconnect, not per-operation. Not
    /// event-sourced (contract's decider laws) — `space_id`/`surface` widen this record so the
    /// admin overview and presence roster can key/filter by them without joining back to `db`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct SyncSessionRecord {
        pub id: String,
        pub space_id: String,
        pub document_id: String,
        pub surface: String,
        pub user_id: Option<String>,
        pub space_role: Option<SpaceRole>,
        pub client_label: String,
        pub connected_at: i64,
        pub disconnected_at: Option<i64>,
    }

    /// @emoji 🎟️ An outstanding (or revoked) space invite. Not event-sourced itself — only its
    /// `invite.redeemed` outcome is (contract's decider laws) — so `token` (the bearer secret)
    /// lives only here, never in the event log. `role` is the role a redeemer is granted.
    #[derive(Clone, Debug, PartialEq)]
    pub struct InviteRecord {
        pub id: String,
        pub token: String,
        pub space_id: String,
        pub role: SpaceRole,
        pub created_at: i64,
        pub expires_at: i64,
        pub revoked_at: Option<i64>,
    }
}
//#endregion 🔖️Model

use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, Hlc};
use directory::os_identity::time_ordered_id;
use error::{DirectoryError, DirectoryResult};
use model::*;
use std::sync::Arc;

//#region 🔖️Wire
// 🔗️ This crate's storage-row vocabulary (`SpaceRole`, plain `kind`/`visibility` strings — see
// `model`'s doc comments for why they are hand-kept-in-lockstep rather than depended on) versus the
// wire vocabulary the schema crate owns (`DirectorySpaceRole`/`DirectorySpaceKind`/
// `DirectorySpaceVisibility`, imported above, never redeclared). Free functions, not inherent impls
// on the wire types — they are foreign here, so the orphan rule blocks `impl DirectorySpaceRole {}`.

/// 🔁️ Local storage role -> wire role (used when an event body needs the wire enum but the value
/// in hand came from a projection read, e.g. an `InviteRecord`). `pub(crate)` — every backend's
/// `//#region 🔖️Projections` uses the reverse direction (`role_from_wire(role).as_str()`) to get
/// the exact `"author"`/`"spectator"` string its `CHECK` constraint speaks.
pub(crate) fn role_to_wire(role: SpaceRole) -> DirectorySpaceRole {
    match role {
        SpaceRole::Author => DirectorySpaceRole::Author,
        SpaceRole::Spectator => DirectorySpaceRole::Spectator,
    }
}

/// 🔁️ Wire role -> local storage role (used when a `HubDirectory` method that still speaks the
/// storage vocabulary — e.g. `create_invite` — needs a role that arrived as a `DirectoryCommand` field).
pub(crate) fn role_from_wire(role: DirectorySpaceRole) -> SpaceRole {
    match role {
        DirectorySpaceRole::Author => SpaceRole::Author,
        DirectorySpaceRole::Spectator => SpaceRole::Spectator,
    }
}

/// 🔡️ Wire space-kind -> the exact lowercase string every backend's `CHECK (kind IN (...))`
/// constraint speaks; used only by each backend's `//#region 🔖️Projections`.
pub(crate) fn kind_to_str(kind: DirectorySpaceKind) -> &'static str {
    match kind {
        DirectorySpaceKind::Atelier => "atelier",
        DirectorySpaceKind::Studio => "studio",
        DirectorySpaceKind::Archive => "archive",
    }
}

/// 🔡️ Wire visibility -> the exact lowercase string every backend's `CHECK` constraint speaks.
pub(crate) fn visibility_to_str(visibility: DirectorySpaceVisibility) -> &'static str {
    match visibility {
        DirectorySpaceVisibility::Private => "private",
        DirectorySpaceVisibility::Public => "public",
    }
}
//#endregion 🔖️Wire

//#region 🔖️Events
fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

/// @emoji ⏱️ A hybrid logical clock: wall-clock milliseconds plus a same-millisecond tiebreak
/// counter, monotone across `tick()` calls regardless of how the OS clock jitters. One instance
/// lives behind `DirectoryService`'s write lock (see `//#region 🔖️Service`) — the lock is what
/// makes a `HubClock`'s stream of `tick()`s a total order across every command this hub instance
/// executes, which `append_events` then turns into a dense backend-assigned `seq`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HubClock {
    physical_ms: i64,
    logical: u32,
}

impl HubClock {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ⏭️ Advances the clock by one tick and returns its new `Hlc`. Same millisecond as the
    /// last tick ⇒ the logical counter increments; a later millisecond ⇒ it resets to 0. Time never
    /// runs backward even if the OS clock does (the physical component only ever holds or advances).
    pub fn tick(&mut self) -> Hlc {
        let observed = now_ms();
        if observed > self.physical_ms {
            self.physical_ms = observed;
            self.logical = 0;
        } else {
            self.logical += 1;
        }
        Hlc { physical_ms: self.physical_ms, logical: self.logical }
    }
}

/// @emoji ✉️ One decided-but-not-yet-persisted directory event: everything `decide` (see
/// `//#region 🔖️Decider`) can determine on its own. `append_events` (the `HubDirectory` trait,
/// implemented once per backend) turns this into a full `DirectoryEvent` by assigning the
/// backend-dense `seq`, minting a uuid v7 `id`, and stamping `recorded_at_ms`.
#[derive(Clone, Debug)]
pub struct NewDirectoryEvent {
    pub hlc: Hlc,
    pub actor: DirectoryActor,
    pub space_id: Option<String>,
    pub user_id: Option<String>,
    pub body: DirectoryEventBody,
}
//#endregion 🔖️Events

//#region 🔖️Decider
/// 🎁️ The extra payload a `decide`d command can carry alongside its events — today only
/// `create-invite` produces one (contract C1's `-> { inviteToken }`); the secret token itself is
/// never an event field (invites are not event-sourced, only their `invite.redeemed` outcome is),
/// so it has to travel back to the caller some other way than the event stream.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandResult {
    pub invite_token: Option<String>,
}

/// 🧾️ What `decide` computed for one `DirectoryCommand`: the events to persist (empty for the two
/// not-event-sourced invite commands, see `decide`'s own doc) plus an optional `CommandResult`.
#[derive(Clone, Debug)]
pub struct Decision {
    pub events: Vec<NewDirectoryEvent>,
    pub result: Option<CommandResult>,
}

fn new_event(clock: &mut HubClock, actor: &DirectoryActor, space_id: Option<String>, user_id: Option<String>, body: DirectoryEventBody) -> NewDirectoryEvent {
    NewDirectoryEvent { hlc: clock.tick(), actor: actor.clone(), space_id, user_id, body }
}

fn single(clock: &mut HubClock, actor: &DirectoryActor, space_id: Option<String>, user_id: Option<String>, body: DirectoryEventBody) -> Decision {
    Decision { events: vec![new_event(clock, actor, space_id, user_id, body)], result: None }
}

async fn require_space(dir: &HubDirectories, space_id: &str) -> DirectoryResult<SpaceRecord> {
    dir.get_space(space_id).await?.ok_or_else(|| DirectoryError::NotFound(format!("space '{space_id}' not found")))
}

/// 🎭️ Extracts the plain user id out of a `User`-kind actor's `user:{user_id}#{shell_session_id}`
/// grammar (contract-freeze.md §C0) — directory events denormalize the plain id, never the
/// per-tab/process actor string. `Admin`/`System` actors have no owning user, so `create-space`
/// (the only command that needs one) rejects them.
fn actor_user_id(actor: &DirectoryActor) -> DirectoryResult<&str> {
    match actor.kind {
        DirectoryActorKind::User => actor.id.strip_prefix("user:").and_then(|rest| rest.split('#').next()).ok_or_else(|| DirectoryError::Backend(format!("malformed user actor id '{}'", actor.id))),
        _ => Err(DirectoryError::Backend("create-space requires a user actor".into())),
    }
}

/// @emoji 🧠️ Decides what a `DirectoryCommand` means: reads projections through `dir` (`get_space`,
/// `list_members`, `get_user_by_email`) and returns the events that would record it — it never
/// writes to `dir`, with one deliberate exception: `create-invite`/`revoke-invite` are NOT
/// event-sourced (contract's decider laws — only an invite's `invite.redeemed` outcome is an
/// event), so for those two variants this function performs the (non-event) write itself, under
/// the same write-lock serialization `DirectoryService::execute` already holds while calling it.
///
/// Authorization is **not** this function's job — `bin.rs` (lane 1-B) checks whether `actor` may
/// issue `command` against the named space *before* calling this; `decide` trusts `actor` as given
/// and only enforces the contract's structural laws:
/// - `create-space`/`archive-space`/`upsert-member` derive/emit the atelier ⇒ ≤1-author and
///   archive ⇒ nobody-writes laws (`archive-space` emits `member.upserted{spectator}` for every
///   current author, then `space.archived`, one event per projection step).
/// - `remove-member` naming the space's own owner ⇒ `DirectoryError::Conflict` (never removable).
/// - Any command naming a missing/deleted space ⇒ `DirectoryError::NotFound`.
/// - `upsert-member` with an email that has no `UserRecord` yet emits `user.created` first, using
///   a freshly minted user id the following `member.upserted` also uses.
pub async fn decide(dir: &HubDirectories, actor: &DirectoryActor, command: DirectoryCommand, clock: &mut HubClock) -> DirectoryResult<Decision> {
    match command {
        DirectoryCommand::CreateSpace { name, space_kind, visibility } => {
            let owner_user_id = actor_user_id(actor)?.to_string();
            let space_id = time_ordered_id();
            let owner_role = if space_kind == DirectorySpaceKind::Archive { DirectorySpaceRole::Spectator } else { DirectorySpaceRole::Author };
            let events = vec![
                new_event(clock, actor, Some(space_id.clone()), Some(owner_user_id.clone()), DirectoryEventBody::SpaceCreated { space_id: space_id.clone(), name, space_kind, visibility, owner_user_id: owner_user_id.clone() }),
                new_event(clock, actor, Some(space_id.clone()), Some(owner_user_id.clone()), DirectoryEventBody::MemberUpserted { space_id, user_id: owner_user_id, role: owner_role }),
            ];
            Ok(Decision { events, result: None })
        }
        DirectoryCommand::RenameSpace { space_id, name } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceRenamed { space_id, name }))
        }
        DirectoryCommand::SetVisibility { space_id, visibility } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility }))
        }
        DirectoryCommand::ArchiveSpace { space_id } => {
            require_space(dir, &space_id).await?;
            let members = dir.list_members(&space_id).await?;
            let mut events: Vec<NewDirectoryEvent> = members
                .into_iter()
                .filter(|(_, role)| *role == SpaceRole::Author)
                .map(|(user, _)| new_event(clock, actor, Some(space_id.clone()), Some(user.id.clone()), DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: user.id, role: DirectorySpaceRole::Spectator }))
                .collect();
            events.push(new_event(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceArchived { space_id }));
            Ok(Decision { events, result: None })
        }
        DirectoryCommand::DeleteSpace { space_id } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceDeleted { space_id }))
        }
        DirectoryCommand::UpsertMember { space_id, email, role } => {
            let space = require_space(dir, &space_id).await?;
            let mut events = Vec::new();
            let user_id = match dir.get_user_by_email(&email).await? {
                Some(existing) => existing.id,
                None => {
                    let user_id = time_ordered_id();
                    let display_name = email.split('@').next().unwrap_or(&email).to_string();
                    events.push(new_event(clock, actor, None, Some(user_id.clone()), DirectoryEventBody::UserCreated { user_id: user_id.clone(), email: email.clone(), display_name }));
                    user_id
                }
            };
            if role == DirectorySpaceRole::Author {
                if space.kind == "archive" {
                    return Err(DirectoryError::Conflict(format!("space '{space_id}' is an archive; no author memberships are allowed")));
                }
                if space.kind == "atelier" {
                    let has_other_author = dir.list_members(&space_id).await?.into_iter().any(|(user, existing_role)| existing_role == SpaceRole::Author && user.id != user_id);
                    if has_other_author {
                        return Err(DirectoryError::Conflict(format!("space '{space_id}' is an atelier; it already has a distinct author")));
                    }
                }
            }
            events.push(new_event(clock, actor, Some(space_id.clone()), Some(user_id.clone()), DirectoryEventBody::MemberUpserted { space_id, user_id, role }));
            Ok(Decision { events, result: None })
        }
        DirectoryCommand::RemoveMember { space_id, user_id } => {
            let space = require_space(dir, &space_id).await?;
            if space.owner_user_id == user_id {
                return Err(DirectoryError::Conflict(format!("space '{space_id}' owner membership cannot be removed")));
            }
            Ok(single(clock, actor, Some(space_id.clone()), Some(user_id.clone()), DirectoryEventBody::MemberRemoved { space_id, user_id }))
        }
        DirectoryCommand::CreateInvite { space_id, role, ttl_secs } => {
            require_space(dir, &space_id).await?;
            let invite = dir.create_invite(&space_id, role_from_wire(role), ttl_secs as i64).await?;
            Ok(Decision { events: Vec::new(), result: Some(CommandResult { invite_token: Some(invite.token) }) })
        }
        DirectoryCommand::RevokeInvite { space_id, invite_id } => {
            require_space(dir, &space_id).await?;
            dir.revoke_invite(&invite_id).await?;
            Ok(Decision { events: Vec::new(), result: None })
        }
    }
}
//#endregion 🔖️Decider

//#region 🔖️Service
/// @emoji 🏭️ The hub's single directory writer. Every command is serialized behind one
/// `tokio::sync::Mutex<HubClock>` (dense, gap-free `seq` — two concurrent commands can never
/// interleave their `append_events` calls) and every persisted event (plus connection/presence
/// messages the caller publishes directly) fans out on one `broadcast` channel every
/// `/directory/ws` connection subscribes to (contract C2).
pub struct DirectoryService {
    dir: Arc<HubDirectories>,
    write: tokio::sync::Mutex<HubClock>,
    tx: tokio::sync::broadcast::Sender<DirectoryStreamMessage>,
}

impl DirectoryService {
    /// @emoji 🏗️ `channel_capacity` sizes the broadcast buffer; a subscriber that falls more than
    /// this many messages behind sees `RecvError::Lagged` and must resync via `events_since`
    /// (`?since=` replay, contract C2) — handled by `bin.rs`'s WS handler, not here.
    pub fn new(dir: Arc<HubDirectories>, channel_capacity: usize) -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel(channel_capacity);
        Self { dir, write: tokio::sync::Mutex::new(HubClock::new()), tx }
    }

    /// @emoji ⚙️ The command pipeline: take the write lock → `decide` → `dir.append_events` →
    /// publish each persisted event on `tx` → release the lock. Authorization already happened in
    /// the caller (`bin.rs`); this trusts `actor` as given.
    pub async fn execute(&self, actor: DirectoryActor, command: DirectoryCommand) -> DirectoryResult<(Vec<DirectoryEvent>, Option<CommandResult>)> {
        let mut clock = self.write.lock().await;
        let decision = decide(self.dir.as_ref(), &actor, command, &mut clock).await?;
        drop(clock);
        let persisted = if decision.events.is_empty() { Vec::new() } else { self.dir.append_events(&decision.events).await? };
        for event in &persisted {
            let _ = self.tx.send(DirectoryStreamMessage::Event { event: event.clone() });
        }
        Ok((persisted, decision.result))
    }

    /// @emoji 🎟️ Redeems a still-valid, unrevoked invite: resolves (or, same unknown-email law as
    /// `upsert-member`, creates) the redeeming user, then emits `invite.redeemed` — appended and
    /// published under the same write lock `execute` uses, so a redemption's `seq` never races a
    /// concurrent command. `POST /directory/invites/{token}/redeem` (contract C2) is not a
    /// `DirectoryCommand` (invites are not event-sourced themselves), so it calls this instead of
    /// `execute`.
    pub async fn redeem_invite(&self, actor: DirectoryActor, token: &str, email: &str, display_name: &str) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut clock = self.write.lock().await;
        let invite = self.dir.get_invite_by_token(token).await?.ok_or_else(|| DirectoryError::NotFound(format!("invite token '{token}' not found")))?;
        if invite.revoked_at.is_some() {
            return Err(DirectoryError::Conflict("invite already revoked".into()));
        }
        if invite.expires_at < now_ms() {
            return Err(DirectoryError::Conflict("invite expired".into()));
        }
        let mut events = Vec::new();
        let user_id = match self.dir.get_user_by_email(email).await? {
            Some(existing) => existing.id,
            None => {
                let user_id = time_ordered_id();
                events.push(new_event(&mut clock, &actor, None, Some(user_id.clone()), DirectoryEventBody::UserCreated { user_id: user_id.clone(), email: email.to_string(), display_name: display_name.to_string() }));
                user_id
            }
        };
        events.push(new_event(
            &mut clock,
            &actor,
            Some(invite.space_id.clone()),
            Some(user_id.clone()),
            DirectoryEventBody::InviteRedeemed { space_id: invite.space_id.clone(), user_id, invite_id: invite.id.clone(), role: role_to_wire(invite.role) },
        ));
        drop(clock);
        let persisted = self.dir.append_events(&events).await?;
        for event in &persisted {
            let _ = self.tx.send(DirectoryStreamMessage::Event { event: event.clone() });
        }
        Ok(persisted)
    }

    /// @emoji 📡️ A fresh receiver over every future published `DirectoryStreamMessage` (events,
    /// connection phases, presence, heartbeats) — `bin.rs`'s `/directory/ws` handler subscribes
    /// once per connection, then replays `events_since(?since=)` before switching to live receive
    /// (contract C2's "subscribe, then replay, gap-free").
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<DirectoryStreamMessage> {
        self.tx.subscribe()
    }

    /// @emoji 📣️ Publishes a non-event stream message (connection open/close, presence roster,
    /// heartbeat) — emitted by the connection/presence layer (`bin.rs`, lane 1-B), not by this
    /// crate's own event pipeline.
    pub fn publish(&self, message: DirectoryStreamMessage) {
        let _ = self.tx.send(message);
    }
}
//#endregion 🔖️Service

//#region 🔖️Trait
/// @emoji 🗄️ Backend-agnostic os-hub identity/tenancy directory. Implemented once per backend
/// (sqlite/postgres/neo4j); `HubState` holds an `Arc<HubDirectories>` (see `//#region 🔖️Dispatch`
/// below) so the directory backend is a deploy-time choice, not a compile-time one — independent of
/// `db::Database`'s own storage backend choice (see `bin.rs`, `OS_HUB_DIRECTORY_BACKEND` vs
/// `OS_HUB_STORAGE_BACKEND`).
pub trait HubDirectory: Send + Sync + 'static {
    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String>;
    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> DirectoryResult<bool>;
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord>;
    /// @emoji 🔎️ Single-user lookup by id — the `member.upserted`/`invite.redeemed` projections
    /// resolve `MemberView.email`/`display_name` through this, not through `get_user_by_email`.
    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>>;
    //#endregion

    //#region Spaces
    /// @emoji 🔎️ Single-space lookup by id — used by the hub handler to read `kind`/`visibility`
    /// (grant compilation, public-visibility fallback) without listing every space. Also `decide`'s
    /// (`//#region 🔖️Decider`) own "does this space exist" read.
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>>;
    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>>;
    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>>;
    /// @emoji 🧑️‍🤝️‍🧑️ The current member roster — `decide` reads this to enforce the atelier/
    /// archive laws and to compute `archive-space`'s demote-every-author events.
    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>>;
    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>>;
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> DirectoryResult<AuthSessionRecord>;
    async fn get_auth_session(&self, id: &str) -> DirectoryResult<Option<AuthSessionRecord>>;
    async fn revoke_auth_session(&self, id: &str) -> DirectoryResult<()>;
    //#endregion

    //#region Invites
    // 🎟️ Not event-sourced (contract's decider laws) — only redemption is (`invite.redeemed`, see
    // `DirectoryService::redeem_invite`). `create_invite`/`revoke_invite` are called directly by
    // `decide` as its one documented write exception (`//#region 🔖️Decider`).
    async fn create_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64) -> DirectoryResult<InviteRecord>;
    async fn get_invite_by_token(&self, token: &str) -> DirectoryResult<Option<InviteRecord>>;
    async fn revoke_invite(&self, invite_id: &str) -> DirectoryResult<()>;
    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>>;
    //#endregion

    //#region SyncSessions
    /// @emoji 🔴️ Widened over the pre-ticket signature with `space_id`/`surface` (contract's
    /// presence scope is `(space_id, document_id, surface)`).
    async fn record_sync_session_open(&self, space_id: &str, document_id: &str, surface: &str, user_id: Option<&str>, space_role: Option<SpaceRole>, client_label: &str) -> DirectoryResult<SyncSessionRecord>;
    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()>;
    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>>;
    /// @emoji 🟢️ Every still-open session, optionally scoped to one space — the admin connections
    /// view and the per-space presence roster both read this instead of iterating documents.
    async fn list_active_sync_sessions(&self, space_id: Option<&str>) -> DirectoryResult<Vec<SyncSessionRecord>>;
    /// @emoji 🧹️ Marks every still-open session closed — called once at hub boot, before any real
    /// connection lands, to clear crash residue from the previous process (a session that never got
    /// its `disconnected_at` because the hub was killed mid-connection).
    async fn close_all_sync_sessions(&self) -> DirectoryResult<()>;
    //#endregion

    //#region EventLog
    /// @emoji ➕️ Persists `events` in one backend transaction: each gets a dense backend-assigned
    /// `seq` (contiguous with the current head, no gaps even under concurrent callers — callers are
    /// expected to already be serialized by `DirectoryService`'s write lock, but a backend MUST NOT
    /// rely on that alone for `seq` density since `rebuild_projections`/tests may call this
    /// directly), a minted uuid v7 `id`, and `recorded_at_ms`; then applies each event's projection
    /// (`//#region 🔖️Projections`) in the same transaction before committing.
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>>;
    /// @emoji 📜️ Every event with `seq > since_seq`, ascending, capped at `limit` — backs both
    /// `GET /directory/events?since=` and `/directory/ws`'s post-subscribe replay (contract C2).
    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>>;
    /// @emoji 🔝️ The current log length (0 when empty) — `DirectoryStreamMessage::Heartbeat`'s
    /// `head_seq` and the admin overview's `headSeq` both read this.
    async fn head_seq(&self) -> DirectoryResult<u64>;
    /// @emoji 🔁️ Truncates every projection table and replays the whole log from `seq` 1 through
    /// each event's projection (`//#region 🔖️Projections`) — returns the number of events replayed
    /// (which must equal `head_seq()` afterward). `POST /admin/api/directory/rebuild` (contract C2).
    async fn rebuild_projections(&self) -> DirectoryResult<u64>;
    //#endregion
}
//#endregion 🔖️Trait

//#region 🔖️Backends
// 🧭️ Top-level `mod` declarations in a real (file-backed, non-inline) module resolve `#[path]`
// relative to THIS file's own directory (🌎️hub/📇️directory/) — no cumulative/leaf-prefixed math
// needed here, that convention is for paths declared inside an entry file's inline nested `mod`
// blocks (see `rustEntryPathRules` in 🔣️taxonomy.json and `bin.rs`'s own `mod directory` line).
#[cfg(feature = "sqlite")]
#[path = "🪶️sqlite/🦀️component.rs"]
pub mod sqlite;

#[cfg(feature = "postgres")]
#[path = "🐘️postgres/🦀️component.rs"]
pub mod postgres;

#[cfg(feature = "neo4j")]
#[path = "🌐️neo4j/🦀️component.rs"]
pub mod neo4j;
//#endregion 🔖️Backends

//#region 🔖️Dispatch
/// 🗄️ Closed-set dispatch enum over every `HubDirectory` backend this crate can compile in.
/// Hand-written, not `dyn_enum_close!`-generated: the macro's DSL has no per-variant `#[cfg]`
/// support (verified against its parser — `DynEnumVariant::parse` never calls
/// `Attribute::parse_outer`), and each variant here is gated on its own Cargo feature — see
/// `📓️terra-dedyn-fw-hub-repo-report.md`. The shape (one `From<Backend>` impl per variant plus a
/// match-delegating `impl HubDirectory`) is otherwise identical to what the macro emits for every
/// other closed-set family in this program (R11). `bin.rs`'s `connect_directory` builds exactly
/// one variant at process startup, chosen by `OS_HUB_DIRECTORY_BACKEND`.
pub enum HubDirectories {
    #[cfg(feature = "sqlite")]
    Sqlite(sqlite::SqliteDirectory),
    #[cfg(feature = "postgres")]
    Postgres(postgres::PostgresDirectory),
    #[cfg(feature = "neo4j")]
    Neo4j(neo4j::Neo4jDirectory),
}

#[cfg(feature = "sqlite")]
impl ::core::convert::From<sqlite::SqliteDirectory> for HubDirectories {
    fn from(value: sqlite::SqliteDirectory) -> Self {
        Self::Sqlite(value)
    }
}

#[cfg(feature = "postgres")]
impl ::core::convert::From<postgres::PostgresDirectory> for HubDirectories {
    fn from(value: postgres::PostgresDirectory) -> Self {
        Self::Postgres(value)
    }
}

#[cfg(feature = "neo4j")]
impl ::core::convert::From<neo4j::Neo4jDirectory> for HubDirectories {
    fn from(value: neo4j::Neo4jDirectory) -> Self {
        Self::Neo4j(value)
    }
}

impl HubDirectory for HubDirectories {
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.create_share_token(document_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.create_share_token(document_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.create_share_token(document_id).await,
        }
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> DirectoryResult<bool> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.authorized_by_token(document_id, token).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.authorized_by_token(document_id, token).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.authorized_by_token(document_id, token).await,
        }
    }

    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
        }
    }

    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_user(user_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_user(user_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_user(user_id).await,
        }
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_user_by_email(email).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_user_by_email(email).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_user_by_email(email).await,
        }
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_user_by_sso_subject(provider, subject).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_user_by_sso_subject(provider, subject).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_user_by_sso_subject(provider, subject).await,
        }
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_users(limit, offset).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_users(limit, offset).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_users(limit, offset).await,
        }
    }

    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_space(space_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_space(space_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_space(space_id).await,
        }
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_spaces_for_user(user_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_spaces_for_user(user_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_spaces_for_user(user_id).await,
        }
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_spaces(limit, offset).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_spaces(limit, offset).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_spaces(limit, offset).await,
        }
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_members(space_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_members(space_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_members(space_id).await,
        }
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_role(space_id, user_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_role(space_id, user_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_role(space_id, user_id).await,
        }
    }

    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> DirectoryResult<AuthSessionRecord> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.create_auth_session(user_id, ttl_secs, sso_provider).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.create_auth_session(user_id, ttl_secs, sso_provider).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.create_auth_session(user_id, ttl_secs, sso_provider).await,
        }
    }

    async fn get_auth_session(&self, id: &str) -> DirectoryResult<Option<AuthSessionRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_auth_session(id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_auth_session(id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_auth_session(id).await,
        }
    }

    async fn revoke_auth_session(&self, id: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_auth_session(id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_auth_session(id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_auth_session(id).await,
        }
    }

    async fn create_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64) -> DirectoryResult<InviteRecord> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.create_invite(space_id, role, ttl_secs).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.create_invite(space_id, role, ttl_secs).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.create_invite(space_id, role, ttl_secs).await,
        }
    }

    async fn get_invite_by_token(&self, token: &str) -> DirectoryResult<Option<InviteRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.get_invite_by_token(token).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.get_invite_by_token(token).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.get_invite_by_token(token).await,
        }
    }

    async fn revoke_invite(&self, invite_id: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.revoke_invite(invite_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.revoke_invite(invite_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.revoke_invite(invite_id).await,
        }
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_invites(space_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_invites(space_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_invites(space_id).await,
        }
    }

    async fn record_sync_session_open(&self, space_id: &str, document_id: &str, surface: &str, user_id: Option<&str>, space_role: Option<SpaceRole>, client_label: &str) -> DirectoryResult<SyncSessionRecord> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.record_sync_session_open(space_id, document_id, surface, user_id, space_role, client_label).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.record_sync_session_open(space_id, document_id, surface, user_id, space_role, client_label).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.record_sync_session_open(space_id, document_id, surface, user_id, space_role, client_label).await,
        }
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.record_sync_session_close(sync_session_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.record_sync_session_close(sync_session_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.record_sync_session_close(sync_session_id).await,
        }
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_sync_sessions_for_document(document_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_sync_sessions_for_document(document_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_sync_sessions_for_document(document_id).await,
        }
    }

    async fn list_active_sync_sessions(&self, space_id: Option<&str>) -> DirectoryResult<Vec<SyncSessionRecord>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.list_active_sync_sessions(space_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.list_active_sync_sessions(space_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.list_active_sync_sessions(space_id).await,
        }
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.close_all_sync_sessions().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.close_all_sync_sessions().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.close_all_sync_sessions().await,
        }
    }

    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.append_events(events).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.append_events(events).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.append_events(events).await,
        }
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.events_since(since_seq, limit).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.events_since(since_seq, limit).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.events_since(since_seq, limit).await,
        }
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.head_seq().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.head_seq().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.head_seq().await,
        }
    }

    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        match self {
            #[cfg(feature = "sqlite")]
            Self::Sqlite(inner) => inner.rebuild_projections().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(inner) => inner.rebuild_projections().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(inner) => inner.rebuild_projections().await,
        }
    }
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
// 🔬️ Exercises `decide`/`DirectoryService` against the sqlite backend (the only one the default
// feature set — Amendment 2 — actually compiles/runs here); postgres/neo4j get the same coverage
// via their own `#[cfg(test)]` modules once `🛢️db`'s optional-dependency gap is fixed (not this
// lane's to fix, see the lane report).
#[cfg(all(test, feature = "sqlite"))]
mod tests {
    use super::sqlite::SqliteDirectory;
    use super::*;
    use std::sync::Arc;

    fn user_actor(user_id: &str) -> DirectoryActor {
        DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{user_id}#s1") }
    }

    // 🌱️ Every test in this module creates a space owned by `user_actor("u-owner")`; `decide`'s
    // `CreateSpace` arm (correctly) never mints the owner's `hub_user` row itself — it only has an
    // actor id, no email, so it cannot self-heal a missing user the way `UpsertMember` can. In
    // production the owner's `hub_user` row always predates `create-space` (an auth session is
    // required to call any directory command, and `POST /auth/sessions` creates the user first via
    // `HubDirectory::create_user`/`get_user_by_email`). This fixture reproduces that precondition by
    // appending a bare `user.created` event for `"u-owner"` under a `System` actor, mirroring
    // `SqliteDirectory::seed`'s own pattern one file over.
    async fn fresh_dir() -> Arc<HubDirectories> {
        let dir = SqliteDirectory::connect(":memory:").await.expect("connect");
        let seed_actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:test-seed".into() };
        let mut clock = HubClock::new();
        let events = vec![new_event(&mut clock, &seed_actor, None, Some("u-owner".into()), DirectoryEventBody::UserCreated { user_id: "u-owner".into(), email: "u-owner@example.com".into(), display_name: "Owner".into() })];
        dir.append_events(&events).await.expect("seed owner user");
        Arc::new(HubDirectories::from(dir))
    }

    async fn create_space(service: &DirectoryService, owner: &DirectoryActor, kind: DirectorySpaceKind) -> String {
        let (events, _) = service.execute(owner.clone(), DirectoryCommand::CreateSpace { name: "Space".into(), space_kind: kind, visibility: DirectorySpaceVisibility::Private }).await.expect("create-space");
        events[0].space_id.clone().expect("space id on space.created")
    }

    // 🔬️ Replaying the whole log through `rebuild_projections` reproduces the exact same
    // projections a live command stream built, and `events_since(0)` is dense `1..=head_seq()`.
    #[tokio::test]
    async fn event_log_replay_matches_projections() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 64);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "member@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
        service.execute(owner.clone(), DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Renamed".into() }).await.expect("rename-space");

        let head = dir.head_seq().await.expect("head seq");
        let seqs: Vec<u64> = dir.events_since(0, 100).await.expect("events since").iter().map(|event| event.seq).collect();
        assert_eq!(seqs, (1..=head).collect::<Vec<_>>(), "seq is dense 1..n");

        let spaces_before = dir.list_spaces(100, 0).await.expect("list spaces");
        let members_before = dir.list_members(&space_id).await.expect("list members");
        let users_before = dir.list_users(100, 0).await.expect("list users");

        let replayed = dir.rebuild_projections().await.expect("rebuild");
        assert_eq!(replayed, head);
        assert_eq!(dir.list_spaces(100, 0).await.expect("list spaces"), spaces_before);
        assert_eq!(dir.list_members(&space_id).await.expect("list members"), members_before);
        assert_eq!(dir.list_users(100, 0).await.expect("list users"), users_before);
    }

    // 🔬️ Decider law: an atelier rejects a second, distinct author (re-upserting the sole existing
    // author is not exercised here — that is the sqlite backend's own membership round-trip test).
    #[tokio::test]
    async fn atelier_rejects_a_second_distinct_author() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Atelier).await;
        let err = service.execute(owner, DirectoryCommand::UpsertMember { space_id, email: "other@example.com".into(), role: DirectorySpaceRole::Author }).await.unwrap_err();
        assert!(matches!(err, DirectoryError::Conflict(_)));
    }

    // 🔬️ Decider law: `archive-space` first demotes every current author to spectator, then
    // archives — nobody is left an author of a frozen space.
    #[tokio::test]
    async fn archive_space_demotes_every_author_to_spectator() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "second@example.com".into(), role: DirectorySpaceRole::Author }).await.expect("second author");
        service.execute(owner.clone(), DirectoryCommand::ArchiveSpace { space_id: space_id.clone() }).await.expect("archive-space");
        let members = dir.list_members(&space_id).await.expect("list members");
        assert_eq!(members.len(), 2);
        assert!(members.iter().all(|(_, role)| *role == SpaceRole::Spectator));
        assert_eq!(dir.get_space(&space_id).await.expect("get space").expect("space exists").kind, "archive");
    }

    // 🔬️ Decider law: the owner's membership can never be removed via `remove-member`.
    #[tokio::test]
    async fn owner_membership_can_never_be_removed() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        let err = service.execute(owner, DirectoryCommand::RemoveMember { space_id, user_id: "u-owner".into() }).await.unwrap_err();
        assert!(matches!(err, DirectoryError::Conflict(_)));
    }

    // 🔬️ Decider law: any command naming a deleted (or otherwise missing) space is `NotFound`.
    #[tokio::test]
    async fn command_naming_a_deleted_space_is_not_found() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        service.execute(owner.clone(), DirectoryCommand::DeleteSpace { space_id: space_id.clone() }).await.expect("delete-space");
        let err = service.execute(owner, DirectoryCommand::RenameSpace { space_id, name: "Nope".into() }).await.unwrap_err();
        assert!(matches!(err, DirectoryError::NotFound(_)));
    }

    // 🔬️ Decider law: `upsert-member` with an email that has no `UserRecord` yet emits
    // `user.created` before `member.upserted`, both under one decision.
    #[tokio::test]
    async fn upsert_member_with_unknown_email_creates_the_user_first() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir, 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
        let (events, _) = service.execute(owner, DirectoryCommand::UpsertMember { space_id, email: "new@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
        assert_eq!(events.len(), 2);
        assert!(matches!(events[0].body, DirectoryEventBody::UserCreated { .. }));
        assert!(matches!(events[1].body, DirectoryEventBody::MemberUpserted { .. }));
    }

    // 🔬️ Invite create -> redeem -> revoke round-trip: redemption grants membership and emits
    // `invite.redeemed`; a revoked invite still round-trips its `revoked_at`.
    #[tokio::test]
    async fn invite_create_redeem_revoke_round_trip() {
        let dir = fresh_dir().await;
        let service = DirectoryService::new(dir.clone(), 16);
        let owner = user_actor("u-owner");
        let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;

        let (_, result) = service.execute(owner.clone(), DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3600 }).await.expect("create-invite");
        let token = result.expect("command result").invite_token.expect("invite token");

        let redeemed = service.redeem_invite(user_actor("u-invited"), &token, "invited@example.com", "Invited").await.expect("redeem");
        assert!(matches!(redeemed.last().expect("at least one event").body, DirectoryEventBody::InviteRedeemed { .. }));
        let members = dir.list_members(&space_id).await.expect("list members");
        assert!(members.iter().any(|(user, role)| user.email == "invited@example.com" && *role == SpaceRole::Spectator));

        let invites = dir.list_invites(&space_id).await.expect("list invites");
        assert_eq!(invites.len(), 1);
        dir.revoke_invite(&invites[0].id).await.expect("revoke");
        let fetched = dir.get_invite_by_token(&invites[0].token).await.expect("get by token").expect("invite still exists");
        assert!(fetched.revoked_at.is_some());
    }
}
//#endregion 🧪️Tests
