use super::*;
use semio_framework_pack_json::{JsonMemberPolicy, from_json_str};
use serde_json::Value;

#[test]
fn terrain_brush_metadata_matches_the_neutral_contract() {
    let fixture: Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let decode = |row: &Value| from_json_str::<TerrainBrushAction>(&row.to_string(), JsonMemberPolicy::Reject);
    for row in fixture["accepted"].as_array().unwrap() {
        assert!(decode(row).unwrap().is_valid(), "{row}");
    }
    for row in fixture["refused"].as_array().unwrap() {
        assert!(!decode(row).is_ok_and(|brush| brush.is_valid()), "{row}");
    }
}
