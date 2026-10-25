//! 🧰️ The harness of the proctor end to end suite: a scratch directory per test, the server booted on an ephemeral port, the wire with the §9a encoding, the websocket client of the presence rooms, and the builders of answers and runs.
//!
//! @see ../🦀️.rs — the end to end suite that mounts this module
//! @see ../../../🧫️fixtures/📚️catalog/🔣️.json — the course played

use super::*;
use semio_framework_test_fixtures::fixture_path;

pub(super) const TENANT: &str = "proctor-fixture";

static NEXT: AtomicU64 = AtomicU64::new(0);

pub(super) struct Scratch(pub(super) PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub(super) fn scratch(label: &str) -> Scratch {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let path = std::env::temp_dir().join(format!("teaching-proctor-e2e-{label}-{}-{nanos}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir_all(&path).expect("scratch directory");
    Scratch(path)
}

pub(super) fn fixtures() -> PathBuf {
    fixture_path!("../../🧪️tests/🧫️fixtures")
}

pub(super) fn id(seed: u32) -> String {
    format!("{seed:032x}")
}

//#region 🔖️Server
pub(super) struct Running {
    pub(super) base: String,
    pub(super) address: SocketAddr,
    stop: CancelToken,
    thread: std::thread::JoinHandle<()>,
}

pub(super) fn boot(data: &Path, gate: Gate) -> Running {
    let (ready, address) = std::sync::mpsc::channel();
    let stop = CancelToken::root_now();
    let token = stop.clone();
    let data = data.to_string_lossy().into_owned();
    let thread = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().expect("runtime");
        runtime.block_on(async move {
            let course = Arc::new(load_course(&fixtures().join("📚️catalog/🔣️.json")).expect("fixture course"));
            let proctor = Proctor::assemble(StorageProfile::Embedded { data_dir: data }, course, gate, PresenceSettings::default()).await.expect("assembled");
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
    pub(super) fn shut_down(self) {
        self.stop.cancel_now();
        self.thread.join().expect("the proctor stopped cleanly");
    }
}

pub(super) fn development() -> Gate {
    Gate { origins: CrossOriginPolicy::LoopbackDevelopment, forwarding: Forwarding::Untrusted, limits: Limits::default() }
}
//#endregion 🔖️Server

//#region 🔖️Wire
fn bytes(text: &str) -> Value {
    Value::Array(text.as_bytes().iter().map(|byte| json!(byte)).collect())
}

pub(super) fn text(bytes: &Value) -> String {
    String::from_utf8(bytes.as_array().expect("byte array").iter().map(|byte| byte.as_u64().expect("byte") as u8).collect()).expect("utf-8")
}

/// 🎯️ The actor a client addresses a command to: the handle of a pseudonym or name being registered —
/// the lowercase hex of its normalized key, as the browser client computes it — and the learner otherwise.
fn target(command: &Value) -> Value {
    let named = command["identity"]["handle"].as_str().filter(|_| command["type"] == "identify-learner").and_then(course::normalize_handle);
    match named {
        Some(handle) => json!({ "tenant": TENANT, "kind": "course-handle", "id": course::handle_actor_id(&handle.key) }),
        None => json!({ "tenant": TENANT, "kind": "course-learner", "id": command["learner"] }),
    }
}

pub(super) fn envelope(command: &Value) -> Value {
    let kind = command["type"].as_str().expect("type");
    let learner = command["learner"].as_str().expect("learner");
    let target = target(command);
    let principal = if kind == "identify-learner" { json!({ "kind": "anonymous" }) } else { json!({ "kind": "user", "id": learner }) };
    json!({
        "commandId": command["id"],
        "kind": format!("course.{kind}"),
        "version": course::WIRE_VERSION,
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

pub(super) fn post(base: &str, path: &str, body: &Value) -> (u16, Value) {
    let (status, _, answer) = post_with(base, path, body, &[]);
    (status, answer)
}

/// 📮️ `POST` a JSON body with extra request headers (never credentials); status, headers and JSON answer.
pub(super) fn post_with(base: &str, path: &str, body: &Value, headers: &[(&str, &str)]) -> (u16, Vec<(String, String)>, Value) {
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
pub(super) fn command(base: &str, command: &Value) -> (Value, Vec<Value>) {
    let (status, outcome) = post(base, "/commands", &envelope(command));
    assert_eq!(status, 200, "{outcome}");
    let events = outcome["events"].as_array().map(|events| events.iter().map(|record| serde_json::from_str(&text(&record["payload"])).expect("event json")).collect()).unwrap_or_default();
    (outcome, events)
}

pub(super) fn accepted(base: &str, sent: &Value) -> Vec<Value> {
    let (outcome, events) = command(base, sent);
    assert_eq!(outcome["status"], "accepted", "{outcome}");
    events
}

pub(super) fn rejected(base: &str, sent: &Value) -> String {
    let (outcome, _) = command(base, sent);
    assert_eq!(outcome["status"], "rejected", "{outcome}");
    assert_eq!(outcome["reason"]["kind"], "invalid", "{outcome}");
    outcome["reason"]["detail"].as_str().expect("detail").to_string()
}

pub(super) fn query_envelope(query: &Value) -> Value {
    let kind = query["type"].as_str().expect("type");
    json!({ "queryId": id(0), "kind": format!("course.{kind}"), "version": course::WIRE_VERSION, "scope": TENANT, "principal": { "kind": "anonymous" }, "arguments": bytes(&query.to_string()), "consistency": { "kind": "authority" }, "cursor": null })
}

pub(super) fn query(base: &str, query: &Value) -> (u16, Value) {
    let (status, result) = post(base, "/queries", &query_envelope(query));
    if status != 200 {
        return (status, result);
    }
    assert_eq!(result["kind"], "snapshot", "{result}");
    (status, serde_json::from_str(&text(&result["value"])).expect("view json"))
}

pub(super) fn view(base: &str, arguments: &Value) -> Value {
    let (status, view) = query(base, arguments);
    assert_eq!(status, 200, "{view}");
    view
}

pub(super) fn get(base: &str, path: &str, headers: &[(&str, &str)]) -> (u16, Vec<(String, String)>, String) {
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

pub(super) fn header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
}

/// 🧵️ One raw HTTP/1.1 request, for paths an URL library would normalize away.
pub(super) fn raw(address: SocketAddr, request: &str) -> String {
    let mut stream = TcpStream::connect(address).expect("connect");
    stream.set_read_timeout(Some(Duration::from_secs(30))).expect("timeout");
    stream.write_all(request.as_bytes()).expect("request");
    let mut answer = String::new();
    let _ = stream.read_to_string(&mut answer);
    answer
}
//#endregion 🔖️Wire

//#region 🔖️Presence
pub(super) type Socket = WebSocket<TcpStream>;

/// 🔌️ Open the presence socket of `scope` with extra handshake headers: the socket, or the refusing
/// status and its `x-semio-refusal`.
pub(super) fn presence(address: SocketAddr, scope: &str, surface: &str, headers: &[(&str, &str)]) -> Result<Socket, (u16, Option<String>)> {
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
pub(super) fn welcome(socket: &mut Socket) -> (String, Value) {
    let welcome = frame(socket);
    assert_eq!(welcome["type"], "welcome", "{welcome}");
    (welcome["session"].as_str().expect("a session").to_string(), welcome)
}

pub(super) fn share(socket: &mut Socket, state: &Value) {
    socket.send(Message::text(json!({ "type": "state", "state": state }).to_string())).expect("sent");
}

/// ⏳️ Read frames until one satisfies `found`; every frame read, that one last.
pub(super) fn until(socket: &mut Socket, found: impl Fn(&Value) -> bool) -> Vec<Value> {
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
pub(super) fn refusal(socket: &mut Socket) -> Value {
    until(socket, |frame| frame["type"] == "refused").pop().expect("a refused frame")["reason"].clone()
}

/// 📦️ Whether `frame` is a batch carrying `session` with `state`.
pub(super) fn carries(frame: &Value, session: &str, state: &Value) -> bool {
    frame["type"] == "batch" && frame["entries"].as_array().expect("entries").iter().any(|entry| entry["session"] == session && &entry["state"] == state)
}

/// 🚶️ Whether `frame` is a batch announcing that `session` left.
pub(super) fn departs(frame: &Value, session: &str) -> bool {
    frame["type"] == "batch" && frame["left"].as_array().expect("left").iter().any(|left| left == session)
}

/// 👀️ Watch `scopes` read-only at `interval_ms`.
pub(super) fn watch(socket: &mut Socket, scopes: &[&str], interval_ms: u64) {
    socket.send(Message::text(json!({ "type": "watch", "scopes": scopes, "intervalMs": interval_ms }).to_string())).expect("sent");
}

/// 👁️ Whether `frame` is a `watched` frame of `scope` carrying `session` with `state`.
pub(super) fn sees(frame: &Value, scope: &str, session: &str, state: &Value) -> bool {
    frame["type"] == "watched" && frame["scope"] == scope && frame["entries"].as_array().expect("entries").iter().any(|entry| entry["session"] == session && &entry["state"] == state)
}

/// 🔚️ Close with the handshake and drain until the server has hung up.
pub(super) fn close(mut socket: Socket) {
    socket.close(None).expect("close");
    while socket.read().is_ok() {}
}
//#endregion 🔖️Presence

//#region 🔖️Answers
pub(super) fn quiz_document(quiz: &str) -> Value {
    let file = match quiz {
        "power" => "⚡️power/🔣️.json",
        _ => "🏠️homes/🔣️.json",
    };
    serde_json::from_str(&std::fs::read_to_string(fixtures().join(file)).expect("quiz file")).expect("quiz json")
}

/// 💯️ The perfect answer to one presented task, read off the quiz file's solutions: the keys
/// assigned where the sheet shows them, the true values guessed where it hides them.
pub(super) fn perfect(quiz: &Value, presented: &Value) -> Value {
    let task = quiz["tasks"].as_array().expect("tasks").iter().find(|task| task["id"] == presented["id"]).expect("task");
    let solution = |item: &Value| task["items"].as_array().expect("items").iter().find(|candidate| candidate["id"] == item["id"]).expect("item").clone();
    let shown = presented["items"].as_array().expect("sheet items");
    let key = |item: &Value| item["id"].as_str().expect("id").to_string();
    match presented["kind"].as_str().expect("kind") {
        "sorting" => {
            let mut order: Vec<Value> = shown.iter().map(solution).collect();
            order.sort_by(|left, right| left["value"].as_f64().unwrap_or_default().total_cmp(&right["value"].as_f64().unwrap_or_default()));
            let mut answer = json!({ "kind": "sorting", "order": order.iter().map(|item| item["id"].clone()).collect::<Vec<_>>() });
            if presented.get("keys").is_none() {
                answer["guesses"] = Value::Object(order.iter().map(|item| (key(item), item["value"].clone())).collect());
            }
            answer
        }
        "classification" => json!({ "kind": "classification", "assignments": shown.iter().map(|item| (key(item), solution(item)["category"].clone())).collect::<serde_json::Map<_, _>>() }),
        _ => {
            let mut answer = json!({ "kind": "matching" });
            for dimension in presented["dimensions"].as_array().expect("dimensions") {
                let id = dimension["id"].as_str().expect("dimension id");
                let value = |item: &Value| solution(item)["values"][id].clone();
                let (member, given): (&str, serde_json::Map<String, Value>) = match dimension["cards"].as_array() {
                    Some(cards) => {
                        let mut used = Vec::new();
                        let assigned = shown.iter().map(|item| {
                            let card = (0..cards.len()).find(|card| cards[*card].as_f64() == value(item).as_f64() && !used.contains(card)).expect("a card of that value");
                            used.push(card);
                            (key(item), json!(card))
                        });
                        ("assignments", assigned.collect())
                    }
                    None => ("guesses", shown.iter().map(|item| (key(item), value(item))).collect()),
                };
                answer[member][id] = Value::Object(given);
            }
            answer
        }
    }
}

/// 🕰️ The wall clock in milliseconds: the clock the proctor in this process decides by.
pub(super) fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64)
}

/// ✍️ A `record-answer` the learner made just now.
pub(super) fn record(learner: &str, run: &str, task: &Value, answer: &Value, seed: u32) -> Value {
    json!({ "type": "record-answer", "id": id(seed), "learner": learner, "run": run, "task": task["id"], "answer": answer, "at": now() })
}

/// 🎬️ A `start-run` of `quiz` at `challenge` the learner made just now.
pub(super) fn start(learner: &str, run: &str, quiz: &str, challenge: &str, seed: u32) -> Value {
    json!({ "type": "start-run", "id": id(seed), "learner": learner, "run": run, "quiz": quiz, "challenge": challenge, "at": now() })
}

/// ⏱️ An `open-task` the device dates `at`.
pub(super) fn open_task(learner: &str, run: &str, task: &Value, at: u64, seed: u32) -> Value {
    json!({ "type": "open-task", "id": id(seed), "learner": learner, "run": run, "task": task["id"], "at": at })
}

/// 📤️ A `submit-run`.
pub(super) fn submit(learner: &str, run: &str, seed: u32) -> Value {
    json!({ "type": "submit-run", "id": id(seed), "learner": learner, "run": run })
}

/// 🏁️ Play a whole perfect run at `challenge` — on a timed run opening each task right before its
/// answer — and return the submission's events.
pub(super) fn play(base: &str, learner: &str, run: &str, quiz: &str, challenge: &str, seed: u32) -> Vec<Value> {
    accepted(base, &start(learner, run, quiz, challenge, seed));
    let sheet = view(base, &json!({ "type": "run", "run": run }))["sheet"].clone();
    let document = quiz_document(quiz);
    for (index, task) in sheet["tasks"].as_array().expect("tasks").iter().enumerate() {
        if task.get("seconds").is_some() {
            accepted(base, &open_task(learner, run, task, now(), seed + 30 + index as u32));
        }
        accepted(base, &record(learner, run, task, &perfect(&document, task), seed + 1 + index as u32));
    }
    accepted(base, &json!({ "type": "submit-run", "id": id(seed + 50), "learner": learner, "run": run }))
}
//#endregion 🔖️Answers
