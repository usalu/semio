
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

/// 🪣️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS §1 decision 4 — a fresh document asks for a
/// hundred fill placements, and the same number is what the JSON schema (`🧬️schema/🔣️.json`) and the
/// TS guard mirror declare, so a config that travels as text comes back asking for the same run.
#[test]
fn default_fill_count_is_a_hundred_and_survives_the_json_round_trip() {
    assert_eq!(Puzzle3dConfig::default().fill_count, 100);
    assert_eq!(Puzzle3dRuntime::default().fill_count, 100);
    let json = dsl::json::to_json_string(&Puzzle3dConfig::default());
    assert!(json.contains("\"fillCount\":100"), "the serialized config carries the default count: {json}");
    let restored: Puzzle3dConfig = dsl::json::from_json_str(&json).expect("config deserializes");
    assert_eq!(restored.fill_count, 100);
    let absent: Puzzle3dConfig = dsl::json::from_json_str("{}").expect("an absent fillCount falls back to the schema default");
    assert_eq!(absent.fill_count, 100, "the schema default, not zero, is what an omitted fillCount means");
}
