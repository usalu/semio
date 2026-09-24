use super::*;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("semio-coordinator-{name}-{}", new_id()));
    std::fs::create_dir_all(&dir).expect("dir");
    dir
}

#[test]
fn golden_envelope_matches_the_frozen_fixture() {
    let mut event = EventEnvelope {
        stream: "coordinator".to_string(),
        sequence: 1,
        id: "evt-1".to_string(),
        generation: 7,
        kind: "ticket.recorded".to_string(),
        payload: serde_json::json!({"id": "T-1", "status": "open"}),
        checksum: String::new(),
    };
    event.checksum = event_checksum(&event);
    assert_eq!(event.checksum, "6d6e63ada99d3b4b6ed21114f00421e3c0bb6d480c830351f4b7f323c6fa82ec");
    assert_eq!(
        encode_event(&event),
        "{\"stream\":\"coordinator\",\"sequence\":1,\"id\":\"evt-1\",\"generation\":7,\"type\":\"ticket.recorded\",\"payload\":{\"id\":\"T-1\",\"status\":\"open\"},\"checksum\":\"6d6e63ada99d3b4b6ed21114f00421e3c0bb6d480c830351f4b7f323c6fa82ec\"}\n"
    );
}

#[test]
fn append_then_replay_is_deterministic_and_refuses_a_duplicate() {
    let dir = temp_dir("append");
    let store = EventStore::open(dir.join("coordinator.events"), StoreLimits::default()).expect("open");
    let input = EventInput {
        stream: "coordinator".to_string(),
        id: "evt-1".to_string(),
        generation: 7,
        kind: "ticket.recorded".to_string(),
        payload: serde_json::json!({"id": "T-1", "status": "open"}),
    };
    let first = store.append(0, std::slice::from_ref(&input)).expect("append");
    assert!(first.committed && !first.duplicate);
    let replayed = store.replay().expect("replay");
    assert_eq!(replayed.len(), 1);
    assert_eq!(replayed[0].sequence, 1);
    let again = store.append(1, std::slice::from_ref(&input)).expect("idempotent");
    assert!(again.duplicate);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn an_injected_fault_preserves_the_committed_prefix() {
    let dir = temp_dir("fault");
    let path = dir.join("coordinator.events");
    let store = EventStore::open(&path, StoreLimits::default()).expect("open");
    let first = EventInput {
        stream: "coordinator".to_string(),
        id: "evt-1".to_string(),
        generation: 1,
        kind: "ticket.recorded".to_string(),
        payload: serde_json::json!({"id": "T-1"}),
    };
    store.append(0, std::slice::from_ref(&first)).expect("first");
    let before = std::fs::read(&path).expect("read");
    store.arm_fault(1);
    let second = EventInput { id: "evt-2".to_string(), ..first };
    assert!(store.append(1, std::slice::from_ref(&second)).is_err());
    store.arm_fault(0);
    assert_eq!(std::fs::read(&path).expect("read"), before);
    let reopened = EventStore::open(&path, StoreLimits::default()).expect("reopen");
    assert_eq!(reopened.replay().expect("replay").len(), 1);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_route_table_answers_every_documented_path() {
    let dir = temp_dir("http");
    let config = Config { address: "127.0.0.1:0".to_string(), database_path: dir.join("coordinator.events").display().to_string(), ..Config::default() };
    let service = start(config).expect("start");
    let server = Arc::clone(service.server());
    let health = server.handle(&Request { method: "GET".to_string(), path: "/healthz".to_string(), ..Request::default() });
    assert_eq!((health.status, health.body), (200, b"ok".to_vec()));
    let opened = server.handle(&Request {
        method: "POST".to_string(),
        path: "/ticket/open".to_string(),
        body: br#"{"ticket_id":"T-1","title":"Demo"}"#.to_vec(),
        ..Request::default()
    });
    assert_eq!(opened.status, 200);
    let missing = server.handle(&Request { method: "GET".to_string(), path: "/ticket/absent".to_string(), ..Request::default() });
    assert_eq!(missing.status, 404);
    let wrong_method = server.handle(&Request { method: "GET".to_string(), path: "/events".to_string(), ..Request::default() });
    assert_eq!(wrong_method.status, 405);
    service.stop();
    std::fs::remove_dir_all(&dir).ok();
}
