use super::*;

#[test]
fn app_config_carries_weights_and_fill_count_but_no_window_or_operation_state() {
    let config = Puzzle2dConfig::default();
    let json = dsl::json::to_json_string(&config);
    let oracle: serde_json::Value = serde_json::from_str(&json).expect("third-party JSON oracle");
    assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(3));
    assert!(oracle.get("nodeKindWeights").is_some());
    assert!(oracle.get("handleKindWeights").is_some());
    assert_eq!(oracle.get("fillCount").and_then(serde_json::Value::as_u64), Some(100));
    for forbidden in ["cameraX", "lodModeByPane", "engagementInputByPane", "brushCandidates", "fillJobOperation", "exampleLoadGeneration"] {
        assert!(oracle.get(forbidden).is_none(), "{forbidden} escaped its owner");
    }
}
