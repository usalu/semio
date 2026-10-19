use super::*;
use semio_framework_replication_mutation::MutationMessage;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🧫️transaction-ref/🔣️.json")).expect("transaction ref fixture parses")
}

fn fixture_clock(case: &serde_json::Value) -> semio_framework_replication_ids::HybridLogicalTimestamp {
    let hlc = &case["hlc"];
    semio_framework_replication_ids::HybridLogicalTimestamp { actor: hlc["actor"].as_u64().expect("hlc actor"), physical_ms: hlc["physical_ms"].as_u64().expect("hlc physical_ms"), logical: hlc["logical"].as_u64().expect("hlc logical") }
}

/// 🪪️ The language-agnostic mint vectors: the first-party mint reproduces every expected id, and the
/// third-party `blake3` crate over the documented material reproduces it independently.
#[test]
fn the_mint_vectors_match_the_first_party_and_third_party_blake3() {
    let fixture = fixture();
    assert_eq!(fixture["schema"].as_str(), Some("semio.replication.transaction-ref"));
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let actor = semio_framework_replication_ids::ActorId(case["actor"].as_str().expect("actor").into());
        let clock = fixture_clock(case);
        let tool = case["tool"].as_str().expect("tool");
        let expected = TransactionRef { id: case["expect"]["id"].as_str().expect("expected id").to_string(), tool: case["expect"]["tool"].as_str().expect("expected tool").to_string() };
        assert_eq!(TransactionRef::mint(&actor, &clock, tool), expected, "{id}: first-party mint");
        let mut material = Vec::new();
        semio_framework_replication_wire_core::write_str(&mut material, &actor.0);
        semio_framework_replication_wire_core::write_varint_u64(&mut material, clock.actor);
        semio_framework_replication_wire_core::write_varint_u64(&mut material, clock.physical_ms);
        semio_framework_replication_wire_core::write_varint_u64(&mut material, clock.logical);
        semio_framework_replication_wire_core::write_str(&mut material, tool);
        assert_eq!(format!("tx-{}", &blake3::hash(&material).to_hex()[..16]), expected.id, "{id}: third-party blake3");
    }
}

/// 🧾️ `MutationMeta.transaction` is wire-omitted when absent and round-trips when present.
#[test]
fn mutation_meta_transaction_is_sparse_and_round_trips() {
    let meta = MutationMeta {
        mutation_id: Some(semio_framework_replication_ids::MutationId("op-1".into())),
        dependencies: Vec::new(),
        base_version: 0,
        author_id: None,
        timestamp: semio_framework_replication_ids::HybridLogicalTimestamp::new(1, 2),
        undo_policy: semio_framework_replication_wire_core::UndoPolicy::ExactBaseOnly,
        payload_hash: None,
        semantic_kind: None,
        label: None,
        group_id: None,
        origin: MutationOrigin::Owner,
        transaction: None,
    };
    let semio_framework_value::DslValue::Object(fields) = semio_framework_value::ToValue::to_value(&meta) else { panic!("meta is an object") };
    assert!(fields.iter().all(|(key, _)| key != "transaction"), "an operation outside a tool transaction omits the key");
    let stamped = MutationMeta { transaction: Some(TransactionRef::mint(&semio_framework_replication_ids::ActorId("alice".into()), &meta.timestamp, "app#select")), ..meta };
    let value = semio_framework_value::ToValue::to_value(&stamped);
    let decoded: MutationMeta = semio_framework_value::FromValue::from_value(value).expect("meta decodes");
    assert_eq!(decoded, stamped);
}

#[test]
fn replication_value_refusals_keep_kind_through_owned_paths_and_terminal_messages() {
    use semio_framework_value::{DslValue, FromValue, ValueRefusalKind};
    #[derive(serde::Deserialize)]
    struct TimestampOracle { actor: u64, physical_ms: u64, logical: u64 }
    #[derive(serde::Deserialize)]
    struct MessageOracle { level: String, code: String, message: String }
    #[derive(serde::Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    enum OriginOracle { Owner, Contributed { plugin_id: String }, Transaction { initiator: String } }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/⚠️value-refusals/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let input = case["input"].clone();
        let oracle_refuses = match case["owner"].as_str().unwrap() {
            "payloadHash" => serde_json::from_value::<[u8; 32]>(input.clone()).is_err(),
            "timestamp" => serde_json::from_value::<TimestampOracle>(input.clone()).is_err(),
            "mutationMessage" => serde_json::from_value::<MessageOracle>(input.clone()).is_err(),
            "mutationOrigin" => serde_json::from_value::<OriginOracle>(input.clone()).is_err(),
            _ => unreachable!(),
        };
        assert!(oracle_refuses);
        let text = serde_json::to_string(&input).unwrap();
        let value = semio_framework_pack_json::from_json_str::<DslValue>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let error = match case["owner"].as_str().unwrap() {
            "payloadHash" => semio_framework_replication_ids::PayloadHash::from_value(value).unwrap_err(),
            "timestamp" => semio_framework_replication_ids::HybridLogicalTimestamp::from_value(value).unwrap_err(),
            "mutationMessage" => MutationMessage::from_value(value).unwrap_err(),
            "mutationOrigin" => MutationOrigin::from_value(value).unwrap_err(),
            _ => unreachable!(),
        };
        assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap());
        assert!(error.message.contains(case["messageContains"].as_str().unwrap()));
        let error = error.under("snapshot").under(7);
        assert_eq!(error.kind, ValueRefusalKind::InvalidValue);
        assert!(error.message.starts_with("7.snapshot."));
        assert_eq!(error.clone().into_message(), error.message);
    }
    let mut cancel = |_| false;
    let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(usize::MAX, &mut cancel);
    let error = control.checkpoint().unwrap_err().under("snapshot");
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    let mut allow = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(0, &mut allow);
    let error = control.charge(1).unwrap_err().under("snapshot");
    assert_eq!(error.kind, ValueRefusalKind::OwnershipLimit);
}
