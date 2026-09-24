use super::*;

fn session(profile: Profile) -> Arc<Session> {
    let server = Arc::new(repository_server(RecordingRepository::new(), profile, Limits::default()).expect("server"));
    server.connect("test", None).expect("session")
}

fn ready(profile: Profile) -> Arc<Session> {
    let session = session(profile);
    let _ = session.dispatch(br#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}"#).expect("initialize");
    let _ = session.dispatch(br#"{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}"#).expect("initialized");
    session
}

#[test]
fn sha256_matches_the_published_vectors() {
    assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"), "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
}

#[test]
fn initialize_ignores_unknown_members() {
    let session = session(Profile::Claude);
    let response = session
        .dispatch(br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{"roots":{"listChanged":true},"elicitation":{},"tasks":{"requests":{}}},"clientInfo":{"name":"claude-code","title":"Claude Code","version":"2.0.0"}}}"#)
        .expect("dispatch")
        .expect("response");
    assert!(response.contains(r#""protocolVersion":"2025-06-18""#), "{response}");
    assert!(response.contains(r#""name":"repo""#), "{response}");
    assert!(!response.contains("error"), "{response}");
}

#[test]
fn tool_arguments_stay_closed() {
    assert!(decode_arguments(&serde_json::json!({"emoji": "x"}), &["emoji"]).is_ok());
    assert!(decode_arguments(&serde_json::json!({"emoji": "x", "surprise": 1}), &["emoji"]).is_err());
}

#[test]
fn every_profile_advertises_its_own_surface() {
    for (profile, name) in [(Profile::Generic, "repo"), (Profile::Cursor, "repo"), (Profile::Kiro, "repo"), (Profile::Copilot, "repo"), (Profile::Claude, "repo"), (Profile::Codex, "repo")] {
        assert_eq!(profile.server_name(), name);
        let open = &tool_schemas(profile).into_iter().find(|tool| tool.name == "ticket_open").expect("ticket_open").input_schema;
        assert_eq!(open.properties.contains_key("plan_id"), matches!(profile, Profile::Cursor | Profile::Copilot | Profile::Claude | Profile::Codex));
        assert_eq!(open.properties.contains_key("spec_id"), profile == Profile::Kiro);
    }
}

#[test]
fn listings_are_sorted_and_complete() {
    let session = ready(Profile::Generic);
    for (method, field, expected) in [("tools/list", "tools", TOOL_NAMES.len()), ("prompts/list", "prompts", PROMPT_NAMES.len()), ("resources/list", "resources", RESOURCE_URIS.len())] {
        let response = session.dispatch(format!(r#"{{"jsonrpc":"2.0","id":"{method}","method":"{method}","params":{{}}}}"#).as_bytes()).expect("dispatch").expect("response");
        let parsed: serde_json::Value = serde_json::from_str(&response).expect("json");
        assert_eq!(parsed["result"][field].as_array().expect("array").len(), expected, "{method}");
    }
}

#[test]
fn unknown_envelope_members_are_rejected() {
    let session = ready(Profile::Generic);
    let response = session.dispatch(br#"{"jsonrpc":"2.0","id":4,"method":"ping","unexpected":true}"#).expect("dispatch").expect("response");
    assert_eq!(response, r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32600,"message":"invalid request"}}"#);
}

#[test]
fn the_event_chain_is_verifiable() {
    let session = ready(Profile::Generic);
    let _ = session.dispatch(br#"{"jsonrpc":"2.0","id":9,"method":"ping","params":{}}"#).expect("dispatch");
    let snapshot = session.server.events().snapshot();
    let replayed = replay_events(&snapshot, 0, 0).expect("replay");
    assert!(replayed.len() >= 4);
    assert_eq!(replayed[0].kind, "session.opened");
    assert!(replayed[0].previous.is_empty());
    for pair in replayed.windows(2) {
        assert_eq!(pair[1].previous, pair[0].hash);
        assert_eq!(pair[1].sequence, pair[0].sequence + 1);
    }
}

#[test]
fn a_broken_chain_is_refused() {
    let session = ready(Profile::Generic);
    let snapshot = session.server.events().snapshot();
    let tampered = snapshot.replacen("session.opened", "session.tampered", 1);
    assert!(replay_events(&tampered, 0, 0).is_err());
}

#[test]
fn the_g2_golden_vectors_match_byte_for_byte() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/2️⃣g2-contract.json")).expect("fixture");
    assert_eq!(fixture["schema"], "semio.mcp.contract/1");
    assert_eq!(fixture["protocolVersion"], PROTOCOL_VERSION);
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let name = vector["name"].as_str().expect("name");
        let server = Arc::new(contract_server(Limits::default()).expect("server"));
        let session = server.connect("fixture", None).expect("session");
        if vector["ready"].as_bool().unwrap_or(false) {
            let request = format!(r#"{{"jsonrpc":"2.0","id":"init","method":"initialize","params":{{"protocolVersion":"{PROTOCOL_VERSION}","capabilities":{{}},"clientInfo":{{"name":"test","version":"1"}}}}}}"#);
            let _ = session.dispatch(request.as_bytes()).expect("initialize");
            let _ = session.dispatch(br#"{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}"#).expect("initialized");
        }
        let response = session.dispatch(vector["request"].as_str().expect("request").as_bytes()).expect("dispatch").unwrap_or_default();
        assert_eq!(response, vector["response"].as_str().expect("response"), "vector {name}");
    }
}

#[test]
fn the_entrypoint_contract_holds() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️entrypoint-contract.json")).expect("fixture");
    assert_eq!(fixture["schema"], "semio.repo.mcp.entrypoint/1");
    assert_eq!(fixture["profileEnvironment"], PROFILE_ENVIRONMENT);
    let versions: Vec<&str> = fixture["protocolVersions"].as_array().expect("versions").iter().map(|value| value.as_str().expect("version")).collect();
    assert_eq!(versions, SUPPORTED_PROTOCOL_VERSIONS.to_vec());
    for profile in fixture["profiles"].as_array().expect("profiles") {
        let parsed = Profile::parse(profile["slug"].as_str().expect("slug")).expect("profile");
        assert_eq!(parsed.kind(), profile["kind"].as_str().expect("kind"));
        assert_eq!(parsed.server_name(), profile["serverName"].as_str().expect("serverName"));
    }
}

#[test]
fn go_json_escaping_is_reproduced() {
    assert_eq!(J::Str("a<b>c&d".to_string()).text(), r#""a\u003cb\u003ec\u0026d""#);
    assert_eq!(compact(b"{ \"a\" : [ 1 , 2 ] }"), r#"{"a":[1,2]}"#);
}
