//! 🛡️ Authorization: roles as data, evaluated at the points where access is actually decided.
//!
//! A role is never an enum branched on inside a handler. `admin`, `manager`, `editor` and `viewer`
//! are values of [`PolicyTemplate`] registered into a [`PolicyEngine`] and assigned to principals,
//! optionally limited to a single [`Scope`]. Adding a role is registering data; it is never editing
//! a match arm, so the whole role set stays inspectable, diffable and serveable next to the module
//! manifests that declared it.
//!
//! Every request is evaluated at exactly one [`PolicyPoint`]. Seven of them are request-time gates
//! that must be passed before anything observable happens — command admission, command execution,
//! query access, subscription, event delivery, blob read and blob write — and two further points
//! cover outbound effects and the administration plane. Hiding a route, a menu entry or a button is
//! user experience only: a route the interface forgot to hide is still denied here, and a route the
//! interface shows is still denied here unless a grant says otherwise.
//!
//! Two rules govern evaluation and neither is negotiable: it is **closed by default** — no matching
//! grant means [`PolicyDecision::Deny`] — and **deny overrides allow** — one matching explicit deny
//! outranks every matching allow, whatever order the templates were registered in.
//!
//! Authentication is a separate concern that answers "who is this", handled by [`ResolverChain`]:
//! the ladder is generic here, its rungs are supplied by the instance. [`AdminGate`] guards the
//! administration plane before policy is consulted at all.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use semio_framework_dispatch_macros::dyn_enum;
use std::future::Future;

use crate::contract::{CapabilityProof, DeviceId, PolicyDecision, PolicyGrant, PolicyPoint, PolicyTemplate, Principal, Scope, SessionId};

//#region 🔖️Matching
/// 🪪️ The stable key a principal is assigned templates under: `user:alice`, `service:indexer`,
/// `device:d1` or `anonymous`. Keys are opaque strings so an assignment table can be persisted,
/// replicated and diffed without depending on the shape of [`Principal`].
pub fn principal_key(principal: &Principal) -> String {
    match principal {
        Principal::User { id } => format!("user:{id}"),
        Principal::ServiceAccount { id } => format!("service:{id}"),
        Principal::Device { id } => format!("device:{id}"),
        Principal::Anonymous => "anonymous".to_string(),
    }
}

/// 🎯️ Whether a grant's resource pattern covers a concrete resource. A pattern ending in `*` is a
/// prefix wildcard (`space:abc/*` covers `space:abc/doc-1`), the bare pattern `*` covers everything,
/// and any other pattern must be exactly equal. There is no infix or multi-segment globbing: a
/// pattern language nobody can read by eye is a pattern language nobody can audit.
fn resource_matches(pattern: &str, resource: &str) -> bool {
    match pattern.strip_suffix('*') {
        Some(prefix) => resource.starts_with(prefix),
        None => pattern == resource,
    }
}

/// 🚫️ Split a grant action into its polarity and the bare action. A leading `!` marks an explicit
/// deny: `!write` denies `write`, and `!*` denies every action on the matched resource.
fn split_action(action: &str) -> (bool, &str) {
    match action.strip_prefix('!') {
        Some(rest) => (true, rest),
        None => (false, action),
    }
}

/// ⚡️ Whether a grant action applies to a requested action: equal, or the `*` catch-all.
fn action_matches(granted: &str, requested: &str) -> bool {
    granted == requested || granted == "*"
}
//#endregion 🔖️Matching

//#region 🔖️Engine
/// 📥️ One authorization question: who wants to do what, where, at which decision point. `scope` is
/// `None` for instance-wide questions, which scope-limited assignments deliberately never answer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRequest {
    pub point: PolicyPoint,
    pub principal: Principal,
    pub scope: Option<Scope>,
    pub resource: String,
    pub action: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Assignment {
    scope: Option<Scope>,
    template: String,
}

impl Assignment {
    fn applies(&self, scope: Option<&Scope>) -> bool {
        match &self.scope {
            None => true,
            Some(assigned) => scope == Some(assigned),
        }
    }
}

/// ⚖️ Evaluates a [`PolicyRequest`] against the templates assigned to its principal.
///
/// Closed by default: a request nothing matches is denied with a reason naming what was missing.
/// Deny overrides allow: a matching grant whose action begins with `!` denies regardless of how
/// many allows also match. Templates referenced by an assignment but never registered contribute
/// nothing — a typo in a role name can only ever remove access, never grant it.
#[derive(Clone, Debug, Default)]
pub struct PolicyEngine {
    templates: BTreeMap<String, PolicyTemplate>,
    assignments: BTreeMap<String, Vec<Assignment>>,
    authenticated: Option<String>,
}

impl PolicyEngine {
    /// 🌱️ An engine holding no templates and no assignments, which therefore denies everything.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📇️ Register a role definition under its own name, replacing any template of that name.
    pub fn register_template(&mut self, template: PolicyTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// 📇️ Template auto-applied to every non-anonymous principal (session-authenticated callers).
    pub fn set_authenticated_template(&mut self, template_name: String) {
        self.authenticated = Some(template_name);
    }

    /// 🎓️ Grant a principal a template everywhere. A principal may hold several templates; their
    /// grants are unioned, subject to deny-overrides-allow.
    pub fn assign(&mut self, principal_key: String, template_name: String) {
        self.assignments.entry(principal_key).or_default().push(Assignment { scope: None, template: template_name });
    }

    /// 🗂️ Grant a principal a template inside one scope only. The assignment contributes nothing to
    /// requests naming another scope, and nothing to instance-wide requests carrying no scope.
    pub fn assign_scoped(&mut self, principal_key: String, scope: Scope, template_name: String) {
        self.assignments.entry(principal_key).or_default().push(Assignment { scope: Some(scope), template: template_name });
    }

    /// 🧮️ Decide one request. Scans every grant reachable from the principal's applicable
    /// assignments; an explicit deny short-circuits, an allow is only returned once no deny was
    /// found anywhere, and nothing matching at all is a denial.
    pub fn evaluate(&self, request: &PolicyRequest) -> PolicyDecision {
        let key = principal_key(&request.principal);
        let mut allowed = false;
        for grant in self.applicable_grants(&key, request.scope.as_ref()) {
            if grant.point != request.point || !resource_matches(&grant.resource, &request.resource) {
                continue;
            }
            let (denied, action) = split_action(&grant.action);
            if !action_matches(action, &request.action) {
                continue;
            }
            if denied {
                return PolicyDecision::Deny { reason: format!("explicit deny: {key} is denied '{}' on '{}' by grant '{}' on '{}' at {:?}", request.action, request.resource, grant.action, grant.resource, request.point) };
            }
            allowed = true;
        }
        if allowed {
            PolicyDecision::Allow
        } else {
            PolicyDecision::Deny {
                reason: format!(
                    "closed by default: no grant lets {key} do '{}' on '{}' at {:?}{}",
                    request.action,
                    request.resource,
                    request.point,
                    match &request.scope {
                        Some(Scope(scope)) => format!(" in scope '{scope}'"),
                        None => String::new(),
                    }
                ),
            }
        }
    }

    /// 🔗️ Every grant reachable from the assignments that apply to this key and scope.
    fn applicable_grants<'a>(&'a self, key: &str, scope: Option<&Scope>) -> Vec<&'a PolicyGrant> {
        let mut grants: Vec<&'a PolicyGrant> = self
            .assignments
            .get(key)
            .into_iter()
            .flatten()
            .filter(|assignment| assignment.applies(scope))
            .filter_map(|assignment| self.templates.get(&assignment.template))
            .flat_map(|template| template.grants.iter())
            .collect();
        if key != "anonymous" {
            if let Some(name) = self.authenticated.as_deref() {
                if let Some(template) = self.templates.get(name) {
                    grants.extend(template.grants.iter());
                }
            }
        }
        grants
    }
}
//#endregion 🔖️Engine

//#region 🔖️Resolver
/// 🎟️ What a caller presented at the edge, normalized away from any transport. `loopback` records
/// that the peer reached the process over a loopback interface — a fact only the transport can
/// establish and never something a header may claim.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub bearer: Option<String>,
    pub capability: Option<CapabilityProof>,
    pub loopback: bool,
}

/// ✅️ Who a credential turned out to be, plus `via`: the name of the rung that recognized it, kept
/// so an audit log can say *how* a principal was established and not merely who it is.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolved {
    pub principal: Principal,
    pub session: Option<SessionId>,
    pub device: Option<DeviceId>,
    pub via: String,
    /// Hub-issued document actor id bound to this credential (session-stable), when the rung knows one.
    pub actor: Option<String>,
}

/// 🪜️ One rung of the authentication ladder. Returning `None` means "not mine", never "denied":
/// refusal is policy's job at [`PolicyEngine::evaluate`], not authentication's.
/// **Send futures, declared not inferred.** Every method of this port returns
/// `impl Future<..> + Send` instead of being written `async fn`, and that is structural, not a
/// style choice: [`ServerState`](crate::gateway::ServerState) reaches this port behind an
/// instance's associated type, so the concrete future is opaque at the call site and axum's
/// handler and socket tasks — which are `Send` by construction — cannot otherwise prove it may
/// cross a thread. An `async fn` here compiles and then fails at every route that uses it. The
/// implementations stay ordinary `async fn`, which Rust accepts against this signature, and so does
/// the delegate `dyn_enum_close!` generates for a set of them: the macro emits `async fn .. -> T`
/// over the future's `Output`, because two match arms cannot unify two distinct opaque futures.
#[dyn_enum]
pub trait PrincipalResolver: Send + Sync {
    /// 🔍️ Recognize this credential, or decline so the next rung may try.
    fn resolve(&self, credential: &Credential) -> impl Future<Output = Option<Resolved>> + Send;

    /// 🏷️ Stable rung name, reported as [`Resolved::via`].
    fn name(&self) -> impl Future<Output = &str> + Send;
}

/// ⛓️ The ladder itself — generic here, its rungs supplied by the instance. The framework owns the
/// order-and-fallback mechanism; a product contributes the rungs it actually has (a bearer session
/// resolver, a share-token resolver reading a [`CapabilityProof`], a public-visibility resolver),
/// and no rung is hard-coded into this crate. First match wins, so the most specific rung is pushed
/// first and the broadest last.
///
/// `R` is the instance's rung type — [`ServerInstance::Resolvers`](crate::gateway::ServerInstance::
/// Resolvers). An instance with one rung names that rung directly; an instance with several closes
/// them into one enum in its own crate. This module deliberately holds no
/// rung of its own, so there is nothing here for an instance to inherit or work around.
pub struct ResolverChain<R: PrincipalResolver> {
    pub resolvers: Vec<R>,
}

impl<R: PrincipalResolver> Default for ResolverChain<R> {
    /// 🌿️ An empty ladder — `derive(Default)` would demand `R: Default`, which a rung carrying a
    /// configured token never is.
    fn default() -> Self {
        Self { resolvers: Vec::new() }
    }
}

impl<R: PrincipalResolver> ResolverChain<R> {
    /// 🌿️ An empty ladder, which resolves everything to [`Principal::Anonymous`].
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Append a rung below every rung already pushed.
    pub fn push(&mut self, resolver: R) {
        self.resolvers.push(resolver);
    }

    /// 🧭️ Walk the rungs in order and take the first that recognizes the credential. Falling off
    /// the bottom is not an error: an unrecognized caller is anonymous, and anonymous is a perfectly
    /// ordinary principal that policy will then almost certainly deny.
    pub async fn resolve(&self, credential: &Credential) -> Resolved {
        for resolver in &self.resolvers {
            if let Some(resolved) = resolver.resolve(credential).await {
                return resolved;
            }
        }
        Resolved { principal: Principal::Anonymous, session: None, device: None, via: "anonymous".to_string(), actor: None }
    }
}
//#endregion 🔖️Resolver

//#region 🔖️Admin
/// 🚪️ The gate in front of the administration plane, checked before policy is consulted at all.
///
/// Deliberately conservative and deliberately dull. With a token configured, only a bearer equal to
/// it passes — no roles, no templates, no resolver ladder. With no token configured, only a
/// loopback peer passes: the dev-default has to be usable on a laptop with zero setup, and it must
/// be worthless the moment the process is reachable from anywhere else. There is no third mode, and
/// in particular no "no token means open", because the failure mode of that default is an exposed
/// administration plane on the first deployment somebody forgets to configure.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminGate {
    pub token: Option<String>,
}

impl AdminGate {
    /// 🔐️ A gate over an optional shared token; `None` selects loopback-only mode.
    pub fn new(token: Option<String>) -> Self {
        Self { token }
    }

    /// 🛂️ Whether this credential may reach the administration plane.
    pub fn allows(&self, credential: &Credential) -> bool {
        match &self.token {
            Some(token) => credential.bearer.as_deref() == Some(token.as_str()),
            None => credential.loopback,
        }
    }

    /// 🔎️ Whether a token is configured, i.e. whether the gate is in shared-token mode rather than
    /// the loopback-only dev default.
    pub fn is_configured(&self) -> bool {
        self.token.is_some()
    }
}
//#endregion 🔖️Admin

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
