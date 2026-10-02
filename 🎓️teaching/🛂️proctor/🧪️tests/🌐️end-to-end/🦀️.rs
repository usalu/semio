//! 🌐️ The proctor end to end: booted on an ephemeral port over a fresh data directory and the
//! fixture catalog, driven through its real HTTP API with the §9a encoding by an independent HTTP
//! client (`ureq`), exactly as the browser client talks to it — then stopped, reopened from the same
//! SQLite file and asked again. Its presence rooms are joined by an independent websocket client
//! (`tungstenite`) speaking `semio.presence.v1`.
//!
//! @see ../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️design.md — §8, §9, §9a, §15
//! @see ../../🧫️fixtures/📚️catalog/🔣️.json — the catalog played

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use proctor::catalog::load_catalog;
use proctor::config::{CrossOriginPolicy, Forwarding, Gate, SIGN_UP};
use proctor::instance::Proctor;
use semio_framework_async::CancelToken;
use serde_json::{json, Value};
use server::contract::EventRecord;
use server::gateway::{PresenceSettings, PRESENCE_PROTOCOL_V1};
use server::storage::StorageProfile;
use server::throttle::{Allowance, Limits, Rate};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::{HeaderName, HeaderValue};
use tungstenite::{HandshakeError, Message, WebSocket};

const TENANT: &str = "proctor-fixture";

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn scratch(label: &str) -> Scratch {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let path = std::env::temp_dir().join(format!("teaching-proctor-e2e-{label}-{}-{nanos}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir_all(&path).expect("scratch directory");
    Scratch(path)
}

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures")
}

fn id(seed: u32) -> String {
    format!("{seed:032x}")
}

//#region 🔖️Server
struct Running {
    base: String,
    address: SocketAddr,
    stop: CancelToken,
    thread: std::thread::JoinHandle<()>,
}

fn boot(data: &Path, gate: Gate) -> Running {
    let (ready, address) = std::sync::mpsc::channel();
    let stop = CancelToken::root_now();
    let token = stop.clone();
    let data = data.to_string_lossy().into_owned();
    let thread = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().expect("runtime");
        runtime.block_on(async move {
            let catalog = Arc::new(load_catalog(&fixtures().join("📚️catalog/🔣️.json")).expect("fixture catalog"));
            let proctor = Proctor::assemble(StorageProfile::Embedded { data_dir: data }, catalog, gate, PresenceSettings::default()).await.expect("assembled");
            proctor.prepare().await.expect("prepared");
            proctor.reconcile().await.expect("reconciled");
            proctor.settle(&CancelToken::root_now(), |_| {}).await.expect("settled");
            let bound = proctor.bind("127.0.0.1:0".parse().expect("address")).await.expect("bound");
            ready.send(bound.local_addr()).expect("ready");
            bound.serve(token).await.expect("served");
        });
    });
    let address = address.recv_timeout(Duration::from_secs(120)).expect("the proctor came up");
    Running { base: format!("http://{address}"), address, stop, thread }
}

impl Running {
    fn shut_down(self) {
        self.stop.cancel_now();
        self.thread.join().expect("the proctor stopped cleanly");
    }
}

fn development() -> Gate {
    Gate { origins: CrossOriginPolicy::LoopbackDevelopment, forwarding: Forwarding::Untrusted, limits: Limits::default() }
}
//#endregion 🔖️Server

//#region 🔖️Wire
fn bytes(text: &str) -> Value {
    Value::Array(text.as_bytes().iter().map(|byte| json!(byte)).collect())
}

fn text(bytes: &Value) -> String {
    String::from_utf8(bytes.as_array().expect("byte array").iter().map(|byte| byte.as_u64().expect("byte") as u8).collect()).expect("utf-8")
}

/// 🎯️ The actor a client addresses a command to: the handle of a pseudonym or name being registered —
/// the lowercase hex of its normalized key, as the browser client computes it — and the learner otherwise.
fn target(command: &Value) -> Value {
    let named = command["identity"]["handle"].as_str().filter(|_| command["type"] == "identify-learner").and_then(quiz::normalize_handle);
    match named {
        Some(handle) => json!({ "tenant": TENANT, "kind": "quiz-handle", "id": quiz::handle_actor_id(&handle.key) }),
        None => json!({ "tenant": TENANT, "kind": "quiz-learner", "id": command["learner"] }),
    }
}

fn envelope(command: &Value) -> Value {
    let kind = command["type"].as_str().expect("type");
    let learner = command["learner"].as_str().expect("learner");
    let target = target(command);
    let principal = if kind == "identify-learner" { json!({ "kind": "anonymous" }) } else { json!({ "kind": "user", "id": learner }) };
    json!({
        "commandId": command["id"],
        "kind": format!("quiz.{kind}"),
        "version": 1,
        "target": target,
        "scope": TENANT,
        "principal": principal,
        "session": null,
        "device": null,
        "payload": bytes(&command.to_string()),
        "causalFrontier": null,
        "clientHlc": { "millis": 1_727_500_000_000u64, "counter": 0 },
        "expectedRevision": null,
        "idempotencyKey": command["id"],
        "capabilityProof": null,
        "trace": { "trace_id": "e2e", "span_id": "e2e" }
    })
}

fn post(base: &str, path: &str, body: &Value) -> (u16, Value) {
    let (status, _, answer) = post_with(base, path, body, &[]);
    (status, answer)
}

/// 📮️ `POST` a JSON body with extra request headers (never credentials); status, headers and JSON answer.
fn post_with(base: &str, path: &str, body: &Value, headers: &[(&str, &str)]) -> (u16, Vec<(String, String)>, Value) {
    let request = headers.iter().fold(ureq::post(&format!("{base}{path}")).set("content-type", "application/json"), |request, (name, value)| request.set(name, value));
    let response = match request.send_string(&body.to_string()) {
        Ok(response) => response,
        Err(ureq::Error::Status(_, response)) => response,
        Err(error) => panic!("POST {path}: {error}"),
    };
    let pairs = response.headers_names().iter().filter_map(|name| response.header(name).map(|value| (name.to_ascii_lowercase(), value.to_string()))).collect();
    (response.status(), pairs, serde_json::from_str(&response.into_string().unwrap_or_default()).unwrap_or(Value::Null))
}

/// 📨️ Submit one quiz command; the outcome and the quiz events it carries.
fn command(base: &str, command: &Value) -> (Value, Vec<Value>) {
    let (status, outcome) = post(base, "/commands", &envelope(command));
    assert_eq!(status, 200, "{outcome}");
    let events = outcome["events"].as_array().map(|events| events.iter().map(|record| serde_json::from_str(&text(&record["payload"])).expect("event json")).collect()).unwrap_or_default();
    (outcome, events)
}

fn accepted(base: &str, sent: &Value) -> Vec<Value> {
    let (outcome, events) = command(base, sent);
    assert_eq!(outcome["status"], "accepted", "{outcome}");
    events
}

fn rejected(base: &str, sent: &Value) -> String {
    let (outcome, _) = command(base, sent);
    assert_eq!(outcome["status"], "rejected", "{outcome}");
    assert_eq!(outcome["reason"]["kind"], "invalid", "{outcome}");
    outcome["reason"]["detail"].as_str().expect("detail").to_string()
}

fn query_envelope(query: &Value) -> Value {
    let kind = query["type"].as_str().expect("type");
    json!({ "queryId": id(0), "kind": format!("quiz.{kind}"), "version": 1, "scope": TENANT, "principal": { "kind": "anonymous" }, "arguments": bytes(&query.to_string()), "consistency": { "kind": "authority" }, "cursor": null })
}

fn query(base: &str, query: &Value) -> (u16, Value) {
    let (status, result) = post(base, "/queries", &query_envelope(query));
    if status != 200 {
        return (status, result);
    }
    assert_eq!(result["kind"], "snapshot", "{result}");
    (status, serde_json::from_str(&text(&result["value"])).expect("view json"))
}

fn view(base: &str, arguments: &Value) -> Value {
    let (status, view) = query(base, arguments);
    assert_eq!(status, 200, "{view}");
    view
}

fn get(base: &str, path: &str, headers: &[(&str, &str)]) -> (u16, Vec<(String, String)>, String) {
    let request = headers.iter().fold(ureq::get(&format!("{base}{path}")), |request, (name, value)| request.set(name, value));
    let response = match request.call() {
        Ok(response) => response,
        Err(ureq::Error::Status(_, response)) => response,
        Err(error) => panic!("GET {path}: {error}"),
    };
    let names = response.headers_names();
    let pairs = names.iter().filter_map(|name| response.header(name).map(|value| (name.to_ascii_lowercase(), value.to_string()))).collect();
    (response.status(), pairs, response.into_string().unwrap_or_default())
}

fn header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
}

/// 🧵️ One raw HTTP/1.1 request, for paths an URL library would normalize away.
fn raw(address: SocketAddr, request: &str) -> String {
    let mut stream = TcpStream::connect(address).expect("connect");
    stream.set_read_timeout(Some(Duration::from_secs(30))).expect("timeout");
    stream.write_all(request.as_bytes()).expect("request");
    let mut answer = String::new();
    let _ = stream.read_to_string(&mut answer);
    answer
}
//#endregion 🔖️Wire

//#region 🔖️Presence
type Socket = WebSocket<TcpStream>;

/// 🔌️ Open the presence socket of `scope` with extra handshake headers: the socket, or the refusing
/// status and its `x-semio-refusal`.
fn presence(address: SocketAddr, scope: &str, surface: &str, headers: &[(&str, &str)]) -> Result<Socket, (u16, Option<String>)> {
    let scope = scope.replace('/', "%2F");
    let mut request = format!("ws://{address}/scopes/{scope}/presence/ws?surface={surface}").into_client_request().expect("a websocket request");
    request.headers_mut().insert("sec-websocket-protocol", HeaderValue::from_static(PRESENCE_PROTOCOL_V1));
    for (name, value) in headers {
        request.headers_mut().insert(HeaderName::from_bytes(name.as_bytes()).expect("a header name"), HeaderValue::from_str(value).expect("a header value"));
    }
    let stream = TcpStream::connect(address).expect("connect");
    stream.set_read_timeout(Some(Duration::from_secs(10))).expect("timeout");
    match tungstenite::client(request, stream) {
        Ok((socket, response)) => {
            assert_eq!(response.headers()["sec-websocket-protocol"], PRESENCE_PROTOCOL_V1);
            Ok(socket)
        }
        Err(HandshakeError::Failure(tungstenite::Error::Http(response))) => Err((response.status().as_u16(), response.headers().get("x-semio-refusal").and_then(|value| value.to_str().ok()).map(str::to_string))),
        Err(error) => panic!("presence handshake: {error}"),
    }
}

/// 📥️ The next text frame, skipping the keepalive.
fn frame(socket: &mut Socket) -> Value {
    loop {
        match socket.read().expect("a frame within the read timeout") {
            Message::Text(text) => return serde_json::from_str(text.as_str()).expect("a json frame"),
            Message::Ping(_) | Message::Pong(_) => {}
            other => panic!("unexpected {other:?}"),
        }
    }
}

/// 👋️ The `welcome` a joining socket receives first: its session and the frame.
fn welcome(socket: &mut Socket) -> (String, Value) {
    let welcome = frame(socket);
    assert_eq!(welcome["type"], "welcome", "{welcome}");
    (welcome["session"].as_str().expect("a session").to_string(), welcome)
}

fn share(socket: &mut Socket, state: &Value) {
    socket.send(Message::text(json!({ "type": "state", "state": state }).to_string())).expect("sent");
}

/// ⏳️ Read frames until one satisfies `found`; every frame read, that one last.
fn until(socket: &mut Socket, found: impl Fn(&Value) -> bool) -> Vec<Value> {
    let mut seen = Vec::new();
    loop {
        let next = frame(socket);
        let done = found(&next);
        seen.push(next);
        if done {
            return seen;
        }
    }
}

/// 🚫️ The reason of the next `refused` frame.
fn refusal(socket: &mut Socket) -> Value {
    until(socket, |frame| frame["type"] == "refused").pop().expect("a refused frame")["reason"].clone()
}

/// 📦️ Whether `frame` is a batch carrying `session` with `state`.
fn carries(frame: &Value, session: &str, state: &Value) -> bool {
    frame["type"] == "batch" && frame["entries"].as_array().expect("entries").iter().any(|entry| entry["session"] == session && &entry["state"] == state)
}

/// 🚶️ Whether `frame` is a batch announcing that `session` left.
fn departs(frame: &Value, session: &str) -> bool {
    frame["type"] == "batch" && frame["left"].as_array().expect("left").iter().any(|left| left == session)
}

/// 👀️ Watch `scopes` read-only at `interval_ms`.
fn watch(socket: &mut Socket, scopes: &[&str], interval_ms: u64) {
    socket.send(Message::text(json!({ "type": "watch", "scopes": scopes, "intervalMs": interval_ms }).to_string())).expect("sent");
}

/// 📦️ Whether `frame` is a `watched` frame of `scope` carrying `session` with `state`.
fn sees(frame: &Value, scope: &str, session: &str, state: &Value) -> bool {
    frame["type"] == "watched" && frame["scope"] == scope && frame["entries"].as_array().expect("entries").iter().any(|entry| entry["session"] == session && &entry["state"] == state)
}

/// 🔚️ Close with the handshake and drain until the server has hung up.
fn close(mut socket: Socket) {
    socket.close(None).expect("close");
    while socket.read().is_ok() {}
}
//#endregion 🔖️Presence

//#region 🔖️Answers
fn quiz_document(quiz: &str) -> Value {
    let file = match quiz {
        "power" => "⚡️power/🔣️.json",
        _ => "🏠️homes/🔣️.json",
    };
    serde_json::from_str(&std::fs::read_to_string(fixtures().join(file)).expect("quiz file")).expect("quiz json")
}

/// 💯️ The perfect answer to one presented task, read off the quiz file's solutions.
fn perfect(quiz: &Value, presented: &Value) -> Value {
    let task = quiz["tasks"].as_array().expect("tasks").iter().find(|task| task["id"] == presented["id"]).expect("task");
    let solution = |item: &Value| task["items"].as_array().expect("items").iter().find(|candidate| candidate["id"] == item["id"]).expect("item").clone();
    let shown = presented["items"].as_array().expect("sheet items");
    match presented["kind"].as_str().expect("kind") {
        "sorting" => {
            let mut order: Vec<Value> = shown.iter().map(solution).collect();
            order.sort_by(|left, right| left["value"].as_f64().unwrap_or_default().total_cmp(&right["value"].as_f64().unwrap_or_default()));
            json!({ "kind": "sorting", "order": order.iter().map(|item| item["id"].clone()).collect::<Vec<_>>() })
        }
        "classification" => json!({ "kind": "classification", "assignments": shown.iter().map(|item| (item["id"].as_str().expect("id").to_string(), solution(item)["category"].clone())).collect::<serde_json::Map<_, _>>() }),
        _ => {
            let dimensions = presented["dimensions"].as_array().expect("dimensions").iter().map(|dimension| {
                let cards = dimension["cards"].as_array().expect("cards");
                let mut used = Vec::new();
                let assignment: serde_json::Map<String, Value> = shown
                    .iter()
                    .map(|item| {
                        let value = &solution(item)["values"][dimension["id"].as_str().expect("dimension id")];
                        let card = (0..cards.len()).find(|card| cards[*card].as_f64() == value.as_f64() && !used.contains(card)).expect("a card of that value");
                        used.push(card);
                        (item["id"].as_str().expect("id").to_string(), json!(card))
                    })
                    .collect();
                (dimension["id"].as_str().expect("dimension id").to_string(), Value::Object(assignment))
            });
            json!({ "kind": "matching", "assignments": dimensions.collect::<serde_json::Map<_, _>>() })
        }
    }
}

fn record(learner: &str, run: &str, task: &Value, answer: &Value, seed: u32) -> Value {
    json!({ "type": "record-answer", "id": id(seed), "learner": learner, "run": run, "task": task["id"], "answer": answer })
}

/// 🏁️ Play a whole perfect run and return the submission's events.
fn play(base: &str, learner: &str, run: &str, quiz: &str, seed: u32) -> Vec<Value> {
    accepted(base, &json!({ "type": "start-run", "id": id(seed), "learner": learner, "run": run, "quiz": quiz }));
    let sheet = view(base, &json!({ "type": "run", "run": run }))["sheet"].clone();
    let document = quiz_document(quiz);
    for (index, task) in sheet["tasks"].as_array().expect("tasks").iter().enumerate() {
        accepted(base, &record(learner, run, task, &perfect(&document, task), seed + 1 + index as u32));
    }
    accepted(base, &json!({ "type": "submit-run", "id": id(seed + 50), "learner": learner, "run": run }))
}
//#endregion 🔖️Answers

#[test]
fn a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart() {
    let data = scratch("lifecycle");
    let running = boot(&data.0, development());
    let base = running.base.clone();

    let (status, _, instance) = get(&base, "/instance", &[]);
    let instance: Value = serde_json::from_str(&instance).expect("instance json");
    assert_eq!((status, instance["id"].as_str(), instance["modules"][0]["id"].as_str()), (200, Some("teaching-proctor"), Some("teaching.proctor")));

    let catalog = view(&base, &json!({ "type": "catalog" }));
    assert_eq!(catalog["id"], TENANT);
    assert_eq!(catalog["quizzes"].as_array().expect("quizzes").iter().map(|quiz| quiz["id"].as_str().expect("id")).collect::<Vec<_>>(), ["power", "homes"]);
    assert!(!catalog.to_string().contains("\"value\""), "the catalog view carries no solutions");

    let (anonymous, ada, other) = (id(0xa1), id(0xada), id(0xc3));
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(1), "learner": anonymous, "identity": { "kind": "anonymous" } }));
    assert_eq!(events, [json!({ "type": "learner-registered", "learner": anonymous, "identity": { "kind": "anonymous" }, "at": events[0]["at"] })]);
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(2), "learner": ada, "identity": { "kind": "pseudonym", "handle": "  Ada   Lovelace " } }));
    assert_eq!(events[0]["identity"], json!({ "kind": "pseudonym", "handle": "Ada Lovelace" }));
    assert_eq!(get(&base, &format!("/actors/{TENANT}/quiz-handle/{}/events", quiz::handle_actor_id("ada lovelace")), &[]).0, 403, "who holds a handle is not readable as a stream");
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
    let (_, _, registered) = get(&base, &format!("/actors/{TENANT}/quiz-learner/{ada}/events"), &[]);
    assert_eq!(serde_json::from_str::<Vec<Value>>(&registered).expect("event records").len(), 1, "recalling a handle appended nothing to its learner");

    let run = id(0x100);
    let events = accepted(&base, &json!({ "type": "start-run", "id": id(10), "learner": ada, "run": run, "quiz": "power" }));
    assert_eq!((events[0]["type"].as_str(), events[0]["revision"].as_str().map(str::len)), (Some("run-started"), Some(64)));
    let open = view(&base, &json!({ "type": "run", "run": run }));
    assert_eq!((open["status"].as_str(), open["sheet"]["tasks"].as_array().map(Vec::len), open["answers"].as_object().map(|answers| answers.len())), (Some("open"), Some(2), Some(0)));
    assert_eq!(open["sheet"]["seed"], events[0]["seed"]);
    assert!(!open["sheet"].to_string().contains("\"value\""), "the sheet carries no solutions");

    let document = quiz_document("power");
    let tasks = open["sheet"]["tasks"].as_array().expect("tasks").clone();
    let first = record(&ada, &run, &tasks[0], &perfect(&document, &tasks[0]), 20);
    assert_eq!(accepted(&base, &first).len(), 1);
    assert!(accepted(&base, &first).is_empty(), "a retried command id is applied once");
    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/quiz-learner/{ada}/events"), &[]);
    let stream: Vec<Value> = serde_json::from_str(&stream).expect("event records");
    assert_eq!(stream.iter().filter(|record| record["kind"] == "quiz.answer-recorded").count(), 1);
    assert_eq!(stream[0]["kind"], "quiz.learner-registered");
    assert_eq!(rejected(&base, &json!({ "type": "submit-run", "id": id(21), "learner": ada, "run": run })), "run-incomplete");
    assert_eq!(rejected(&base, &record(&ada, &run, &tasks[1], &json!({ "kind": "sorting", "order": [] }), 22)), "answer-invalid");
    accepted(&base, &record(&ada, &run, &tasks[1], &perfect(&document, &tasks[1]), 23));

    let submitted = accepted(&base, &json!({ "type": "submit-run", "id": id(24), "learner": ada, "run": run }));
    assert_eq!(submitted.iter().map(|event| event["type"].as_str().expect("type")).collect::<Vec<_>>(), ["run-submitted", "badge-awarded", "badge-awarded"]);
    assert_eq!(submitted[0]["result"]["score"], json!(1.0));
    assert_eq!(submitted.iter().skip(1).map(|event| event["badge"].as_str().expect("badge")).collect::<Vec<_>>(), ["perfect-power", "sorter"]);
    assert_eq!(rejected(&base, &json!({ "type": "submit-run", "id": id(25), "learner": ada, "run": run })), "run-closed");
    assert_eq!(rejected(&base, &json!({ "type": "start-run", "id": id(26), "learner": ada, "run": id(0x101), "quiz": "cooling" })), "unknown-quiz");
    assert_eq!(rejected(&base, &json!({ "type": "start-run", "id": id(27), "learner": id(0xdead), "run": id(0x102), "quiz": "power" })), "unknown-learner");

    let finished = view(&base, &json!({ "type": "run", "run": run }));
    assert_eq!((finished["status"].as_str(), finished["result"]["score"].as_f64(), finished["submittedAt"].is_u64()), (Some("submitted"), Some(1.0), true));
    let learner = view(&base, &json!({ "type": "learner", "learner": ada }));
    assert_eq!((learner["best"]["power"].as_f64(), learner["total"].as_f64()), (Some(1.0), Some(100.0)));
    assert_eq!(learner["badges"].as_array().expect("badges").iter().map(|award| award["badge"].as_str().expect("badge")).collect::<Vec<_>>(), ["perfect-power", "sorter"]);
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": ada }));
    assert_eq!(board["rows"].as_array().expect("rows").iter().map(|row| (row["rank"].as_u64(), row["tag"].as_str(), row["total"].as_f64())).collect::<Vec<_>>(), [(Some(1), Some(quiz::learner_tag(&ada).as_str()), Some(100.0))]);
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
    assert_eq!(query(&base, &json!({ "type": "leaderboard", "period": "weekly", "quiz": "cooling" })).0, 404, "a quiz the catalog does not list has no leaderboard");
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
    let completed = play(&base, &ada, &id(0x200), "homes", 30);
    assert!(completed.iter().any(|event| event["badge"] == "complete"), "{completed:?}");
    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/quiz-learner/{ada}/events"), &[]);
    let sequences: Vec<u64> = serde_json::from_str::<Vec<Value>>(&stream).expect("records").iter().map(|record| record["seq"].as_u64().expect("seq")).collect();
    assert_eq!(sequences, (1..=sequences.len() as u64).collect::<Vec<_>>(), "the restarted authority appends after its history");
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time" }));
    assert_eq!((board["rows"][0]["total"].as_f64(), board["rows"][0]["badges"].as_array().map(Vec::len), board["submissions"].as_u64()), (Some(200.0), Some(3), Some(2)));
    let homes = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "quiz": "homes", "learner": ada }));
    assert_eq!((homes["own"]["total"].as_f64(), homes["own"]["best"].clone(), homes["own"]["runs"].as_u64(), homes["own"]["badges"].clone(), homes["submissions"].as_u64()), (Some(100.0), json!({ "homes": 1.0 }), Some(1), json!(["complete"]), Some(2)), "the board of one quiz is made of that quiz's runs and the badges they earned: {homes}");
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
    let (ada, grace) = (quiz::learner_tag(&id(0xada)), quiz::learner_tag(&id(0x9ace)));

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

    assert_eq!(presence(running.address, "other-catalog", "home", &local).err(), Some((403, None)), "another catalog's roster is no room here");
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
    let (ada_tag, grace_tag) = (quiz::learner_tag(&ada), quiz::learner_tag(&grace));
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
    share(&mut thinker, &json!({ "tag": ada_tag, "answers": { "sources": { "kind": "matching", "values": { "hours": { "rooftop-pv": 10000 } } } } }));
    assert_eq!(refusal(&mut thinker), "value-unknown /answers/sources/values/hours/rooftop-pv");

    let mut dragger = presence(address, &quiz_room, "run", &local).expect("joins the quiz room");
    let (dragger_session, _) = welcome(&mut dragger);
    let dragging = json!({ "tag": grace_tag, "cursor": { "anchor": "item:kettle", "x": 0.5, "y": 0.5 }, "drag": { "item": "kettle" } });
    share(&mut dragger, &dragging);
    until(&mut home, |frame| sees(frame, &quiz_room, &dragger_session, &dragging));
    share(&mut dragger, &json!({ "tag": grace_tag, "drag": { "item": "pellets" } }));
    assert_eq!(refusal(&mut dragger), "id-unknown /drag/item");
    watch(&mut home, &["other-catalog/home"], 250);
    assert_eq!(refusal(&mut home), "forbidden other-catalog/home");
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
    play(&base, &ada, &ada_run, "power", 100);
    accepted(&base, &json!({ "type": "start-run", "id": id(40), "learner": grace, "run": grace_run, "quiz": "power" }));
    let document = quiz_document("power");
    for (index, task) in view(&base, &json!({ "type": "run", "run": grace_run }))["sheet"]["tasks"].as_array().expect("tasks").iter().enumerate() {
        let mut answer = perfect(&document, task);
        if let Some(order) = answer.get_mut("order").and_then(Value::as_array_mut) {
            order.reverse();
        }
        accepted(&base, &record(&grace, &grace_run, task, &answer, 41 + index as u32));
    }
    accepted(&base, &json!({ "type": "submit-run", "id": id(70), "learner": grace, "run": grace_run }));
    let results: Vec<quiz::RunResult> = [&ada_run, &grace_run].iter().map(|run| serde_json::from_value(view(&base, &json!({ "type": "run", "run": run }))["result"].clone()).expect("a run result")).collect();
    let catalog = load_catalog(&fixtures().join("📚️catalog/🔣️.json")).expect("fixture catalog");
    let oracle = serde_json::to_string(&quiz::crowd_view(&catalog.entry("power").expect("power").quiz, &results)).expect("oracle json");
    let answered = crowd(&base);
    assert_eq!(answered, oracle, "the proctor's crowd is the core's crowd_view, byte for byte");
    let answered: Value = serde_json::from_str(&answered).expect("crowd json");
    assert_eq!(answered["runs"], 2);
    let means: Vec<f64> = answered["tasks"][0]["items"].as_array().expect("sorting items").iter().map(|item| item["meanPosition"].as_f64().expect("a mean")).collect();
    assert!(means.iter().all(|mean| (mean - 0.5).abs() < 1e-12), "a perfect and a reversed order meet in the middle: {means:?}");
    let counts = |bins: &Value| -> Vec<u64> { bins.as_array().expect("counts").iter().map(|count| count.as_u64().expect("a count")).collect() };
    let scores: Vec<Vec<u64>> = std::iter::once(&answered).chain(answered["tasks"].as_array().expect("tasks")).map(|scored| counts(&scored["scores"])).collect();
    assert!(scores.iter().all(|bins| bins.len() == quiz::CROWD_SCORE_BINS && bins.iter().sum::<u64>() == 2), "both runs fall into the ten score bins of the quiz, of every task and of every dimension: {scores:?}");
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

/// ⌨️ Run the built `proctor` binary over `data` with `stdin`, answering its exit success and stdout.
fn operator(data: &Path, port: u16, arguments: &[&str], stdin: &[u8]) -> (bool, Vec<u8>) {
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_proctor"))
        .args(arguments)
        .env("PROCTOR_DATA", data)
        .env("PROCTOR_PORT", port.to_string())
        .env_remove("PROCTOR_BIND")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("the proctor binary runs");
    child.stdin.take().expect("stdin").write_all(stdin).expect("stdin written");
    let output = child.wait_with_output().expect("the proctor binary exits");
    (output.status.success(), output.stdout)
}

/// 🗂️ The file names in `directory`, ascending.
fn files(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(directory).expect("listing").filter_map(Result::ok).map(|entry| entry.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    names
}

/// 🔍️ Whether any file of `directory` holds the bytes of `needle`.
fn holds(directory: &Path, needle: &str) -> bool {
    files(directory).iter().any(|file| std::fs::read(directory.join(file)).expect("file").windows(needle.len()).any(|window| window == needle.as_bytes()))
}

#[test]
fn the_operator_probes_backs_up_while_serving_and_restores_elsewhere() {
    let (data, elsewhere, backups, third) = (scratch("operator"), scratch("operator-restore"), scratch("operator-backups"), scratch("operator-restore-file"));
    let running = boot(&data.0, development());
    let (base, port) = (running.base.clone(), running.address.port());
    assert!(operator(&data.0, port, &["health"], b"").0, "a serving proctor is healthy");

    let (ada, run) = (id(0xada), id(0x100));
    accepted(&base, &json!({ "type": "identify-learner", "id": id(1), "learner": ada, "identity": { "kind": "pseudonym", "handle": "Ada" } }));
    play(&base, &ada, &run, "power", 10);
    let views = |base: &str| [json!({ "type": "learner", "learner": ada }), json!({ "type": "run", "run": run }), json!({ "type": "leaderboard", "period": "all-time", "learner": ada }), json!({ "type": "crowd", "quiz": "power" }), json!({ "type": "handle", "handle": "ada" }), json!({ "type": "catalog" })].map(|asked| view(base, &asked));
    let before = views(&base);

    let (backed_up, backup) = operator(&data.0, port, &["backup", "-"], b"");
    assert!(backed_up, "a backup is taken while the proctor serves");
    assert_eq!(&backup[..16], b"SQLite format 3\0");
    let directory = backups.0.to_string_lossy().into_owned();
    let (filed, printed) = operator(&data.0, port, &["backup", &directory], b"");
    let file = PathBuf::from(String::from_utf8(printed).expect("a path").trim_end());
    let name = file.file_name().expect("a file name").to_string_lossy().into_owned();
    assert!(filed && file.parent() == Some(backups.0.as_path()) && name.starts_with("proctor-2") && name.ends_with("Z.sqlite") && name.len() == 31, "a backup into a directory prints the timestamped file it wrote: {}", file.display());
    assert_eq!((files(&backups.0), std::fs::read(&file).expect("the backup").len()), (vec![name], backup.len()), "one whole file and nothing partial beside it");
    assert!(!operator(&data.0, port, &["backup", &file.to_string_lossy()], b"").0, "a backup is never overwritten");
    assert_eq!(files(&data.0).iter().filter(|name| name.contains(".backup-") || name.contains(".partial-")).count(), 0, "the spool file is gone");

    play(&base, &ada, &id(0x200), "homes", 100);
    assert_ne!(views(&base), before, "the proctor kept committing after the backup");
    assert!(!operator(&data.0, port, &["restore", "-"], &backup).0, "a serving proctor's database is never replaced");
    running.shut_down();
    assert!(!operator(&data.0, port, &["health"], b"").0, "a stopped proctor is not healthy");

    assert!(operator(&elsewhere.0, port, &["restore", "-"], &backup).0, "the backup restores into an empty directory");
    let restored = boot(&elsewhere.0, development());
    assert_eq!(views(&restored.base), before, "the proctor started on the copy answers the views of the moment the backup was taken");
    let completed = play(&restored.base, &ada, &id(0x300), "homes", 200);
    assert!(completed.iter().any(|event| event["badge"] == "complete"), "the restored authority appends after its history: {completed:?}");
    restored.shut_down();

    assert!(operator(&third.0, port, &["restore", &file.to_string_lossy()], b"").0, "the backup file restores as well");
    let restored = boot(&third.0, development());
    assert_eq!(views(&restored.base), before, "and answers the same");
    restored.shut_down();
}

#[test]
fn the_operator_erases_one_learner_on_request_and_its_handle_is_free_again() {
    const NAME: &str = "Ada Lovelace";
    let data = scratch("operator-erase");
    let running = boot(&data.0, development());
    let (base, port) = (running.base.clone(), running.address.port());
    let (ada, grace) = (id(0xada), id(0x9ace));
    accepted(&base, &json!({ "type": "identify-learner", "id": id(1), "learner": ada, "identity": { "kind": "name", "handle": NAME } }));
    accepted(&base, &json!({ "type": "identify-learner", "id": id(2), "learner": grace, "identity": { "kind": "anonymous" } }));
    play(&base, &ada, &id(0x100), "power", 10);
    play(&base, &grace, &id(0x101), "power", 100);
    let others = |base: &str| [json!({ "type": "learner", "learner": grace }), json!({ "type": "run", "run": id(0x101) })].map(|asked| view(base, &asked));
    let before = others(&base);
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": grace }));
    assert_eq!((board["learners"].as_u64(), board["own"]["rank"].as_u64(), board["rows"][0]["identity"]["handle"].as_str()), (Some(2), Some(2), Some(NAME)));
    assert_eq!(view(&base, &json!({ "type": "crowd", "quiz": "power" }))["runs"], 2);
    assert!(!operator(&data.0, port, &["erase", "--handle", NAME], b"").0, "nobody is erased from a database that is being served");
    running.shut_down();

    let tag = quiz::learner_tag(&ada);
    let (dry, report) = operator(&data.0, port, &["erase", "--tag", &tag, "--dry-run"], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(dry && report.starts_with(&format!("would erase learner #{tag} name {NAME:?} of catalog {TENANT}\n")) && report.contains("  handle \"ada lovelace\": 1 events") && report.ends_with("dry run: nothing was changed\n"), "{report}");
    assert!(holds(&data.0, NAME) && holds(&data.0, &ada), "a dry run changes nothing");
    assert!(!operator(&data.0, port, &["erase", "--handle", "Grace Hopper"], b"").0, "nobody holds that handle");
    let (erased, report) = operator(&data.0, port, &["erase", "--handle", " ada  LOVELACE "], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(erased && report.starts_with(&format!("erased learner #{tag} name {NAME:?} of catalog {TENANT}\n")) && !report.contains(&ada), "{report}");
    assert!(!holds(&data.0, NAME) && !holds(&data.0, "ada lovelace") && !holds(&data.0, &ada) && holds(&data.0, &grace), "neither the name nor the learner id is left in any file of the data directory: {:?}", files(&data.0));
    assert!(!operator(&data.0, port, &["erase", "--learner", &ada], b"").0, "an erased learner is nobody");

    let running = boot(&data.0, development());
    let base = running.base.clone();
    assert_eq!(others(&base), before, "the other learner's views are what they were");
    assert_eq!(query(&base, &json!({ "type": "learner", "learner": ada })).0, 404);
    assert_eq!(query(&base, &json!({ "type": "run", "run": id(0x100) })).0, 404);
    assert_eq!(view(&base, &json!({ "type": "handle", "handle": NAME })), json!({ "display": NAME }), "the handle is free");
    let board = view(&base, &json!({ "type": "leaderboard", "period": "all-time", "learner": grace }));
    assert_eq!((board["learners"].as_u64(), board["own"]["rank"].as_u64(), board["rows"].as_array().map(Vec::len)), (Some(1), Some(1), Some(1)), "the erased scores are gone from the board: {board}");
    assert!(!board.to_string().contains(NAME) && !board.to_string().contains(&tag), "{board}");
    assert_eq!(view(&base, &json!({ "type": "crowd", "quiz": "power" }))["runs"], 1, "and from the crowd");
    assert_eq!(rejected(&base, &json!({ "type": "start-run", "id": id(20), "learner": ada, "run": id(0x102), "quiz": "power" })), "unknown-learner");
    let successor = id(0xb0b);
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(21), "learner": successor, "identity": { "kind": "pseudonym", "handle": NAME } }));
    assert_eq!((events[0]["type"].as_str(), events[0]["learner"].as_str()), (Some("learner-registered"), Some(successor.as_str())), "somebody else takes the freed handle");
    assert_eq!(view(&base, &json!({ "type": "handle", "handle": NAME }))["holder"]["learner"], successor.as_str());
    running.shut_down();
}

/// 🧮️ How many registrations a proctor booted over the stopped `data` directory counts: what its
/// cap of learners is held against.
fn registrations(data: &Path) -> u64 {
    let data = data.to_string_lossy().into_owned();
    tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().expect("runtime").block_on(async move {
        let catalog = Arc::new(load_catalog(&fixtures().join("📚️catalog/🔣️.json")).expect("fixture catalog"));
        let proctor = Proctor::assemble(StorageProfile::Embedded { data_dir: data }, catalog, development(), PresenceSettings::default()).await.expect("assembled");
        proctor.prepare().await.expect("prepared");
        proctor.reconcile().await.expect("reconciled");
        proctor.settle(&CancelToken::root_now(), |_| {}).await.expect("settled");
        proctor.admission().learners()
    })
}

#[test]
fn the_operator_prunes_the_registrations_nobody_played_under_and_every_player_stays() {
    let data = scratch("operator-prune");
    let running = boot(&data.0, development());
    let (base, port) = (running.base.clone(), running.address.port());
    let (ada, grace, hoarder, linus) = (id(0xada), id(0x9ace), id(0x5200), id(0x11a5));
    let register = |seed: u32, learner: &str, identity: Value| accepted(&base, &json!({ "type": "identify-learner", "id": id(seed), "learner": learner, "identity": identity }));
    register(1, &ada, json!({ "kind": "name", "handle": "Ada Lovelace" }));
    register(2, &grace, json!({ "kind": "anonymous" }));
    play(&base, &ada, &id(0x100), "power", 10);
    play(&base, &grace, &id(0x101), "power", 100);
    for junk in 0..40 {
        register(0x6000 + junk, &id(0x5000 + junk), json!({ "kind": "anonymous" }));
    }
    for junk in 0..20 {
        register(0x6100 + junk, &id(0x5100 + junk), json!({ "kind": "pseudonym", "handle": format!("Junk Number {junk:02}") }));
    }
    register(0x6200, &hoarder, json!({ "kind": "anonymous" }));
    for hoard in 0..5 {
        register(0x6201 + hoard, &hoarder, json!({ "kind": "pseudonym", "handle": format!("Hoard Number {hoard}") }));
    }
    for spare in 0..3 {
        register(0x6300 + spare, &ada, json!({ "kind": "pseudonym", "handle": format!("Spare Number {spare}") }));
    }
    register(3, &linus, json!({ "kind": "name", "handle": "Linus Halfway" }));
    accepted(&base, &json!({ "type": "start-run", "id": id(4), "learner": linus, "run": id(0x400), "quiz": "power" }));
    let players = |base: &str| [json!({ "type": "learner", "learner": ada }), json!({ "type": "run", "run": id(0x100) }), json!({ "type": "learner", "learner": grace }), json!({ "type": "leaderboard", "period": "all-time", "learner": grace }), json!({ "type": "crowd", "quiz": "power" }), json!({ "type": "handle", "handle": "ada lovelace" })].map(|asked| view(base, &asked));
    let before = players(&base);
    assert_eq!(view(&base, &json!({ "type": "handle", "handle": "junk number 07" }))["holder"]["learner"], id(0x5107).as_str());
    assert!(!operator(&data.0, port, &["prune", "--older-than", "1s"], b"").0, "nothing is pruned from a database that is being served");
    running.shut_down();
    assert_eq!(registrations(&data.0), 72, "forty-two anonymous learners and thirty handles");

    let junk = ["Junk Number 07", "junk number 07", "Hoard Number 3", "Spare Number 1", "spare number 1", "Linus Halfway"];
    let strangers = [id(0x5007), id(0x5107), id(0x6007), id(0x6107), hoarder.clone(), linus.clone(), quiz::handle_actor_id("junk number 07"), quiz::handle_actor_id("spare number 1")];
    for wrong in [&["prune"][..], &["prune", "--older-than", "7"], &["prune", "--older-than", "soon"], &["prune", "--dry-run"]] {
        assert!(!operator(&data.0, port, wrong, b"").0, "{wrong:?}");
    }
    let (recent, report) = operator(&data.0, port, &["prune", "--older-than", "1h"], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(recent && report.starts_with(&format!("pruning 0 registrations of catalog {TENANT} older than 1h (registered before 2")) && report.ends_with("nothing to prune: nothing was changed\n"), "{report}");
    std::thread::sleep(Duration::from_millis(1500));
    let (dry, report) = operator(&data.0, port, &["prune", "--older-than", "1s", "--dry-run"], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(dry && report.starts_with(&format!("would prune 70 registrations of catalog {TENANT} older than 1s (registered before 2")), "{report}");
    assert!(report.ends_with("Z)\n  learners that never submitted a run: 62 (41 anonymous, 20 under a pseudonym, 1 under a name), holding 26 handles\n  handles claimed beside the identity of a learner that stays: 3\n  staying: 2 learners (2 of them submitted a run), 1 handles, 2 registrations\ndry run: nothing was changed\n"), "{report}");
    assert!(junk.iter().all(|name| holds(&data.0, name)) && strangers.iter().all(|stranger| holds(&data.0, stranger)), "a dry run changes nothing");
    assert!(!junk.iter().any(|name| report.contains(name)) && !strangers.iter().any(|stranger| report.contains(stranger.as_str())), "and its report counts, it names nobody: {report}");
    assert_eq!(registrations(&data.0), 72);

    let (pruned, report) = operator(&data.0, port, &["prune", "--older-than", "1s"], b"");
    let report = String::from_utf8(report).expect("a report");
    assert!(pruned && report.starts_with("pruning 70 registrations") && report.contains("\nremoved 91 of 91 streams: 92 events, ") && report.ends_with("done: the handles are free again, the learner ids are unknown to the proctor; 2 registrations are left\n"), "{report}");
    assert!(junk.iter().all(|name| !holds(&data.0, name)) && strangers.iter().all(|stranger| !holds(&data.0, stranger)), "no name, learner id, command id or handle stream of theirs is left in any file of the data directory: {:?}", files(&data.0));
    assert!(holds(&data.0, &ada) && holds(&data.0, &grace) && holds(&data.0, "Ada Lovelace"));
    let (again, report) = operator(&data.0, port, &["prune", "--older-than", "1s"], b"");
    assert!(again && String::from_utf8(report).expect("a report").ends_with("nothing to prune: nothing was changed\n"));
    assert_eq!(registrations(&data.0), 2, "the count the cap is held against went down to who played");

    let running = boot(&data.0, development());
    let base = running.base.clone();
    assert_eq!(players(&base), before, "the views of everybody who played are what they were");
    for gone in [id(0x5007), id(0x5107), hoarder, linus.clone()] {
        assert_eq!(query(&base, &json!({ "type": "learner", "learner": gone })).0, 404, "{gone}");
    }
    for free in ["Junk Number 07", "Hoard Number 3", "Spare Number 1", "Linus Halfway"] {
        assert_eq!(view(&base, &json!({ "type": "handle", "handle": free })), json!({ "display": free }), "the handle is free");
    }
    assert_eq!(rejected(&base, &json!({ "type": "start-run", "id": id(5), "learner": linus, "run": id(0x401), "quiz": "power" })), "unknown-learner");
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(0x6007), "learner": id(0x5007), "identity": { "kind": "anonymous" } }));
    assert_eq!(events.len(), 1, "a pruned learner id and its command id are unknown again, not a replay");
    for (seed, successor, handle) in [(6, id(0xb0b), "junk number 07"), (7, id(0xb0c), "Spare Number 1")] {
        let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(seed), "learner": successor, "identity": { "kind": "name", "handle": handle } }));
        assert_eq!((events[0]["type"].as_str(), events[0]["identity"]["handle"].as_str()), (Some("learner-registered"), Some(handle)), "somebody takes a pruned handle");
        assert_eq!(view(&base, &json!({ "type": "learner", "learner": successor }))["identity"]["handle"], handle, "and is relayed to its learner, whoever the handle was relayed to before");
    }
    running.shut_down();
}

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

fn sign_up(seed: u32, learner: &str) -> Value {
    json!({ "type": "identify-learner", "id": id(seed), "learner": learner, "identity": { "kind": "anonymous" } })
}

/// 📏️ The size of the largest `record-answer` body a catalog can produce: every item of every task
/// answered, encoded as the client sends it.
fn largest_answer(catalog: &Path) -> usize {
    let read = |path: &Path| -> Value { serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))).expect("json") };
    let ids = |values: &Value| -> Vec<String> { values.as_array().map(|values| values.iter().map(|value| value["id"].as_str().expect("id").to_string()).collect()).unwrap_or_default() };
    let (learner, run) = (id(0xffff_ffff), id(0xffff_fffe));
    let mut largest = 0;
    for quiz in read(catalog)["quizzes"].as_array().expect("quizzes") {
        let document = read(&catalog.parent().expect("a directory").join(quiz.as_str().expect("a quiz path")));
        for task in document["tasks"].as_array().expect("tasks") {
            let items = ids(&task["items"]);
            let answer = match task["kind"].as_str().expect("kind") {
                "sorting" => json!({ "kind": "sorting", "order": items }),
                "classification" => {
                    let category = ids(&task["categories"]).into_iter().max_by_key(String::len).expect("a category");
                    json!({ "kind": "classification", "assignments": items.iter().map(|item| (item.clone(), json!(category))).collect::<serde_json::Map<_, _>>() })
                }
                _ => json!({ "kind": "matching", "assignments": ids(&task["dimensions"]).into_iter().map(|dimension| (dimension, Value::Object(items.iter().enumerate().map(|(card, item)| (item.clone(), json!(1000 + card))).collect()))).collect::<serde_json::Map<_, _>>() }),
            };
            largest = largest.max(envelope(&record(&learner, &run, task, &answer, 0xffff_fffd)).to_string().len());
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
        let elsewhere = envelope(&json!({ "type": "start-run", "id": key, "learner": intruder, "run": id(0x666), "quiz": "power" }));
        let mut relayed = elsewhere.clone();
        relayed["kind"] = json!("quiz.enroll-learner");
        relayed["principal"] = json!({ "kind": "serviceAccount", "id": "proctor" });
        [occupying, elsewhere, relayed]
    };

    let rehearsal = scratch("edge-key-read");
    let running = boot(&rehearsal.0, development());
    let (outcome, _) = command(&running.base, &named);
    assert_eq!(outcome["status"], "accepted", "{outcome}");
    let committed: EventRecord = serde_json::from_value(outcome["events"][0].clone()).expect("the committed event record");
    let key = proctor::actors::enrollment(&committed).expect("a named sign-up is relayed to its learner").command_id.0;
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
    assert_eq!(accepted(&base, &json!({ "type": "start-run", "id": id(2), "learner": ada, "run": id(0x100), "quiz": "power" }))[0]["type"], "run-started");
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
        let mut sent = envelope(&json!({ "type": "start-run", "id": id(100 + seed as u32), "learner": learner, "run": id(200 + seed as u32), "quiz": "power" }));
        sent["target"]["id"] = json!(target);
        let (status, outcome) = post(&base, "/commands", &sent);
        assert_eq!((status, outcome["status"].as_str(), outcome["reason"]["kind"].as_str()), (200, Some("rejected"), Some("invalid")), "{outcome}");
        assert!(outcome["reason"]["detail"].as_str().is_some_and(|detail| detail.starts_with("id-invalid")), "target {:?}: {outcome}", &target[..target.len().min(40)]);
    }
    let mut keyed = envelope(&json!({ "type": "start-run", "id": id(300), "learner": learner, "run": id(301), "quiz": "power" }));
    keyed["commandId"] = json!("k".repeat(4000));
    keyed["idempotencyKey"] = keyed["commandId"].clone();
    let (_, outcome) = post(&base, "/commands", &keyed);
    assert_eq!((outcome["status"].as_str(), outcome["reason"]["kind"].as_str()), (Some("rejected"), Some("invalid")), "an oversized key is no key: {outcome}");
    assert_eq!(accepted(&base, &json!({ "type": "start-run", "id": id(400), "learner": learner, "run": id(401), "quiz": "power" }))[0]["type"], "run-started", "the learner is untouched by the refusals");
    running.shut_down();
}

#[test]
fn a_body_past_the_limit_is_refused_and_the_largest_real_command_is_far_below_it() {
    let limit = Limits::default().body_bytes.expect("the production limits cap the body");
    for catalog in [fixtures().join("📚️catalog/🔣️.json"), Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🏛️architecture/❓️quiz/🔣️.json")] {
        let largest = largest_answer(&catalog);
        assert!(largest > 1000 && largest * 4 <= limit, "{}: the largest answer is {largest} bytes, the limit {limit}", catalog.display());
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

    let (status, _, started) = sent(&json!({ "type": "start-run", "id": id(40), "learner": ada, "run": id(0x100), "quiz": "power" }), &hall);
    assert_eq!((status, started["status"].as_str()), (200, Some("accepted")), "who is registered plays on: only sign-ups are spent: {started}");
    let (status, _, recalled) = post_with(&base, "/queries", &query_envelope(&json!({ "type": "handle", "handle": "ada lovelace" })), &hall);
    assert_eq!((status, serde_json::from_str::<Value>(&text(&recalled["value"])).expect("a view")["holder"]["learner"].as_str().map(str::to_string)), (200, Some(ada)), "and a returning learner is recalled by a read, which is no sign-up");

    for seed in 0..3u32 {
        registered(&sign_up(50 + seed, &id(0x800 + seed)), &script);
    }
    let (status, _, refused) = sent(&sign_up(53, &id(0x803)), &script);
    assert_eq!((status, refused["allowance"].as_str()), (429, Some("sign-up")), "another address has an allowance of its own, and no more");
    nothing(&json!({ "type": "start-run", "id": id(54), "learner": id(0x804), "run": id(0x101), "quiz": "power" }), &script, "unknown-learner");
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
    let mut stream = socket(address, &format!("/actors/{TENANT}/quiz-learner/{learner}/events/ws"), &[]);
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
