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

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use serde::{Deserialize, Serialize};

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
        self.assignments.get(key).into_iter().flatten().filter(|assignment| assignment.applies(scope)).filter_map(|assignment| self.templates.get(&assignment.template)).flat_map(|template| template.grants.iter()).collect()
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
}

/// 🪜️ One rung of the authentication ladder. Returning `None` means "not mine", never "denied":
/// refusal is policy's job at [`PolicyEngine::evaluate`], not authentication's.
#[dyn_enum]
pub trait PrincipalResolver: Send + Sync {
    /// 🔍️ Recognize this credential, or decline so the next rung may try.
    async fn resolve(&self, credential: &Credential) -> Option<Resolved>;

    /// 🏷️ Stable rung name, reported as [`Resolved::via`].
    async fn name(&self) -> &str;
}

/// 🪜️ A minimal reference [`PrincipalResolver`] rung: recognizes exactly one configured bearer
/// token. Kept in production scope (not only in tests) so [`PrincipalResolvers`] closes over a real
/// variant; a product's own rungs (session cookie, share-token, public-visibility) are added as
/// further `PrincipalResolvers` variants alongside it.
pub struct BearerTokenResolver {
    pub name: String,
    pub bearer: String,
    pub principal: Principal,
}

impl PrincipalResolver for BearerTokenResolver {
    async fn resolve(&self, credential: &Credential) -> Option<Resolved> {
        if credential.bearer.as_deref() != Some(self.bearer.as_str()) {
            return None;
        }
        Some(Resolved { principal: self.principal.clone(), session: Some(SessionId(format!("session-{}", self.name))), device: Some(DeviceId("d1".to_string())), via: self.name.clone() })
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

dyn_enum_close! {
    pub enum PrincipalResolvers: PrincipalResolver {
        BearerToken(BearerTokenResolver),
    }
}

/// ⛓️ The ladder itself — generic here, its rungs supplied by the instance. The framework owns the
/// order-and-fallback mechanism; a product contributes the rungs it actually has (a bearer session
/// resolver, a share-token resolver reading a [`CapabilityProof`], a public-visibility resolver),
/// and no rung is hard-coded into this crate. First match wins, so the most specific rung is pushed
/// first and the broadest last.
#[derive(Default)]
pub struct ResolverChain {
    pub resolvers: Vec<PrincipalResolvers>,
}

impl ResolverChain {
    /// 🌿️ An empty ladder, which resolves everything to [`Principal::Anonymous`].
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Append a rung below every rung already pushed.
    pub fn push(&mut self, resolver: PrincipalResolvers) {
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
        Resolved { principal: Principal::Anonymous, session: None, device: None, via: "anonymous".to_string() }
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
mod tests {
    use super::*;

    fn template(name: &str, grants: &[(PolicyPoint, &str, &str)]) -> PolicyTemplate {
        PolicyTemplate { name: name.to_string(), grants: grants.iter().map(|(point, resource, action)| PolicyGrant { point: *point, resource: (*resource).to_string(), action: (*action).to_string() }).collect() }
    }

    fn alice() -> Principal {
        Principal::User { id: "alice".to_string() }
    }

    fn request(point: PolicyPoint, principal: Principal, scope: Option<&str>, resource: &str, action: &str) -> PolicyRequest {
        PolicyRequest { point, principal, scope: scope.map(|scope| Scope(scope.to_string())), resource: resource.to_string(), action: action.to_string() }
    }

    fn resolver(name: &str, bearer: &str, principal: Principal) -> PrincipalResolvers {
        PrincipalResolvers::BearerToken(BearerTokenResolver { name: name.to_string(), bearer: bearer.to_string(), principal })
    }

    //#region 🔖️ClosedByDefault
    #[test]
    fn empty_engine_denies_every_request() {
        let engine = PolicyEngine::new();
        let decision = engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), Some("space-1"), "doc-1", "read"));
        assert!(!decision.is_allowed());
        assert!(matches!(&decision, PolicyDecision::Deny { reason } if reason.contains("closed by default") && reason.contains("user:alice")));
    }

    #[test]
    fn assignment_to_an_unregistered_template_grants_nothing() {
        let mut engine = PolicyEngine::new();
        engine.assign("user:alice".to_string(), "editorr".to_string());
        assert!(!engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "doc-1", "read")).is_allowed());
    }

    #[test]
    fn a_matching_grant_at_another_point_does_not_leak() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("viewer", &[(PolicyPoint::QueryAccess, "*", "*")]));
        engine.assign("user:alice".to_string(), "viewer".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "doc-1", "read")).is_allowed());
        assert!(!engine.evaluate(&request(PolicyPoint::BlobWrite, alice(), None, "doc-1", "read")).is_allowed());
    }

    #[test]
    fn anonymous_gets_its_own_key_and_may_be_granted_like_anyone() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("public", &[(PolicyPoint::QueryAccess, "space:pub/*", "read")]));
        assert!(!engine.evaluate(&request(PolicyPoint::QueryAccess, Principal::Anonymous, None, "space:pub/doc", "read")).is_allowed());
        engine.assign("anonymous".to_string(), "public".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, Principal::Anonymous, None, "space:pub/doc", "read")).is_allowed());
    }
    //#endregion 🔖️ClosedByDefault

    //#region 🔖️Matching
    #[test]
    fn principal_key_is_stable_per_variant() {
        assert_eq!(principal_key(&alice()), "user:alice");
        assert_eq!(principal_key(&Principal::ServiceAccount { id: "indexer".to_string() }), "service:indexer");
        assert_eq!(principal_key(&Principal::Device { id: "d1".to_string() }), "device:d1");
        assert_eq!(principal_key(&Principal::Anonymous), "anonymous");
    }

    #[test]
    fn exact_resources_match_only_themselves() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("one_doc", &[(PolicyPoint::QueryAccess, "space:abc/doc-1", "read")]));
        engine.assign("user:alice".to_string(), "one_doc".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:abc/doc-1", "read")).is_allowed());
        assert!(!engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:abc/doc-2", "read")).is_allowed());
        assert!(!engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:abc/doc-1/child", "read")).is_allowed());
    }

    #[test]
    fn trailing_star_is_a_prefix_wildcard() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("space_abc", &[(PolicyPoint::QueryAccess, "space:abc/*", "read")]));
        engine.assign("user:alice".to_string(), "space_abc".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:abc/doc-1", "read")).is_allowed());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:abc/nested/doc-2", "read")).is_allowed());
        assert!(!engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:xyz/doc-1", "read")).is_allowed());
    }

    #[test]
    fn bare_star_matches_every_resource_and_every_action() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("admin", &[(PolicyPoint::CommandAdmission, "*", "*")]));
        engine.assign("user:alice".to_string(), "admin".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), None, "anything/at/all", "delete")).is_allowed());
        assert!(engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), Some("space-9"), "", "publish")).is_allowed());
    }

    #[test]
    fn a_grant_for_another_action_does_not_match() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("viewer", &[(PolicyPoint::CommandAdmission, "space:abc/*", "read")]));
        engine.assign("user:alice".to_string(), "viewer".to_string());
        assert!(!engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), None, "space:abc/doc-1", "write")).is_allowed());
    }

    #[test]
    fn several_templates_on_one_principal_are_unioned() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("reader", &[(PolicyPoint::QueryAccess, "space:abc/*", "read")]));
        engine.register_template(template("writer", &[(PolicyPoint::CommandAdmission, "space:abc/*", "write")]));
        engine.assign("user:alice".to_string(), "reader".to_string());
        engine.assign("user:alice".to_string(), "writer".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:abc/doc-1", "read")).is_allowed());
        assert!(engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), None, "space:abc/doc-1", "write")).is_allowed());
    }
    //#endregion 🔖️Matching

    //#region 🔖️DenyOverrides
    #[test]
    fn an_explicit_deny_beats_a_matching_allow() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("editor", &[(PolicyPoint::CommandAdmission, "space:abc/*", "*")]));
        engine.register_template(template("frozen", &[(PolicyPoint::CommandAdmission, "space:abc/locked", "!write")]));
        engine.assign("user:alice".to_string(), "editor".to_string());
        engine.assign("user:alice".to_string(), "frozen".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), None, "space:abc/doc-1", "write")).is_allowed());
        let decision = engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), None, "space:abc/locked", "write"));
        assert!(matches!(&decision, PolicyDecision::Deny { reason } if reason.contains("explicit deny")));
        assert!(engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), None, "space:abc/locked", "read")).is_allowed());
    }

    #[test]
    fn deny_wins_regardless_of_registration_order() {
        let allow = template("editor", &[(PolicyPoint::BlobWrite, "*", "write")]);
        let deny = template("frozen", &[(PolicyPoint::BlobWrite, "*", "!write")]);
        for (first, second) in [(&allow, &deny), (&deny, &allow)] {
            let mut engine = PolicyEngine::new();
            engine.register_template(first.clone());
            engine.register_template(second.clone());
            engine.assign("user:alice".to_string(), first.name.clone());
            engine.assign("user:alice".to_string(), second.name.clone());
            assert!(!engine.evaluate(&request(PolicyPoint::BlobWrite, alice(), None, "blob-1", "write")).is_allowed());
        }
    }

    #[test]
    fn a_deny_all_action_blocks_every_action_on_the_matched_resource() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("admin", &[(PolicyPoint::QueryAccess, "*", "*")]));
        engine.register_template(template("quarantine", &[(PolicyPoint::QueryAccess, "space:secret/*", "!*")]));
        engine.assign("user:alice".to_string(), "admin".to_string());
        engine.assign("user:alice".to_string(), "quarantine".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:open/doc", "read")).is_allowed());
        assert!(!engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:secret/doc", "read")).is_allowed());
        assert!(!engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "space:secret/doc", "list")).is_allowed());
    }
    //#endregion 🔖️DenyOverrides

    //#region 🔖️Scope
    #[test]
    fn a_scoped_assignment_does_not_leak_into_another_scope() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("editor", &[(PolicyPoint::CommandAdmission, "*", "write")]));
        engine.assign_scoped("user:alice".to_string(), Scope("space-1".to_string()), "editor".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), Some("space-1"), "doc-1", "write")).is_allowed());
        assert!(!engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), Some("space-2"), "doc-1", "write")).is_allowed());
    }

    #[test]
    fn a_scoped_assignment_does_not_answer_instance_wide_requests() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("admin", &[(PolicyPoint::Administration, "*", "*")]));
        engine.assign_scoped("user:alice".to_string(), Scope("space-1".to_string()), "admin".to_string());
        assert!(!engine.evaluate(&request(PolicyPoint::Administration, alice(), None, "instance", "restart")).is_allowed());
    }

    #[test]
    fn an_unscoped_assignment_answers_scoped_requests() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("viewer", &[(PolicyPoint::QueryAccess, "*", "read")]));
        engine.assign("user:alice".to_string(), "viewer".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), Some("space-7"), "doc-1", "read")).is_allowed());
        assert!(engine.evaluate(&request(PolicyPoint::QueryAccess, alice(), None, "doc-1", "read")).is_allowed());
    }

    #[test]
    fn a_scoped_deny_only_bites_inside_its_scope() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("editor", &[(PolicyPoint::CommandAdmission, "*", "write")]));
        engine.register_template(template("readonly", &[(PolicyPoint::CommandAdmission, "*", "!write")]));
        engine.assign("user:alice".to_string(), "editor".to_string());
        engine.assign_scoped("user:alice".to_string(), Scope("space-2".to_string()), "readonly".to_string());
        assert!(engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), Some("space-1"), "doc-1", "write")).is_allowed());
        assert!(!engine.evaluate(&request(PolicyPoint::CommandAdmission, alice(), Some("space-2"), "doc-1", "write")).is_allowed());
    }

    #[test]
    fn assignments_are_per_principal() {
        let mut engine = PolicyEngine::new();
        engine.register_template(template("editor", &[(PolicyPoint::CommandAdmission, "*", "write")]));
        engine.assign("user:alice".to_string(), "editor".to_string());
        let bob = Principal::User { id: "bob".to_string() };
        assert!(!engine.evaluate(&request(PolicyPoint::CommandAdmission, bob, None, "doc-1", "write")).is_allowed());
    }
    //#endregion 🔖️Scope

    //#region 🔖️Resolver
    #[semio_framework_async_macros::async_test]
    async fn the_chain_takes_the_first_rung_that_recognizes_a_credential() {
        let mut chain = ResolverChain::new();
        chain.push(resolver("session", "tok", Principal::User { id: "alice".to_string() }));
        chain.push(resolver("share", "tok", Principal::Anonymous));
        let resolved = chain.resolve(&Credential { bearer: Some("tok".to_string()), ..Default::default() }).await;
        assert_eq!(resolved.via, "session");
        assert_eq!(resolved.principal, alice());
        assert_eq!(resolved.session, Some(SessionId("session-session".to_string())));
        assert_eq!(resolved.device, Some(DeviceId("d1".to_string())));
    }

    #[semio_framework_async_macros::async_test]
    async fn a_later_rung_answers_what_an_earlier_one_declined() {
        let mut chain = ResolverChain::new();
        chain.push(resolver("session", "session-tok", alice()));
        chain.push(resolver("share", "share-tok", Principal::Device { id: "d9".to_string() }));
        let resolved = chain.resolve(&Credential { bearer: Some("share-tok".to_string()), ..Default::default() }).await;
        assert_eq!(resolved.via, "share");
        assert_eq!(resolved.principal, Principal::Device { id: "d9".to_string() });
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unrecognized_credential_falls_back_to_anonymous() {
        let mut chain = ResolverChain::new();
        chain.push(resolver("session", "session-tok", alice()));
        let resolved = chain.resolve(&Credential { bearer: Some("garbage".to_string()), ..Default::default() }).await;
        assert_eq!(resolved.principal, Principal::Anonymous);
        assert_eq!(resolved.via, "anonymous");
        assert_eq!(resolved.session, None);
        assert_eq!(resolved.device, None);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_chain_resolves_everything_to_anonymous() {
        let resolved = ResolverChain::new().resolve(&Credential::default()).await;
        assert_eq!(resolved.principal, Principal::Anonymous);
        assert_eq!(resolved.via, "anonymous");
    }

    #[semio_framework_async_macros::async_test]
    async fn rungs_report_their_own_name() {
        let rung = resolver("share", "tok", Principal::Anonymous);
        assert_eq!(rung.name().await, "share");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_anonymous_fallback_is_still_subject_to_policy() {
        let engine = PolicyEngine::new();
        let resolved = ResolverChain::new().resolve(&Credential::default()).await;
        let decision = engine.evaluate(&request(PolicyPoint::QueryAccess, resolved.principal, None, "doc-1", "read"));
        assert!(!decision.is_allowed());
    }
    //#endregion 🔖️Resolver

    //#region 🔖️Admin
    #[test]
    fn a_configured_gate_admits_only_the_matching_bearer() {
        let gate = AdminGate::new(Some("secret".to_string()));
        assert!(gate.is_configured());
        assert!(gate.allows(&Credential { bearer: Some("secret".to_string()), ..Default::default() }));
        assert!(!gate.allows(&Credential { bearer: Some("wrong".to_string()), ..Default::default() }));
        assert!(!gate.allows(&Credential::default()));
    }

    #[test]
    fn a_configured_gate_ignores_loopback() {
        let gate = AdminGate::new(Some("secret".to_string()));
        assert!(!gate.allows(&Credential { loopback: true, ..Default::default() }));
        assert!(gate.allows(&Credential { bearer: Some("secret".to_string()), loopback: false, ..Default::default() }));
    }

    #[test]
    fn an_unconfigured_gate_admits_only_loopback() {
        let gate = AdminGate::default();
        assert!(!gate.is_configured());
        assert!(gate.allows(&Credential { loopback: true, ..Default::default() }));
        assert!(!gate.allows(&Credential::default()));
        assert!(!gate.allows(&Credential { bearer: Some("anything".to_string()), loopback: false, ..Default::default() }));
    }
    //#endregion 🔖️Admin
}
