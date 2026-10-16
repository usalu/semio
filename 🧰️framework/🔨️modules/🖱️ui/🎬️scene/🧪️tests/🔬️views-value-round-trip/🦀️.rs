//! 🧾️ Value-codec round trips of the table and block-list scenes and their refusals.

use super::*;
use serde_json::Value;

#[test]
fn missing_required_field_reports_the_field_name() {
    let empty = DslValue::object([]);
    assert_eq!(TableScene::from_value(empty), Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "missing field `columnsJson`")));
}

#[test]
fn table_scene_round_trips() {
    let table = TableScene { lanes: Vec::new(), columns_json: "[]".into(), rows_json: "[]".into(), selection_json: Some("{}".into()), row_drag_mime: None, drop_action_json: None, sort_json: None, domain_id: Some("d".into()), domain_granularity_id: Some("g".into()) };
    assert_eq!(TableScene::from_value(table.to_value()), Ok(table));
}

#[test]
fn block_list_scene_round_trips() {
    let blocks = BlockListScene { steps: Vec::new(), palette: Vec::new(), selected_id: Some("s1".into()), dragging_id: None, domain_id: None };
    assert_eq!(BlockListScene::from_value(blocks.to_value()), Ok(blocks));
}

/// 🔬️ The shared typed scene stays structural through the native Pack and value boundaries.
#[test]
fn typed_block_list_shared_fixture_round_trips_without_nested_text() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧩️block-list/🔣️.json")).expect("authored block-list fixture");
    let scene:BlockListScene=serde_json::from_value(fixture["scene"].clone()).expect("independent typed Serde admission");
    let value=scene.to_value();
    assert!(value.get("steps").is_some_and(|steps| steps.as_array().is_some()));
    assert!(value.get("palette").is_some_and(|palette| palette.as_array().is_some()));
    assert!(value.get("stepsJson").is_none());
    assert_eq!(BlockListScene::from_value(value),Ok(scene.clone()));
    assert_eq!(BlockListScene::decode_pack(&scene.encode_pack().expect("typed scene packs")).expect("typed scene unpacks"),scene);
    for row in fixture["refusals"].as_array().expect("refusal corpus") {
        let parsed=semio_framework_pack_json::parse(&row["value"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral refusal is valid JSON");
        assert!(BlockListScene::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).is_err(),"owned typed refusal {}",row["name"]);
        assert!(serde_json::from_value::<BlockListScene>(row["value"].clone()).is_err(),"independent typed refusal {}",row["name"]);
    }
    let mut duplicate=scene.to_value().into_object().expect("owned scene fields");duplicate.push(("steps".into(),DslValue::Array(Vec::new())));assert!(BlockListScene::from_value(DslValue::Object(duplicate)).is_err());
    println!("[DEBUG] Native typed block-list: sections={} palette={} refusals={} duplicateFieldsRefused=true valueRoundTrip=true packRoundTrip=true independentSerde=true",scene.steps.len(),scene.palette.len(),fixture["refusals"].as_array().unwrap().len());
}
