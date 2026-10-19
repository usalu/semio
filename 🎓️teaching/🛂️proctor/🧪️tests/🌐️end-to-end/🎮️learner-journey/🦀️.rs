//! 🎮️ The proctor end to end, the learner journey: a learner plays through the real API and the facts survive a restart, the API serves the CDN site across origins, learners share presence and cursors, a production proctor opens presence to the site origin only and learners see what the others think live.
//!
//! @see ../🦀️.rs — the end to end suite that mounts this module
//! @see ../../../🧫️fixtures/📚️course/🔣️.json — the course played

use super::*;

#[test]
fn a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart() {
    let data = scratch("lifecycle");
    let running = boot(&data.0, development());
    let base = running.base.clone();

    let (status, _, instance) = get(&base, "/instance", &[]);
    let instance: Value = serde_json::from_str(&instance).expect("instance json");
    assert_eq!((status, instance["id"].as_str(), instance["modules"][0]["id"].as_str()), (200, Some("teaching-proctor"), Some("teaching.proctor")));
    let declared: Vec<(String, u64)> = ["commands", "queries"].iter().flat_map(|group| instance["modules"][0][group].as_array().expect("kinds").iter().map(|kind| (kind["kind"].as_str().expect("kind").to_string(), kind["version"].as_u64().expect("version")))).collect();
    assert!(declared.len() == 12 && declared.iter().all(|(kind, version)| kind.starts_with("course.") && *version == u64::from(course::WIRE_VERSION)), "a client agrees on the contract by the versions GET /instance declares: {declared:?}");

    let course = view(&base, &json!({ "type": "course" }));
    assert_eq!(course["id"], TENANT);
    assert_eq!(course["quizzes"].as_array().expect("quizzes").iter().map(|quiz| quiz["id"].as_str().expect("id")).collect::<Vec<_>>(), ["power", "homes"]);
    assert!(!course.to_string().contains("\"value\""), "the course view carries no solutions");

    let (anonymous, ada, other) = (id(0xa1), id(0xada), id(0xc3));
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(1), "learner": anonymous, "identity": { "kind": "anonymous" } }));
    assert_eq!(events, [json!({ "type": "learner-registered", "learner": anonymous, "identity": { "kind": "anonymous" }, "at": events[0]["at"] })]);
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(2), "learner": ada, "identity": { "kind": "pseudonym", "handle": "  Ada   Lovelace " } }));
    assert_eq!(events[0]["identity"], json!({ "kind": "pseudonym", "handle": "Ada Lovelace" }));
    assert_eq!(get(&base, &format!("/actors/{TENANT}/course-handle/{}/events", course::handle_actor_id("ada lovelace")), &[]).0, 403, "who holds a handle is not readable as a stream");
    assert_eq!(rejected(&base, &json!({ "type": "identify-learner", "id": id(3), "learner": other, "identity": { "kind": "name", "handle": "ada lovelace" } })), "handle-claimed", "a claimed handle registers nobody else");
    for refused in [" \t ", "A\u{200b}da", "\u{202e}adA", "Ade\u{301}", "\u{410}da"] {
        assert_eq!(rejected(&base, &json!({ "type": "identify-learner", "id": id(4), "learner": other, "identity": { "kind": "pseudonym", "handle": refused } })), "handle-invalid", "{refused:?}");
    }
    assert_eq!(rejected(&base, &json!({ "type": "identify-learner", "id": id(5), "learner": anonymous, "identity": { "kind": "anonymous" } })), "learner-exists");
    let recalled = view(&base, &json!({ "type": "handle", "handle": " ADA  lovelace" }));
    assert_eq!(recalled, json!({ "display": "ADA lovelace", "holder": { "learner": ada, "identity": { "kind": "pseudonym", "handle": "Ada Lovelace" } } }), "a handle is recalled by a read, however it is typed");
    assert_eq!(view(&base, &json!({ "type": "handle", "handle": "Grace" })), json!({ "display": "Grace" }), "a free handle has no holder");
    let (status, refusal) = query(&base, &json!({ "type": "handle", "handle": "A\u{200b}da" }));
    assert!(status == 400 && refusal["message"].as_str().is_some_and(|message| message.contains("handle-invalid")), "{status} {refusal}");
    let (status, refusal) = query(&base, &json!({ "type": "learner", "learner": "../../etc/passwd" }));
    assert!(status == 400 && refusal["message"].as_str().is_some_and(|message| message.contains("id-invalid")), "{status} {refusal}");
    let learner = view(&base, &json!({ "type": "learner", "learner": ada }));
    assert_eq!((learner["identity"]["handle"].as_str(), learner["runs"].as_array().map(Vec::len)), (Some("Ada Lovelace"), Some(0)));
    assert_eq!(view(&base, &json!({ "type": "learner", "learner": anonymous }))["identity"], json!({ "kind": "anonymous" }));
    assert_eq!(query(&base, &json!({ "type": "learner", "learner": other })).0, 404);
    let (_, _, registered) = get(&base, &format!("/actors/{TENANT}/course-learner/{ada}/events"), &[]);
    assert_eq!(serde_json::from_str::<Vec<Value>>(&registered).expect("event records").len(), 1, "recalling a handle appended nothing to its learner");

    let run = id(0x100);
    let events = accepted(&base, &start(&ada, &run, "power", "medium", 10));
    assert_eq!((events[0]["type"].as_str(), events[0]["challenge"].as_str(), events[0]["revision"].as_str().map(str::len)), (Some("run-started"), Some("medium"), Some(64)));
    let open = view(&base, &json!({ "type": "run", "run": run }));
    assert_eq!((open["status"].as_str(), open["sheet"]["tasks"].as_array().map(Vec::len), open["answers"].as_object().map(|answers| answers.len())), (Some("open"), Some(2), Some(0)));
    assert_eq!((&open["sheet"]["seed"], open["sheet"]["challenge"].as_str()), (&events[0]["seed"], Some("medium")));
    assert!(!open["sheet"].to_string().contains("\"value\""), "the sheet never says which item a number belongs to");

    let document = quiz_document("power");
    let tasks = open["sheet"]["tasks"].as_array().expect("tasks").clone();
    let first = record(&ada, &run, &tasks[0], &perfect(&document, &tasks[0]), 20);
    assert_eq!(accepted(&base, &first).len(), 1);
    assert!(accepted(&base, &first).is_empty(), "a retried command id is applied once");
    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/course-learner/{ada}/events"), &[]);
    let stream: Vec<Value> = serde_json::from_str(&stream).expect("event records");
    assert_eq!(stream.iter().filter(|record| record["kind"] == "course.answer-recorded").count(), 1);
    assert_eq!(stream[0]["kind"], "course.learner-registered");
    assert_eq!(rejected(&base, &record(&ada, &run, &tasks[1], &json!({ "kind": "sorting", "order": [] }), 22)), "answer-invalid");
    accepted(&base, &record(&ada, &run, &tasks[1], &perfect(&document, &tasks[1]), 23));

    let submitted = accepted(&base, &json!({ "type": "submit-run", "id": id(24), "learner": ada, "run": run }));
    assert_eq!(submitted.iter().map(|event| event["type"].as_str().expect("type")).collect::<Vec<_>>(), ["run-submitted", "badge-awarded", "badge-awarded"]);
    assert_eq!(submitted[0]["result"]["score"], json!(1.0));
    assert_eq!(submitted.iter().skip(1).map(|event| event["badge"].as_str().expect("badge")).collect::<Vec<_>>(), ["perfect-power", "sorter"]);
    assert_eq!(rejected(&base, &json!({ "type": "submit-run", "id": id(25), "learner": ada, "run": run })), "run-closed");
    assert_eq!(rejected(&base, &start(&ada, &id(0x101), "cooling", "medium", 26)), "unknown-quiz");
    assert_eq!(rejected(&base, &start(&id(0xdead), &id(0x102), "power", "medium", 27)), "unknown-learner");
    let mut unchallenged = start(&ada, &id(0x103), "power", "medium", 28);
    unchallenged.as_object_mut().expect("a command").remove("challenge");
    assert!(rejected(&base, &unchallenged).starts_with("command-malformed"), "a run starts at a challenge");
    unchallenged["challenge"] = json!("legendary");
    assert!(rejected(&base, &unchallenged).starts_with("command-malformed"), "one of the four");
    let mut undated = start(&ada, &id(0x103), "power", "hard", 29);
    undated.as_object_mut().expect("a command").remove("at");
    assert!(rejected(&base, &undated).starts_with("command-malformed"), "a run starts at the instant the device claims");
    for (at, refusal) in [(json!(-1), "command-malformed"), (json!(1.5), "command-malformed"), (json!(u64::MAX), "id-invalid"), (json!(course::MAX_TIMESTAMP + 1), "id-invalid")] {
        undated["at"] = at.clone();
        assert!(rejected(&base, &undated).starts_with(refusal), "an instant {at} beyond a safe integer is refused: {refusal}");
    }

    let finished = view(&base, &json!({ "type": "run", "run": run }));
    assert_eq!((finished["status"].as_str(), finished["result"]["score"].as_f64(), finished["result"]["challenge"].as_str(), finished["result"]["points"].as_f64(), finished["submittedAt"].is_u64()), (Some("submitted"), Some(1.0), Some("medium"), Some(200.0), true));
    let learner = view(&base, &json!({ "type": "learner", "learner": ada }));
    assert_eq!((&learner["best"]["power"], learner["total"].as_f64()), (&json!({ "challenge": "medium", "score": 1.0, "points": 200.0 }), Some(200.0)));
    assert_eq!((learner["runs"][0]["challenge"].as_str(), learner["runs"][0]["points"].as_f64()), (Some("medium"), Some(200.0)));
    assert_eq!(learner["badges"].as_array().expect("badges").iter().map(|award| award["badge"].as_str().expect("badge")).collect::<Vec<_>>(), ["perfect-power", "sorter"]);
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": ada }));
    assert_eq!(board["rows"].as_array().expect("rows").iter().map(|row| (row["rank"].as_u64(), row["tag"].as_str(), row["total"].as_f64())).collect::<Vec<_>>(), [(Some(1), Some(course::learner_tag(&ada).as_str()), Some(200.0))]);
    assert_eq!(board["rows"][0]["best"], learner["best"], "a row names the challenge and the points of each best");
    assert_eq!((board["learners"].as_u64(), &board["own"]), (Some(1), &board["rows"][0]), "the board counts the ranked learners and answers the caller's own row");
    assert_eq!(board.as_object().map(|board| board.keys().map(String::as_str).collect::<std::collections::BTreeSet<_>>()), Some(["learners", "own", "period", "rows", "submissions"].into()), "the all-time board is exactly its period, its top rows, its counts and the caller's row");
    assert_eq!((board["period"].as_str(), board["submissions"].as_u64()), (Some("all-time"), Some(1)));
    assert!(!board.to_string().contains(ada.as_str()), "the public leaderboard never carries a learner id");
    let submitted_at = finished["submittedAt"].as_u64().expect("submittedAt");
    for period in ["daily", "weekly", "monthly"] {
        for (quiz, played) in [(None, true), (Some("power"), true), (Some("homes"), false)] {
            let mut asked = json!({ "type": "leaderboard", "period": period, "learner": ada });
            if let Some(quiz) = quiz {
                asked["quiz"] = json!(quiz);
            }
            let scoped = view(&base, &asked);
            let (from, until) = (scoped["window"]["from"].as_u64().expect("from"), scoped["window"]["until"].as_u64().expect("until"));
            let counted = played && from <= submitted_at && submitted_at < until;
            assert_eq!((scoped["period"].as_str(), scoped["quiz"].as_str(), scoped["submissions"].as_u64()), (Some(period), quiz, Some(1)), "{scoped}");
            assert_eq!((scoped["learners"].as_u64(), scoped["rows"].as_array().map(Vec::len), scoped.get("own").is_some()), (Some(u64::from(counted)), Some(usize::from(counted)), counted), "the {period} board of {quiz:?} counts the runs of its window and quiz: {scoped}");
            assert!(from < until && until - from <= 31 * 86_400_000, "{scoped}");
            if counted {
                assert_eq!((&scoped["rows"], &scoped["own"]), (&board["rows"], &board["own"]), "the only run there is makes the same row on every board that counts it");
            }
        }
    }
    assert_eq!(query(&base, &json!({ "type": "leaderboard", "period": "weekly", "quiz": "cooling" })).0, 404, "a quiz the course does not list has no leaderboard");
    assert_eq!(query(&base, &json!({ "type": "leaderboard" })).0, 400, "a leaderboard query names its period");
    assert_eq!(query(&base, &json!({ "type": "leaderboard", "period": "yearly" })).0, 400);
    assert_eq!(query(&base, &json!({ "type": "leaderboard", "period": "daily", "quiz": "Power" })).0, 400);
    let unranked = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": anonymous }));
    assert_eq!((unranked["rows"].clone(), unranked["learners"].as_u64(), unranked.get("own")), (board["rows"].clone(), Some(1), None), "a learner without a submitted run has no row of its own");
    assert_eq!(view(&base, &json!({ "type": "leaderboard", "period": "all-time" })), unranked, "nor has a caller that names nobody");
    let before = (learner, finished, board, recalled);
    running.shut_down();

    let running = boot(&data.0, development());
    let base = running.base.clone();
    let after = (view(&base, &json!({ "type": "learner", "learner": ada })), view(&base, &json!({ "type": "run", "run": run })), view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": ada })), view(&base, &json!({ "type": "handle", "handle": " ADA  lovelace" })));
    assert_eq!(after, before, "the reopened SQLite file answers the same views");
    assert_eq!(rejected(&base, &json!({ "type": "identify-learner", "id": id(6), "learner": other, "identity": { "kind": "pseudonym", "handle": "ADA LOVELACE" } })), "handle-claimed", "the handle is still held after a restart");
    let early_run = id(0x110);
    accepted(&base, &start(&anonymous, &early_run, "power", "medium", 40));
    let early_open = view(&base, &json!({ "type": "run", "run": early_run }));
    let early_tasks = early_open["sheet"]["tasks"].as_array().expect("tasks");
    accepted(&base, &record(&anonymous, &early_run, &early_tasks[0], &perfect(&document, &early_tasks[0]), 41));
    let early = accepted(&base, &json!({ "type": "submit-run", "id": id(42), "learner": anonymous, "run": early_run }));
    let early_score = early[0]["result"]["score"].as_f64().expect("score");
    assert!(early_score > 0.0 && early_score < 1.0, "an open task scores as a miss and the answered task still earns its share: {early_score}");
    assert_eq!(early[0]["result"]["points"].as_f64(), Some(early_score * 200.0));
    let completed = play(&base, &ada, &id(0x200), "homes", "medium", 30);
    assert!(completed.iter().any(|event| event["badge"] == "complete"), "{completed:?}");
    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/course-learner/{ada}/events"), &[]);
    let sequences: Vec<u64> = serde_json::from_str::<Vec<Value>>(&stream).expect("records").iter().map(|record| record["seq"].as_u64().expect("seq")).collect();
    assert_eq!(sequences, (1..=sequences.len() as u64).collect::<Vec<_>>(), "the restarted authority appends after its history");
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time" }));
    assert_eq!((board["rows"][0]["total"].as_f64(), board["rows"][0]["badges"].as_array().map(Vec::len), board["submissions"].as_u64()), (Some(400.0), Some(3), Some(3)));
    let homes = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "quiz": "homes", "learner": ada }));
    assert_eq!((homes["own"]["total"].as_f64(), homes["own"]["best"].clone(), homes["own"]["runs"].as_u64(), homes["own"]["badges"].clone(), homes["submissions"].as_u64()), (Some(200.0), json!({ "homes": { "challenge": "medium", "score": 1.0, "points": 200.0 } }), Some(1), json!(["complete"]), Some(3)), "the board of one quiz is made of that quiz's runs and the badges they earned: {homes}");
    running.shut_down();
}

#[test]
fn the_api_serves_the_cdn_site_across_origins_behind_the_gate() {
    const SITE: &str = "https://quizzes.example";
    const FOREIGN: &str = "https://evil.example";
    let data = scratch("gate-data");
    let running = boot(&data.0, Gate { origins: CrossOriginPolicy::Allowlist(vec![SITE.to_string()]), forwarding: Forwarding::TerminatingProxy, limits: Limits::default() });
    let base = running.base.clone();
    let https = [("x-forwarded-proto", "https")];

    let (status, headers, _) = get(&base, "/instance", &[]);
    assert_eq!((status, header(&headers, "x-semio-refusal")), (403, Some("insecure-transport")));
    let (status, headers, _) = get(&base, "/instance", &https);
    assert_eq!((status, header(&headers, "content-type")), (200, Some("application/json")));
    for path in ["/", "/index.html", "/quiz/power/run/123", "/assets/app-B3xK9aQz.js"] {
        let (status, headers, body) = get(&base, path, &https);
        assert_eq!((status, header(&headers, "content-type")), (404, Some("application/json")), "{path}");
        assert_eq!(serde_json::from_str::<Value>(&body).expect("json error")["kind"], "notFound", "{path}");
    }

    for path in ["/commands", "/queries"] {
        let preflight = |origin: &str| raw(running.address, &format!("OPTIONS {path} HTTP/1.1\r\nHost: proctor\r\nX-Forwarded-Proto: https\r\nOrigin: {origin}\r\nAccess-Control-Request-Method: POST\r\nAccess-Control-Request-Headers: content-type\r\nConnection: close\r\n\r\n")).to_ascii_lowercase();
        let granted = preflight(SITE);
        assert!(granted.starts_with("http/1.1 204"), "{path}: {granted}");
        for expected in ["access-control-allow-origin: https://quizzes.example", "access-control-allow-methods: get, post, head, options", "access-control-allow-headers: content-type", "access-control-max-age: 7200", "vary: origin"] {
            assert!(granted.contains(expected), "{path} lacks {expected}: {granted}");
        }
        let refused = preflight(FOREIGN);
        assert!(refused.starts_with("http/1.1 204") && !refused.contains("access-control-allow-origin") && !refused.contains("access-control-max-age"), "{path}: {refused}");
    }

    let learner = id(0xc0de);
    let identify = envelope(&json!({ "type": "identify-learner", "id": id(0xc1), "learner": learner, "identity": { "kind": "anonymous" } }));
    let (status, headers, outcome) = post_with(&base, "/commands", &identify, &[("origin", SITE), ("x-forwarded-proto", "https")]);
    assert_eq!((status, outcome["status"].as_str(), header(&headers, "access-control-allow-origin")), (200, Some("accepted"), Some(SITE)), "{outcome}");
    let (status, headers, result) = post_with(&base, "/queries", &query_envelope(&json!({ "type": "learner", "learner": learner })), &[("origin", SITE), ("x-forwarded-proto", "https")]);
    assert_eq!((status, result["kind"].as_str(), header(&headers, "access-control-allow-origin")), (200, Some("snapshot"), Some(SITE)), "{result}");
    assert_eq!(serde_json::from_str::<Value>(&text(&result["value"])).expect("learner view")["identity"], json!({ "kind": "anonymous" }));
    let (status, headers, _) = post_with(&base, "/queries", &query_envelope(&json!({ "type": "leaderboard", "period": "all-time" })), &[("origin", FOREIGN), ("x-forwarded-proto", "https")]);
    assert_eq!((status, header(&headers, "access-control-allow-origin"), header(&headers, "vary")), (200, None, Some("Origin")));
    running.shut_down();
}

#[test]
fn learners_share_presence_and_cursors_through_the_gateway() {
    let data = scratch("presence");
    let running = boot(&data.0, development());
    let local = [("origin", "http://localhost:6061")];
    let (ada, grace) = (course::learner_tag(&id(0xada)), course::learner_tag(&id(0x9ace)));

    let mut first = presence(running.address, TENANT, "home", &local).expect("joins the roster");
    let (first_session, greeting) = welcome(&mut first);
    assert_eq!(first_session.len(), 32);
    assert_eq!(greeting["roster"], json!([{ "session": first_session, "colour": greeting["colour"], "surface": "home", "state": null }]));
    let at_home = json!({ "tag": ada, "identity": { "kind": "pseudonym", "handle": "Ada" }, "place": { "screen": "home" }, "active": true });
    share(&mut first, &at_home);
    until(&mut first, |frame| carries(frame, &first_session, &at_home));

    let mut second = presence(running.address, TENANT, "home", &local).expect("joins the roster");
    let (second_session, greeting) = welcome(&mut second);
    assert_ne!(second_session, first_session);
    let roster = greeting["roster"].as_array().expect("roster");
    assert_eq!(roster.len(), 2, "{greeting}");
    assert!(roster.iter().any(|entry| entry["session"] == first_session.as_str() && entry["state"] == at_home), "the roster carries the state already shared: {greeting}");
    assert_ne!(roster[0]["colour"], roster[1]["colour"], "{greeting}");
    until(&mut first, |frame| carries(frame, &second_session, &Value::Null));
    let in_run = json!({ "tag": grace, "identity": { "kind": "anonymous" }, "place": { "screen": "run", "quiz": "power", "task": "sources" }, "active": true });
    share(&mut second, &in_run);
    until(&mut first, |frame| carries(frame, &second_session, &in_run));

    let mut shouting = in_run.clone();
    shouting["tag"] = json!("NOT-A-TAG");
    share(&mut second, &shouting);
    assert_eq!(refusal(&mut second), "tag-invalid /tag");
    share(&mut second, &json!({ "tag": grace, "place": { "screen": "home" } }));
    assert_eq!(refusal(&mut second), "state-invalid");
    share(&mut second, &json!({ "tag": grace, "identity": { "kind": "anonymous" }, "place": { "screen": "run", "quiz": "cooling" }, "active": true }));
    assert_eq!(refusal(&mut second), "quiz-unknown /place/quiz");

    let room = format!("{TENANT}/quiz/power");
    let mut pointer = presence(running.address, &room, "run", &local).expect("joins the quiz room");
    let (pointer_session, _) = welcome(&mut pointer);
    let mut watcher = presence(running.address, &room, "run", &local).expect("joins the quiz room");
    let (_, greeting) = welcome(&mut watcher);
    assert_eq!(greeting["roster"].as_array().map(Vec::len), Some(2), "{greeting}");
    let cursor = |x: f64| json!({ "tag": ada, "cursor": { "anchor": "task:sources", "x": x, "y": 0.5 }, "focus": "task:sources" });
    for step in 0..10 {
        share(&mut pointer, &cursor(f64::from(step) / 10.0));
    }
    let seen = until(&mut watcher, |frame| carries(frame, &pointer_session, &cursor(0.9)));
    let moves = seen.iter().filter(|frame| frame["type"] == "batch" && frame["entries"].as_array().expect("entries").iter().any(|entry| entry["session"] == pointer_session.as_str() && !entry["state"].is_null())).count();
    assert!((1..=2).contains(&moves), "ten moves inside one tick coalesce into at most two batches: {seen:?}");
    share(&mut pointer, &at_home);
    assert_eq!(refusal(&mut pointer), "state-invalid", "a quiz room carries cursors, not presence");

    assert_eq!(presence(running.address, "other-course", "home", &local).err(), Some((403, None)), "another course's roster is no room here");
    assert_eq!(presence(running.address, &format!("{TENANT}/quiz/cooling"), "run", &local).err(), Some((403, None)), "an unknown quiz has no room");
    assert_eq!(presence(running.address, TENANT, &"s".repeat(65), &local).err(), Some((400, None)), "the surface is bounded");

    close(second);
    until(&mut first, |frame| departs(frame, &second_session));
    close(pointer);
    until(&mut watcher, |frame| departs(frame, &pointer_session));
    close(first);
    close(watcher);
    running.shut_down();
}

#[test]
fn a_production_proctor_opens_presence_to_the_site_origin_only() {
    const SITE: &str = "https://quizzes.example";
    let data = scratch("presence-gate");
    let running = boot(&data.0, Gate { origins: CrossOriginPolicy::Allowlist(vec![SITE.to_string()]), forwarding: Forwarding::TerminatingProxy, limits: Limits::default() });
    let https = ("x-forwarded-proto", "https");
    assert_eq!(presence(running.address, TENANT, "home", &[("origin", SITE)]).err(), Some((403, Some("insecure-transport".to_string()))));
    assert_eq!(presence(running.address, TENANT, "home", &[("origin", "https://evil.example"), https]).err(), Some((403, None)), "a foreign page may not open a socket");
    assert_eq!(presence(running.address, TENANT, "home", &[("origin", "http://localhost:6061"), https]).err(), Some((403, None)), "production admits no loopback page");
    let mut socket = presence(running.address, TENANT, "home", &[("origin", SITE), https]).expect("the site joins");
    let (session, _) = welcome(&mut socket);
    let state = json!({ "tag": "0a1b2c3d", "identity": { "kind": "anonymous" }, "place": { "screen": "leaderboard" }, "active": false });
    share(&mut socket, &state);
    until(&mut socket, |frame| carries(frame, &session, &state));
    close(socket);
    running.shut_down();
}

#[test]
fn learners_see_what_the_others_think_live_and_what_they_answered() {
    let data = scratch("thinking");
    let running = boot(&data.0, development());
    let (base, address) = (running.base.clone(), running.address);
    let local = [("origin", "http://localhost:6061")];
    let (ada, grace) = (id(0xada), id(0x9ace));
    for (seed, learner) in [(1, &ada), (2, &grace)] {
        accepted(&base, &json!({ "type": "identify-learner", "id": id(seed), "learner": learner, "identity": { "kind": "anonymous" } }));
    }
    let (ada_tag, grace_tag) = (course::learner_tag(&ada), course::learner_tag(&grace));
    let (thinking, quiz_room, leaderboard) = (format!("{TENANT}/quiz/power/thinking"), format!("{TENANT}/quiz/power"), format!("{TENANT}/leaderboard"));

    let mut thinker = presence(address, &thinking, "run", &local).expect("joins the thinking room");
    let (thinker_session, _) = welcome(&mut thinker);
    let mut peer = presence(address, &thinking, "run", &local).expect("joins the thinking room");
    let (_, greeting) = welcome(&mut peer);
    assert_eq!(greeting["roster"].as_array().map(Vec::len), Some(2), "{greeting}");
    let mut home = presence(address, &format!("{TENANT}/home"), "home", &local).expect("joins home");
    welcome(&mut home);
    watch(&mut home, &[&thinking, &quiz_room, &leaderboard], 250);
    let snapshots = until(&mut home, |frame| frame["type"] == "watched" && frame["scope"] == thinking.as_str());
    let scoped: Vec<(&str, bool)> = snapshots.iter().filter(|frame| frame["type"] == "watched").map(|frame| (frame["scope"].as_str().expect("scope"), frame["snapshot"] == true)).collect();
    assert_eq!(scoped, [(leaderboard.as_str(), true), (quiz_room.as_str(), true), (thinking.as_str(), true)], "one snapshot per watched scope, in scope order");
    assert_eq!(snapshots.last().expect("thinking")["entries"].as_array().map(Vec::len), Some(2), "the snapshot holds both thinkers");

    let draft = json!({ "tag": ada_tag, "answers": { "appliances": { "kind": "sorting", "order": ["kettle", "laptop"] } } });
    share(&mut thinker, &draft);
    until(&mut peer, |frame| carries(frame, &thinker_session, &draft));
    until(&mut home, |frame| sees(frame, &thinking, &thinker_session, &draft));
    let revised = json!({ "tag": ada_tag, "answers": { "appliances": { "kind": "sorting", "order": ["phone-charger", "laptop", "kettle"] }, "sources": { "kind": "matching", "values": { "hours": { "rooftop-pv": 950 } } } } });
    share(&mut thinker, &revised);
    until(&mut peer, |frame| carries(frame, &thinker_session, &revised));
    until(&mut home, |frame| sees(frame, &thinking, &thinker_session, &revised));
    share(&mut thinker, &json!({ "tag": ada_tag, "answers": { "sources": { "kind": "matching", "values": { "capacity": { "rooftop-pv": -10000 } } } } }));
    assert_eq!(refusal(&mut thinker), "value-invalid /answers/sources/values/capacity/rooftop-pv");
    let guessing = json!({ "tag": ada_tag, "answers": { "appliances": { "kind": "sorting", "order": ["laptop", "kettle"], "guesses": { "laptop": 60, "kettle": 2200 } }, "sources": { "kind": "matching", "values": { "capacity": { "rooftop-pv": 12000 }, "hours": { "rooftop-pv": 1000 } } } } });
    share(&mut thinker, &guessing);
    until(&mut peer, |frame| carries(frame, &thinker_session, &guessing));

    let mut dragger = presence(address, &quiz_room, "run", &local).expect("joins the quiz room");
    let (dragger_session, _) = welcome(&mut dragger);
    let dragging = json!({ "tag": grace_tag, "cursor": { "anchor": "item:kettle", "x": 0.5, "y": 0.5 }, "drag": { "item": "kettle" } });
    share(&mut dragger, &dragging);
    until(&mut home, |frame| sees(frame, &quiz_room, &dragger_session, &dragging));
    share(&mut dragger, &json!({ "tag": grace_tag, "drag": { "item": "pellets" } }));
    assert_eq!(refusal(&mut dragger), "id-unknown /drag/item");
    watch(&mut home, &["other-course/home"], 250);
    assert_eq!(refusal(&mut home), "forbidden other-course/home");
    for socket in [thinker, peer, home, dragger] {
        close(socket);
    }

    let crowd = |base: &str| {
        let (status, result) = post(base, "/queries", &query_envelope(&json!({ "type": "crowd", "quiz": "power" })));
        assert_eq!(status, 200, "{result}");
        text(&result["value"])
    };
    assert_eq!(serde_json::from_str::<Value>(&crowd(&base)).expect("crowd json")["runs"], 0, "a quiz nobody submitted has an empty crowd");
    assert_eq!(query(&base, &json!({ "type": "crowd", "quiz": "cooling" })).0, 404);
    let (ada_run, grace_run) = (id(0x100), id(0x101));
    play(&base, &ada, &ada_run, "power", "medium", 100);
    accepted(&base, &start(&grace, &grace_run, "power", "hard", 40));
    let document = quiz_document("power");
    for (index, task) in view(&base, &json!({ "type": "run", "run": grace_run }))["sheet"]["tasks"].as_array().expect("tasks").iter().enumerate() {
        let mut answer = perfect(&document, task);
        if let Some(order) = answer.get_mut("order").and_then(Value::as_array_mut) {
            order.reverse();
            let mut values: Vec<f64> = answer["guesses"].as_object().expect("guesses").values().map(|value| value.as_f64().expect("a guess")).collect();
            values.sort_by(f64::total_cmp);
            answer["guesses"] = Value::Object(answer["order"].as_array().expect("order").iter().zip(values).map(|(item, value)| (item.as_str().expect("id").to_string(), json!(value))).collect());
        }
        for guessed in answer.get_mut("guesses").and_then(Value::as_object_mut).into_iter().flat_map(|dimensions| dimensions.values_mut()).filter_map(Value::as_object_mut) {
            guessed.values_mut().for_each(|guess| *guess = json!(guess.as_f64().expect("a guess") * 1.5));
        }
        accepted(&base, &record(&grace, &grace_run, task, &answer, 41 + index as u32));
    }
    accepted(&base, &json!({ "type": "submit-run", "id": id(70), "learner": grace, "run": grace_run }));
    let results: Vec<course::RunResult> = [&ada_run, &grace_run].iter().map(|run| serde_json::from_value(view(&base, &json!({ "type": "run", "run": run }))["result"].clone()).expect("a run result")).collect();
    let course = load_course(&fixtures().join("📚️course/🔣️.json")).expect("fixture course");
    let oracle = serde_json::to_string(&course::crowd_view(&course.entry("power").expect("power").quiz, &results)).expect("oracle json");
    let answered = crowd(&base);
    assert_eq!(answered, oracle, "the proctor's crowd is the core's crowd_view, byte for byte");
    let answered: Value = serde_json::from_str(&answered).expect("crowd json");
    assert_eq!(answered["runs"], 2);
    let means: Vec<f64> = answered["tasks"][0]["items"].as_array().expect("sorting items").iter().map(|item| item["meanPosition"].as_f64().expect("a mean")).collect();
    assert!(means.iter().all(|mean| (mean - 0.5).abs() < 1e-12), "a perfect and a reversed order meet in the middle: {means:?}");
    let counts = |bins: &Value| -> Vec<u64> { bins.as_array().expect("counts").iter().map(|count| count.as_u64().expect("a count")).collect() };
    let scores: Vec<Vec<u64>> = std::iter::once(&answered).chain(answered["tasks"].as_array().expect("tasks")).map(|scored| counts(&scored["scores"])).collect();
    assert!(scores.iter().all(|bins| bins.len() == course::CROWD_SCORE_BINS && bins.iter().sum::<u64>() == 2), "both runs fall into the ten score bins of the quiz, of every task and of every dimension: {scores:?}");
    assert_eq!((scores[0][9], scores[1][9]), (1, 1), "the perfect run and its perfect order are in the last bin: {scores:?}");
    for item in answered["tasks"][0]["items"].as_array().expect("sorting items") {
        let places = counts(&item["places"]);
        assert_eq!((places.len(), places.iter().sum::<u64>(), places.iter().rev().copied().collect::<Vec<_>>()), (document["tasks"][0]["items"].as_array().expect("items").len(), 2, places.clone()), "a perfect and a reversed order put an item at mirrored places");
    }
    running.shut_down();

    let running = boot(&data.0, development());
    assert_eq!(crowd(&running.base), oracle, "the crowd survives a restart");
    running.shut_down();
}
