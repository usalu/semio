use super::*;
use crate::catalog::tests::fixture;
use server::contract::{ActorKey, CommandId, HybridLogicalClock, IdempotencyKey, Principal, Scope, TenantId, TraceContext};
use server::policy::PolicyRequest;
use server::throttle::{Allowance, Limits, Rate};
use std::sync::atomic::AtomicUsize;

fn admission(principal: Principal, target: &str, action: &str) -> PolicyRequest {
    PolicyRequest { point: PolicyPoint::CommandAdmission, principal, scope: Some(Scope("proctor-fixture".into())), resource: target.into(), action: action.into() }
}

#[test]
fn the_manifest_declares_the_quiz_surface() {
    let manifest = manifest("proctor-fixture");
    assert_eq!(manifest.commands.iter().map(|command| command.kind.as_str()).collect::<Vec<_>>(), ["quiz.identify-learner", "quiz.start-run", "quiz.record-answer", "quiz.submit-run", ENROLL]);
    assert_eq!(manifest.commands.iter().map(|command| command.actor_kind.as_str()).collect::<Vec<_>>(), [HANDLE, LEARNER, LEARNER, LEARNER, LEARNER]);
    assert_eq!(manifest.queries.iter().map(|query| (query.kind.as_str(), query.projection.as_str())).collect::<Vec<_>>(), [("quiz.catalog", "quiz.catalog"), ("quiz.learner", LEARNERS), ("quiz.run", RUNS), ("quiz.leaderboard", LEADERBOARD), ("quiz.crowd", CROWDS), ("quiz.handle", HANDLES)]);
    assert_eq!(manifest.projections, [STATES, LEARNERS, RUNS, LEADERBOARD, HANDLES, TALLIES, CROWDS, META]);
    assert_eq!(manifest.actor_kinds, [HANDLE, LEARNER]);
    assert_eq!(manifest.policies.iter().map(|template| template.name.as_str()).collect::<Vec<_>>(), [LEARNER_TEMPLATE, PROCTOR_TEMPLATE, PRESENCE_TEMPLATE]);
}

#[tokio::test]
async fn every_caller_may_learn_and_only_the_proctor_may_enroll() {
    let proctor = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: CrossOriginPolicy::LoopbackDevelopment, forwarding: Forwarding::Untrusted, limits: Limits::default() }, PresenceSettings::default()).await.expect("assembled");
    assert_eq!(proctor.definition().id, INSTANCE_ID);
    let policy = proctor.state().policy.read().unwrap();
    let service = Principal::ServiceAccount { id: PROCTOR_SERVICE.into() };
    for (target, action) in [("quiz-handle/616461", "quiz.identify-learner"), ("quiz-learner/l1", "quiz.identify-learner"), ("quiz-learner/l1", "quiz.start-run"), ("quiz-learner/l1", "quiz.record-answer"), ("quiz-learner/l1", "quiz.submit-run")] {
        assert!(policy.evaluate(&admission(Principal::Anonymous, target, action)).is_allowed(), "{action}");
    }
    assert!(!policy.evaluate(&admission(Principal::Anonymous, "quiz-learner/l1", ENROLL)).is_allowed());
    assert!(!policy.evaluate(&admission(Principal::Anonymous, "quiz-handle/616461", "quiz.start-run")).is_allowed());
    assert!(!policy.evaluate(&admission(Principal::Anonymous, "quiz-handle/616461", ENROLL)).is_allowed());
    assert!(policy.evaluate(&admission(service.clone(), "quiz-learner/l1", ENROLL)).is_allowed());
    assert!(!policy.evaluate(&admission(service, "quiz-handle/616461", ENROLL)).is_allowed());
    let read = |resource: &str, point: PolicyPoint, action: &str| policy.evaluate(&PolicyRequest { point, principal: Principal::Anonymous, scope: None, resource: resource.into(), action: action.into() }).is_allowed();
    assert!(read("stream:proctor-fixture/quiz-learner/l1", PolicyPoint::EventDelivery, "read"));
    assert!(!read("stream:proctor-fixture/quiz-handle/616461", PolicyPoint::EventDelivery, "read"), "who holds a handle is answered by the handle read only");
    for query in QueryKind::ALL {
        assert!(policy.evaluate(&PolicyRequest { point: PolicyPoint::QueryAccess, principal: Principal::Anonymous, scope: Some(Scope("proctor-fixture".into())), resource: query.wire().into(), action: "read".into() }).is_allowed(), "{query:?}");
    }
}

#[tokio::test]
async fn the_admission_the_bus_asks_is_the_one_the_deciders_hold_and_carries_the_caps() {
    let proctor = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: site(), forwarding: Forwarding::Untrusted, limits: Limits::default() }, PresenceSettings::default()).await.expect("assembled");
    assert_eq!((proctor.admission().tenant(), proctor.admission().caps(), proctor.admission().learners()), ("proctor-fixture", quiz::DEFAULT_LIMITS, 0));
    let capped = proctor.capped(quiz::Limits { learners: 1, runs: 2, ..quiz::DEFAULT_LIMITS });
    assert_eq!(capped.admission().caps(), quiz::Limits { learners: 1, runs: 2, ..quiz::DEFAULT_LIMITS });
    let module = ProctorModule::new(Arc::new(fixture()), Arc::clone(capped.admission()));
    let id = "0123456789abcdef0123456789abcdef";
    let start = |learner: &str| {
        let command = quiz::Command::StartRun { id: id.into(), learner: learner.into(), run: id.into(), quiz: "power".into() };
        CommandEnvelope { payload: serde_json::to_vec(&command).expect("encoded"), ..addressed(LEARNER, learner, id, Some(id)) }
    };
    assert_eq!(module.command_admission(&start(id)), Ok(()));
    let megabyte = "a".repeat(1 << 20);
    for learner in ["", "roster", "0123456789ABCDEF0123456789ABCDEF", "0123456789abcdef0123456789abcde", "0123456789abcdef0123456789abcdef0", megabyte.as_str(), "enroll:proctor-fixture:roster:1"] {
        assert_eq!(module.command_admission(&start(learner)), Err(Rejection::Invalid { detail: "id-invalid".into() }), "learner {:?}", &learner[..learner.len().min(40)]);
    }
    assert!(module.command_admission(&addressed(LEARNER, id, id, Some(id))).is_err(), "an envelope without a quiz command is refused");
}

#[tokio::test]
async fn every_caller_may_share_presence_inside_the_catalog_rooms_only() {
    let proctor = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: site(), forwarding: Forwarding::Untrusted, limits: Limits::default() }, PresenceSettings::default()).await.expect("assembled");
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
    let module = ProctorModule::new(Arc::new(fixture()), Arc::clone(proctor.admission()));
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
    assert!(!headers.contains_key(header::ACCESS_CONTROL_ALLOW_CREDENTIALS), "the client sends no credentials, so none are allowed");
    assert_eq!(headers[header::ACCESS_CONTROL_EXPOSE_HEADERS], "retry-after", "a page of the site may read how long a refusal asks it to wait");
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_METHODS], "GET, POST, HEAD, OPTIONS");
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_HEADERS], "content-type");
    let mut bare = HeaderMap::new();
    assert!(!grant(&mut bare, None, &site()));
    assert!(!bare.contains_key(header::VARY) && !bare.contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
    let mut framework = HeaderMap::new();
    framework.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("true"));
    assert!(grant(&mut framework, Some(&HeaderValue::from_static("https://quizzes.example")), &site()));
    assert!(!framework.contains_key(header::ACCESS_CONTROL_ALLOW_CREDENTIALS), "a credentials grant from further in never leaves the gate");
}

#[tokio::test]
async fn the_gate_hands_its_limits_and_its_address_source_to_the_framework_edge() {
    let limits = Limits { writes: Rate::per_second(7, 11), allowances: vec![Allowance { name: SIGN_UP, rate: Rate::per_hour(3, 5) }], sockets: 5, body_bytes: Some(4096), ..Limits::default() };
    let direct = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: site(), forwarding: Forwarding::Untrusted, limits: limits.clone() }, PresenceSettings::default()).await.expect("assembled");
    assert_eq!((direct.state().throttle.limits(), direct.state().addressing), (&limits, ClientAddressing::Peer));
    let proxied = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: site(), forwarding: Forwarding::TerminatingProxy, limits }, PresenceSettings::default()).await.expect("assembled");
    assert_eq!(proxied.state().addressing, ClientAddressing::Forwarded, "behind the trusted proxy the client is the address it names");
    assert!(proxied.definition().modules[0].policies.len() == 3, "in process the definition is whole; GET /instance publishes it without the policy templates");
}

#[tokio::test]
async fn a_registration_is_the_one_command_counted_against_the_sign_up_allowance() {
    let proctor = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: site(), forwarding: Forwarding::Untrusted, limits: Limits::default() }, PresenceSettings::default()).await.expect("assembled");
    let module = ProctorModule::new(Arc::new(fixture()), Arc::clone(proctor.admission()));
    let id = "0123456789abcdef0123456789abcdef";
    let of = |kind: &str, actor: &str| CommandEnvelope { kind: kind.into(), ..addressed(actor, id, id, Some(id)) };
    assert_eq!(module.command_allowance(&of(IDENTIFY, LEARNER)), Some(SIGN_UP), "an anonymous registration");
    assert_eq!(module.command_allowance(&of(IDENTIFY, HANDLE)), Some(SIGN_UP), "a handle claim");
    for kind in ["quiz.start-run", "quiz.record-answer", "quiz.submit-run", ENROLL, "quiz.unknown"] {
        assert_eq!(module.command_allowance(&of(kind, LEARNER)), None, "{kind}");
    }
    assert_eq!((IDENTIFY, SIGN_UP), ("quiz.identify-learner", "sign-up"));
    let mislabelled = CommandEnvelope { payload: serde_json::to_vec(&quiz::Command::IdentifyLearner { id: id.into(), learner: id.into(), identity: quiz::Identity::Anonymous }).expect("encoded"), ..of("quiz.start-run", LEARNER) };
    assert_eq!(module.command_allowance(&mislabelled), None);
    assert!(module.command_admission(&mislabelled).is_err(), "a registration under another kind is counted nowhere because it is admitted nowhere");
}

fn addressed(kind: &str, id: &str, command: &str, idempotency: Option<&str>) -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId(command.into()),
        kind: "quiz.start-run".into(),
        version: 1,
        target: ActorKey { tenant: TenantId("proctor-fixture".into()), kind: kind.into(), id: id.into() },
        scope: Scope("proctor-fixture".into()),
        principal: Principal::Anonymous,
        session: None,
        device: None,
        payload: Vec::new(),
        causal_frontier: None,
        client_hlc: HybridLogicalClock::default(),
        expected_revision: None,
        idempotency_key: idempotency.map(|key| IdempotencyKey(key.into())),
        capability_proof: None,
        trace: TraceContext::default(),
    }
}

#[tokio::test]
async fn callers_arriving_together_share_one_settle_and_at_most_one_more() {
    let flight = Arc::new(Flight::default());
    let runs = Arc::new(AtomicUsize::new(0));
    let finish = Arc::new(tokio::sync::Notify::new());
    let caller = |flight: Arc<Flight>, runs: Arc<AtomicUsize>, finish: Arc<tokio::sync::Notify>| async move {
        flight
            .join(|| async move {
                runs.fetch_add(1, Ordering::AcqRel);
                finish.notified().await;
            })
            .await;
    };
    let first = tokio::spawn(caller(Arc::clone(&flight), Arc::clone(&runs), Arc::clone(&finish)));
    while runs.load(Ordering::Acquire) == 0 {
        tokio::task::yield_now().await;
    }
    let waiting: Vec<_> = (0..200).map(|_| tokio::spawn(caller(Arc::clone(&flight), Arc::clone(&runs), Arc::clone(&finish)))).collect();
    for _ in 0..4 {
        tokio::task::yield_now().await;
    }
    assert_eq!(runs.load(Ordering::Acquire), 1, "nobody runs beside the run under way");
    finish.notify_one();
    first.await.expect("the first run");
    finish.notify_one();
    for waiter in waiting {
        waiter.await.expect("a waiter");
    }
    assert_eq!(runs.load(Ordering::Acquire), 2, "two hundred callers that arrived during one run are served by the one run after it");
    finish.notify_one();
    caller(Arc::clone(&flight), Arc::clone(&runs), Arc::clone(&finish)).await;
    assert_eq!(runs.load(Ordering::Acquire), 3, "a caller that arrives when nothing runs gets a run of its own");
}

#[tokio::test]
async fn a_request_waits_only_for_what_was_committed_since_the_last_relay_and_fold() {
    let proctor = Proctor::assemble(StorageProfile::Ephemeral, Arc::new(fixture()), Gate { origins: site(), forwarding: Forwarding::Untrusted, limits: Limits::default() }, PresenceSettings::default()).await.expect("assembled");
    proctor.prepare().await.expect("prepared");
    let (settler, state) = (proctor.settler.clone(), proctor.state().clone());
    let owed = || state.committed();
    assert!(!settler.folding.is_current(owed().events) && !settler.relaying.is_current(owed().outbox), "a proctor that just started has settled nothing, whatever its log holds");
    settler.settle_quietly().await;
    assert!(settler.folding.is_current(0) && settler.relaying.is_current(0));

    let sign_up = |seed: u8, identity: quiz::Identity| {
        let (id, learner) = (format!("{:032x}", 0x100 + u32::from(seed)), format!("{:032x}", 0x200 + u32::from(seed)));
        let target = match &identity {
            quiz::Identity::Anonymous => (LEARNER, learner.clone()),
            quiz::Identity::Pseudonym { handle } | quiz::Identity::Name { handle } => (HANDLE, quiz::handle_actor_id(&quiz::normalize_handle(handle).expect("a handle").key)),
        };
        let command = quiz::Command::IdentifyLearner { id: id.clone(), learner: learner.clone(), identity };
        (learner, CommandEnvelope { kind: "quiz.identify-learner".into(), payload: serde_json::to_vec(&command).expect("encoded"), ..addressed(target.0, &target.1, &id, Some(&id)) })
    };

    let (_, anonymous) = sign_up(1, quiz::Identity::Anonymous);
    assert!(matches!(state.submit(anonymous.clone()).await, CommandOutcome::Accepted { .. }));
    assert!(!settler.relaying.is_current(owed().outbox), "the saga has not seen the registration yet");
    settler.relay_quietly().await;
    assert!(settler.relaying.is_current(owed().outbox) && !settler.folding.is_current(owed().events), "a command waits for the relay and not for the read models");
    assert_eq!(proctor.admission().learners(), 0);
    settler.settle_quietly().await;
    assert!(settler.folding.is_current(owed().events), "the query after it waits for the fold");
    assert_eq!(proctor.admission().learners(), 1);

    let settled = owed();
    assert!(matches!(state.submit(anonymous).await, CommandOutcome::Accepted { ref events, .. } if events.is_empty()), "the same command again commits nothing");
    assert_eq!(owed(), settled);
    assert!(settler.relaying.is_current(owed().outbox) && settler.folding.is_current(owed().events), "a command that committed nothing owes no relay and the query after it no fold");

    let (learner, named) = sign_up(2, quiz::Identity::Pseudonym { handle: "Ada Lovelace".into() });
    assert!(matches!(state.submit(named).await, CommandOutcome::Accepted { .. }));
    let stream = ActorKey { tenant: TenantId("proctor-fixture".into()), kind: LEARNER.into(), id: learner };
    assert!(state.replay_events(&stream, 0).await.expect("read").is_empty(), "a named sign-up owes its learner the relay");
    settler.relay_quietly().await;
    assert_eq!(state.replay_events(&stream, 0).await.expect("read").len(), 1, "when the relay returns the learner is registered");
    assert!(settler.relaying.is_current(owed().outbox), "and the registration the relay committed has been shown to the saga as well");
    assert!(!settler.folding.is_current(owed().events));
    settler.settle_quietly().await;
    assert!(settler.folding.is_current(owed().events));
    assert_eq!(proctor.admission().learners(), 2, "both registrations are counted once they are folded");
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
