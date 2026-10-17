/// 📄️ Transaction law fixture readers: the fixture text and its JSON accessors, the clock and reference decoding, the yield decoding and the transaction constructor.

fn transaction_law_text() -> &'static str {
    crate::fixture_text("🧪️tests/🧫️fixtures/🧫️transaction-law/🔣️.json")
}

fn law() -> Value {
    serde_json::from_str(transaction_law_text()).expect("fixture parses")
}

fn text(value: &Value) -> &str {
    value.as_str().expect("string")
}

fn int(value: &Value) -> i64 {
    value.as_i64().expect("integer")
}

fn clock(value: &Value) -> HybridLogicalTimestamp {
    HybridLogicalTimestamp { actor: value["actor"].as_u64().expect("u64"), physical_ms: value["physical_ms"].as_u64().expect("u64"), logical: value["logical"].as_u64().expect("u64") }
}

fn reference(value: &Value) -> TransactionRef {
    TransactionRef { id: text(&value["id"]).to_string(), tool: text(&value["tool"]).to_string() }
}

fn reference_json(reference: &TransactionRef) -> Value {
    json!({ "id": reference.id, "tool": reference.tool })
}

fn yielded(value: &Value) -> ToolYield<Value> {
    match text(&value["kind"]) {
        "upsert" => ToolYield::upsert(text(&value["key"]), value["mutation"].clone()),
        "retract" => ToolYield::retract(text(&value["key"])),
        "commit" => ToolYield::Commit,
        "abort" => ToolYield::Abort,
        other => panic!("unknown yield {other}"),
    }
}

fn entries_json<M>(entries: &[(String, M)], mutation: impl Fn(&M) -> Value) -> Value {
    Value::Array(entries.iter().map(|(key, value)| json!({ "key": key, "mutation": mutation(value) })).collect())
}

fn sample_yield(kind: ToolYieldKind) -> ToolYield<Value> {
    match kind {
        ToolYieldKind::Upsert => ToolYield::upsert("k", json!(1)),
        ToolYieldKind::Retract => ToolYield::retract("k"),
        ToolYieldKind::Commit => ToolYield::Commit,
        ToolYieldKind::Abort => ToolYield::Abort,
    }
}

fn transaction_in(state: ToolTransactionState) -> ToolTransaction<Value> {
    let mut transaction = ToolTransaction::open(TransactionRef { id: "tx-0000000000000000".into(), tool: "law#matrix".into() });
    match state {
        ToolTransactionState::Open => {}
        ToolTransactionState::Committed => transaction.apply(ToolYield::Commit).expect("open accepts commit"),
        ToolTransactionState::Aborted => transaction.apply(ToolYield::Abort).expect("open accepts abort"),
    }
    transaction
}
