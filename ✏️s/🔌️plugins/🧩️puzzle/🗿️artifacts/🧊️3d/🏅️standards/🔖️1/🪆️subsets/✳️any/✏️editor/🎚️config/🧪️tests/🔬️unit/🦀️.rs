
use super::*;

#[test]
fn config_round_trip_has_no_process_identity() {
    let config = Puzzle3dConfig::default();
    let json = dsl::json::to_json_string(&config);
    let restored: Puzzle3dConfig = dsl::json::from_json_str(&json).expect("config deserializes");
    assert_eq!(config, restored);
    let mutation = Puzzle3dConfigMutation::Snapshot { config };
    let encoded = protocol::OpBinary::encode_op(&mutation).expect("mutation encodes");
    assert!(!String::from_utf8_lossy(&encoded).contains("runtimeSessionId"));
}
