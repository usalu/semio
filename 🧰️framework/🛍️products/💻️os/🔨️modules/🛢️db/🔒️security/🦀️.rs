//! 🗄️ `db_security` — the `db` family's security seam: multi-granularity authz (database/
//! document/command-kind/object/field/historical/preview), an injectable-`Signer`/
//! `SignatureVerifier` signing bridge (`protocol_core`'s crypto traits — this crate never picks a
//! concrete scheme), a bounded replay guard, token-bucket DoS budgets, structural field
//! redaction, tenant isolation, and audit event emission over `Emit`. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: this crate sits BELOW `db_artifact` per the dependency table, so per the
//! contract's "command payloads are opaque ... below `db_artifact`" rule it never interprets
//! operation/diff *semantics*. It does read `protocol::MutationEnvelope`'s routing fields
//! (`mutation_id`/`document_id`/`actor`/`timestamp`) for replay/authz addressing — those are
//! envelope plumbing, not payload interpretation — and it walks a `serde_json::Value` payload
//! purely structurally (field paths in, redacted value out) for field-level redaction, never
//! reasoning about what the payload means. `serde_json` is added as a genuine dependency beyond
//! the contract's bare `db_core, protocol` table entry: `protocol_causal::ArtifactDiff.payload`
//! is already typed `serde_json::Value` (protocol's own generic-JSON choice, not a new format
//! this crate invents), and field redaction has no way to walk "the field named X" without
//! sharing that concrete type.
//!
//! 🎯️ `SecurityGate` is this crate's composition root: it is the piece `db_artifact`'s pipeline
//! ("admit → dedupe → base-resolve → authz → ...") is expected to call for the admit/dedupe/authz
//! stages. Every building block (`RoleBasedPolicy`, `ReplayGuard`, `BudgetRegistry`, signing
//! functions, `redact_fields`, `check_tenant`) is also usable standalone, since a deployment may
//! wire only a subset (e.g. no signing, no per-tenant budgets).
use crate::*;
/// @emoji 🏢️ A tenant's identity — the isolation unit `check_tenant` gates on. Kept as its own
/// newtype (distinct from `protocol::ActorId`/`ActorId`) since one tenant spans many
/// actors and a `Principal` always belongs to exactly one.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TenantId(pub String);

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for TenantId {
    fn from(value: &str) -> Self {
        TenantId(value.to_string())
    }
}

impl From<String> for TenantId {
    fn from(value: String) -> Self {
        TenantId(value)
    }
}

/// @emoji 🪪️ The authenticated caller a `SecurityGate` decision is made against: which actor,
/// which tenant they belong to, and which roles `RoleBasedPolicy` grants are matched against.
/// Deliberately carries `protocol::ActorId` (not `ActorId`) — see module doc: this crate
/// authorizes the same actor identity that flows through `protocol::MutationEnvelope`.
#[derive(Clone, Debug)]
pub struct Principal {
    pub actor: protocol::ActorId,
    pub tenant: TenantId,
    pub roles: Vec<String>,
}

impl Principal {
    /// @emoji 🆕️ Builds a principal from its three parts.
    // 🚫️async: E1 pure constructor, used inside a sync `Fn(&ActorId) -> Principal` closure slot — see R9
    pub fn new(actor: protocol::ActorId, tenant: TenantId, roles: Vec<String>) -> Self {
        Self { actor, tenant, roles }
    }

    /// @emoji 🔎️ True iff `self` holds `role` (exact string match — role names are this
    /// deployment's own vocabulary, not this crate's concern).
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}

/// @emoji 🚧️ Tenant isolation: succeeds only if `principal` belongs to `resource_tenant`. Kept as
/// a standalone check (not folded into `RoleBasedPolicy`) since it is a hard boundary that must
/// hold regardless of any role grant — a policy misconfiguration must never leak across tenants.
pub fn check_tenant(principal: &Principal, resource_tenant: &TenantId) -> Result<(), DbError> {
    if &principal.tenant == resource_tenant {
        Ok(())
    } else {
        Err(DbError::Unauthorized(format!("tenant isolation: principal '{}' (tenant {}) may not act on resource tenant {}", principal.actor.0, principal.tenant.0, resource_tenant.0)))
    }
}
//#endregion 🔖️Identity

//#region 🔖️Scope
/// @emoji 🎯️ What action a scope check is being made for. Kept as one flat set applicable to
/// every `AuthzScope` variant (rather than a per-scope action vocabulary) — the resource
/// granularity already lives in `AuthzScope`, so `Action` only needs to answer "read, mutate, or
/// administer this resource".
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Action {
    Read,
    Write,
    Admin,
}

/// @emoji 🗺️ The seven authz granularities the contract names: whole database, one document, one
/// command kind within a document, one object, one field of one object, a document's historical
/// view, and a document's preview (speculative) view.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AuthzScope {
    Database,
    Document { document: protocol::ArtifactId },
    CommandKind { document: protocol::ArtifactId, kind: String },
    Object { document: protocol::ArtifactId, object_id: String },
    Field { document: protocol::ArtifactId, object_id: String, field: String },
    Historical { document: protocol::ArtifactId },
    Preview { document: protocol::ArtifactId },
}

impl AuthzScope {
    /// @emoji 🧵️ Renders this scope as a canonical path-segment sequence, matched against
    /// `Grant::pattern` by `pattern_matches`. Every non-`Database` scope is nested under
    /// `["db", "document", <id>, ...]` so a `**` grant rooted at a document (e.g.
    /// `["db", "document", "doc-1", "**"]`) covers every scope kind under that one document.
    pub fn segments(&self) -> Vec<String> {
        match self {
            AuthzScope::Database => vec!["db".to_string()],
            AuthzScope::Document { document } => {
                vec!["db".to_string(), "document".to_string(), document.0.clone()]
            }
            AuthzScope::CommandKind { document, kind } => {
                vec!["db".to_string(), "document".to_string(), document.0.clone(), "kind".to_string(), kind.clone()]
            }
            AuthzScope::Object { document, object_id } => {
                vec!["db".to_string(), "document".to_string(), document.0.clone(), "object".to_string(), object_id.clone()]
            }
            AuthzScope::Field { document, object_id, field } => vec!["db".to_string(), "document".to_string(), document.0.clone(), "object".to_string(), object_id.clone(), "field".to_string(), field.clone()],
            AuthzScope::Historical { document } => {
                vec!["db".to_string(), "document".to_string(), document.0.clone(), "historical".to_string()]
            }
            AuthzScope::Preview { document } => {
                vec!["db".to_string(), "document".to_string(), document.0.clone(), "preview".to_string()]
            }
        }
    }

    /// @emoji 📄️ The document this scope is nested under, or `None` for `AuthzScope::Database`.
    pub async fn document(&self) -> Option<&protocol::ArtifactId> {
        match self {
            AuthzScope::Database => None,
            AuthzScope::Document { document } | AuthzScope::CommandKind { document, .. } | AuthzScope::Object { document, .. } | AuthzScope::Field { document, .. } | AuthzScope::Historical { document } | AuthzScope::Preview { document } => {
                Some(document)
            }
        }
    }
}
//#endregion 🔖️Scope

//#region 🔖️Policy
/// @emoji ⚖️ Whether a matching `Grant` permits or forbids the action.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Effect {
    Allow,
    Deny,
}

/// @emoji 🧑️‍⚖️ The result of `RoleBasedPolicy::evaluate`: either the action is allowed, or it is
/// denied with a human-readable reason (surfaced to `DbError::Unauthorized` and to the audit
/// trail).
#[derive(Clone, PartialEq, Debug)]
pub enum Decision {
    Allow,
    Deny { reason: String },
}

impl Decision {
    // 🚫️async: E1 pure accessor consumed synchronously by dozens of test/production call sites — see R9
    pub fn is_allowed(&self) -> bool {
        matches!(self, Decision::Allow)
    }

    /// @emoji 🔀️ Collapses the decision into the family's standard `Result` shape.
    pub async fn into_result(self) -> Result<(), DbError> {
        match self {
            Decision::Allow => Ok(()),
            Decision::Deny { reason } => Err(DbError::Unauthorized(reason)),
        }
    }
}

/// @emoji 📜️ One role-scoped rule: if `role` is among the principal's roles, `action` is in
/// `actions`, and `pattern` matches the scope's `segments()`, this grant applies with `effect`.
#[derive(Clone, Debug)]
pub struct Grant {
    pub role: String,
    pub pattern: Vec<String>,
    pub actions: std::collections::BTreeSet<Action>,
    pub effect: Effect,
}

impl Grant {
    /// @emoji ✅️ An allow rule. `pattern` segments may be `"*"` (matches exactly one segment) or
    /// a trailing `"**"` (matches the remainder, zero or more segments — see `pattern_matches`).
    pub fn allow(role: impl Into<String>, pattern: &[&str], actions: &[Action]) -> Self {
        Self { role: role.into(), pattern: pattern.iter().map(|s| s.to_string()).collect(), actions: actions.iter().copied().collect(), effect: Effect::Allow }
    }

    /// @emoji 🚫️ A deny rule — see `RoleBasedPolicy::evaluate`: any matching deny short-circuits
    /// to `Decision::Deny` regardless of any allow grant, matched or not yet evaluated.
    // 🚫️async: E1 pure constructor consumed synchronously by `impl Default` — see R9
    pub fn deny(role: impl Into<String>, pattern: &[&str], actions: &[Action]) -> Self {
        Self { role: role.into(), pattern: pattern.iter().map(|s| s.to_string()).collect(), actions: actions.iter().copied().collect(), effect: Effect::Deny }
    }
}

/// @emoji 🧩️ Matches a `Grant::pattern` against a scope's `segments()`. `"*"` matches exactly one
/// segment; `"**"` — only valid as the LAST pattern segment, this crate's own choice for a simple,
/// unambiguous matcher — matches every remaining segment (including zero). Any other pattern
/// segment must match the corresponding scope segment exactly.
fn pattern_matches(pattern: &[String], segments: &[String]) -> bool {
    for (index, part) in pattern.iter().enumerate() {
        if part == "**" {
            return index == pattern.len() - 1;
        }
        match segments.get(index) {
            Some(segment) if part == "*" || part == segment => continue,
            _ => return false,
        }
    }
    pattern.len() == segments.len()
}

/// @emoji 🛡️ A default-deny role-based policy engine: a scope/action is allowed only if at least
/// one `Grant` explicitly allows it and no `Grant` explicitly denies it (deny always wins).
#[derive(Clone, Debug, Default)]
pub struct RoleBasedPolicy {
    grants: Vec<Grant>,
}

impl RoleBasedPolicy {
    // 🚫️async: E1 pure constructor — used as a bare `fold`/`Default`-adjacent builder
    // (`RoleBasedPolicy::with_grant` as an `Iterator::fold` fn-pointer accumulator, and
    // `ArtifactEngineConfig`'s `impl Default`, itself E1) throughout this crate — see R9
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕️ Adds one grant (builder-style).
    // 🚫️async: E1 pure builder, used as a bare `Iterator::fold` fn-pointer accumulator — see R9
    pub fn with_grant(mut self, grant: Grant) -> Self {
        self.grants.push(grant);
        self
    }

    /// @emoji ⚖️ Evaluates every grant whose `role` the principal holds and whose `actions`
    /// contains `action`, matching `pattern` against `scope.segments()`. A matching `Deny`
    /// short-circuits immediately; otherwise the result is `Allow` iff at least one matching
    /// `Allow` grant was found, `Deny` (default-deny) otherwise.
    pub async fn evaluate(&self, principal: &Principal, scope: &AuthzScope, action: Action) -> Decision {
        let segments = scope.segments();
        let mut allowed_by: Option<&Grant> = None;
        for grant in &self.grants {
            if !principal.has_role(&grant.role) || !grant.actions.contains(&action) || !pattern_matches(&grant.pattern, &segments) {
                continue;
            }
            match grant.effect {
                Effect::Deny => {
                    return Decision::Deny { reason: format!("denied by role '{}' grant {:?} for {action:?} on {segments:?}", grant.role, grant.pattern) };
                }
                Effect::Allow => allowed_by = allowed_by.or(Some(grant)),
            }
        }
        match allowed_by {
            Some(_) => Decision::Allow,
            None => Decision::Deny { reason: format!("no grant allows {action:?} on {segments:?} for roles {:?}", principal.roles) },
        }
    }
}
//#endregion 🔖️Policy

//#region 🔖️SpaceGrants
/// @emoji 🪐️ Compiles a hub space's `kind` (`"atelier"|"studio"|"archive"`, string-identical to the
/// wasm-facing `space` crate's `SpaceKind` — see `🌎️hub/🔨️modules/📇️directory`'s `SpaceRecord`) into
/// this crate's `Grant`s: `"author"` gets `Read`+`Write` over the space, `"spectator"` gets `Read`
/// only, and an `"archive"` space additionally gets an explicit `Grant::deny` on `"author"` writes
/// — deny-overrides semantics (`RoleBasedPolicy::evaluate`: a matching `Deny` short-circuits
/// regardless of any `Allow`) so an archive stays frozen even if a membership row is ever
/// (incorrectly) left with an `Author` role. Purely mechanical/data-driven per this crate's module
/// doc: it interprets `kind` as an opaque string, never a hub- or space-crate-specific type.
///
/// `space_id` is accepted for API symmetry and future audit/log context, but the returned grants'
/// patterns use a wildcard document segment (`["db", "document", "*", "**"]`) rather than a literal
/// `space_id` segment — a hub document id is the compound `"{space_id}:{document_id}"` string (see
/// `🌎️hub`'s `scope_key`), which `AuthzScope::segments()` carries as ONE opaque segment, so a literal
/// match can't select "this space's documents" by substring. This is sound because the caller
/// (`🌎️hub`'s WS handler) builds a fresh `RoleBasedPolicy`/`SecurityGate` per connection, and a
/// connection is already scoped to exactly one space's one document — the wildcard never needs to
/// discriminate across spaces because it is never evaluated against another space's document.
pub async fn space_grants(_space_id: &str, kind: &str) -> Vec<Grant> {
    let mut grants = vec![Grant::allow("author", &["db", "document", "*", "**"], &[Action::Read, Action::Write]), Grant::allow("spectator", &["db", "document", "*", "**"], &[Action::Read])];
    if kind == "archive" {
        grants.push(Grant::deny("author", &["db", "document", "*", "**"], &[Action::Write]));
    }
    grants
}
//#endregion 🔖️SpaceGrants

//#region 🔖️Signing
/// @emoji ✍️ A detached signature over a 32-byte message, plus the scheme/key it was produced
/// with — the shape `protocol::Signer`/`protocol::SignatureVerifier` exchange, packaged so
/// callers don't have to carry the three fields separately.
#[derive(Clone, PartialEq, Debug)]
pub struct Signature {
    pub scheme: String,
    pub key_id: String,
    pub bytes: Vec<u8>,
}

/// @emoji 🔀️ Maps `protocol::ProtocolError` (the error type `Signer`/`SignatureVerifier` return)
/// onto this family's `DbError` by category, mirroring `db_core`'s `From<pack::PackError>`.
// 🚫️async: E4 fn-pointer slot
fn map_protocol_error(err: protocol::ProtocolError) -> DbError {
    match err {
        protocol::ProtocolError::LimitExceeded(what) => DbError::LimitExceeded(what),
        protocol::ProtocolError::Io(message) => DbError::Io(message),
        protocol::ProtocolError::SignatureInvalid { .. } | protocol::ProtocolError::VerifierRequired => DbError::Unauthorized(err.to_string()),
        other => DbError::Internal(other.to_string()),
    }
}

/// @emoji ✍️ Signs `message` (typically a commit/frontier `chain_hash`) with an injected
/// `Signer` — this crate never picks a concrete scheme, matching the contract's "signatures via
/// injected `Signer`/`Verifier` traits" wording.
// 🔀️ `signer: &impl protocol::Signer` (was `&dyn`) — `Signer` is an OPEN extension point (its own
// module doc: "zero impls in this family — supplied by the integration layer"), and its `async fn`
// methods make `dyn Signer` non-object-safe (R1). R11(a): parameter-position dyn erasure is
// trivially generic, so this is just a signature change, no design question.
pub async fn sign_message(signer: &impl protocol::Signer, message: &[u8; 32]) -> Result<Signature, DbError> {
    let bytes = signer.sign(message).await.map_err(map_protocol_error)?;
    Ok(Signature { scheme: signer.scheme().await.to_string(), key_id: signer.key_id().await.to_string(), bytes })
}

/// @emoji ✅️ Verifies `signature` over `message` with an injected `SignatureVerifier`. Distinct
/// from a raw `bool` return: a cryptographically-valid-but-false verification and a
/// verifier-internal error both surface as `Err`, but with different `DbError` variants, so a
/// caller can tell "rejected" from "verifier broken" without inspecting a string.
// 🔀️ `verifier: &impl protocol::SignatureVerifier` (was `&dyn`) — same rationale as `sign_message`.
pub async fn verify_signature(verifier: &impl protocol::SignatureVerifier, signature: &Signature, message: &[u8; 32]) -> Result<(), DbError> {
    let ok = verifier.verify(&signature.scheme, &signature.key_id, message, &signature.bytes).await.map_err(map_protocol_error)?;
    if ok {
        Ok(())
    } else {
        Err(DbError::Unauthorized(format!("signature verification failed for key '{}'", signature.key_id)))
    }
}
//#endregion 🔖️Signing

//#region 🔖️Replay
/// @emoji 🩹️ A bounded, per-actor sliding-time-window replay guard: rejects an `MutationId` a
/// given actor has already submitted within `window_ms`. Deliberately NOT a permanent ledger —
/// bounded by both `window_ms` (time) and `capacity_per_actor` (space, oldest-evicted-first) so
/// memory never grows unboundedly under a hostile or buggy high-volume actor; durable dedupe
/// beyond the window is `db_wal`/`db_artifact`'s job (WAL sequencing already rejects a
/// truly-duplicate commit once applied), this guard's job is only to catch replay CHEAPLY, in
/// memory, before that heavier machinery runs.
#[derive(Clone, Debug)]
pub struct ReplayGuard {
    window_ms: u64,
    capacity_per_actor: usize,
    order: std::collections::HashMap<protocol::ActorId, std::collections::VecDeque<(protocol::MutationId, u64)>>,
    seen: std::collections::HashMap<protocol::ActorId, std::collections::HashSet<protocol::MutationId>>,
}

impl ReplayGuard {
    pub fn new(window_ms: u64, capacity_per_actor: usize) -> Self {
        Self { window_ms, capacity_per_actor, order: std::collections::HashMap::new(), seen: std::collections::HashMap::new() }
    }

    /// @emoji 🔍️ Evicts `actor`'s entries older than `window_ms` relative to `physical_ms`, then
    /// rejects with `DbError::Conflict` if `mutation_id` is still tracked; otherwise records it
    /// (evicting the oldest entry first if `capacity_per_actor` would be exceeded) and returns
    /// `Ok`.
    // 🚫️async: E1 pure accessor, no suspension in its body — same shape as `BudgetRegistry::
    // try_consume` above, which this crate already keeps sync — see R9. Was previously `async fn`
    // for no real reason; `SecurityGate::admit_command`'s `if lock(&self.replay).check_and_record
    // (..).await.is_err()` needed the `MutexGuard` alive across that `.await` to make the call at
    // all, making the enclosing future non-`Send` (R7) — removing `async` here removes the
    // suspension point instead of working around holding the guard across it.
    pub fn check_and_record(&mut self, actor: &protocol::ActorId, mutation_id: &protocol::MutationId, physical_ms: u64) -> Result<(), DbError> {
        let deque = self.order.entry(actor.clone()).or_default();
        let set = self.seen.entry(actor.clone()).or_default();

        while let Some((_, timestamp)) = deque.front() {
            if physical_ms.saturating_sub(*timestamp) > self.window_ms {
                if let Some((expired_id, _)) = deque.pop_front() {
                    set.remove(&expired_id);
                }
            } else {
                break;
            }
        }

        if set.contains(mutation_id) {
            return Err(DbError::Conflict(format!("replayed operation '{}' by actor '{}' within {}ms window", mutation_id.0, actor.0, self.window_ms)));
        }

        if deque.len() >= self.capacity_per_actor {
            if let Some((evicted_id, _)) = deque.pop_front() {
                set.remove(&evicted_id);
            }
        }
        deque.push_back((mutation_id.clone(), physical_ms));
        set.insert(mutation_id.clone());
        Ok(())
    }
}
//#endregion 🔖️Replay

//#region 🔖️Budget
/// @emoji 🪙️ A single token bucket: `capacity` tokens, refilling at `refill_per_sec` tokens/sec,
/// lazily caught up to `now_ms` on every `try_consume` call (no background timer needed).
#[derive(Clone, Debug)]
struct TokenBucket {
    capacity: f64,
    tokens: f64,
    refill_per_ms: f64,
    last_refill_ms: u64,
}

impl TokenBucket {
    fn new(capacity: u32, refill_per_sec: u32, now_ms: u64) -> Self {
        Self { capacity: capacity as f64, tokens: capacity as f64, refill_per_ms: refill_per_sec as f64 / 1000.0, last_refill_ms: now_ms }
    }

    fn try_consume(&mut self, cost: u32, now_ms: u64) -> bool {
        let elapsed_ms = now_ms.saturating_sub(self.last_refill_ms) as f64;
        self.tokens = (self.tokens + elapsed_ms * self.refill_per_ms).min(self.capacity);
        self.last_refill_ms = now_ms;
        if self.tokens >= cost as f64 {
            self.tokens -= cost as f64;
            true
        } else {
            false
        }
    }
}

/// @emoji 🛡️ DoS budgets: one token bucket per string key (actor id, tenant id, command kind —
/// whatever granularity the caller wants to rate-limit at; this crate is deliberately
/// key-agnostic). Every unseen key gets a fresh bucket at full `capacity` on first use.
#[derive(Clone, Debug)]
pub struct BudgetRegistry {
    capacity: u32,
    refill_per_sec: u32,
    buckets: std::collections::HashMap<String, TokenBucket>,
}

impl BudgetRegistry {
    pub fn new(capacity: u32, refill_per_sec: u32) -> Self {
        Self { capacity, refill_per_sec, buckets: std::collections::HashMap::new() }
    }

    /// @emoji 🎟️ Attempts to consume `cost` tokens from `key`'s bucket at `now_ms`. Returns
    /// `DbError::LimitExceeded` if the bucket doesn't have enough tokens yet.
    // 🚫️async: E1 pure accessor, `TokenBucket`'s own methods are sync — see R9
    pub fn try_consume(&mut self, key: &str, cost: u32, now_ms: u64) -> Result<(), DbError> {
        let (capacity, refill_per_sec) = (self.capacity, self.refill_per_sec);
        let bucket = self.buckets.entry(key.to_string()).or_insert_with(|| TokenBucket::new(capacity, refill_per_sec, now_ms));
        if bucket.try_consume(cost, now_ms) {
            Ok(())
        } else {
            Err(DbError::LimitExceeded("dos budget exceeded"))
        }
    }
}
//#endregion 🔖️Budget

//#region 🔖️Redaction
/// @emoji 🛡️ A depth ceiling for `redact_fields`'s recursion. Not a contract number — this
/// crate's own hardening choice, mirroring `check_len`'s "validate before allocating"
/// spirit for the recursive case: an adversarially-deep JSON payload could otherwise blow the
/// stack before any redaction decision is even made, so depth beyond the ceiling is redacted
/// outright (deny-by-default) rather than passed through unredacted or allowed to recurse further.
const MAX_REDACT_DEPTH: usize = 64;

/// @emoji 🕳️ The marker `redact_fields` substitutes for a denied subtree — an object tagged
/// `{"$redacted": true}` rather than `null`, so a caller can tell "hidden" from "genuinely null".
async fn redacted_marker() -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert("$redacted".to_string(), serde_json::Value::Bool(true));
    serde_json::Value::Object(object)
}

/// @emoji 🫥️ Structurally walks `value` (an object's JSON payload — e.g.
/// `protocol::ArtifactDiff.payload`), replacing every dotted field path the `policy` denies
/// `Action::Read` on (scoped as `AuthzScope::Field { document, object_id, field: <path> }`) with
/// `redacted_marker()`. Only walks JSON objects (arrays are left as opaque leaves — this crate's
/// own scope choice: array-element-level redaction would need array-index-stable paths, which is
/// payload-schema knowledge this crate must not depend on per the module doc). A denied subtree
/// is never recursed into further — its whole value is replaced, so nested fields under a denied
/// field are never separately evaluated (and can't leak).
pub async fn redact_fields(policy: &RoleBasedPolicy, principal: &Principal, document: &protocol::ArtifactId, object_id: &str, value: &serde_json::Value) -> serde_json::Value {
    redact_fields_at(policy, principal, document, object_id, "", value, 0).await
}

async fn redact_fields_at(policy: &RoleBasedPolicy, principal: &Principal, document: &protocol::ArtifactId, object_id: &str, path: &str, value: &serde_json::Value, depth: usize) -> serde_json::Value {
    if depth > MAX_REDACT_DEPTH {
        return redacted_marker().await;
    }
    if !path.is_empty() {
        let scope = AuthzScope::Field { document: document.clone(), object_id: object_id.to_string(), field: path.to_string() };
        if !policy.evaluate(principal, &scope, Action::Read).await.is_allowed() {
            return redacted_marker().await;
        }
    }
    match value {
        serde_json::Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (key, child) in map {
                let child_path = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
                out.insert(key.clone(), Box::pin(redact_fields_at(policy, principal, document, object_id, &child_path, child, depth + 1)).await);
            }
            serde_json::Value::Object(out)
        }
        other => other.clone(),
    }
}
//#endregion 🔖️Redaction

//#region 🔖️Audit
/// @emoji 📣️ Emits a `security.authz_allowed`/`security.authz_denied` `EmitEvent` for
/// one policy decision, via the family's shared `Emit` seam (see `Emit`'s doc for why this crate
/// stays generic over `E: Emit` rather than depending on `db_observe` directly — dedyn-emit-runtime,
/// O1/R11(a): a caller-supplied, stored implementation is trivially generic, exactly like
/// `db_artifact::ArtifactEngineConfig`'s own `A: AuthzHook`/`V: VersionGraph` params).
pub async fn audit_decision<E: Emit>(emit: &E, principal: &Principal, scope: &AuthzScope, action: Action, decision: &Decision) {
    let name = if decision.is_allowed() { "security.authz_allowed" } else { "security.authz_denied" };
    let mut event = EmitEvent::new(name)
        .field("actor", EmitField::Text(principal.actor.0.clone()))
        .field("tenant", EmitField::Text(principal.tenant.0.clone()))
        .field("action", EmitField::Text(format!("{action:?}")))
        .field("scope", EmitField::Text(scope.segments().join("/")));
    if let Some(document) = scope.document().await {
        event = event.with_document(ArtifactId::from(document.0.clone()));
    }
    if let Decision::Deny { reason } = decision {
        event = event.field("reason", EmitField::Text(reason.clone()));
    }
    emit.emit(event).await;
}

/// @emoji 📣️ Emits a `security.replay_rejected` event — `SecurityGate::admit_command` calls this
/// when `ReplayGuard` rejects an operation, so a replay attempt is auditable even though it never
/// reaches a `Decision`.
pub async fn audit_replay_rejected<E: Emit>(emit: &E, actor: &protocol::ActorId, mutation_id: &protocol::MutationId, document: &protocol::ArtifactId) {
    emit.emit(EmitEvent::new("security.replay_rejected").with_document(ArtifactId::from(document.0.clone())).field("actor", EmitField::Text(actor.0.clone())).field("mutation_id", EmitField::Text(mutation_id.0.clone()))).await;
}

/// @emoji 📣️ Emits a `security.budget_exceeded` event — `SecurityGate::admit_command` calls this
/// when `BudgetRegistry` rejects a submission.
pub async fn audit_budget_exceeded<E: Emit>(emit: &E, key: &str, document: &protocol::ArtifactId) {
    emit.emit(EmitEvent::new("security.budget_exceeded").with_document(ArtifactId::from(document.0.clone())).field("key", EmitField::Text(key.to_string()))).await;
}
//#endregion 🔖️Audit

//#region 🔖️Gate
/// @emoji 🔓️ Recovers a `Mutex`'s inner value even if a prior holder panicked while holding it —
/// mirrors `db_observe`'s `lock` helper: a security check must never itself become a source of
/// panic-under-panic for the actor code that calls into it, often mid-crash-handling.
// 🚫️async: E1 pure accessor (no suspension) — see R9
fn lock<T>(mutex: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// @emoji 🚪️ The composition root: everything `db_artifact`'s pipeline needs from this crate for
/// its "admit → dedupe → ... → authz" stages, bundled behind one type. Every piece
/// (`RoleBasedPolicy`, `ReplayGuard`, `BudgetRegistry`, `Emit`) is independently usable —
/// `SecurityGate` is a convenience, not the only entry point.
///
/// 🔀️ dedyn-emit-runtime, O1/R11(a): generic over `E: Emit` (default `NullEmit`) rather than
/// `Arc<dyn Emit>` — this crate's own tests supply `RecordingEmit` to assert audit events fire,
/// while every production wiring (`db_artifact`, `db_engine`, `🌎️hub`) uses the default `NullEmit`
/// and never writes the type parameter out, so this is source-compatible everywhere unparameterized.
pub struct SecurityGate<E: Emit + 'static = NullEmit> {
    policy: RoleBasedPolicy,
    replay: std::sync::Mutex<ReplayGuard>,
    budgets: std::sync::Mutex<BudgetRegistry>,
    emit: std::sync::Arc<E>,
}

impl<E: Emit + 'static> SecurityGate<E> {
    pub fn new(policy: RoleBasedPolicy, replay: ReplayGuard, budgets: BudgetRegistry, emit: std::sync::Arc<E>) -> Self {
        Self { policy, replay: std::sync::Mutex::new(replay), budgets: std::sync::Mutex::new(budgets), emit }
    }

    /// @emoji ⚖️ Evaluates `scope`/`action` against the gate's policy, audits the decision, and
    /// collapses it into a `Result` — the single authz entry point every scope granularity (db,
    /// document, command-kind, object, field, historical, preview) shares.
    pub async fn authorize(&self, principal: &Principal, scope: &AuthzScope, action: Action) -> Result<(), DbError> {
        let decision = self.policy.evaluate(principal, scope, action).await;
        audit_decision(self.emit.as_ref(), principal, scope, action, &decision).await;
        decision.into_result().await
    }

    /// @emoji ✅️ The admit/dedupe/authz stages for one command submission: tenant isolation,
    /// then `Action::Write` authz on `AuthzScope::CommandKind`, then the DoS budget (keyed by
    /// `principal.actor`), then replay-guard dedupe (keyed by the envelope's own actor/operation
    /// id, which may differ from `principal.actor` under delegated/service submission — this
    /// crate does not assume they're always the same identity).
    #[allow(clippy::too_many_arguments)]
    pub async fn admit_command(&self, principal: &Principal, resource_tenant: &TenantId, document: &protocol::ArtifactId, kind: &str, envelope_actor: &protocol::ActorId, mutation_id: &protocol::MutationId, physical_ms: u64) -> Result<(), DbError> {
        check_tenant(principal, resource_tenant)?;
        self.authorize(principal, &AuthzScope::CommandKind { document: document.clone(), kind: kind.to_string() }, Action::Write).await?;
        if lock(&self.budgets).try_consume(&principal.actor.0, 1, physical_ms).is_err() {
            audit_budget_exceeded(self.emit.as_ref(), &principal.actor.0, document).await;
            return Err(DbError::LimitExceeded("dos budget exceeded"));
        }
        if lock(&self.replay).check_and_record(envelope_actor, mutation_id, physical_ms).is_err() {
            audit_replay_rejected(self.emit.as_ref(), envelope_actor, mutation_id, document).await;
            return Err(DbError::Conflict(format!("replayed operation '{}' by actor '{}'", mutation_id.0, envelope_actor.0)));
        }
        Ok(())
    }

    /// @emoji 🫥️ Convenience forward to the free `redact_fields` function using the gate's own
    /// policy.
    pub async fn redact(&self, principal: &Principal, document: &protocol::ArtifactId, object_id: &str, value: &serde_json::Value) -> serde_json::Value {
        redact_fields(&self.policy, principal, document, object_id, value).await
    }
}
//#endregion 🔖️Gate

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
