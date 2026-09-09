use super::*;

#[test]
fn fill_text_capacity_is_exact() {
    let maximum = "x".repeat(PUZZLE2D_FILL_TEXT_CAPACITY);
    let over = "x".repeat(PUZZLE2D_FILL_TEXT_CAPACITY + 1);
    let Some(text) = Puzzle2dFillText::try_from_str(&maximum) else { panic!("MAX fill text must fit") };
    assert_eq!(text.as_str(), maximum);
    assert!(Puzzle2dFillText::try_from_str(&over).is_none());
}

#[test]
fn app_config_excludes_window_and_operation_state() {
    let config = Puzzle2dConfig::default();
    let json = dsl::json::to_json_string(&config);
    let oracle: serde_json::Value = serde_json::from_str(&json).expect("third-party JSON oracle");
    assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(2));
    assert!(oracle.get("nodeKindWeights").is_some());
    assert!(oracle.get("handleKindWeights").is_some());
    for forbidden in ["cameraX", "lodModeByPane", "engagementInputByPane", "brushCandidates", "fillJobOperation", "exampleLoadGeneration"] {
        assert!(oracle.get(forbidden).is_none(), "{forbidden} escaped its owner");
    }
}
