use super::*;
use crate::catalog::tests::fixture;
use crate::config::Forwarding;
use server::contract::{Principal, Scope};
use server::policy::PolicyRequest;

fn admission(principal: Principal, target: &str, action: &str) -> PolicyRequest {
    PolicyRequest { point: PolicyPoint::CommandAdmission, principal, scope: Some(Scope("proctor-fixture".into())), resource: target.into(), action: action.into() }
}

#[test]
fn the_manifest_declares_the_quiz_surface() {
    let manifest = manifest("proctor-fixture");
    assert_eq!(manifest.commands.iter().map(|command| command.kind.as_str()).collect::<Vec<_>>(), ["quiz.identify-learner", "quiz.start-run", "quiz.record-answer", "quiz.submit-run", ENROLL]);
    assert_eq!(manifest.queries.iter().map(|query| query.kind.as_str()).collect::<Vec<_>>(), ["quiz.catalog", "quiz.learner", "quiz.run", "quiz.leaderboard", "quiz.crowd"]);
    assert_eq!(manifest.actor_kinds, [ROSTER, LEARNER]);
    assert_eq!(manifest.policies.iter().map(|template| template.name.as_str()).collect::<Vec<_>>(), [LEARNER_TEMPLATE, PROCTOR_TEMPLATE, PRESENCE_TEMPLATE]);
}

#[tokio::test]
async fn every_caller_may_learn_and_only_the_proctor_may_enroll() {
    let proctor = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: CrossOriginPolicy::LoopbackDevelopment, forwarding: Forwarding::Untrusted }, PresenceSettings::default()).await.expect("assembled");
    assert_eq!(proctor.definition().id, INSTANCE_ID);
    let policy = proctor.state().policy.read().unwrap();
    let service = Principal::ServiceAccount { id: PROCTOR_SERVICE.into() };
    for (target, action) in [("quiz-roster/roster", "quiz.identify-learner"), ("quiz-learner/l1", "quiz.start-run"), ("quiz-learner/l1", "quiz.record-answer"), ("quiz-learner/l1", "quiz.submit-run")] {
        assert!(policy.evaluate(&admission(Principal::Anonymous, target, action)).is_allowed(), "{action}");
    }
    assert!(!policy.evaluate(&admission(Principal::Anonymous, "quiz-learner/l1", ENROLL)).is_allowed());
    assert!(!policy.evaluate(&admission(Principal::Anonymous, "quiz-roster/roster", "quiz.start-run")).is_allowed());
    assert!(policy.evaluate(&admission(service, "quiz-learner/l1", ENROLL)).is_allowed());
    let read = |resource: &str, point: PolicyPoint, action: &str| policy.evaluate(&PolicyRequest { point, principal: Principal::Anonymous, scope: None, resource: resource.into(), action: action.into() }).is_allowed();
    assert!(read("stream:proctor-fixture/quiz-learner/l1", PolicyPoint::EventDelivery, "read"));
    assert!(!read("stream:proctor-fixture/quiz-roster/roster", PolicyPoint::EventDelivery, "read"));
    assert!(policy.evaluate(&PolicyRequest { point: PolicyPoint::QueryAccess, principal: Principal::Anonymous, scope: Some(Scope("proctor-fixture".into())), resource: "quiz.leaderboard".into(), action: "read".into() }).is_allowed());
}

#[tokio::test]
async fn every_caller_may_share_presence_inside_the_catalog_rooms_only() {
    let proctor = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: site(), forwarding: Forwarding::Untrusted }, PresenceSettings::default()).await.expect("assembled");
    let presence = |principal: Principal, scope: &str, action: &str| proctor.state().policy.read().unwrap().evaluate(&PolicyRequest { point: PolicyPoint::Subscription, principal, scope: Some(Scope(scope.into())), resource: PRESENCE_RESOURCE.into(), action: action.into() }).is_allowed();
    for scope in ["proctor-fixture", "proctor-fixture/introduction", "proctor-fixture/home", "proctor-fixture/leaderboard", "proctor-fixture/badges", "proctor-fixture/quiz/power", "proctor-fixture/quiz/homes", "proctor-fixture/quiz/power/thinking", "proctor-fixture/quiz/homes/thinking"] {
        assert!([PRESENCE_JOIN, PRESENCE_PUBLISH, PRESENCE_WATCH].iter().all(|action| presence(Principal::Anonymous, scope, action)), "{scope}");
    }
    for scope in ["other-catalog", "proctor-fixture/quiz/cooling", "proctor-fixture/quiz/cooling/thinking", "proctor-fixture/identity", "proctor-fixture/learner", "proctor-fixture/preferences"] {
        assert!(!presence(Principal::Anonymous, scope, PRESENCE_JOIN) && !presence(Principal::Anonymous, scope, PRESENCE_WATCH), "{scope}");
    }
    assert!(!presence(Principal::Anonymous, "proctor-fixture", "moderate"));
    let admits = proctor.state().origin_admission.clone().expect("the cross-origin policy guards presence");
    assert!(admits("https://quizzes.example") && !admits("https://evil.example"));
    let module = ProctorModule::new(Arc::new(fixture()));
    let state = serde_json::json!({ "tag": "0a1b2c3d", "cursor": { "anchor": "card:power", "x": 0.5, "y": 0.5 } });
    assert_eq!(module.presence_admission(&Scope("proctor-fixture/home".into()), &state), Ok(()));
    assert_eq!(module.presence_admission(&Scope("proctor-fixture".into()), &state), Err(crate::presence::STATE_INVALID.to_string()));
    assert_eq!(module.presence_admission(&Scope("other-catalog/home".into()), &state), Err(crate::presence::SCOPE_UNKNOWN.to_string()));
}

fn site() -> CrossOriginPolicy {
    CrossOriginPolicy::Allowlist(vec!["https://quizzes.example".into()])
}

#[test]
fn the_cross_origin_grant_follows_the_policy() {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("https://evil.example"));
    assert!(!grant(&mut headers, Some(&HeaderValue::from_static("https://evil.example")), &site()));
    assert!(!headers.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
    assert!(!headers.contains_key(header::ACCESS_CONTROL_ALLOW_CREDENTIALS));
    assert_eq!(headers[header::VARY], "Origin");
    assert!(grant(&mut headers, Some(&HeaderValue::from_static("https://quizzes.example")), &site()));
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_ORIGIN], "https://quizzes.example");
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_CREDENTIALS], "true");
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_METHODS], "GET, POST, HEAD, OPTIONS");
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_HEADERS], "content-type");
    let mut bare = HeaderMap::new();
    assert!(!grant(&mut bare, None, &site()));
    assert!(!bare.contains_key(header::VARY) && !bare.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
}

#[test]
fn a_granted_preflight_is_cacheable_and_a_refused_one_is_not() {
    let granted = preflight(Some(&HeaderValue::from_static("https://quizzes.example")), &site());
    assert_eq!(granted.status(), StatusCode::NO_CONTENT);
    assert_eq!(granted.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN], "https://quizzes.example");
    assert_eq!(granted.headers()[header::ACCESS_CONTROL_MAX_AGE], PREFLIGHT_MAX_AGE);
    let refused = preflight(Some(&HeaderValue::from_static("https://evil.example")), &site());
    assert_eq!(refused.status(), StatusCode::NO_CONTENT);
    assert!(!refused.headers().contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
    assert!(!refused.headers().contains_key(header::ACCESS_CONTROL_MAX_AGE));
    assert_eq!(refused.headers()[header::VARY], "Origin");
}
