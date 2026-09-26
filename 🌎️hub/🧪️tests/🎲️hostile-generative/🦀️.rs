//! 🎲️ The generative hostile-input law: `🧫️fixtures/🚧️hostile-input-v1` `generative` drives drawn mutations of every
//! route's own request and drawn frame sequences on every socket, from deterministic language-neutral seeds, within a
//! bounded time, and shrinks what it finds before reporting it with its seed.

use super::*;

/// 🧫️ The fixture this law and its language-neutral oracle (`🧪️tests/🚧️hostile-input/🟦️.ts`) read.
const HOSTILE_FIXTURE: &str = include_str!("../../🧫️fixtures/🚧️hostile-input-v1/🔣️.json");

/// 🎲️ Deterministic draws: the k-th little-endian u64 of the blocks SHA-256(`semio.hub.hostile-draws/v1` ‖ le64(seed) ‖
/// le64(block)).
struct HostileDrawsV1 {
    seed: u64,
    block: u64,
    buffered: Vec<u64>,
}

impl HostileDrawsV1 {
    fn new(seed: u64) -> Self {
        Self { seed, block: 0, buffered: Vec::new() }
    }

    fn next(&mut self) -> u64 {
        if self.buffered.is_empty() {
            let mut input = b"semio.hub.hostile-draws/v1".to_vec();
            input.extend_from_slice(&self.seed.to_le_bytes());
            input.extend_from_slice(&self.block.to_le_bytes());
            let digest = Sha256::digest(&input);
            self.buffered = digest.chunks_exact(8).rev().map(|chunk| u64::from_le_bytes(chunk.try_into().expect("eight bytes"))).collect();
            self.block += 1;
        }
        self.buffered.pop().expect("a refilled block")
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound.max(1) as u64) as usize
    }

    fn chance(&mut self, numerator: usize, denominator: usize) -> bool {
        self.below(denominator) < numerator
    }

    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }

    fn text(&mut self, maximum: usize) -> String {
        const ALPHABET: [&str; 16] = ["a", "Z", "0", "-", "_", ".", "/", "%", "\"", "\\", " ", "é", "😀", "\u{0}", "\u{202e}", "{"];
        (0..self.below(maximum + 1)).map(|_| *self.pick(&ALPHABET)).collect()
    }

    fn json(&mut self, depth: usize, names: &[String]) -> serde_json::Value {
        match self.below(if depth == 0 { 5 } else { 8 }) {
            0 => serde_json::Value::Null,
            1 => serde_json::Value::Bool(self.chance(1, 2)),
            2 => serde_json::json!(*self.pick(&[0i64, -1, 1, i64::MAX, i64::MIN, 9_007_199_254_740_993])),
            3 => serde_json::json!(*self.pick(&[0.5f64, -1e308, 1e-308, 3.141_592_653_589_793])),
            4 => serde_json::Value::String(self.text(48)),
            5 | 6 => {
                let fields = self.below(6);
                let mut object = serde_json::Map::new();
                for _ in 0..fields {
                    let name = if !names.is_empty() && self.chance(3, 4) { self.pick(names).clone() } else { self.text(12) };
                    object.insert(name, self.json(depth - 1, names));
                }
                serde_json::Value::Object(object)
            }
            _ => serde_json::Value::Array((0..self.below(5)).map(|_| self.json(depth - 1, names)).collect()),
        }
    }
}

/// 🧬️ One drawn change to a route's own request.
#[derive(Clone, Debug)]
enum HostileMutationV1 {
    Method(String),
    Credential(String),
    ContentType(String),
    Body(Vec<u8>),
    Segment { index: usize, value: String },
    Query(String),
    Header(String, String),
}

/// 🚨️ What one answer violated, by class, so a shrunk case is kept only while it still violates the same class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HostileFindingV1 {
    Dropped,
    ServerFailure,
    UntypedRefusal,
    CredentialBypassed,
    HubUnhealthy,
}

/// 🛣️ One fixture route with everything a case needs.
struct HostileRouteV1 {
    method: String,
    path: String,
    public: Option<String>,
    body_kind: Option<String>,
    names: Vec<String>,
}

/// 📚️ The property names a route's declared body schema mentions (its definition and every `$defs` entry it
/// references, one level deep), so drawn bodies reach past the first field check.
fn declared_body_names(route: &serde_json::Value) -> Vec<String> {
    let (Some(module), Some(def)) = (route["body"]["schema"]["module"].as_str(), route["body"]["schema"]["def"].as_str()) else { return Vec::new() };
    let Ok(text) = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(module)) else { return Vec::new() };
    let Ok(document) = serde_json::from_str::<serde_json::Value>(&text) else { return Vec::new() };
    let mut names = std::collections::BTreeSet::new();
    let mut definitions = vec![document["$defs"][def].clone()];
    for reference in document["$defs"][def]["properties"].as_object().into_iter().flatten().filter_map(|(_, property)| property["$ref"].as_str()) {
        definitions.push(document["$defs"][reference.trim_start_matches("#/$defs/")].clone());
    }
    for definition in definitions {
        names.extend(definition["properties"].as_object().into_iter().flatten().map(|(name, _)| name.clone()));
    }
    names.into_iter().collect()
}

/// 🎲️ Draws one case for `route`: one to four mutations.
fn draw_case(draws: &mut HostileDrawsV1, route: &HostileRouteV1, generative: &serde_json::Value) -> Vec<HostileMutationV1> {
    let strings = |key: &str| generative[key].as_array().expect("generative strings").iter().map(|value| value.as_str().expect("string").to_string()).collect::<Vec<_>>();
    let segments = route.path.split('?').next().unwrap_or_default().split('/').filter(|segment| !segment.is_empty()).count();
    (0..1 + draws.below(4))
        .map(|_| match draws.below(7) {
            0 => HostileMutationV1::Method(draws.pick(&strings("methods")).clone()),
            1 => HostileMutationV1::Credential(draws.pick(&strings("credentials")).clone()),
            2 => HostileMutationV1::ContentType(draws.pick(&strings("contentTypes")).clone()),
            3 => {
                let json = draws.json(4, &route.names);
                let mut body = serde_json::to_vec(&json).expect("drawn JSON");
                if draws.chance(1, 4) {
                    let cut = draws.below(body.len() + 1);
                    body.truncate(cut);
                }
                if draws.chance(1, 8) {
                    body = (0..draws.below(512)).map(|_| draws.next() as u8).collect();
                }
                HostileMutationV1::Body(body)
            }
            4 => HostileMutationV1::Segment { index: draws.below(segments.max(1)), value: if draws.chance(1, 3) { draws.text(24).bytes().map(|byte| format!("%{byte:02X}")).collect() } else { draws.pick(&strings("segments")).clone() } },
            5 => HostileMutationV1::Query(if draws.chance(1, 3) { draws.text(64).bytes().map(|byte| format!("%{byte:02X}")).collect() } else { draws.pick(&strings("queries")).clone() }),
            _ => {
                let headers = generative["headers"].as_array().expect("generative headers");
                let header = draws.pick(headers);
                HostileMutationV1::Header(header[0].as_str().expect("header name").into(), header[1].as_str().expect("header value").into())
            }
        })
        .collect()
}

/// 🌐️ The request one case sends: the route's own request, a live bearer, then every mutation in order.
fn case_request(route: &HostileRouteV1, mutations: &[HostileMutationV1], bearer: &str, forged: &str) -> (String, String, Vec<(String, String)>, Vec<u8>, bool) {
    let mut method = route.method.clone();
    let (mut path, mut query) = match route.path.split_once('?') {
        Some((path, query)) => (path.to_string(), Some(query.to_string())),
        None => (route.path.clone(), None),
    };
    let mut credential = vec![("authorization".to_string(), bearer.to_string())];
    let mut credentialed = true;
    let mut content_type = matches!(route.body_kind.as_deref(), Some("json")).then(|| "application/json".to_string());
    let mut body = matches!(route.body_kind.as_deref(), Some("json")).then(|| b"{}".to_vec()).unwrap_or_default();
    let mut extra = Vec::new();
    for mutation in mutations {
        match mutation {
            HostileMutationV1::Method(drawn) => method = drawn.clone(),
            HostileMutationV1::Credential(kind) => {
                credentialed = kind == "valid";
                credential = match kind.as_str() {
                    "valid" => vec![("authorization".into(), bearer.into())],
                    "absent" => Vec::new(),
                    "forged" => vec![("authorization".into(), forged.into())],
                    "empty-bearer" => vec![("authorization".into(), "Bearer ".into())],
                    "basic" => vec![("authorization".into(), "Basic dXNlcjpwYXNz".into())],
                    "lowercase-scheme" => vec![("authorization".into(), bearer.replacen("Bearer", "bearer", 1))],
                    "oversized" => vec![("authorization".into(), format!("Bearer {}", "A".repeat(16 * 1024)))],
                    _ => vec![("authorization".into(), forged.into()), ("authorization".into(), bearer.into())],
                };
            }
            HostileMutationV1::ContentType(drawn) => content_type = Some(drawn.clone()),
            HostileMutationV1::Body(drawn) => body = drawn.clone(),
            HostileMutationV1::Segment { index, value } => {
                let mut segments: Vec<String> = path.split('/').filter(|segment| !segment.is_empty()).map(str::to_string).collect();
                if let Some(segment) = segments.get_mut(*index) {
                    *segment = value.clone();
                }
                path = format!("/{}", segments.join("/"));
            }
            HostileMutationV1::Query(drawn) => query = Some(drawn.clone()),
            HostileMutationV1::Header(name, value) => extra.push((name.clone(), value.clone())),
        }
    }
    let target = match query {
        Some(query) => format!("{path}?{query}"),
        None => path,
    };
    let mut headers = credential;
    headers.extend(content_type.map(|value| ("content-type".to_string(), value)));
    headers.push(("origin".into(), "http://127.0.0.1:6066".into()));
    headers.extend(extra);
    (method, target, headers.into_iter().filter(|(_, value)| !value.contains('\u{0}') && !value.contains('\r') && !value.contains('\n')).collect(), body, credentialed)
}

/// ⚖️ Judges one answer with the fixed oracle.
fn judge_generated(answer: &Result<RawHttpResponse, String>, route: &HostileRouteV1, credentialed: bool, maximum: u16, unavailable: u16) -> Option<HostileFindingV1> {
    let response = match answer {
        Ok(response) => response,
        Err(_) => return Some(HostileFindingV1::Dropped),
    };
    if response.status < 400 {
        return (!credentialed && route.public.is_none()).then_some(HostileFindingV1::CredentialBypassed);
    }
    if response.status > maximum && response.status != unavailable {
        return Some(HostileFindingV1::ServerFailure);
    }
    let head = response.headers.to_ascii_lowercase();
    if !head.lines().any(|line| line.starts_with(&format!("{}: ", semio_hub::refusal::HUB_REFUSAL_HEADER))) {
        return Some(HostileFindingV1::UntypedRefusal);
    }
    None
}

/// 🩺️ Whether the hub still answers its liveness probe.
async fn hub_is_live(addr: SocketAddr) -> bool {
    hostile_http_request(addr, "GET", "/healthz", &[], &[]).await.is_ok_and(|response| response.status == 200)
}

/// 🧪️ The member session every generated request carries unless a mutation replaces it: a fresh one per request,
/// so a drawn sign-out or credential change never leaks into the next case.
const HOSTILE_GENERATIVE_MEMBER: &str = "hostile-generative-member@example.com";

/// 🧪️ Sends one case under a fresh member session and judges it, liveness included.
async fn run_generated_case(state: &HubState, addr: SocketAddr, route: &HostileRouteV1, mutations: &[HostileMutationV1], forged: &str, maximum: u16, unavailable: u16) -> Option<HostileFindingV1> {
    let bearer = format!("Bearer {}", issue_test_session(state, HOSTILE_GENERATIVE_MEMBER).await.token);
    let (method, target, headers, body, credentialed) = case_request(route, mutations, &bearer, forged);
    let headers: Vec<(&str, &str)> = headers.iter().map(|(name, value)| (name.as_str(), value.as_str())).collect();
    let answer = hostile_http_request(addr, &method, &target, &headers, &body).await;
    let finding = judge_generated(&answer, route, credentialed, maximum, unavailable);
    if finding.is_none() && !hub_is_live(addr).await {
        return Some(HostileFindingV1::HubUnhealthy);
    }
    finding
}

/// ✂️ Shrinks a failing case: drops mutations while the same finding persists, then halves bodies and strings.
async fn shrink_case(state: &HubState, addr: SocketAddr, route: &HostileRouteV1, mut mutations: Vec<HostileMutationV1>, finding: HostileFindingV1, forged: &str, maximum: u16, unavailable: u16, mut steps: usize) -> Vec<HostileMutationV1> {
    let mut index = mutations.len();
    while index > 0 && steps > 0 {
        index -= 1;
        let mut candidate = mutations.clone();
        candidate.remove(index);
        steps -= 1;
        if run_generated_case(state, addr, route, &candidate, forged, maximum, unavailable).await == Some(finding) {
            mutations = candidate;
        }
    }
    let mut changed = true;
    while changed && steps > 0 {
        changed = false;
        for index in 0..mutations.len() {
            let halved = match &mutations[index] {
                HostileMutationV1::Body(body) if body.len() > 1 => Some(HostileMutationV1::Body(body[..body.len() / 2].to_vec())),
                HostileMutationV1::Query(query) if query.len() > 1 => Some(HostileMutationV1::Query(query.chars().take(query.chars().count() / 2).collect())),
                HostileMutationV1::Segment { index: at, value } if value.len() > 1 => Some(HostileMutationV1::Segment { index: *at, value: value.chars().take(value.chars().count() / 2).collect() }),
                _ => None,
            };
            let Some(halved) = halved else { continue };
            if steps == 0 {
                break;
            }
            steps -= 1;
            let mut candidate = mutations.clone();
            candidate[index] = halved;
            if run_generated_case(state, addr, route, &candidate, forged, maximum, unavailable).await == Some(finding) {
                mutations = candidate;
                changed = true;
            }
        }
    }
    mutations
}

/// 🔌️ One drawn frame for an open socket.
fn draw_frame(draws: &mut HostileDrawsV1, valid: &[u8]) -> WsMessage {
    match draws.below(8) {
        0 => WsMessage::Binary((0..draws.below(512)).map(|_| draws.next() as u8).collect::<Vec<u8>>().into()),
        1 => {
            let mut flipped = valid.to_vec();
            for _ in 0..1 + draws.below(4) {
                if !flipped.is_empty() {
                    let at = draws.below(flipped.len());
                    flipped[at] ^= 1 << draws.below(8);
                }
            }
            WsMessage::Binary(flipped.into())
        }
        2 => WsMessage::Binary(valid[..draws.below(valid.len() + 1)].to_vec().into()),
        3 => WsMessage::Text(serde_json::to_string(&draws.json(3, &[])).expect("drawn JSON").into()),
        4 => WsMessage::Binary(Vec::new().into()),
        5 => WsMessage::Ping((0..draws.below(125)).map(|_| draws.next() as u8).collect::<Vec<u8>>().into()),
        6 => WsMessage::Binary(vec![0xA5; 1024 * 1024 + draws.below(4096)].into()),
        _ => WsMessage::Binary(valid.to_vec().into()),
    }
}

/// 🔌️ Reads a socket after a drawn sequence: it must end with a close frame (a declared code) or stay open; a bare
/// disconnect is a finding.
async fn socket_outcome<S>(socket: &mut S) -> Result<Option<u16>, String>
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_millis(1_500);
    loop {
        match tokio::time::timeout_at(deadline, socket.next()).await {
            Err(_) => return Ok(None),
            Ok(Some(Ok(WsMessage::Close(Some(frame))))) => {
                let code: u16 = frame.code.into();
                return if (1000..=1014).contains(&code) && code != 1005 && code != 1006 || (4000..=4999).contains(&code) { Ok(Some(code)) } else { Err(format!("close code {code}")) };
            }
            Ok(Some(Ok(WsMessage::Close(None)))) => return Ok(Some(1005)),
            Ok(Some(Ok(_))) => continue,
            Ok(Some(Err(error))) => return Err(format!("transport error {error}")),
            Ok(None) => return Err("disconnected without a close frame".into()),
        }
    }
}

mod long {
    use super::*;

    /// 🎲️ The draws are the fixture's language-neutral vectors.
    #[test]
    fn hostile_draws_reproduce_the_language_neutral_vectors() {
        let fixture: serde_json::Value = serde_json::from_str(HOSTILE_FIXTURE).unwrap();
        for vector in fixture["generative"]["drawVectors"].as_array().unwrap() {
            let mut draws = HostileDrawsV1::new(vector["seed"].as_u64().unwrap());
            let drawn: Vec<String> = (0..vector["draws"].as_array().unwrap().len()).map(|_| format!("{:016x}", draws.next())).collect();
            assert_eq!(serde_json::json!(drawn), vector["draws"]);
        }
    }

    /// 🎲️ LAW: every route survives drawn hostile requests — every seed's cases for every route in the fixture,
    /// within the declared budget — and every socket survives drawn frame sequences. A finding is shrunk and
    /// reported with its seed, route and minimal mutations.
    #[test]
    fn every_route_and_socket_survives_generated_hostile_input() {
        run_socket_test(|| async {
            let fixture: serde_json::Value = serde_json::from_str(HOSTILE_FIXTURE).unwrap();
            let generative = &fixture["generative"];
            let seeds: Vec<u64> = match std::env::var("SEMIO_HUB_HOSTILE_SEED").ok().and_then(|seed| seed.parse().ok()) {
                Some(seed) => vec![seed],
                None => generative["seeds"].as_array().unwrap().iter().map(|seed| seed.as_u64().unwrap()).collect(),
            };
            let cases = generative["casesPerRoute"].as_u64().unwrap() as usize;
            let budget = std::time::Duration::from_millis(generative["budgetMs"].as_u64().unwrap());
            let shrink_steps = generative["maxShrinkSteps"].as_u64().unwrap() as usize;
            let maximum = fixture["allowedStatuses"]["max"].as_u64().unwrap() as u16;
            let unavailable = fixture["allowedStatuses"]["typedUnavailable"].as_u64().unwrap() as u16;
            let forged = fixture["bodies"]["forgedBearer"].as_str().unwrap().to_string();
            let state = test_state().await;
            let owner = issue_test_session(&state, "hostile-generative-owner@example.com").await;
            issue_test_session(&state, HOSTILE_GENERATIVE_MEMBER).await;
            let space_id = create_space_for_test(&state, &owner.user_id, "Hostile generative", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            upsert_member_for_test(&state, &space_id, HOSTILE_GENERATIVE_MEMBER, DirectorySpaceRole::Author).await;
            announce_document_for_test(&state, &space_id, "hostile-doc").await;
            let addr = spawn_server(state.clone()).await;
            let routes: Vec<HostileRouteV1> = fixture["routes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|route| HostileRouteV1 {
                    method: route["method"].as_str().unwrap().into(),
                    path: route["path"].as_str().unwrap().replace("space-h", &space_id).replace("doc-h", "hostile-doc"),
                    public: route["public"].as_str().map(str::to_string),
                    body_kind: route["body"]["kind"].as_str().map(str::to_string),
                    names: declared_body_names(route),
                })
                .collect();
            let started = std::time::Instant::now();
            let (mut sent, mut findings) = (0usize, Vec::new());
            'seeds: for seed in &seeds {
                let mut draws = HostileDrawsV1::new(*seed);
                for case in 0..cases {
                    for route in &routes {
                        if started.elapsed() > budget {
                            break 'seeds;
                        }
                        let mutations = draw_case(&mut draws, route, generative);
                        sent += 1;
                        if let Some(finding) = run_generated_case(&state, addr, route, &mutations, &forged, maximum, unavailable).await {
                            let minimal = shrink_case(&state, addr, route, mutations.clone(), finding, &forged, maximum, unavailable, shrink_steps).await;
                            findings.push(format!("seed {seed} case {case} {} {}: {finding:?} minimal {minimal:?}", route.method, route.path));
                        }
                    }
                }
            }
            let socket_cases = generative["socketCases"].as_u64().unwrap() as usize;
            let frames = generative["framesPerSocketCase"].as_u64().unwrap() as usize;
            let hello = protocol::encode_client_frame(&socket_hello(), Lane::Command).await;
            let socket_space = create_space_for_test(&state, &owner.user_id, "Hostile generative sockets", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            announce_document_for_test(&state, &socket_space, "hostile-socket-doc").await;
            let mut sequences = 0usize;
            'sockets: for seed in &seeds {
                let mut draws = HostileDrawsV1::new(seed.wrapping_add(1 << 32));
                for case in 0..socket_cases {
                    for socket_route in ["directory", "document"] {
                        if started.elapsed() > budget {
                            break 'sockets;
                        }
                        let connected = match socket_route {
                            "directory" => {
                                let receipt = issue_directory_socket_grant(bearer_headers(&owner.token), State(state.clone()), Bytes::new()).await.expect("directory socket grant").0;
                                let since = state.directory.head_seq().await.expect("directory head");
                                connect_async(socket_request(&format!("ws://{addr}/directory/socket/v1?since={since}"), &receipt.grant)).await
                            }
                            _ => {
                                issue_document_socket_grant_fixture(Path((socket_space.clone(), "hostile-socket-doc".to_string())), bearer_headers(&owner.token), State(state.clone())).await.expect("document socket grant");
                                connect_async(document_socket_request(&format!("ws://{addr}/scopes/{socket_space}%2Fhostile-socket-doc/document/ws"), &owner.token)).await
                            }
                        };
                        let (mut socket, _) = connected.unwrap_or_else(|error| panic!("{socket_route} socket connects before seed {seed} case {case}: {error}"));
                        let drawn: Vec<WsMessage> = (0..frames).map(|_| draw_frame(&mut draws, &hello)).collect();
                        let mut sending = Ok(());
                        for frame in drawn.iter().cloned() {
                            sending = socket.send(frame).await;
                            if sending.is_err() {
                                break;
                            }
                        }
                        sequences += 1;
                        let outcome = socket_outcome(&mut socket).await;
                        if let Err(finding) = outcome.as_ref() {
                            let kinds: Vec<String> = drawn.iter().map(|frame| format!("{}:{}", match frame { WsMessage::Binary(_) => "binary", WsMessage::Text(_) => "text", WsMessage::Ping(_) => "ping", _ => "other" }, frame.len())).collect();
                            findings.push(format!("seed {seed} socket case {case} {socket_route}: {finding} after {kinds:?} (send {sending:?})"));
                        }
                        if !hub_is_live(addr).await {
                            findings.push(format!("seed {seed} socket case {case} {socket_route}: the hub stopped answering /healthz"));
                            break 'sockets;
                        }
                    }
                }
            }
            eprintln!("hostile-generative: {sent} requests, {sequences} socket sequences, {} findings, {:?}", findings.len(), started.elapsed());
            assert!(findings.is_empty(), "{} generated hostile inputs broke the oracle:\n{}", findings.len(), findings.join("\n"));
            assert!(sent >= routes.len() * seeds.len().min(1), "every route received at least one drawn case within the budget: {sent}");
            assert!(sequences >= 2, "both sockets received at least one drawn sequence within the budget: {sequences}");
        });
    }
}
