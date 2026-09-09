
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

#[test]
fn persistence_contract_matches_neutral_scope_progress_and_terminal_traces() {
    let fixture = fixture();
    let requests = fixture["requests"].as_array().unwrap();
    assert_eq!(requests.len(), 4);
    assert_eq!(fixture["traces"].as_array().unwrap().len(), 15);
    for trace in fixture["traces"].as_array().unwrap() {
        let row = requests.iter().find(|row| row["id"] == trace["request"]).unwrap();
        let request = PersistenceRequestV1::parse(&serde_json::to_vec(&row["value"]).unwrap()).unwrap();
        let scope = request.correlation.scope.clone();
        let mut pending = PendingPersistenceV1::new(request, &scope).unwrap();
        let mut publications = 0;
        for step in trace["steps"].as_array().unwrap() {
            let outcome = PersistenceEventV1::parse(&serde_json::to_vec(&step["value"]).unwrap()).and_then(|event| pending.observe(&event));
            assert_eq!(outcome.is_ok(), step["accept"].as_bool().unwrap(), "{}: {step}", trace["id"]);
            publications += usize::from(outcome == Ok(PersistenceObservation::DurablePublication));
        }
        assert_eq!(publications as u64, trace["published"].as_u64().unwrap(), "{}", trace["id"]);
    }
}

#[test]
fn persistence_contract_rejects_closed_fields_and_cross_scope_receipts() {
    let fixture = fixture();
    let source = &fixture["requests"][0]["value"];
    assert_eq!(fixture["requestNegatives"].as_array().unwrap().len(), 16);
    for row in fixture["requestNegatives"].as_array().unwrap() {
        let mut hostile = source.clone();
        let path: Vec<&str> = row["path"].as_str().unwrap().split('.').collect();
        let mut parent = &mut hostile;
        for key in &path[..path.len() - 1] {
            parent = &mut parent[*key];
        }
        parent[path[path.len() - 1]] = row["value"].clone();
        assert!(PersistenceRequestV1::parse(&serde_json::to_vec(&hostile).unwrap()).is_err(), "{}", row["id"]);
    }
    let request = PersistenceRequestV1::parse(&serde_json::to_vec(source).unwrap()).unwrap();
    let mut scope = request.correlation.scope.clone();
    scope.window_id = "window-b".into();
    assert!(matches!(PendingPersistenceV1::new(request, &scope), Err(PersistenceContractError::Scope)));
    assert_eq!(PersistenceRequestV1::parse(&vec![b' '; PERSISTENCE_MESSAGE_LIMIT + 1]), Err(PersistenceContractError::TooLarge));
    assert_eq!(PersistenceEventV1::parse(&vec![b' '; PERSISTENCE_MESSAGE_LIMIT + 1]), Err(PersistenceContractError::TooLarge));
    for extra in ["path", "token", "message"] {
        let mut event = fixture["traces"][0]["steps"][4]["value"].clone();
        event["outcome"][extra] = serde_json::json!("private");
        assert!(PersistenceEventV1::parse(&serde_json::to_vec(&event).unwrap()).is_err());
    }
    for digest in ["0".repeat(64), "A".repeat(64), "a".repeat(63)] {
        let mut event = fixture["traces"][0]["steps"][4]["value"].clone();
        event["outcome"]["contentDigest"] = serde_json::json!(digest);
        assert!(PersistenceEventV1::parse(&serde_json::to_vec(&event).unwrap()).is_err());
    }
    for key in ["requestId", "generation"] {
        let mut event = fixture["traces"][0]["steps"][4]["value"].clone();
        let correlation = if key == "requestId" { &mut event["correlation"] } else { &mut event["correlation"]["scope"] };
        correlation[key] = serde_json::json!(SAFE_INTEGER + 1);
        assert!(PersistenceEventV1::parse(&serde_json::to_vec(&event).unwrap()).is_err());
    }
    for key in ["windowId", "spaceId", "sessionId", "generation"] {
        let request = PersistenceRequestV1::parse(&serde_json::to_vec(source).unwrap()).unwrap();
        let scope = request.correlation.scope.clone();
        let mut pending = PendingPersistenceV1::new(request, &scope).unwrap();
        let mut event = fixture["traces"][0]["steps"][0]["value"].clone();
        event["correlation"]["scope"][key] = if key == "generation" { serde_json::json!(8) } else { serde_json::json!("another-owner") };
        let event = PersistenceEventV1::parse(&serde_json::to_vec(&event).unwrap()).unwrap();
        assert_eq!(pending.observe(&event), Err(PersistenceContractError::Scope));
        assert_eq!(pending.phase, None);
        assert_eq!(pending.completed, 0);
        assert!(!pending.terminal);
    }
}
