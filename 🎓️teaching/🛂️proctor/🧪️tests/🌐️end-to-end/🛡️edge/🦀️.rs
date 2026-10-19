//! 🛡️ The proctor end to end, the edge: the gate of a proctor behind its terminating proxy — enrollment keys, refused ids, body limits, per-address allowances, sign-up counting, its own routes only, socket limits and trickling bodies.
//!
//! @see ../🦀️.rs — the end to end suite that mounts this module
//! @see ../../../🧫️fixtures/📚️course/🔣️.json — the course played

use super::*;
use semio_framework_test_fixtures::fixture_path;

//#region 🔖️Edge
const EDGE_SITE: &str = "https://quizzes.example";

/// 🛡️ The gate of a proctor behind its terminating proxy, with `limits` at the edge.
fn proxied(limits: Limits) -> Gate {
    Gate { origins: CrossOriginPolicy::Allowlist(vec![EDGE_SITE.to_string()]), forwarding: Forwarding::TerminatingProxy, limits }
}

/// 📨️ The headers the terminating proxy forwards a request of the site with, for the client at `address`.
fn forwarded(address: &'static str) -> [(&'static str, &'static str); 3] {
    [("x-forwarded-proto", "https"), ("x-forwarded-for", address), ("origin", EDGE_SITE)]
}

pub(super) fn sign_up(seed: u32, learner: &str) -> Value {
    json!({ "type": "identify-learner", "id": id(seed), "learner": learner, "identity": { "kind": "anonymous" } })
}

/// 📏️ The size of the largest `record-answer` body a course can produce: every presented item
/// answered, using the longest ids allowed by each pool slot or draw and the longest numeric guesses,
/// encoded as the client sends it.
fn largest_answer(course: &Path) -> usize {
    let read = |path: &Path| -> Value { serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))).expect("json") };
    let ids = |values: &Value| -> Vec<String> { values.as_array().map(|values| values.iter().map(|value| value["id"].as_str().expect("id").to_string()).collect()).unwrap_or_default() };
    let (learner, run) = (id(0xffff_ffff), id(0xffff_fffe));
    let longest = json!(-1.234_567_890_123_456_7e-300);
    let mut largest = 0;
    for quiz in read(course)["quizzes"].as_array().expect("quizzes") {
        let document = read(&course.parent().expect("a directory").join(quiz.as_str().expect("a quiz path")));
        for task in document["tasks"].as_array().expect("tasks") {
            let items = if let Some(slots) = task["pool"]["slots"].as_array() {
                slots.iter().map(|slot| slot.as_array().expect("slot").iter().map(|item| item.as_str().expect("item id")).max_by_key(|item| item.len()).expect("nonempty slot").to_string()).collect::<Vec<_>>()
            } else {
                let mut items = ids(&task["items"]);
                items.sort_by_key(|item| std::cmp::Reverse(item.len()));
                items.truncate(task["draw"].as_u64().map_or(items.len(), |draw| draw as usize));
                items
            };
            let guessed = || Value::Object(items.iter().map(|item| (item.clone(), longest.clone())).collect());
            let answers = match task["kind"].as_str().expect("kind") {
                "sorting" => vec![json!({ "kind": "sorting", "order": items }), json!({ "kind": "sorting", "order": items, "guesses": guessed() })],
                "classification" => {
                    let category = ids(&task["categories"]).into_iter().max_by_key(String::len).expect("a category");
                    vec![json!({ "kind": "classification", "assignments": items.iter().map(|item| (item.clone(), json!(category))).collect::<serde_json::Map<_, _>>() })]
                }
                _ => vec![
                    json!({ "kind": "matching", "assignments": ids(&task["dimensions"]).into_iter().map(|dimension| (dimension, Value::Object(items.iter().enumerate().map(|(card, item)| (item.clone(), json!(1000 + card))).collect()))).collect::<serde_json::Map<_, _>>() }),
                    json!({ "kind": "matching", "guesses": ids(&task["dimensions"]).into_iter().map(|dimension| (dimension, guessed())).collect::<serde_json::Map<_, _>>() }),
                ],
            };
            for answer in answers {
                largest = largest.max(envelope(&record(&learner, &run, task, &answer, 0xffff_fffd)).to_string().len());
            }
        }
    }
    largest
}

/// 🔌️ Open any websocket of the proctor at `path`.
fn socket(address: SocketAddr, path: &str, headers: &[(&str, &str)]) -> Socket {
    let mut request = format!("ws://{address}{path}").into_client_request().expect("a websocket request");
    for (name, value) in headers {
        request.headers_mut().insert(HeaderName::from_bytes(name.as_bytes()).expect("a header name"), HeaderValue::from_str(value).expect("a header value"));
    }
    let stream = TcpStream::connect(address).expect("connect");
    stream.set_read_timeout(Some(Duration::from_secs(10))).expect("timeout");
    tungstenite::client(request, stream).expect("the socket opens").0
}

/// 🔚️ Read until the server has ended the socket; the frames it still sent before.
fn until_closed(socket: &mut Socket) -> Vec<Value> {
    let mut seen = Vec::new();
    loop {
        match socket.read() {
            Ok(Message::Text(text)) => seen.push(serde_json::from_str(text.as_str()).expect("a json frame")),
            Ok(Message::Close(_)) | Err(_) => return seen,
            Ok(_) => {}
        }
    }
}

#[test]
fn an_anonymous_caller_can_neither_read_nor_occupy_an_enrollment_key() {
    let (ada, intruder) = (id(0xada), id(0xbad));
    let named = json!({ "type": "identify-learner", "id": id(1), "learner": ada, "identity": { "kind": "pseudonym", "handle": "Ada Lovelace" } });
    let probes = |key: &str| {
        let occupying = envelope(&json!({ "type": "identify-learner", "id": key, "learner": intruder, "identity": { "kind": "anonymous" } }));
        let elsewhere = envelope(&json!({ "type": "start-run", "id": key, "learner": intruder, "run": id(0x666), "quiz": "power", "challenge": "medium", "at": now() }));
        let mut relayed = elsewhere.clone();
        relayed["kind"] = json!("course.enroll-learner");
        relayed["principal"] = json!({ "kind": "serviceAccount", "id": "proctor" });
        [occupying, elsewhere, relayed]
    };

    let rehearsal = scratch("edge-key-read");
    let running = boot(&rehearsal.0, development());
    let (outcome, _) = command(&running.base, &named);
    assert_eq!(outcome["status"], "accepted", "{outcome}");
    let committed: EventRecord = serde_json::from_value(outcome["events"][0].clone()).expect("the committed event record");
    let key = teaching_proctor_domain::actors::enrollment(&committed).expect("a named sign-up is relayed to its learner").command_id.0;
    assert_eq!(view(&running.base, &json!({ "type": "learner", "learner": ada }))["identity"]["handle"], "Ada Lovelace", "the relay ran under that key");
    for probe in probes(&key) {
        let (status, _, answer) = post_with(&running.base, "/commands", &probe, &[]);
        assert!(!answer.to_string().contains(&ada), "replaying the enrollment key {key} answered a learner id ({status}): {answer}");
        assert!(answer["status"] != "accepted" || answer["receipt"]["actor"]["id"] != ada.as_str(), "{answer}");
    }
    running.shut_down();

    let data = scratch("edge-key-occupy");
    let running = boot(&data.0, development());
    let base = running.base.clone();
    for probe in probes(&key) {
        assert_eq!(post(&base, "/commands", &probe).0, 200);
    }
    let events = accepted(&base, &named);
    assert_eq!((events[0]["type"].as_str(), events[0]["learner"].as_str()), (Some("learner-registered"), Some(ada.as_str())));
    assert_eq!(view(&base, &json!({ "type": "learner", "learner": ada }))["identity"]["handle"], "Ada Lovelace", "the real sign-up is relayed although its key was asked for first");
    assert_eq!(accepted(&base, &start(&ada, &id(0x100), "power", "medium", 2))[0]["type"], "run-started");
    running.shut_down();
}

#[test]
fn a_command_for_an_id_no_actor_can_have_is_refused_before_it_costs_anything() {
    let data = scratch("edge-ids");
    let running = boot(&data.0, development());
    let base = running.base.clone();
    let learner = id(7);
    accepted(&base, &sign_up(1, &learner));
    let long = "a".repeat(4000);
    for (seed, target) in [long.as_str(), "not-a-learner-id", "0000000000000000000000000000000G", "", " ", "enroll:proctor-fixture:roster:1"].into_iter().enumerate() {
        let mut sent = envelope(&start(&learner, &id(200 + seed as u32), "power", "medium", 100 + seed as u32));
        sent["target"]["id"] = json!(target);
        let (status, outcome) = post(&base, "/commands", &sent);
        assert_eq!((status, outcome["status"].as_str(), outcome["reason"]["kind"].as_str()), (200, Some("rejected"), Some("invalid")), "{outcome}");
        assert!(outcome["reason"]["detail"].as_str().is_some_and(|detail| detail.starts_with("id-invalid")), "target {:?}: {outcome}", &target[..target.len().min(40)]);
    }
    let mut keyed = envelope(&start(&learner, &id(301), "power", "medium", 300));
    keyed["commandId"] = json!("k".repeat(4000));
    keyed["idempotencyKey"] = keyed["commandId"].clone();
    let (_, outcome) = post(&base, "/commands", &keyed);
    assert_eq!((outcome["status"].as_str(), outcome["reason"]["kind"].as_str()), (Some("rejected"), Some("invalid")), "an oversized key is no key: {outcome}");
    assert_eq!(accepted(&base, &start(&learner, &id(401), "power", "medium", 400))[0]["type"], "run-started", "the learner is untouched by the refusals");
    running.shut_down();
}

#[test]
fn a_body_past_the_limit_is_refused_and_the_largest_real_command_is_far_below_it() {
    let limit = Limits::default().body_bytes.expect("the production limits cap the body");
    for course in [fixtures().join("📚️course/🔣️.json"), fixture_path!("../../../🏛️architecture/📚️course/🔣️.json")] {
        let largest = largest_answer(&course);
        assert!(largest > 1000 && largest * 3 <= limit, "{}: the largest answer is {largest} bytes, the limit {limit}", course.display());
    }
    let data = scratch("edge-body");
    let running = boot(&data.0, development());
    let base = running.base.clone();
    let mut padded = envelope(&sign_up(1, &id(7)));
    assert_eq!(post(&base, "/commands", &padded).1["status"], "accepted");
    padded["commandId"] = json!(id(2));
    padded["idempotencyKey"] = json!(id(2));
    padded["trace"]["span_id"] = json!("x".repeat(limit));
    for path in ["/commands", "/queries"] {
        let (status, _, answer) = post_with(&base, path, &padded, &[]);
        assert_eq!((status, answer["kind"].as_str()), (413, Some("payloadTooLarge")), "{path}: {answer}");
    }
    let (status, _, answer) = post_with(&base, "/commands", &Value::Null, &[]);
    assert_eq!((status, answer["kind"].as_str()), (400, Some("badRequest")), "{answer}");
    running.shut_down();
}

#[test]
fn one_address_past_its_allowance_is_told_to_wait_and_no_other_is_slowed() {
    let data = scratch("edge-throttle");
    let running = boot(&data.0, proxied(Limits { writes: Rate::per_second(1, 3), ..Limits::default() }));
    let base = running.base.clone();
    let script = forwarded("203.0.113.66");
    let (status, _, outcome) = post_with(&base, "/commands", &envelope(&sign_up(1, &id(0x103))), &script);
    assert_eq!((status, outcome["status"].as_str()), (200, Some("accepted")), "{outcome}");
    let spent = (0..50).find(|_| post_with(&base, "/commands", &json!({}), &script).0 == 429);
    assert!(spent.is_some_and(|posts| posts >= 2), "the burst is three commands and then the address waits: {spent:?}");
    let refused = id(0x104);
    let (status, headers, answer) = post_with(&base, "/commands", &envelope(&sign_up(4, &refused)), &script);
    assert_eq!((status, answer["kind"].as_str(), header(&headers, "retry-after")), (429, Some("throttled"), Some("1")), "{answer}");
    assert!(answer["retryAfterMs"].as_u64().is_some_and(|wait| (1..=1000).contains(&wait)), "{answer}");
    assert_eq!((header(&headers, "access-control-allow-origin"), header(&headers, "access-control-expose-headers"), header(&headers, "access-control-allow-credentials")), (Some(EDGE_SITE), Some("retry-after"), None), "a page of the site reads the refusal and its wait, and no credentials are allowed");
    let behind_a_claim = [("x-forwarded-proto", "https"), ("x-forwarded-for", "198.51.100.9, 203.0.113.66"), ("origin", EDGE_SITE)];
    assert_eq!(post_with(&base, "/commands", &envelope(&sign_up(5, &id(0x105))), &behind_a_claim).0, 429, "an address claimed in front of the proxy's own entry buys no allowance");

    let hall = forwarded("203.0.113.7");
    let (status, headers, outcome) = post_with(&base, "/commands", &envelope(&sign_up(6, &id(0x106))), &hall);
    assert_eq!((status, outcome["status"].as_str(), header(&headers, "access-control-allow-credentials")), (200, Some("accepted"), None), "another address is served: {outcome}");
    let (status, _, _) = post_with(&base, "/queries", &query_envelope(&json!({ "type": "learner", "learner": refused })), &script);
    assert_eq!(status, 404, "the throttled address still reads, and the refused sign-up reached no turn");
    let (status, _, result) = post_with(&base, "/queries", &query_envelope(&json!({ "type": "learner", "learner": id(0x103) })), &script);
    assert_eq!((status, result["kind"].as_str()), (200, Some("snapshot")), "{result}");
    running.shut_down();
}

#[test]
fn sign_ups_are_counted_per_address_by_what_they_register_and_a_spent_allowance_is_named() {
    let data = scratch("edge-sign-ups");
    let running = boot(&data.0, proxied(Limits { allowances: vec![Allowance { name: SIGN_UP, rate: Rate::per_hour(60, 3) }], ..Limits::default() }));
    let base = running.base.clone();
    let (hall, script) = (forwarded("198.51.100.10"), forwarded("203.0.113.66"));
    let sent = |command: &Value, from: &[(&str, &str)]| post_with(&base, "/commands", &envelope(command), from);
    let registered = |command: &Value, from: &[(&str, &str)]| {
        let (status, _, outcome) = sent(command, from);
        assert_eq!((status, outcome["status"].as_str(), outcome["events"].as_array().map(Vec::len)), (200, Some("accepted"), Some(1)), "{outcome}");
    };
    let nothing = |command: &Value, from: &[(&str, &str)], expected: &str| {
        let (status, _, outcome) = sent(command, from);
        let answered = outcome["reason"]["detail"].as_str().map_or_else(|| format!("{} with {} events", outcome["status"].as_str().unwrap_or("?"), outcome["events"].as_array().map_or(0, Vec::len)), str::to_string);
        assert_eq!((status, answered.as_str()), (200, expected), "{outcome}");
    };
    let (ada, bob, cy) = (id(0xada), id(0xb0b), id(0xc1));
    let named = json!({ "type": "identify-learner", "id": id(1), "learner": ada, "identity": { "kind": "name", "handle": "Ada Lovelace" } });
    registered(&named, &hall);
    for attempt in 0..6u32 {
        nothing(&json!({ "type": "identify-learner", "id": id(10 + attempt), "learner": id(0x700 + attempt), "identity": { "kind": "pseudonym", "handle": "ada  LOVELACE" } }), &hall, "handle-claimed");
        nothing(&json!({ "type": "identify-learner", "id": id(20 + attempt), "learner": id(0x700 + attempt), "identity": { "kind": "pseudonym", "handle": "A\u{200b}da" } }), &hall, "handle-invalid");
        nothing(&named, &hall, "accepted with 0 events");
    }
    registered(&sign_up(2, &bob), &hall);
    for _ in 0..6 {
        nothing(&json!({ "type": "identify-learner", "id": id(30), "learner": bob, "identity": { "kind": "anonymous" } }), &hall, "learner-exists");
    }
    registered(&json!({ "type": "identify-learner", "id": id(3), "learner": cy, "identity": { "kind": "pseudonym", "handle": "Cy" } }), &hall);

    let late = id(0xd0);
    let (status, headers, answer) = sent(&sign_up(4, &late), &hall);
    assert!(status == 429 && header(&headers, "retry-after").and_then(|seconds| seconds.parse::<u64>().ok()).is_some_and(|seconds| (55..=60).contains(&seconds)), "twenty-four refused sign-ups and replays cost nothing, the fourth registration of the address waits: {status} {headers:?} {answer}");
    assert_eq!((answer["kind"].as_str(), answer["allowance"].as_str(), answer["message"].as_str()), (Some("throttled"), Some("sign-up"), Some("the sign-up allowance of this address is spent")), "{answer}");
    assert!(answer["retryAfterMs"].as_u64().is_some_and(|wait| (55_000..=60_000).contains(&wait)), "the wait is the time the next sign-up takes at sixty per hour: {answer}");
    assert_eq!((header(&headers, "access-control-allow-origin"), header(&headers, "access-control-expose-headers")), (Some(EDGE_SITE), Some("retry-after")), "a page of the site reads the refusal");
    let (status, _, _) = post_with(&base, "/queries", &query_envelope(&json!({ "type": "learner", "learner": late })), &hall);
    assert_eq!(status, 404, "the refused sign-up registered nobody");
    let (status, _, refused) = sent(&json!({ "type": "identify-learner", "id": id(5), "learner": id(0xd1), "identity": { "kind": "name", "handle": "Dee" } }), &hall);
    assert_eq!((status, refused["allowance"].as_str()), (429, Some("sign-up")), "a handle claim is a sign-up like an anonymous one");

    let (status, _, started) = sent(&start(&ada, &id(0x100), "power", "medium", 40), &hall);
    assert_eq!((status, started["status"].as_str()), (200, Some("accepted")), "who is registered plays on: only sign-ups are spent: {started}");
    let (status, _, recalled) = post_with(&base, "/queries", &query_envelope(&json!({ "type": "handle", "handle": "ada lovelace" })), &hall);
    assert_eq!((status, serde_json::from_str::<Value>(&text(&recalled["value"])).expect("a view")["holder"]["learner"].as_str().map(str::to_string)), (200, Some(ada)), "and a returning learner is recalled by a read, which is no sign-up");

    for seed in 0..3u32 {
        registered(&sign_up(50 + seed, &id(0x800 + seed)), &script);
    }
    let (status, _, refused) = sent(&sign_up(53, &id(0x803)), &script);
    assert_eq!((status, refused["allowance"].as_str()), (429, Some("sign-up")), "another address has an allowance of its own, and no more");
    nothing(&start(&id(0x804), &id(0x101), "power", "medium", 54), &script, "unknown-learner");
    running.shut_down();
    assert_eq!(registrations(&data.0), 6, "three registrations per address were made, whatever was asked");
}

#[test]
fn the_proctor_answers_on_its_own_routes_only_and_says_nothing_of_its_policy() {
    let data = scratch("edge-routes");
    let running = boot(&data.0, development());
    let base = running.base.clone();
    let hash = "00".repeat(32);
    for request in ["GET /apps".to_string(), "GET /apps/admin/index.html".to_string(), format!("GET /blobs/{hash}"), format!("HEAD /blobs/{hash}"), format!("PUT /blobs/{hash}"), format!("POST /scopes/{TENANT}/ephemeral")] {
        let answer = raw(running.address, &format!("{request} HTTP/1.1\r\nHost: proctor\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}"));
        assert!(answer.starts_with("HTTP/1.1 404"), "{request}: {answer}");
        assert!(request.starts_with("HEAD") || answer.contains("\"kind\":\"notFound\""), "{request}: {answer}");
    }
    let (status, _, instance) = get(&base, "/instance", &[]);
    let instance: Value = serde_json::from_str(&instance).expect("instance json");
    assert_eq!((status, instance["id"].as_str(), instance["modules"][0]["policies"].as_array().map(Vec::len)), (200, Some("teaching-proctor"), Some(0)), "the instance document publishes no policy template");
    assert!(instance["modules"][0]["commands"].as_array().is_some_and(|commands| !commands.is_empty()) && !instance.to_string().contains("grants"), "{instance}");
    let (status, _, denied) = get(&base, &format!("/actors/{TENANT}/no-such-kind/{}/events", id(1)), &[]);
    assert_eq!((status, serde_json::from_str::<Value>(&denied).expect("an error body")), (403, json!({ "kind": "forbidden", "message": "forbidden" })), "a denial names neither the principal nor the grant that was missing");
    running.shut_down();
}

#[test]
fn sockets_are_counted_per_address_and_an_oversized_message_ends_the_socket() {
    let data = scratch("edge-sockets");
    let running = boot(&data.0, Gate { limits: Limits { sockets_per_client: 3, ..Limits::default() }, ..development() });
    let address = running.address;
    let local = [("origin", "http://localhost:6061")];
    let mut first = presence(address, TENANT, "home", &local).expect("the first socket");
    let (session, _) = welcome(&mut first);
    let mut second = presence(address, &format!("{TENANT}/home"), "home", &local).expect("the second socket");
    welcome(&mut second);
    let learner = id(7);
    let mut stream = socket(address, &format!("/actors/{TENANT}/course-learner/{learner}/events/ws"), &[]);
    assert_eq!(presence(address, TENANT, "home", &local).err(), Some((429, None)), "one address holds no more sockets than its cap");

    stream.send(Message::text("x".repeat(2048))).expect("sent");
    assert!(until_closed(&mut stream).is_empty(), "the durable lane takes no message from its client");
    let mut third = None;
    for _ in 0..100 {
        match presence(address, TENANT, "home", &local) {
            Ok(opened) => {
                third = Some(opened);
                break;
            }
            Err(refused) => assert_eq!(refused, (429, None)),
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let mut third = third.expect("a closed socket frees its place");
    welcome(&mut third);

    share(&mut second, &json!({ "tag": "0a1b2c3d", "padding": "x".repeat(3000) }));
    assert_eq!(refusal(&mut second), "state-too-large", "a state somewhat over the limit is read and refused");
    first.send(Message::text(json!({ "type": "state", "state": { "padding": "x".repeat(8192) } }).to_string())).expect("sent");
    let before_the_end = until_closed(&mut first);
    assert!(before_the_end.iter().all(|frame| frame["type"] != "refused"), "a message past the protocol limit is never read, so never answered: {before_the_end:?}");
    until(&mut third, |frame| departs(frame, &session));
    close(second);
    close(third);
    running.shut_down();
}

/// 📨️ The head of a `POST` of `length` body bytes as the terminating proxy forwards it for `client`.
fn posting(path: &str, client: &str, length: usize) -> String {
    format!("POST {path} HTTP/1.1\r\nHost: proctor\r\nContent-Type: application/json\r\nContent-Length: {length}\r\nX-Forwarded-Proto: https\r\nX-Forwarded-For: {client}\r\nOrigin: {EDGE_SITE}\r\n\r\n")
}

/// 📬️ Read one HTTP/1.1 response off an open connection: its status, its head in lowercase and its body.
fn answer(stream: &mut TcpStream) -> (u16, String, String) {
    let mut received = Vec::new();
    let mut byte = [0u8; 1];
    while !received.ends_with(b"\r\n\r\n") {
        assert_eq!(stream.read(&mut byte).expect("the connection is still open"), 1, "the server closed the connection: {:?}", String::from_utf8_lossy(&received));
        received.push(byte[0]);
    }
    let head = String::from_utf8(received).expect("an ascii head").to_ascii_lowercase();
    let status = head.split(' ').nth(1).and_then(|status| status.parse().ok()).expect("a status line");
    let length = head.lines().find_map(|line| line.strip_prefix("content-length: ")).and_then(|length| length.trim().parse::<usize>().ok()).unwrap_or(0);
    let mut body = vec![0u8; length];
    stream.read_exact(&mut body).expect("the whole body");
    (status, head, String::from_utf8(body).expect("a utf-8 body"))
}

#[test]
fn a_refused_request_keeps_its_connection_and_a_body_that_trickles_holds_no_place() {
    let data = scratch("edge-bodies");
    let running = boot(&data.0, proxied(Limits { writes: Rate::per_second(1, 2), in_flight: 2, body_patience: Duration::from_millis(400), ..Limits::default() }));
    let script = "203.0.113.66";

    let mut connection = TcpStream::connect(running.address).expect("connect");
    connection.set_nodelay(true).expect("nodelay");
    connection.set_read_timeout(Some(Duration::from_secs(30))).expect("timeout");
    let mut statuses = Vec::new();
    for seed in 0..8u32 {
        let body = envelope(&sign_up(1 + seed, &id(0x200 + seed))).to_string();
        connection.write_all(posting("/commands", script, body.len()).as_bytes()).expect("the head");
        std::thread::sleep(Duration::from_millis(40));
        connection.write_all(body.as_bytes()).expect("the body");
        let (status, head, _) = answer(&mut connection);
        assert!(!head.contains("connection: close"), "request {seed} ({status}) costs the connection: {head}");
        statuses.push(status);
    }
    assert!(statuses[..2] == [200, 200] && statuses.iter().all(|status| [200, 429].contains(status)) && statuses.iter().filter(|status| **status == 429).count() >= 4, "eight commands whose bodies follow their heads, on one connection: the burst is served and every refusal still reads its body: {statuses:?}");

    let stalled: Vec<TcpStream> = (0..4)
        .map(|slow| {
            let mut stream = TcpStream::connect(running.address).expect("connect");
            stream.set_read_timeout(Some(Duration::from_secs(30))).expect("timeout");
            stream.write_all(posting("/queries", &format!("203.0.113.{}", 70 + slow), 600).as_bytes()).expect("the head");
            stream.write_all(b"{\"queryId\":").expect("the beginning of a body");
            stream
        })
        .collect();
    std::thread::sleep(Duration::from_millis(100));
    let (status, _, result) = post_with(&running.base, "/queries", &query_envelope(&json!({ "type": "learner", "learner": id(0x200) })), &forwarded("203.0.113.7"));
    assert_eq!((status, result["kind"].as_str()), (200, Some("snapshot")), "four bodies on their way hold none of the two places requests are served in: {result}");
    for mut stream in stalled {
        let (status, _, body) = answer(&mut stream);
        assert_eq!((status, serde_json::from_str::<Value>(&body).expect("an error body")["kind"].as_str().map(str::to_string)), (408, Some("stalled".to_string())), "a body that does not arrive is given up on");
    }
    running.shut_down();
}

#[test]
fn every_sign_up_reads_itself_back_while_others_sign_up() {
    let data = scratch("edge-read-your-writes");
    let running = boot(&data.0, development());
    let signing: Vec<_> = (0..16u32)
        .map(|hall| {
            let base = running.base.clone();
            std::thread::spawn(move || {
                for round in 0..8u32 {
                    let seed = 0x1000 + hall * 0x100 + round;
                    let (learner, handle) = (id(seed), format!("Hall {hall} round {round}"));
                    accepted(&base, &json!({ "type": "identify-learner", "id": id(seed | 0x4000_0000), "learner": learner, "identity": { "kind": "pseudonym", "handle": handle } }));
                    let (status, found) = query(&base, &json!({ "type": "learner", "learner": learner }));
                    assert_eq!((status, found["identity"]["handle"].as_str()), (200, Some(handle.as_str())), "a named sign-up is relayed and folded before its command answers, whoever else commits meanwhile: {found}");
                }
            })
        })
        .collect();
    for hall in signing {
        hall.join().expect("every sign-up read itself back");
    }
    running.shut_down();
}
//#endregion 🔖️Edge
