//! 📤️ Original return-content invocation crosses the OS AppFrame channel exactly.
use semio_framework::kernel::{Effect, MessageEndpoint, PluginInstanceId};
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/🧫️fixtures/🔣️.json")).unwrap() }
fn unhex(value: &str) -> Vec<u8> { (0..value.len()).step_by(2).map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16).unwrap()).collect() }

#[semio_framework_async_macros::async_test]
async fn return_content_existing_dialect_invocation_remains_exact_app_frame() {
    use semio_framework_os_kernel::channel::{decode_app_frame, encode_app_frame, AppFrame};
    let fixture = fixture();
    let row = &fixture["invocation"];
    let bytes = |name: &str| serde_json::from_value::<Vec<u8>>(row[name].clone()).unwrap();
    let frame = AppFrame::Invocation {
        in_reply_to: row["inReplyTo"].as_u64().unwrap(),
        output: bytes("output"),
        diagnostics: bytes("diagnostics"),
        ui_scope: bytes("uiScope"),
        history_patch: bytes("historyPatch"),
        messages: bytes("messages"),
        mutations: bytes("mutations"),
        inverse_group: bytes("inverseGroup"),
    };
    let encoded = encode_app_frame(&frame).await;
    assert_eq!(encoded, unhex(row["appFrameHex"].as_str().unwrap()));
    assert_eq!(decode_app_frame(&encoded).await.unwrap(), frame);
    let effect = Effect::SendMessage { target: MessageEndpoint::Shell { instance: PluginInstanceId(row["shellInstance"].as_u64().unwrap().to_string()) }, payload: encoded };
    let Effect::SendMessage { payload, .. } = effect else { unreachable!() };
    assert_eq!(payload, unhex(row["appFrameHex"].as_str().unwrap()));
    assert!(semio_framework_os_kernel::pack_rt::decode_wire_value(&payload).is_err());
    assert_eq!(row["uiAcknowledgement"], false);
}
