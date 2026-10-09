use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧫️transaction-ref/🔣️.json")).expect("transaction ref fixture parses")
}

fn fixture_clock(case: &serde_json::Value) -> crate::ids::HybridLogicalTimestamp {
    let hlc = &case["hlc"];
    crate::ids::HybridLogicalTimestamp { actor: hlc["actor"].as_u64().expect("hlc actor"), physical_ms: hlc["physical_ms"].as_u64().expect("hlc physical_ms"), logical: hlc["logical"].as_u64().expect("hlc logical") }
}

/// 🪪️ The language-agnostic mint vectors: the first-party mint reproduces every expected id, and the
/// third-party `blake3` crate over the documented material reproduces it independently.
#[test]
fn the_mint_vectors_match_the_first_party_and_third_party_blake3() {
    let fixture = fixture();
    assert_eq!(fixture["schema"].as_str(), Some("semio.replication.transaction-ref"));
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let actor = crate::ids::ActorId(case["actor"].as_str().expect("actor").into());
        let clock = fixture_clock(case);
        let tool = case["tool"].as_str().expect("tool");
        let expected = TransactionRef { id: case["expect"]["id"].as_str().expect("expected id").to_string(), tool: case["expect"]["tool"].as_str().expect("expected tool").to_string() };
        assert_eq!(TransactionRef::mint(&actor, &clock, tool), expected, "{id}: first-party mint");
        let mut material = Vec::new();
        crate::write_str(&mut material, &actor.0);
        crate::wire::write_varint_u64(&mut material, clock.actor);
        crate::wire::write_varint_u64(&mut material, clock.physical_ms);
        crate::wire::write_varint_u64(&mut material, clock.logical);
        crate::write_str(&mut material, tool);
        assert_eq!(format!("tx-{}", &blake3::hash(&material).to_hex()[..16]), expected.id, "{id}: third-party blake3");
    }
}

/// 🧾️ `MutationMeta.transaction` is wire-omitted when absent and round-trips when present.
#[test]
fn mutation_meta_transaction_is_sparse_and_round_trips() {
    let meta = MutationMeta {
        mutation_id: Some(crate::ids::MutationId("op-1".into())),
        dependencies: Vec::new(),
        base_version: 0,
        author_id: None,
        timestamp: crate::ids::HybridLogicalTimestamp::new(1, 2),
        undo_policy: crate::UndoPolicy::ExactBaseOnly,
        payload_hash: None,
        semantic_kind: None,
        label: None,
        group_id: None,
        origin: MutationOrigin::Owner,
        transaction: None,
    };
    let crate::value::DslValue::Object(fields) = crate::value::ToValue::to_value(&meta) else { panic!("meta is an object") };
    assert!(fields.iter().all(|(key, _)| key != "transaction"), "an operation outside a tool transaction omits the key");
    let stamped = MutationMeta { transaction: Some(TransactionRef::mint(&crate::ids::ActorId("alice".into()), &meta.timestamp, "app#select")), ..meta };
    let value = crate::value::ToValue::to_value(&stamped);
    let decoded: MutationMeta = crate::value::FromValue::from_value(value).expect("meta decodes");
    assert_eq!(decoded, stamped);
}
