//! 📑️ Unit proofs of the document-style scenes: complete unicode table lanes.

use super::*;
use serde_json::Value;

/// 📊️ Large table carriers preserve all records while their scene header remains bounded.
#[test]
fn table_lanes_preserve_complete_unicode_documents() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🚚️table-lanes/🔣️.json")).unwrap();
    let rows: Vec<_> = (0..fixture["rowCount"].as_u64().unwrap()).map(|index| {
        let mut row = fixture["row"].clone();
        row["id"] = index.to_string().into();
        row
    }).collect();
    let scene = TableScene::base(serde_json::to_string(&fixture["columns"]).unwrap(), serde_json::to_string(&rows).unwrap());
    let (spine, lanes) = scene.split_lanes();
    assert_eq!(lanes.len(), 2);
    assert!(spine.columns_json.is_empty());
    assert!(spine.rows_json.is_empty());
    assert!(spine.encode_pack().unwrap().len() < 4096);
    let mut restored = TableScene::decode_pack(&spine.encode_pack().unwrap()).unwrap();
    for (lane, definition) in lanes.iter().zip(fixture["lanes"].as_array().unwrap()) {
        assert_eq!(lane.key, definition["bodyKey"].as_str().unwrap());
        assert!(restored.merge_lane(lane.key, lane.payload.clone()));
    }
    assert!(!restored.merge_lane("other", "[]".into()));
    restored.lanes.clear();
    assert_eq!(restored, scene);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&restored.rows_json).unwrap(), serde_json::to_value(rows).unwrap());
}
