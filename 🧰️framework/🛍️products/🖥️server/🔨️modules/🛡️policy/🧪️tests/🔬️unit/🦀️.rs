
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
