//! 🧬️ Server contract: every type two parties must agree on without sharing a runtime.
//!
//! Three layers stay strictly separate and are never collapsed into one "message" abstraction:
//! the **CQRS dual bus** ([`CommandEnvelope`]/[`QueryEnvelope`]) carries application intent and
//! projection reads; the **actor turn protocol** ([`ActorKey`], [`Decision`]) is the consistency
//! boundary a command is serialized through; the **replication protocol** (`protocol` crate) moves
//! causal state between replicas. A UI action is none of the three and never reaches this crate.
//!
//! Pure data only — no axum, no storage driver, no clock. The optimistic client replica and the
//! authority both link this crate and rerun the same deciders against these types.

use protocol::causal::FrontierSummary;
use serde::{Deserialize, Serialize};

//#region 🔖️Identity
/// @emoji 🏢️ The instance-wide tenancy root. Distinct from a space: a tenant owns spaces, billing
/// and membership; a space scopes documents. Hub aliased the two, this contract does not.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TenantId(pub String);

/// @emoji 🗂️ A scope inside a tenant — the project/space a command or query is addressed within.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Scope(pub String);

/// @emoji 🎭️ The durable address of one authority actor: the serialized consistency boundary a
/// command is executed inside. `kind` selects the registered actor implementation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ActorKey {
    pub tenant: TenantId,
    pub kind: String,
    pub id: String,
}

/// @emoji 🙋️ Who is acting. A principal is never a role — roles are policy templates evaluated
/// against a principal, never an enum branched on inside a handler.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Principal {
    User { id: String },
    ServiceAccount { id: String },
    Device { id: String },
    Anonymous,
}

/// @emoji 🎫️ One authenticated session of a principal on one device.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

/// @emoji 📱️ A device a session runs on; an offline outbox belongs to exactly one.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);
//#endregion 🔖️Identity

//#region 🔖️Command
/// @emoji 🆔️ Client-minted identity of one command submission; stable across retries.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(pub String);

/// @emoji 🔁️ Deduplication key. Two submissions carrying the same key must produce one effect and
/// the same receipt, however many times the client retries after a timeout.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdempotencyKey(pub String);

/// @emoji 🔢️ An actor's monotonically increasing revision, used for optimistic concurrency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Revision(pub u64);

/// @emoji 🕰️ Client hybrid-logical clock reading, carried so the authority can order concurrent
/// submissions without trusting wall clocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct HybridLogicalClock {
    pub millis: u64,
    pub counter: u32,
}

/// @emoji 🔍️ Distributed-trace correlation for one submission.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceContext {
    pub trace_id: String,
    pub span_id: String,
}

/// @emoji 🛂️ A capability the caller presents to justify an action policy would otherwise deny —
/// e.g. a share token granting one document to an otherwise anonymous principal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityProof(pub String);

/// @emoji 📨️ One durable intent addressed to one authority actor. The payload stays opaque: this
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

/// @emoji 🧾️ Proof the authority processed a command, returned identically on every retry of the
/// same [`IdempotencyKey`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandReceipt {
    pub command_id: CommandId,
    pub actor: ActorKey,
    pub revision: Revision,
    pub accepted_at: HybridLogicalClock,
}

/// @emoji 🚫️ Why an authority refused a command. Never a panic, never a bare string at the edge.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Rejection {
    Unauthorized { detail: String },
    RevisionConflict { expected: Revision, actual: Revision },
    Invalid { detail: String },
    UnknownCommandKind { command_kind: String },
    ActorUnavailable { detail: String },
}

/// @emoji 💬️ A human-facing note attached to an outcome (validation warning, coercion notice).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notice {
    pub code: String,
    pub message: String,
}

/// @emoji 🧵️ A long-running workflow a command started; the caller polls or subscribes for it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessId(pub String);

/// @emoji 🎯️ What the authority decided. `Transformed` is the collaborative case: the command was
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

/// @emoji 📴️ What a command kind is allowed to do while the replica is detached. Declared per kind
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
/// @emoji 🆔️ Identity of one query submission.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueryId(pub String);

/// @emoji 📑️ Opaque continuation token for a paged or subscribed read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryCursor(pub String);

/// @emoji 🧭️ How fresh an answer must be. Client-facing semantics — the engine's own richer modes
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

/// @emoji ❓️ One read addressed at a projection, never at an actor's private state.
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

/// @emoji 📤️ What a query returns: a whole value, one page, or a live subscription handle.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum QueryResult {
    Snapshot { value: Vec<u8>, frontier: Option<FrontierSummary> },
    Page { items: Vec<Vec<u8>>, next: Option<QueryCursor>, frontier: Option<FrontierSummary> },
    Subscription { subscription: SubscriptionId, initial: Vec<u8>, cursor: Option<QueryCursor>, frontier: Option<FrontierSummary> },
}

/// @emoji 🔔️ Identity of an established live projection subscription.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubscriptionId(pub String);
//#endregion 🔖️Query

//#region 🔖️Lanes
/// @emoji 📚️ One durable, replayable fact an actor emitted. Persisted, sequenced, causally tracked
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

/// @emoji 💨️ One lossy, expiring frame — cursors, selections, previews, typing, connection quality.
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

//#region 🔖️Policy
/// @emoji 🚦️ Every point authorization is evaluated at. Hiding a route is user experience; these
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

/// @emoji ⚖️ The result of one policy evaluation. Deny always wins over allow.
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
/// @emoji 📇️ What one command kind declares to the instance that registers it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandDescriptor {
    pub kind: String,
    pub version: u32,
    pub actor_kind: String,
    pub offline: OfflinePolicy,
}

/// @emoji 📇️ What one query kind declares.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDescriptor {
    pub kind: String,
    pub version: u32,
    pub projection: String,
}

/// @emoji 🎓️ A named bundle of grants. `admin`/`manager`/`editor`/`viewer` are values of this type,
/// never hard-coded enums inside handlers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyTemplate {
    pub name: String,
    pub grants: Vec<PolicyGrant>,
}

/// @emoji 🔑️ One grant inside a template: an action on a resource pattern at a policy point.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyGrant {
    pub point: PolicyPoint,
    pub resource: String,
    pub action: String,
}

/// @emoji 🧾️ The declarative half of a server module — what it contributes, with no runtime types,
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

/// @emoji 🏛️ One deployable server: its identity plus the modules composing it. Hub and Zentrale
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
mod tests {
    use super::*;

    fn envelope() -> CommandEnvelope {
        CommandEnvelope {
            command_id: CommandId("cmd-1".into()),
            kind: "artifact.mutate".into(),
            version: 1,
            target: ActorKey { tenant: TenantId("t1".into()), kind: "artifact".into(), id: "doc-1".into() },
            scope: Scope("space-1".into()),
            principal: Principal::User { id: "alice".into() },
            session: Some(SessionId("s1".into())),
            device: Some(DeviceId("d1".into())),
            payload: vec![1, 2, 3],
            causal_frontier: None,
            client_hlc: HybridLogicalClock { millis: 7, counter: 2 },
            expected_revision: Some(Revision(4)),
            idempotency_key: Some(IdempotencyKey("k1".into())),
            capability_proof: None,
            trace: TraceContext::default(),
        }
    }

    //#region 🔖️Command
    #[test]
    fn command_envelope_round_trips_through_json() {
        let original = envelope();
        let text = serde_json::to_string(&original).unwrap();
        assert_eq!(serde_json::from_str::<CommandEnvelope>(&text).unwrap(), original);
    }

    #[test]
    fn command_outcome_variants_are_tagged_by_status() {
        let receipt = CommandReceipt { command_id: CommandId("cmd-1".into()), actor: envelope().target, revision: Revision(5), accepted_at: HybridLogicalClock::default() };
        let accepted = CommandOutcome::Accepted { receipt: receipt.clone(), events: vec![], frontier: None };
        assert!(serde_json::to_string(&accepted).unwrap().contains("\"status\":\"accepted\""));
        let rejected = CommandOutcome::Rejected { receipt, reason: Rejection::Invalid { detail: "no".into() }, notices: vec![] };
        assert!(serde_json::to_string(&rejected).unwrap().contains("\"status\":\"rejected\""));
    }
    //#endregion 🔖️Command

    //#region 🔖️Query
    #[test]
    fn query_consistency_round_trips_every_variant() {
        for consistency in [QueryConsistency::Local, QueryConsistency::Authority] {
            let text = serde_json::to_string(&consistency).unwrap();
            assert_eq!(serde_json::from_str::<QueryConsistency>(&text).unwrap(), consistency);
        }
    }
    //#endregion 🔖️Query

    //#region 🔖️Module
    #[test]
    fn unknown_command_kinds_default_to_authority_required() {
        let instance = ServerInstanceDefinition {
            id: "hub".into(),
            version: "0.1.0".into(),
            modules: vec![ModuleManifest { id: "documents".into(), commands: vec![CommandDescriptor { kind: "artifact.mutate".into(), version: 1, actor_kind: "artifact".into(), offline: OfflinePolicy::Optimistic }], ..Default::default() }],
        };
        assert_eq!(instance.offline_policy("artifact.mutate"), OfflinePolicy::Optimistic);
        assert_eq!(instance.offline_policy("directory.inviteMember"), OfflinePolicy::AuthorityRequired);
    }
    //#endregion 🔖️Module

    //#region 🔖️Policy
    #[test]
    fn deny_is_not_allowed() {
        assert!(PolicyDecision::Allow.is_allowed());
        assert!(!PolicyDecision::Deny { reason: "nope".into() }.is_allowed());
    }
    //#endregion 🔖️Policy
}
