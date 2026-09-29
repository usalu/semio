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
use proctor::config::{CrossOriginPolicy, Forwarding, Gate};
use proctor::instance::Proctor;
use semio_framework_async::CancelToken;
use serde_json::{json, Value};
use server::gateway::{PresenceSettings, PRESENCE_PROTOCOL_V1};
use server::storage::StorageProfile;
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
    Gate { origins: CrossOriginPolicy::LoopbackDevelopment, forwarding: Forwarding::Untrusted }
}
//#endregion 🔖️Server

//#region 🔖️Wire
fn bytes(text: &str) -> Value {
    Value::Array(text.as_bytes().iter().map(|byte| json!(byte)).collect())
}

fn text(bytes: &Value) -> String {
    String::from_utf8(bytes.as_array().expect("byte array").iter().map(|byte| byte.as_u64().expect("byte") as u8).collect()).expect("utf-8")
}

fn envelope(command: &Value) -> Value {
    let kind = command["type"].as_str().expect("type");
    let learner = command["learner"].as_str().expect("learner");
    let target = if kind == "identify-learner" { json!({ "tenant": TENANT, "kind": "quiz-roster", "id": "roster" }) } else { json!({ "tenant": TENANT, "kind": "quiz-learner", "id": learner }) };
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
    let events = accepted(&base, &json!({ "type": "identify-learner", "id": id(3), "learner": other, "identity": { "kind": "name", "handle": "ada lovelace" } }));
    assert_eq!((events[0]["type"].as_str(), events[0]["learner"].as_str()), (Some("learner-recalled"), Some(ada.as_str())));
    assert_eq!(rejected(&base, &json!({ "type": "identify-learner", "id": id(4), "learner": other, "identity": { "kind": "pseudonym", "handle": " \t " } })), "handle-invalid");
    let learner = view(&base, &json!({ "type": "learner", "learner": ada }));
    assert_eq!((learner["identity"]["handle"].as_str(), learner["runs"].as_array().map(Vec::len)), (Some("Ada Lovelace"), Some(0)));
    assert_eq!(view(&base, &json!({ "type": "learner", "learner": anonymous }))["identity"], json!({ "kind": "anonymous" }));
    assert_eq!(query(&base, &json!({ "type": "learner", "learner": other })).0, 404);

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
    assert_eq!(get(&base, &format!("/actors/{TENANT}/quiz-roster/roster/events"), &[]).0, 403, "the handle index is not readable");
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
    let board = view(&base, &json!({ "type": "leaderboard" }));
    assert_eq!(board["rows"].as_array().expect("rows").iter().map(|row| (row["rank"].as_u64(), row["tag"].as_str(), row["total"].as_f64())).collect::<Vec<_>>(), [(Some(1), Some(quiz::learner_tag(&ada).as_str()), Some(100.0))]);
    assert!(!board.to_string().contains(ada.as_str()), "the public leaderboard never carries a learner id");
    let before = (learner, finished, board);
    running.shut_down();

    let running = boot(&data.0, development());
    let base = running.base.clone();
    let after = (view(&base, &json!({ "type": "learner", "learner": ada })), view(&base, &json!({ "type": "run", "run": run })), view(&base, &json!({ "type": "leaderboard" })));
    assert_eq!(after, before, "the reopened SQLite file answers the same views");
    let completed = play(&base, &ada, &id(0x200), "homes", 30);
    assert!(completed.iter().any(|event| event["badge"] == "complete"), "{completed:?}");
    let (_, _, stream) = get(&base, &format!("/actors/{TENANT}/quiz-learner/{ada}/events"), &[]);
    let sequences: Vec<u64> = serde_json::from_str::<Vec<Value>>(&stream).expect("records").iter().map(|record| record["seq"].as_u64().expect("seq")).collect();
    assert_eq!(sequences, (1..=sequences.len() as u64).collect::<Vec<_>>(), "the restarted authority appends after its history");
    let board = view(&base, &json!({ "type": "leaderboard" }));
    assert_eq!((board["rows"][0]["total"].as_f64(), board["rows"][0]["badges"].as_array().map(Vec::len)), (Some(200.0), Some(3)));
    running.shut_down();
}

#[test]
fn the_api_serves_the_cdn_site_across_origins_behind_the_gate() {
    const SITE: &str = "https://quizzes.example";
    const FOREIGN: &str = "https://evil.example";
    let data = scratch("gate-data");
    let running = boot(&data.0, Gate { origins: CrossOriginPolicy::Allowlist(vec![SITE.to_string()]), forwarding: Forwarding::TerminatingProxy });
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
    let (status, headers, _) = post_with(&base, "/queries", &query_envelope(&json!({ "type": "leaderboard" })), &[("origin", FOREIGN), ("x-forwarded-proto", "https")]);
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
    let running = boot(&data.0, Gate { origins: CrossOriginPolicy::Allowlist(vec![SITE.to_string()]), forwarding: Forwarding::TerminatingProxy });
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
    running.shut_down();

    let running = boot(&data.0, development());
    assert_eq!(crowd(&running.base), oracle, "the crowd survives a restart");
    running.shut_down();
}
