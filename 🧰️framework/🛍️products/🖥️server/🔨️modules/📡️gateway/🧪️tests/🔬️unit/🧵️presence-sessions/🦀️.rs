//! 🧵️ Presence socket sessions, rosters and watched rooms of the gateway unit laws. See [`super`].

use super::*;

//#region 🔖️PresenceStream
#[tokio::test]
async fn presence_admission_needs_the_join_grant_a_short_surface_and_an_admitted_origin() {
    let state = state().await;
    let scope = Scope("space-1".into());
    let refused = admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect_err("closed by default");
    assert_eq!(refused.status(), StatusCode::FORBIDDEN);
    grant(&state, PolicyPoint::Subscription, PRESENCE_JOIN);
    assert!(!admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect("joins").may_publish, "join alone does not publish");
    grant(&state, PolicyPoint::Subscription, PRESENCE_PUBLISH);
    assert_eq!(admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect("joins and publishes"), PresenceGrant { principal: Principal::Anonymous, may_publish: true, client: caller() });
    let long = "x".repeat(65);
    assert_eq!(admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, &long).await.expect_err("too long").status(), StatusCode::BAD_REQUEST);

    let mut gated = state.clone();
    gated.origin_admission = Some(Arc::new(|origin: &str| origin == "https://quizzes.example"));
    let mut foreign = HeaderMap::new();
    foreign.insert(header::ORIGIN, HeaderValue::from_static("https://evil.example"));
    assert_eq!(admit_presence(&gated, &foreign, Some(loopback()), &scope, "home").await.expect_err("foreign origin").status(), StatusCode::FORBIDDEN);
    let mut site = HeaderMap::new();
    site.insert(header::ORIGIN, HeaderValue::from_static("https://quizzes.example"));
    assert!(admit_presence(&gated, &site, Some(loopback()), &scope, "home").await.is_ok());
    assert!(admit_presence(&gated, &HeaderMap::new(), Some(loopback()), &scope, "home").await.is_ok(), "a non-browser client presents no origin");
}

#[tokio::test]
async fn two_sessions_share_coalesced_state_and_see_each_other_leave() {
    let state = presence_server(quick_presence(Duration::from_secs(30))).await;
    let (alice, mut alice_frames, _alice_task) = connect(&state, "room-1", "home");
    let PresenceFrame::Welcome { session: alice_session, colour: alice_colour, roster } = next_frame(&mut alice_frames).await else { panic!("welcome first") };
    assert_eq!(roster.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>(), [alice_session.as_str()]);

    let (bob, mut bob_frames, bob_task) = connect(&state, "room-1", "leaderboard");
    let PresenceFrame::Welcome { session: bob_session, colour: bob_colour, roster } = next_frame(&mut bob_frames).await else { panic!("welcome first") };
    assert_ne!(alice_colour, bob_colour);
    assert_eq!(roster.len(), 2);
    assert!(roster.iter().any(|entry| entry.session == alice_session && entry.surface == "home"));

    for n in 0..5 {
        say(&alice, &format!(r#"{{"type":"state","state":{{"n":{n}}}}}"#));
    }
    let mut seen = Vec::new();
    while seen.last() != Some(&serde_json::json!({ "n": 4 })) {
        if let PresenceFrame::Batch { entries, .. } = next_frame(&mut bob_frames).await {
            seen.extend(entries.into_iter().filter(|entry| entry.session == alice_session).map(|entry| entry.state).filter(|state| !state.is_null()));
        }
    }
    assert!(seen.len() <= 2, "five states sent within one tick arrive coalesced: {seen:?}");

    say(&alice, r#"{"type":"state","state":{"refuse":"not-here"}}"#);
    let mut refusal = next_frame(&mut alice_frames).await;
    while let PresenceFrame::Batch { .. } = refusal {
        refusal = next_frame(&mut alice_frames).await;
    }
    assert_eq!(refusal, PresenceFrame::Refused { reason: "not-here".into() });
    alice.unbounded_send(Ok(Message::Binary(vec![1, 2].into()))).expect("open");
    assert_eq!(next_frame(&mut alice_frames).await, PresenceFrame::Refused { reason: REFUSED_INVALID.into() });

    drop(bob);
    tokio::time::timeout(Duration::from_secs(5), bob_task).await.expect("bob's session ends").expect("no panic");
    loop {
        if let PresenceFrame::Batch { left, .. } = next_frame(&mut alice_frames).await {
            if left.contains(&bob_session) {
                break;
            }
        }
    }
    assert_eq!(state.presence_rooms.roster(&presence_lane(&Scope("room-1".into()))).len(), 1);
    assert_eq!(state.presence.colour_of(&presence_lane(&Scope("room-1".into())), &bob_session), None);
}

#[tokio::test]
async fn an_idle_session_is_closed_and_leaves_the_room() {
    let state = presence_server(quick_presence(Duration::from_millis(150))).await;
    let (_silent, mut frames, task) = connect(&state, "room-idle", "home");
    assert!(matches!(next_frame(&mut frames).await, PresenceFrame::Welcome { .. }));
    tokio::time::timeout(Duration::from_secs(5), task).await.expect("the idle session is closed").expect("no panic");
    let rest: Vec<Message> = frames.collect().await;
    assert!(matches!(rest.last(), Some(Message::Close(_))), "{rest:?}");
    assert!(state.presence_rooms.roster(&presence_lane(&Scope("room-idle".into()))).is_empty());
}

fn scopes(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

pub(super) fn watched(scope: &str, entries: Vec<PresenceEntry>, left: &[&str], snapshot: bool) -> PresenceFrame {
    PresenceFrame::Watched { scope: Scope(scope.into()), entries, left: left.iter().map(|session| (*session).to_string()).collect(), snapshot }
}

/// 📏️ The bytes of entries and departures a server frame carries.
fn carried_bytes(frame: &PresenceFrame) -> usize {
    let (entries, left) = match frame {
        PresenceFrame::Welcome { roster, .. } => (roster, None),
        PresenceFrame::Batch { entries, left } | PresenceFrame::Watched { entries, left, .. } => (entries, Some(left)),
        _ => return 0,
    };
    let listed = |texts: Vec<String>| texts.iter().map(String::len).sum::<usize>() + texts.len().saturating_sub(1);
    listed(entries.iter().map(|entry| serde_json::to_string(entry).expect("an entry")).collect()) + listed(left.into_iter().flatten().map(|session| serde_json::to_string(session).expect("a session")).collect())
}

#[tokio::test]
async fn a_roster_larger_than_a_frame_arrives_over_several_frames_and_so_does_a_watched_one() {
    let frame_bytes = 400;
    let state = presence_server(PresenceSettings { max_frame_bytes: frame_bytes, ..quick_presence(Duration::from_secs(30)) }).await;
    grant_scoped(&state, PRESENCE_WATCH, &["hall"]);
    let mut members = Vec::new();
    for _ in 0..12 {
        let (client, mut frames, _task) = connect(&state, "hall", "home");
        assert!(matches!(next_frame(&mut frames).await, PresenceFrame::Welcome { .. }));
        members.push((client, frames));
    }
    let mut everyone: Vec<String> = state.presence_rooms.roster(&presence_lane(&Scope("hall".into()))).into_iter().map(|entry| entry.session).collect();

    let (_late, mut late_frames, _late_task) = connect(&state, "hall", "home");
    let PresenceFrame::Welcome { session: late_session, roster, .. } = next_frame(&mut late_frames).await else { panic!("welcome first") };
    assert_eq!(roster.first().map(|entry| entry.session.as_str()), Some(late_session.as_str()), "the welcome is led by the joiner");
    assert!(roster.len() < 13, "thirteen entries do not fit a frame of {frame_bytes} bytes");
    let mut known: BTreeSet<String> = roster.into_iter().map(|entry| entry.session).collect();
    let mut frames = 1;
    while known.len() < 13 {
        let frame = next_frame(&mut late_frames).await;
        assert!(carried_bytes(&frame) <= frame_bytes, "{frame:?}");
        let PresenceFrame::Batch { entries, .. } = frame else { panic!("the roster continues in batches") };
        known.extend(entries.into_iter().map(|entry| entry.session));
        frames += 1;
    }
    everyone.push(late_session);
    assert_eq!(known, everyone.into_iter().collect::<BTreeSet<_>>());
    assert!(frames >= 3, "the roster took {frames} frames");

    let (watcher, mut watcher_frames, _watcher_task) = connect(&state, "lobby", "home");
    assert!(matches!(next_frame(&mut watcher_frames).await, PresenceFrame::Welcome { .. }));
    say(&watcher, r#"{"type":"watch","scopes":["hall"],"intervalMs":40}"#);
    let PresenceFrame::Watched { entries, snapshot: true, .. } = watched_frame(&mut watcher_frames).await else { panic!("a snapshot first") };
    let mut seen: BTreeSet<String> = entries.into_iter().map(|entry| entry.session).collect();
    while seen.len() < 13 {
        let frame = watched_frame(&mut watcher_frames).await;
        assert!(carried_bytes(&frame) <= frame_bytes, "{frame:?}");
        let PresenceFrame::Watched { entries, snapshot: false, .. } = frame else { panic!("the snapshot continues in changes") };
        seen.extend(entries.into_iter().map(|entry| entry.session));
    }
    assert_eq!(seen, known);
}

#[tokio::test]
async fn a_peer_that_takes_no_frame_is_let_go_and_holds_nothing() {
    let state = presence_server(quick_presence(Duration::from_millis(300))).await;
    let (client, mut inbound) = unbounded::<Result<Message, std::convert::Infallible>>();
    let (mut outbound, _never_read) = futures::channel::mpsc::channel::<Message>(0);
    let scope = Scope("stuck".to_string());
    let session = state.clone();
    let stuck = tokio::spawn(async move { run_presence(&session, &scope, "home", &PresenceGrant { principal: Principal::Anonymous, may_publish: true, client: caller() }, &mut outbound, &mut inbound).await });
    let chatter = tokio::spawn(async move {
        let mut n = 0;
        while client.unbounded_send(Ok(Message::Text(format!(r#"{{"type":"state","state":{n}}}"#).into()))).is_ok() {
            n += 1;
            tokio::time::sleep(Duration::from_millis(40)).await;
        }
    });
    tokio::time::timeout(Duration::from_secs(5), stuck).await.expect("a session that keeps talking and takes none of its own echoes ends").expect("no panic");
    tokio::time::timeout(Duration::from_secs(5), chatter).await.expect("its peer finds the socket closed").expect("no panic");
    assert_eq!(state.presence_rooms.rooms(), 0, "and it left the room");
}

#[test]
fn the_watch_interval_is_clamped_between_the_tick_and_the_slowest_interval() {
    let settings = PresenceSettings::default();
    assert_eq!(watch_interval(0, &settings), settings.tick);
    assert_eq!(watch_interval(40, &settings), settings.tick);
    assert_eq!(watch_interval(250, &settings), Duration::from_millis(250));
    assert_eq!(watch_interval(u64::MAX, &settings), settings.max_watch_interval);
    let mut budget = StateBudget::new(settings.max_states_per_second);
    assert_eq!(admit_frame(r#"{"type":"watch","scopes":["a"],"intervalMs":250}"#, &settings, &mut budget, Instant::now()), Ok(Some(PresenceFrame::Watch { scopes: vec![Scope("a".into())], interval_ms: 250 })));
    assert_eq!(admit_frame(r#"{"type":"watched","scope":"a","entries":[],"left":[]}"#, &settings, &mut budget, Instant::now()), Err(REFUSED_INVALID.to_string()), "watched travels server → client only");
}

fn grant_scoped(state: &ServerState<TestInstance>, action: &str, scopes: &[&str]) {
    let mut engine = state.policy.write().unwrap();
    let name = format!("presence-{action}");
    engine.register_template(PolicyTemplate { name: name.clone(), auto_apply: false, grants: vec![PolicyGrant { point: PolicyPoint::Subscription, resource: PRESENCE_RESOURCE.into(), action: action.to_string() }] });
    for scope in scopes {
        engine.assign_scoped("anonymous".to_string(), Scope((*scope).to_string()), name.clone());
    }
}

#[tokio::test]
async fn a_watch_is_admitted_per_scope_and_bounded() {
    let state = state().await;
    let wanted = [Scope("room-a".into()), Scope("room-b".into()), Scope("room-a".into())];
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Err("forbidden room-a".to_string()), "closed by default");
    grant_scoped(&state, PRESENCE_WATCH, &["room-a"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Err("forbidden room-b".to_string()));
    grant_scoped(&state, PRESENCE_WATCH, &["room-b"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Ok(scopes(&["room-a", "room-b"])));
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &[]), Ok(BTreeSet::new()), "an empty watch stops watching");
    let many: Vec<Scope> = (0..=state.presence_settings.max_watch_scopes).map(|_| Scope("room-a".into())).collect();
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &many), Err(REFUSED_WATCH_TOO_MANY.to_string()));
    grant_scoped(&state, PRESENCE_JOIN, &["room-c"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &[Scope("room-c".into())]), Err("forbidden room-c".to_string()), "joining is not watching");
}

async fn watched_frame(frames: &mut ServerFrames) -> PresenceFrame {
    loop {
        let frame = next_frame(frames).await;
        if matches!(frame, PresenceFrame::Watched { .. } | PresenceFrame::Refused { .. }) {
            return frame;
        }
    }
}

#[tokio::test]
async fn a_socket_watches_other_rooms_read_only_at_its_own_interval() {
    let state = presence_server(quick_presence(Duration::from_secs(30))).await;
    grant_scoped(&state, PRESENCE_WATCH, &["room-a", "room-b"]);
    let (alice, mut alice_frames, _alice_task) = connect(&state, "room-a", "home");
    let PresenceFrame::Welcome { session: alice_session, .. } = next_frame(&mut alice_frames).await else { panic!("welcome") };
    let (bob, mut bob_frames, bob_task) = connect(&state, "room-b", "home");
    let PresenceFrame::Welcome { session: bob_session, .. } = next_frame(&mut bob_frames).await else { panic!("welcome") };
    let (carol, mut carol_frames, carol_task) = connect(&state, "room-c", "home");
    let PresenceFrame::Welcome { session: carol_session, .. } = next_frame(&mut carol_frames).await else { panic!("welcome") };

    say(&carol, r#"{"type":"watch","scopes":["room-b","room-a"],"intervalMs":80}"#);
    let PresenceFrame::Watched { scope, entries, left, snapshot: true } = watched_frame(&mut carol_frames).await else { panic!("a snapshot") };
    assert_eq!((scope.0.as_str(), entries.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>(), left.len()), ("room-a", vec![alice_session.as_str()], 0));
    let PresenceFrame::Watched { scope, entries, snapshot: true, .. } = watched_frame(&mut carol_frames).await else { panic!("a snapshot") };
    assert_eq!((scope.0.as_str(), entries.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>()), ("room-b", vec![bob_session.as_str()]));

    let started = Instant::now();
    for n in 0..8 {
        say(&alice, &format!(r#"{{"type":"state","state":{{"n":{n}}}}}"#));
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let intervals = started.elapsed().as_millis() / 80 + 2;
    let mut seen = Vec::new();
    while seen.last() != Some(&serde_json::json!({ "n": 7 })) {
        let PresenceFrame::Watched { scope, entries, snapshot: false, .. } = watched_frame(&mut carol_frames).await else { panic!("a coalesced change") };
        if scope.0 == "room-a" {
            seen.extend(entries.into_iter().filter(|entry| entry.session == alice_session && !entry.state.is_null()).map(|entry| entry.state));
        }
    }
    assert!(seen.len() < 8 && seen.len() as u128 <= intervals, "eight states reach an 80 ms watcher at most once per interval ({intervals}): {seen:?}");

    say(&carol, r#"{"type":"state","state":{"here":"room-c"}}"#);
    say(&carol, r#"{"type":"watch","scopes":["room-a","room-z"],"intervalMs":80}"#);
    assert_eq!(watched_frame(&mut carol_frames).await, PresenceFrame::Refused { reason: "forbidden room-z".into() });
    assert!(state.presence_rooms.roster(&presence_lane(&Scope("room-a".into()))).iter().all(|entry| entry.session != carol_session), "watching never joins");

    drop(bob);
    tokio::time::timeout(Duration::from_secs(5), bob_task).await.expect("bob's session ends").expect("no panic");
    assert_eq!(watched_frame(&mut carol_frames).await, watched("room-b", Vec::new(), &[bob_session.as_str()], false), "a refused watch kept the previous set");

    say(&carol, r#"{"type":"watch","scopes":[],"intervalMs":80}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;
    say(&alice, r#"{"type":"state","state":{"n":8}}"#);
    let quiet = tokio::time::timeout(Duration::from_millis(400), watched_frame(&mut carol_frames)).await;
    assert!(quiet.is_err(), "an empty watch stops watching: {quiet:?}");

    drop((alice, carol));
    tokio::time::timeout(Duration::from_secs(5), carol_task).await.expect("carol's session ends").expect("no panic");
    let deadline = Instant::now() + Duration::from_secs(5);
    while state.presence_rooms.rooms() > 0 && Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert_eq!(state.presence_rooms.rooms(), 0, "closing releases every room the socket joined or watched");
}
//#endregion 🔖️PresenceStream
