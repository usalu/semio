use semio_framework_ui_contract_values::compare_protocol::*;
use semio_framework_ui_contract_values::*;
use semio_framework_ui_contract_values::testkit::*;
use semio_framework_test_fixtures::fixture_text;
use std::mem::size_of;
fn fixture()->serde_json::Value{serde_json::from_str(fixture_text!("../../../../../⚖️compare/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn retained_component_compare_frame_storage_matches_exact_bounded_domains() {
    let fixture = fixture();
    let frame = &fixture["frame"];
    assert_eq!(UI_VALUE_NESTING_DEPTH, frame["logicalDepth"].as_u64().unwrap() as usize);
    assert_eq!(UI_VALUE_AGGREGATE_ITEMS, frame["pageCount"].as_u64().unwrap() as usize);
    assert_eq!(UI_TEXT_MAX_BYTES, frame["maximumTextBytes"].as_u64().unwrap() as usize);
    assert_eq!(size_of::<ValueFrame>(), frame["bytes"].as_u64().unwrap() as usize);
    assert_eq!(ValueFrame::checked_page(UI_VALUE_NONE).unwrap(), u16::MAX);
    assert_eq!(ValueFrame::checked_page(UI_VALUE_AGGREGATE_ITEMS - 1).unwrap(), 1791);
    for index in [UI_VALUE_AGGREGATE_ITEMS, usize::from(u16::MAX), usize::from(u16::MAX) + 1] {
        assert!(ValueFrame::checked_page(index).is_err());
    }
    assert_eq!(ValueFrame::checked_position(2 * UI_TEXT_MAX_BYTES).unwrap(), 1024);
    let mut bytes = [0u8; 8];
    bytes[..2].copy_from_slice(&ValueFrame::checked_page(UI_VALUE_AGGREGATE_ITEMS - 1).unwrap().to_le_bytes());
    bytes[4..6].copy_from_slice(&ValueFrame::checked_position(2 * UI_TEXT_MAX_BYTES).unwrap().to_le_bytes());
    assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), frame["littleEndian"].as_str().unwrap());
    for position in [2 * UI_TEXT_MAX_BYTES + 1, usize::from(u16::MAX) + 1, usize::MAX] {
        assert!(ValueFrame::checked_position(position).is_err());
    }
}
