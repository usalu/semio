//! 🌐️ The proctor end to end: booted on an ephemeral port over a fresh data directory and the
//! fixture catalog, driven through its real HTTP API with the §9a encoding by an independent HTTP
//! client (`ureq`), exactly as the browser client talks to it — then stopped, reopened from the same
//! SQLite file and asked again.
//!
//! @see ../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️design.md — §8, §9, §9a
//! @see ../../🧫️fixtures/📚️catalog/🔣️.json — the catalog played

use std::io::{Read, Write};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use proctor::catalog::load_catalog;
use proctor::config::{CrossOriginPolicy, Forwarding, Gate};
use proctor::instance::Proctor;
use proctor::site::SiteHost;
use semio_framework_async::CancelToken;
use serde_json::{json, Value};
use server::storage::StorageProfile;

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

fn boot(data: &Path, site: Option<&Path>, gate: Gate) -> Running {
    let (ready, address) = std::sync::mpsc::channel();
    let stop = CancelToken::root_now();
    let token = stop.clone();
    let data = data.to_string_lossy().into_owned();
    let site = site.map(Path::to_path_buf);
    let thread = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().expect("runtime");
        runtime.block_on(async move {
            let catalog = Arc::new(load_catalog(&fixtures().join("📚️catalog/🔣️.json")).expect("fixture catalog"));
            let site = site.map(|site| SiteHost::open(&site).expect("site"));
            let proctor = Proctor::assemble(StorageProfile::Embedded { data_dir: data }, catalog, gate, site).await.expect("assembled");
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
    match ureq::post(&format!("{base}{path}")).set("content-type", "application/json").send_string(&body.to_string()) {
        Ok(response) => (response.status(), serde_json::from_str(&response.into_string().expect("body")).expect("json")),
        Err(ureq::Error::Status(status, response)) => (status, serde_json::from_str(&response.into_string().unwrap_or_default()).unwrap_or(Value::Null)),
        Err(error) => panic!("POST {path}: {error}"),
    }
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

fn query(base: &str, query: &Value) -> (u16, Value) {
    let kind = query["type"].as_str().expect("type");
    let envelope = json!({ "queryId": id(0), "kind": format!("quiz.{kind}"), "version": 1, "scope": TENANT, "principal": { "kind": "anonymous" }, "arguments": bytes(&query.to_string()), "consistency": { "kind": "authority" }, "cursor": null });
    let (status, result) = post(base, "/queries", &envelope);
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
    let mut stream = std::net::TcpStream::connect(address).expect("connect");
    stream.set_read_timeout(Some(Duration::from_secs(30))).expect("timeout");
    stream.write_all(request.as_bytes()).expect("request");
    let mut answer = String::new();
    let _ = stream.read_to_string(&mut answer);
    answer
}
//#endregion 🔖️Wire

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
    let running = boot(&data.0, None, development());
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

    let running = boot(&data.0, None, development());
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
fn the_site_is_served_behind_the_gate_with_spa_fallback_and_cache_headers() {
    let data = scratch("site-data");
    let site = scratch("site");
    std::fs::create_dir_all(site.0.join("assets")).expect("assets");
    std::fs::write(site.0.join("index.html"), "<!doctype html><title>quizze</title>").expect("index");
    std::fs::write(site.0.join("assets/app-B3xK9aQz.js"), "console.log(1);").expect("asset");
    std::fs::write(site.0.join("assets/🌐️-Djvsi-pa.js"), "console.log(2);").expect("dashed asset");
    std::fs::create_dir_all(site.0.join("🖼️assets")).expect("public assets");
    std::fs::write(site.0.join("🖼️assets/compressed.woff2"), "wOF2").expect("font");
    let running = boot(&data.0, Some(&site.0), Gate { origins: CrossOriginPolicy::LoopbackDevelopment, forwarding: Forwarding::TerminatingProxy });
    let base = running.base.clone();
    let https = [("x-forwarded-proto", "https")];

    let (status, headers, _) = get(&base, "/", &[]);
    assert_eq!((status, header(&headers, "x-semio-refusal")), (403, Some("insecure-transport")));
    let (status, headers, body) = get(&base, "/", &https);
    assert_eq!((status, header(&headers, "content-type"), header(&headers, "cache-control")), (200, Some("text/html; charset=utf-8"), Some("no-cache")));
    assert!(body.contains("<title>quizze</title>"));
    let (status, headers, body) = get(&base, "/quiz/power/run/123", &https);
    assert_eq!((status, header(&headers, "cache-control")), (200, Some("no-cache")));
    assert!(body.contains("quizze"));
    let (status, headers, body) = get(&base, "/assets/app-B3xK9aQz.js", &https);
    assert_eq!((status, header(&headers, "content-type"), header(&headers, "cache-control"), body.as_str()), (200, Some("text/javascript; charset=utf-8"), Some("public, max-age=31536000, immutable"), "console.log(1);"));
    assert_eq!(get(&base, "/assets/gone-B3xK9aQz.js", &https).0, 404);
    let secure = |method: &str, path: &str| raw(running.address, &format!("{method} {path} HTTP/1.1\r\nHost: proctor\r\nX-Forwarded-Proto: https\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")).to_ascii_lowercase();
    let dashed = secure("GET", "/assets/%F0%9F%8C%90%EF%B8%8F-Djvsi-pa.js");
    assert!(dashed.starts_with("http/1.1 200") && dashed.contains("cache-control: public, max-age=31536000, immutable") && dashed.ends_with("console.log(2);"), "{dashed}");
    let font = secure("HEAD", "/%F0%9F%96%BC%EF%B8%8Fassets/compressed.woff2");
    assert!(font.starts_with("http/1.1 200") && font.contains("cache-control: public, max-age=3600") && font.contains("content-type: font/woff2"), "{font}");
    for method in ["POST", "PUT", "DELETE"] {
        let refused = secure(method, "/quiz/power");
        assert!(refused.starts_with("http/1.1 405") && refused.contains("allow: get, head") && refused.contains("\"kind\":\"methodnotallowed\""), "{method}: {refused}");
    }
    let (status, headers, _) = get(&base, "/instance", &https);
    assert_eq!((status, header(&headers, "content-type")), (200, Some("application/json")));
    for traversal in ["/%2e%2e/%2e%2e/Cargo.toml", "/..%2f..%2fCargo.toml", "/assets/..%5c..%5cCargo.toml"] {
        let answer = raw(running.address, &format!("GET {traversal} HTTP/1.1\r\nHost: proctor\r\nX-Forwarded-Proto: https\r\nConnection: close\r\n\r\n"));
        assert!(answer.starts_with("HTTP/1.1 400"), "{traversal}: {answer}");
    }
    let preflight = raw(running.address, "OPTIONS /commands HTTP/1.1\r\nHost: proctor\r\nX-Forwarded-Proto: https\r\nOrigin: http://localhost:6061\r\nAccess-Control-Request-Method: POST\r\nConnection: close\r\n\r\n").to_ascii_lowercase();
    assert!(preflight.starts_with("http/1.1 204") && preflight.contains("access-control-allow-origin: http://localhost:6061"), "{preflight}");
    let foreign = raw(running.address, "OPTIONS /commands HTTP/1.1\r\nHost: proctor\r\nX-Forwarded-Proto: https\r\nOrigin: https://evil.example\r\nConnection: close\r\n\r\n").to_ascii_lowercase();
    assert!(foreign.starts_with("http/1.1 204") && !foreign.contains("access-control-allow-origin"), "{foreign}");
    running.shut_down();
}
