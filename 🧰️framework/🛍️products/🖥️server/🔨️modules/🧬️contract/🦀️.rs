//! 🧬️ Server contract: every type two parties must agree on without sharing a runtime.
//!
//! Three layers stay strictly separate and are never collapsed into one "message" abstraction:
//! the **CQRS dual bus** ([`CommandEnvelope`]/[`QueryEnvelope`]) carries application intent and
//! projection reads; the **actor turn protocol** ([`ActorKey`], [`Decision`](crate::authority::Decision)) is the consistency
//! boundary a command is serialized through; the **replication protocol** (`protocol` crate) moves
//! causal state between replicas. A UI action is none of the three and never reaches this crate.
//!
//! Pure data only — no axum, no storage driver, no clock. The optimistic client replica and the
//! authority both link this crate and rerun the same deciders against these types.

use protocol::causal::FrontierSummary;
use serde::{Deserialize, Serialize};

//#region 🔖️Identity
/// 🏢️ The instance-wide tenancy root. Distinct from a space: a tenant owns spaces, billing
/// and membership; a space scopes documents. Hub aliased the two, this contract does not.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TenantId(pub String);

/// 🗂️ A scope inside a tenant — the project/space a command or query is addressed within.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Scope(pub String);

/// 🎭️ The durable address of one authority actor: the serialized consistency boundary a
/// command is executed inside. `kind` selects the registered actor implementation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ActorKey {
    pub tenant: TenantId,
    pub kind: String,
    pub id: String,
}

/// 🙋️ Who is acting. A principal is never a role — roles are policy templates evaluated
/// against a principal, never an enum branched on inside a handler.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Principal {
    User { id: String },
    ServiceAccount { id: String },
    Device { id: String },
    Anonymous,
}

/// 🎫️ One authenticated session of a principal on one device.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

/// 📱️ A device a session runs on; an offline outbox belongs to exactly one.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);
//#endregion 🔖️Identity

//#region 🔖️Command
/// 🆔️ Client-minted identity of one command submission; stable across retries.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(pub String);

/// 🔁️ Deduplication key. Two submissions carrying the same key must produce one effect and
/// the same receipt, however many times the client retries after a timeout.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdempotencyKey(pub String);

/// 🔢️ An actor's monotonically increasing revision, used for optimistic concurrency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Revision(pub u64);

/// 🕰️ Client hybrid-logical clock reading, carried so the authority can order concurrent
/// submissions without trusting wall clocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct HybridLogicalClock {
    pub millis: u64,
    pub counter: u32,
}

/// 🔍️ Distributed-trace correlation for one submission.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceContext {
    pub trace_id: String,
    pub span_id: String,
}

/// 🛂️ A capability the caller presents to justify an action policy would otherwise deny —
/// e.g. a share token granting one document to an otherwise anonymous principal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityProof(pub String);

/// 📨️ One durable intent addressed to one authority actor. The payload stays opaque: this
/// contract never parses a domain command, it only routes, deduplicates and authorizes it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandEnvelope {
    pub command_id: CommandId,
    pub kind: String,
    pub version: u32,
    pub target: ActorKey,
    pub scope: Scope,
    pub principal: Principal,
    pub session: Option<SessionId>,
    pub device: Option<DeviceId>,
    pub payload: Vec<u8>,
    pub causal_frontier: Option<FrontierSummary>,
    pub client_hlc: HybridLogicalClock,
    pub expected_revision: Option<Revision>,
    pub idempotency_key: Option<IdempotencyKey>,
    pub capability_proof: Option<CapabilityProof>,
    pub trace: TraceContext,
}

/// 🧾️ Proof the authority processed a command, returned identically on every retry of the
/// same [`IdempotencyKey`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandReceipt {
    pub command_id: CommandId,
    pub actor: ActorKey,
    pub revision: Revision,
    pub accepted_at: HybridLogicalClock,
}

/// 🚫️ Why an authority refused a command. Never a panic, never a bare string at the edge.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Rejection {
    Unauthorized { detail: String },
    RevisionConflict { expected: Revision, actual: Revision },
    Invalid { detail: String },
    UnknownCommandKind { command_kind: String },
    ActorUnavailable { detail: String },
}

/// 💬️ A human-facing note attached to an outcome (validation warning, coercion notice).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notice {
    pub code: String,
    pub message: String,
}

/// 🧵️ A long-running workflow a command started; the caller polls or subscribes for it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessId(pub String);

/// 🎯️ What the authority decided. `Transformed` is the collaborative case: the command was
/// accepted but rebased, so the client must roll back its speculative apply and take the canonical
/// events instead.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum CommandOutcome {
    Accepted { receipt: CommandReceipt, events: Vec<EventRecord>, frontier: Option<FrontierSummary> },
    Transformed { receipt: CommandReceipt, canonical_events: Vec<EventRecord>, frontier: Option<FrontierSummary>, notices: Vec<Notice> },
    Rejected { receipt: CommandReceipt, reason: Rejection, notices: Vec<Notice> },
    Pending { receipt: CommandReceipt, process: ProcessId },
}

/// 📴️ What a command kind is allowed to do while the replica is detached. Declared per kind
/// so "local-first" can never be read as "membership and permissions may be decided offline".
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OfflinePolicy {
    /// Applied speculatively at once; the authority may later transform or reject it.
    Optimistic,
    /// Queued durably in the outbox, never applied locally, submitted on reattach.
    Deferred,
    /// Refused outright while detached — the authority alone may decide it.
    AuthorityRequired,
}
//#endregion 🔖️Command

//#region 🔖️Query
/// 🆔️ Identity of one query submission.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueryId(pub String);

/// 📑️ Opaque continuation token for a paged or subscribed read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryCursor(pub String);

/// 🧭️ How fresh an answer must be. Client-facing semantics — the engine's own richer modes
/// are an implementation detail translated at the adapter boundary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum QueryConsistency {
    /// Answer from the local replica/projection as-is; never waits.
    Local,
    /// Do not answer until the projection reflects this frontier (read-your-writes).
    AtFrontier { frontier: FrontierSummary },
    /// Answer from the current authority state.
    Authority,
}

/// ❓️ One read addressed at a projection, never at an actor's private state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryEnvelope {
    pub query_id: QueryId,
    pub kind: String,
    pub version: u32,
    pub scope: Scope,
    pub principal: Principal,
    pub arguments: Vec<u8>,
    pub consistency: QueryConsistency,
    pub cursor: Option<QueryCursor>,
}

/// 📤️ What a query returns: a whole value, one page, or a live subscription handle.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum QueryResult {
    Snapshot { value: Vec<u8>, frontier: Option<FrontierSummary> },
    Page { items: Vec<Vec<u8>>, next: Option<QueryCursor>, frontier: Option<FrontierSummary> },
    Subscription { subscription: SubscriptionId, initial: Vec<u8>, cursor: Option<QueryCursor>, frontier: Option<FrontierSummary> },
}

/// 🔔️ Identity of an established live projection subscription.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubscriptionId(pub String);
//#endregion 🔖️Query

//#region 🔖️Lanes
/// 📚️ One durable, replayable fact an actor emitted. Persisted, sequenced, causally tracked
/// and authorized on delivery.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRecord {
    pub stream: ActorKey,
    pub seq: u64,
    pub hlc: HybridLogicalClock,
    pub kind: String,
    pub payload: Vec<u8>,
}

/// 💨️ One lossy, expiring frame — cursors, selections, previews, typing, connection quality.
/// A separate type from [`EventRecord`] on purpose: an ephemeral frame is never replayed into
/// durable state and is rate-limited and authorized separately.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralFrame {
    pub scope: Scope,
    pub principal: Principal,
    pub kind: String,
    pub payload: Vec<u8>,
}
//#endregion 🔖️Lanes

//#region 🔖️Presence
/// @emoji 🧩️ An opaque JSON value an instance defines and the gateway never interprets — the state
/// one presence session shares. Reexported so an instance names it without naming the JSON library.
pub use serde_json::Value as OpaqueJson;

/// @emoji 🧍️ One session of a presence room as every member sees it: the server-generated session
/// id, the palette slot it leases, the surface it joined from and its latest shared state (`null`
/// until it sent one).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceEntry {
    pub session: String,
    pub colour: u8,
    pub surface: String,
    pub state: OpaqueJson,
}

/// @emoji 👥️ One text frame of the presence socket (`semio.presence.v1`). `welcome` (the joining
/// session, its colour and the whole roster, itself included) and a per-tick `batch` (the sessions
/// whose state changed since the last tick, and those that left) travel server → client; `state`
/// (the sender's latest state, latest wins) travels client → server; `refused` answers a frame the
/// server dropped while the socket stays open. A second `welcome` on the same socket replaces the
/// roster wholesale — the resynchronization a lagging subscriber receives.
///
/// Watching is read-only presence in other rooms: `watch` (client → server) replaces the set of
/// scopes the socket watches and the interval it wants their changes at (an empty set stops
/// watching); `watched` (server → client) carries one watched scope's changes since the last
/// interval, or — flagged `snapshot` — its whole roster, sent when the scope becomes watched and
/// whenever the watcher fell behind, replacing what the client held for that scope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PresenceFrame {
    Welcome { session: String, colour: u8, roster: Vec<PresenceEntry> },
    State { state: OpaqueJson },
    Batch { entries: Vec<PresenceEntry>, left: Vec<String> },
    Refused { reason: String },
    Watch {
        scopes: Vec<Scope>,
        #[serde(rename = "intervalMs")]
        interval_ms: u64,
    },
    Watched {
        scope: Scope,
        entries: Vec<PresenceEntry>,
        left: Vec<String>,
        #[serde(default, skip_serializing_if = "is_false")]
        snapshot: bool,
    },
}

/// 🏳️ Whether an optional flag is unset, so the wire leaves it out.
fn is_false(flag: &bool) -> bool {
    !*flag
}
//#endregion 🔖️Presence

//#region 🔖️Policy
/// 🚦️ Every point authorization is evaluated at. Hiding a route is user experience; these
/// are where access is actually decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PolicyPoint {
    CommandAdmission,
    CommandExecution,
    QueryAccess,
    Subscription,
    EventDelivery,
    BlobRead,
    BlobWrite,
    Effect,
    Administration,
}

/// ⚖️ The result of one policy evaluation. Deny always wins over allow.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum PolicyDecision {
    Allow,
    Deny { reason: String },
}

impl PolicyDecision {
    /// 🕳️ Whether this decision permits the action.
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allow)
    }
}
//#endregion 🔖️Policy

//#region 🔖️Module
/// 📇️ What one command kind declares to the instance that registers it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandDescriptor {
    pub kind: String,
    pub version: u32,
    pub actor_kind: String,
    pub offline: OfflinePolicy,
}

/// 📇️ What one query kind declares.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDescriptor {
    pub kind: String,
    pub version: u32,
    pub projection: String,
}

/// 🎓️ A named bundle of grants. `admin`/`manager`/`editor`/`viewer` are values of this type,
/// never hard-coded enums inside handlers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyTemplate {
    pub name: String,
    #[serde(default)]
    pub auto_apply: bool,
    pub grants: Vec<PolicyGrant>,
}

/// 🔑️ One grant inside a template: an action on a resource pattern at a policy point.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyGrant {
    pub point: PolicyPoint,
    pub resource: String,
    pub action: String,
}

/// 🧾️ The declarative half of a server module — what it contributes, with no runtime types,
/// so an instance definition can be inspected, diffed and served without constructing a server.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleManifest {
    pub id: String,
    pub commands: Vec<CommandDescriptor>,
    pub queries: Vec<QueryDescriptor>,
    pub projections: Vec<String>,
    pub policies: Vec<PolicyTemplate>,
    pub actor_kinds: Vec<String>,
}

/// 🏛️ One deployable server: its identity plus the modules composing it. Hub and Zentrale
/// are two values of this type, not two forks of a server.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInstanceDefinition {
    pub id: String,
    pub version: String,
    pub modules: Vec<ModuleManifest>,
}

impl ServerInstanceDefinition {
    /// 🔎️ The manifest that declares `kind`, if any module does.
    pub fn command(&self, kind: &str) -> Option<&CommandDescriptor> {
        self.modules.iter().flat_map(|module| &module.commands).find(|command| command.kind == kind)
    }

    /// 📴️ The offline policy declared for `kind`; unknown kinds are authority-only by default.
    pub fn offline_policy(&self, kind: &str) -> OfflinePolicy {
        self.command(kind).map_or(OfflinePolicy::AuthorityRequired, |command| command.offline)
    }
}
//#endregion 🔖️Module

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
