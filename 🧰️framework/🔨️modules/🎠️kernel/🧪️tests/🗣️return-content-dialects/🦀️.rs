//#region 🧪️OriginalFieldDialects
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../📤️return/📦️content/🧫️fixtures/🔣️.json")).unwrap()
}
fn unhex(value: &str) -> Vec<u8> {
    (0..value.len()).step_by(2).map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16).unwrap()).collect()
}

#[test]
fn return_content_existing_dialect_presence_preserves_all_render_plane_fields() {
    let fixture = fixture();
    let row = &fixture["presence"];
    let update: PresenceUpdate = serde_json::from_value(row["value"].clone()).unwrap();
    let value = semio_framework_value::ToValue::to_value(&update);
    let mut accepted = |_| true;
    let encoded = pack::record::intrinsic::encode_body(&value, &pack::record::EncodeOptions::default(), &mut semio_framework_value::NativeEncodeControl::new(16 * 1024 * 1024, &mut accepted)).unwrap();
    let mut accepted = |_| true;
    let decoded = pack::record::intrinsic::decode_body(&encoded, &pack::record::DecodeOptions::default(), &mut semio_framework_value::NativeDecodeControl::new(16 * 1024 * 1024, &mut accepted)).unwrap();
    let recovered: PresenceUpdate = semio_framework_value::FromValue::from_value(decoded).unwrap();
    assert_eq!(serde_json::to_value(recovered).unwrap(), row["value"]);
    assert_eq!(update.node_key.as_bytes(), unhex(row["nodeKeyUtf8Hex"].as_str().unwrap()));
    assert_eq!(update.peers[0].label.as_bytes(), unhex(row["labelUtf8Hex"].as_str().unwrap()));
    assert_eq!(row["recordTag"], 6);
    assert_eq!(row["documentMutation"], false);
    assert_eq!(row["uiAcknowledgement"], false);
}
