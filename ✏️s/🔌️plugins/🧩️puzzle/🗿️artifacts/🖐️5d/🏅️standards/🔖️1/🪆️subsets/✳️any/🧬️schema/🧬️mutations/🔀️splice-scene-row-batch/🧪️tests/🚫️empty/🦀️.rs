//! 🚫️ An empty scene batch refuses atomically.
use super::*;

#[test]
fn empty_batch_refuses_without_editing_the_scene() {
    let policy = semio_framework_pack_json::JsonMemberPolicy::Reject;
    let before: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-row-batch/🚫️empty/📸️snapshot/⬅️before/🔣️.json"), policy).unwrap();
    let after: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-row-batch/🚫️empty/📸️snapshot/➡️after/🔣️.json"), policy).unwrap();
    let op: Puzzle5dMutation = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-row-batch/🚫️empty/🦠️mutation/🔣️.json"), policy).unwrap();
    let outcome: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-row-batch/🚫️empty/🎯️outcome/🔣️.json")).unwrap();
    let delta = ColdMutation::<Puzzle5dSnapshot>::diff(&op, &before);
    assert_eq!(outcome["status"], "rejected");
    assert_eq!(delta.messages().len(), 1);
    assert_eq!(delta.messages()[0].code.0.as_str(), outcome["code"].as_str().unwrap());
    assert!(include_bytes!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-row-batch/🚫️empty/🔺️diff/🚫️.absent").is_empty());
    assert_eq!(protocol::apply_diff_cold(delta.diff(), &before).unwrap(), after);
    assert_eq!(after, before);
    assert!(inverse_puzzle5d_mutation(&before, &op).expect("valid retained mutation inverse fixture").is_empty(), "splice-scene-row-batch/empty: a refusal must yield no inverse step");
}
