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
    assert_eq!(manifest.queries.iter().map(|query| query.kind.as_str()).collect::<Vec<_>>(), ["quiz.catalog", "quiz.learner", "quiz.run", "quiz.leaderboard"]);
    assert_eq!(manifest.actor_kinds, [ROSTER, LEARNER]);
    assert_eq!(manifest.policies.iter().map(|template| template.name.as_str()).collect::<Vec<_>>(), [LEARNER_TEMPLATE, PROCTOR_TEMPLATE]);
}

#[tokio::test]
async fn every_caller_may_learn_and_only_the_proctor_may_enroll() {
    let proctor = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: CrossOriginPolicy::LoopbackDevelopment, forwarding: Forwarding::Untrusted }, None).await.expect("assembled");
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

#[test]
fn the_cross_origin_grant_follows_the_policy() {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("https://evil.example"));
    grant(&mut headers, Some(&HeaderValue::from_static("https://evil.example")), &CrossOriginPolicy::Allowlist(vec!["https://quizze.example".into()]));
    assert!(!headers.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
    assert_eq!(headers[header::VARY], "Origin");
    grant(&mut headers, Some(&HeaderValue::from_static("https://quizze.example")), &CrossOriginPolicy::Allowlist(vec!["https://quizze.example".into()]));
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_ORIGIN], "https://quizze.example");
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_CREDENTIALS], "true");
}
