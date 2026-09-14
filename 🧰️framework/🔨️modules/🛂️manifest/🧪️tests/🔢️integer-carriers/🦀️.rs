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
use dsl::Number;

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
    let DslValue::Object(entries) = value else { panic!("{key} is not reachable: the carrier is not an object") };
    &entries.iter().find(|(name, _)| name == key).unwrap_or_else(|| panic!("the carried context keeps {key}")).1
}

fn replace(value: &mut DslValue, path: &[String], replacement: DslValue) {
    let Some(key) = path.first() else {
        *value = replacement;
        return;
    };
    let DslValue::Object(entries) = value else { panic!("{key} is not reachable: the carrier is not an object") };
    let slot = &mut entries.iter_mut().find(|(name, _)| name == key).unwrap_or_else(|| panic!("the fixture context carries {key}")).1;
    replace(slot, &path[1..], replacement);
}

fn fixture_view(fixture: &IntegerCarrierFixture) -> ViewModel {
    dsl::os_pack::json::from_json_str(&fixture.view_context.to_string()).expect("the fixture context decodes")
}

#[semio_framework_async_macros::async_test]
async fn both_doors_carry_the_view_context_as_the_same_exact_pack_bytes() {
    let fixture = fixture();
    let bytes = dsl::pack_rt::encode_wire_value(&dsl::to_dsl_value(&fixture_view(&fixture)).expect("a view model is a dynamic value"));
    assert_eq!(hex(&bytes), fixture.pack_hex, "the wgpu door's `view_state_pack_base64` and the React door's `encodePackValue` must agree byte for byte");
}

#[semio_framework_async_macros::async_test]
async fn the_guest_decodes_every_integer_field_exactly() {
    let fixture = fixture();
    let value = dsl::pack_rt::decode_wire_value(&unhex(&fixture.pack_hex)).expect("the fixture pack decodes");
    for path in &fixture.integer_paths {
        let mut cursor = &value;
        for key in path {
            cursor = entry(cursor, key);
        }
        assert!(matches!(cursor, DslValue::Number(Number::UInt(_))), "{path:?} must cross as an exact integer carrier, found {cursor:?}");
    }
    let ingress = <ViewModel as dsl::FromValue>::from_value(value.clone()).expect("actor ingress decodes the carried context");
    assert_eq!(ingress.tool_run_trace_cursor_by_window_id.get("procedural-preview"), Some(&ToolRunTraceCursor { run: 1, generation: 0, page: 0 }), "actor ingress (`decode_wire_serialized`) keeps every echoed cursor");
    let view: ViewModel = dsl::from_dsl_value(value).expect("the carried context decodes in the guest");
    assert_eq!(view.tool_run_trace_cursor_by_window_id.get("procedural-preview"), Some(&ToolRunTraceCursor { run: 1, generation: 0, page: 0 }));
}

#[semio_framework_async_macros::async_test]
async fn a_widened_float_is_refused_and_never_rounded() {
    let fixture = fixture();
    let carried = dsl::to_dsl_value(&fixture_view(&fixture)).expect("a view model is a dynamic value");

    let mut widened = carried.clone();
    replace(&mut widened, &fixture.integer_paths[0], DslValue::Number(Number::Float(1.0)));
    let error = dsl::from_dsl_value::<ViewModel>(widened).expect_err("a whole float in a u64 slot must be refused");
    assert_eq!(error, fixture.guest_refusal);

    let mut fractional = carried;
    replace(&mut fractional, &fixture.integer_paths[0], DslValue::Number(Number::Float(fixture.non_integral)));
    assert!(dsl::from_dsl_value::<ViewModel>(fractional).is_err(), "a non-integral float in a u64 slot must be refused");
}
