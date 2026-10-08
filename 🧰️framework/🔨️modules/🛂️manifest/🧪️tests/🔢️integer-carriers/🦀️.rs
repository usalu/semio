//! 🔢️ The GUEST half of the integer-carrier law, driven by the SAME language-agnostic
//! `🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json` the TypeScript twin
//! (`📺️renderer/🧑‍🎨engine/🧪️tests/📌️view-state-carriage/🟦️.ts`) reads.
//!
//! `toolRunTraceCursorByWindowId.<window>.run` is the FIRST integer-typed field a view context ever
//! carried — `locale`, `terminology`, `windowInstances` and `activeUtilityByWindowId` are all
//! strings — so no crossing had ever had to carry an integer. Both renderer doors encode the view
//! context as pack, and pack keeps `UInt(1)` and `Float(1.0)` apart exactly as the guest's
//! `FromValue` does: this law pins the bytes both doors must produce, the `ViewModel` they decode
//! to, and the refusal a widened float earns (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use super::*;
use semio_framework_value::Number;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct IntegerCarrierFixture {
    view_context: serde_json::Value,
    integer_paths: Vec<Vec<String>>,
    pack_hex: String,
    guest_refusal: String,
    non_integral: f64,
}

fn fixture() -> IntegerCarrierFixture {
    serde_json::from_str(include_str!("../../🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json")).expect("the integer-carrier fixture parses")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("the fixture pack is hex")).collect()
}

fn entry<'a>(value: &'a DslValue, key: &str) -> &'a DslValue {
    let semio_framework_value::DslValue::Object(entries) = value else { panic!("{key} is not reachable: the carrier is not an object") };
    &entries.iter().find(|(name, _)| name == key).unwrap_or_else(|| panic!("the carried context keeps {key}")).1
}

fn replace(value: &mut DslValue, path: &[String], replacement: DslValue) {
    let Some(key) = path.first() else {
        *value = replacement;
        return;
    };
    let semio_framework_value::DslValue::Object(entries) = value else { panic!("{key} is not reachable: the carrier is not an object") };
    let slot = &mut entries.iter_mut().find(|(name, _)| name == key).unwrap_or_else(|| panic!("the fixture context carries {key}")).1;
    replace(slot, &path[1..], replacement);
}

fn scene_view(fixture: &IntegerCarrierFixture) -> ViewModel {
    semio_framework_pack_json::from_json_str(&fixture.view_context.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the fixture context decodes")
}

#[semio_framework_async_macros::async_test]
async fn both_doors_carry_the_view_context_as_the_same_exact_pack_bytes() {
    let fixture = fixture();
    let mut accepted = |_| true;
    let bytes = pack::record::intrinsic::encode_body(&semio_framework_value::ToValue::to_value(&scene_view(&fixture)), &pack::record::EncodeOptions::default(), &mut semio_framework_value::NativeEncodeControl::new(16 * 1024 * 1024, &mut accepted)).expect("the fixture context encodes");
    assert_eq!(hex(&bytes), fixture.pack_hex, "the wgpu door's `view_state_pack_base64` and the React door's `encodePackValue` must agree byte for byte");
}

#[semio_framework_async_macros::async_test]
async fn the_guest_decodes_every_integer_field_exactly() {
    let fixture = fixture();
    let mut accepted = |_| true;
    let value = pack::record::intrinsic::decode_body(&unhex(&fixture.pack_hex), &pack::record::DecodeOptions::default(), &mut semio_framework_value::NativeDecodeControl::new(16 * 1024 * 1024, &mut accepted)).expect("the fixture pack decodes");
    for path in &fixture.integer_paths {
        let mut cursor = &value;
        for key in path {
            cursor = entry(cursor, key);
        }
        assert!(matches!(cursor, DslValue::Number(Number::UInt(_))), "{path:?} must cross as an exact integer carrier, found {cursor:?}");
    }
    let ingress = <ViewModel as semio_framework_value::FromValue>::from_value(value.clone()).expect("actor ingress decodes the carried context");
    assert_eq!(ingress.tool_run_trace_cursor_by_window_id.get("procedural-preview"), Some(&ToolRunTraceCursor { run: 1, generation: 0, page: 0 }), "actor ingress (`decode_wire_serialized`) keeps every echoed cursor");
    let view: ViewModel = semio_framework_value::FromValue::from_value(value).expect("the carried context decodes in the guest");
    assert_eq!(view.tool_run_trace_cursor_by_window_id.get("procedural-preview"), Some(&ToolRunTraceCursor { run: 1, generation: 0, page: 0 }));
}

#[semio_framework_async_macros::async_test]
async fn a_widened_float_is_refused_and_never_rounded() {
    let fixture = fixture();
    let carried = semio_framework_value::ToValue::to_value(&scene_view(&fixture));

    let mut widened = carried.clone();
    replace(&mut widened, &fixture.integer_paths[0], semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(1.0)));
    let error = <ViewModel as semio_framework_value::FromValue>::from_value(widened).expect_err("a whole float in a u64 slot must be refused");
    assert_eq!(error.to_string(), fixture.guest_refusal);

    let mut fractional = carried;
    replace(&mut fractional, &fixture.integer_paths[0], semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(fixture.non_integral)));
    assert!(<ViewModel as semio_framework_value::FromValue>::from_value(fractional).is_err(), "a non-integral float in a u64 slot must be refused");
}
