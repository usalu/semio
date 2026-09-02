//#region 🧪️BorrowedMessageRecord
use super::return_message::ReturnMessageCursor;
use super::{Effect, MessageEndpoint, PluginInstanceId};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }
fn unhex(value: &str) -> Vec<u8> { (0..value.len()).step_by(2).map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16).unwrap()).collect() }
fn endpoint(value: &serde_json::Value) -> MessageEndpoint {
    match value["kind"].as_str().unwrap() {
        "shell" => MessageEndpoint::Shell { instance: PluginInstanceId(value["instance"].as_str().unwrap().into()) },
        "pluginInstance" => MessageEndpoint::PluginInstance { id: PluginInstanceId(value["id"].as_str().unwrap().into()) },
        "backbone" => MessageEndpoint::Backbone { uri: value["uri"].as_str().unwrap().into() },
        "extension" => MessageEndpoint::Extension { id: value["id"].as_str().unwrap().into() },
        "topic" => MessageEndpoint::Topic { name: value["name"].as_str().unwrap().into() },
        _ => unreachable!(),
    }
}

fn encode(effect: &Effect, grant: usize) -> Vec<u8> {
    let mut cursor = ReturnMessageCursor::new(effect).unwrap();
    let mut result = Vec::new();
    for _ in 0..20_000 {
        let mut page = [73; 4096];
        let zero = cursor.write(&mut page, 0, grant);
        assert_eq!((zero.advanced_items, zero.written_bytes), (0, 0));
        assert_eq!(page, [73; 4096]);
        let zero = cursor.write(&mut page, 1, 0);
        assert_eq!((zero.advanced_items, zero.written_bytes), (0, 0));
        let step = cursor.write(&mut page, 1, grant);
        assert!(step.advanced_items <= 1 && step.written_bytes <= grant);
        assert!(page[step.written_bytes..].iter().all(|byte| *byte == 73));
        result.extend_from_slice(&page[..step.written_bytes]);
        if step.complete { return result; }
    }
    panic!("borrowed message cursor did not terminate");
}

#[test]
fn return_content_message_all_endpoints_match_independent_bytes_without_payload_parsing() {
    let fixture = fixture();
    for row in fixture["vectors"].as_array().unwrap() {
        let effect = Effect::SendMessage { target: endpoint(&row["endpoint"]), payload: unhex(fixture["payloadHex"].as_str().unwrap()) };
        for grant in [1, 64, 4096] { assert_eq!(encode(&effect, grant), unhex(row["recordHex"].as_str().unwrap())); }
    }
    let common: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixture/🔣️.json")).unwrap();
    let effect = Effect::SendMessage { target: MessageEndpoint::Shell { instance: PluginInstanceId("7".into()) }, payload: unhex(common["invocation"]["appFrameHex"].as_str().unwrap()) };
    assert_eq!(encode(&effect, 1), unhex(common["invocation"]["effectRecordHex"].as_str().unwrap()));
}

#[test]
fn return_content_message_large_payload_and_cancel_keep_original_source_allocation() {
    assert!(!std::mem::needs_drop::<ReturnMessageCursor<'_>>());
    assert!(std::mem::size_of::<ReturnMessageCursor<'_>>() <= 256);
    let fixture = fixture();
    let length = fixture["largePayload"]["length"].as_u64().unwrap() as usize;
    let payload: Vec<_> = (0..length).map(|index| ((index * 37 + 11) % 256) as u8).collect();
    let pointer = payload.as_ptr();
    let effect = Effect::SendMessage { target: MessageEndpoint::Shell { instance: PluginInstanceId("7".into()) }, payload };
    let mut expected = unhex(fixture["largePayload"]["prefixHex"].as_str().unwrap());
    if let Effect::SendMessage { payload, .. } = &effect { expected.extend_from_slice(payload); }
    for grant in [1, 64, 4096] { assert_eq!(encode(&effect, grant), expected); }
    for frontier in 0..8 {
        let mut cursor = ReturnMessageCursor::new(&effect).unwrap();
        for _ in 0..frontier { let _ = cursor.write(&mut [0; 64], 1, 64); }
        drop(cursor);
        assert!(matches!(&effect, Effect::SendMessage { payload, .. } if payload.as_ptr() == pointer && payload.len() == length));
    }
}

#[test]
fn return_content_message_invalid_instance_and_wrong_effect_refuse_without_consuming_source() {
    for value in fixture()["invalidInstances"].as_array().unwrap() {
        let effect = Effect::SendMessage { target: MessageEndpoint::Shell { instance: PluginInstanceId(value.as_str().unwrap().into()) }, payload: vec![17, 19] };
        assert!(ReturnMessageCursor::new(&effect).is_err());
        assert!(matches!(&effect, Effect::SendMessage { payload, .. } if payload == &[17, 19]));
    }
    assert!(ReturnMessageCursor::new(&Effect::RequestSync).is_err());
}
//#endregion 🧪️BorrowedMessageRecord
