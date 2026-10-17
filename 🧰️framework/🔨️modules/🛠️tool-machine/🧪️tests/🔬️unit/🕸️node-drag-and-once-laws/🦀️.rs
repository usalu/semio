/// 🕸️ Node drag and once laws: the node drag rows and their commit as one transaction, the one-step tool and the independent dispatches.

//#region 🔖️NodeDragLaws
fn node_drag_law_text() -> &'static str {
    crate::fixture_text("🧪️tests/🧫️fixtures/🧫️node-drag-law/🔣️.json")
}

fn node_drag_row(json: &Value) -> protocol::DslValue {
    match json {
        Value::Null => protocol::DslValue::Null,
        Value::Bool(flag) => protocol::DslValue::Bool(*flag),
        Value::Number(number) => protocol::DslValue::float(number.as_f64().expect("a finite fixture number")),
        Value::String(text) => protocol::DslValue::String(text.clone()),
        Value::Array(items) => protocol::DslValue::Array(items.iter().map(node_drag_row).collect()),
        Value::Object(fields) => protocol::DslValue::Object(fields.iter().map(|(name, value)| (name.clone(), node_drag_row(value))).collect()),
    }
}

/// ⚖️ LAW (design §13.3): every valid gesture record row decodes to its record, re-encodes to a row that decodes to the same
/// record, and moves as the fixture states; every invalid row — the retired absolute `move{nodeId,x,y}` among them — is
/// refused by name.
#[test]
fn node_drag_rows_decode_to_their_records_and_invalid_rows_are_refused() {
    let law: Value = serde_json::from_str(node_drag_law_text()).expect("fixture parses");
    for case in law["valid"].as_array().expect("valid") {
        let id = text(&case["id"]);
        let record = NodeDragRecord::from_row(&node_drag_row(&case["row"])).unwrap_or_else(|reason| panic!("{id}: {reason}"));
        let expected = &case["record"];
        assert_eq!(record, NodeDragRecord { gesture_id: text(&expected["gestureId"]).into(), node_ids: expected["nodeIds"].as_array().expect("ids").iter().map(|id| text(id).to_string()).collect(), dx: expected["dx"].as_f64().expect("dx"), dy: expected["dy"].as_f64().expect("dy") }, "{id}");
        assert_eq!(NodeDragRecord::from_row(&record.to_row()).as_ref(), Ok(&record), "{id}: the row round trips");
        assert_eq!(record.moves(), case["moves"].as_bool().expect("moves"), "{id}");
    }
    for case in law["invalid"].as_array().expect("invalid") {
        assert!(NodeDragRecord::from_row(&node_drag_row(&case["row"])).is_err(), "{}: {}", text(&case["id"]), text(&case["reason"]));
    }
}

/// ⚖️ LAW: the node-drag machine commits a release as ONE transaction of the guest's leaves, its ref minted from the
/// authoring actor, the clock and the tool; a release that yields nothing commits nothing.
#[test]
fn a_released_node_drag_commits_one_transaction_of_its_leaves() {
    let law: Value = serde_json::from_str(node_drag_law_text()).expect("fixture parses");
    let commit = &law["commit"];
    let (tool, actor, at) = (text(&commit["tool"]), ActorId(text(&commit["actor"]).into()), clock(&commit["clock"]));
    let leaves: Vec<Value> = commit["leaves"].as_array().expect("leaves").clone();
    let (transaction, committed) = node_drag_commit(tool, actor.clone(), text(&commit["gesture"]), leaves.clone(), at).expect("the release commits");
    assert_eq!(transaction, TransactionRef::mint(&actor, &at, tool));
    assert_eq!(committed, leaves);
    assert_eq!(node_drag_commit::<Value>(tool, actor, text(&commit["gesture"]), Vec::new(), at), None, "an empty release leaves zero trace");
}
//#endregion 🔖️NodeDragLaws

//#region 🎯️OnceLaws
/// 🎯️ LAW (design §22.32): a one-step tool dispatch is ONE transaction of its leaves stamped `<appId>#<verb>`; a dispatch
/// that yields nothing leaves zero trace; without an admission the leaves publish plainly.
#[test]
fn a_one_step_tool_commits_one_transaction_of_its_leaves() {
    let NodeDragEmit::Commit(transaction, leaves) = tool_once_emit("demo@1/*#editor", "place", "seed", vec![1, 2]) else { panic!("an admitted one-step dispatch commits") };
    assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
    assert_eq!((transaction.tool.as_str(), leaves), ("demo@1/*#editor#place", vec![1, 2]));
    assert_eq!(tool_once_emit::<i64>("demo@1/*#editor", "place", "seed", Vec::new()), NodeDragEmit::Nothing);
    assert_eq!(tool_once_emit("demo@1/*#editor", "place", "", vec![3]), NodeDragEmit::Plain(vec![3]));
}
//#endregion 🎯️OnceLaws

#[test]
fn independent_one_step_dispatches_never_share_a_transaction() {
    let mut ids = std::collections::BTreeSet::new();
    let fixture: Value = serde_json::from_str(node_drag_law_text()).expect("fixture");
    for _ in 0..fixture["independentReleases"]["count"].as_u64().expect("count") {
        let NodeDragEmit::Commit(transaction, _) = tool_once_emit("demo@1/*#editor", "place", "seed", vec![1]) else { panic!("one-step dispatch commits") };
        assert!(ids.insert(transaction.id), "each independent gesture owns its transaction");
    }
    assert_eq!(ids.len() as u64, fixture["independentReleases"]["uniqueTransactions"].as_u64().expect("unique transactions"));
}
