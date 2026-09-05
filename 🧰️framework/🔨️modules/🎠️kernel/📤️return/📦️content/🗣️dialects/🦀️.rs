//#region 🧪️OriginalFieldDialects
use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap() }
fn unhex(value: &str) -> Vec<u8> { (0..value.len()).step_by(2).map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16).unwrap()).collect() }

#[semio_framework_async_macros::async_test]
async fn return_content_existing_dialect_invocation_remains_exact_app_frame() {
    use semio_framework_os_kernel::channel::{encode_app_frame, decode_app_frame, AppFrame};
    let fixture = fixture();
    let row = &fixture["invocation"];
    let bytes = |name: &str| serde_json::from_value::<Vec<u8>>(row[name].clone()).unwrap();
    let frame = AppFrame::Invocation { in_reply_to: row["inReplyTo"].as_u64().unwrap(), output: bytes("output"), diagnostics: bytes("diagnostics"), ui_scope: bytes("uiScope"), history_patch: bytes("historyPatch"), messages: bytes("messages") };
    let encoded = encode_app_frame(&frame).await;
    assert_eq!(encoded, unhex(row["appFrameHex"].as_str().unwrap()));
    assert_eq!(decode_app_frame(&encoded).await.unwrap(), frame);
    let effect = Effect::SendMessage { target: MessageEndpoint::Shell { instance: PluginInstanceId(row["shellInstance"].as_u64().unwrap().to_string()) }, payload: encoded };
    let Effect::SendMessage { payload, .. } = effect else { unreachable!() };
    assert_eq!(payload, unhex(row["appFrameHex"].as_str().unwrap()));
    assert!(semio_framework_os_kernel::pack_rt::decode_wire_value(&payload).is_err());
    assert_eq!(row["uiAcknowledgement"], false);
}

#[test]
fn return_content_existing_dialect_presence_preserves_all_render_plane_fields() {
    let fixture = fixture();
    let row = &fixture["presence"];
    let update: PresenceUpdate = serde_json::from_value(row["value"].clone()).unwrap();
    let value = semio_framework_os_kernel::to_dsl_value(&update).unwrap();
    let encoded = semio_framework_os_kernel::pack_rt::encode_wire_value(&value);
    let decoded = semio_framework_os_kernel::pack_rt::decode_wire_value(&encoded).unwrap();
    let recovered: PresenceUpdate = semio_framework_os_kernel::from_dsl_value(decoded).unwrap();
    assert_eq!(serde_json::to_value(recovered).unwrap(), row["value"]);
    assert_eq!(update.node_key.as_bytes(), unhex(row["nodeKeyUtf8Hex"].as_str().unwrap()));
    assert_eq!(update.peers[0].label.as_bytes(), unhex(row["labelUtf8Hex"].as_str().unwrap()));
    assert_eq!(row["recordTag"], 6);
    assert_eq!(row["documentMutation"], false);
    assert_eq!(row["uiAcknowledgement"], false);
}
//#endregion 🧪️OriginalFieldDialects
