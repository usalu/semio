//! 🔀️ A middle-row replacement preserves its neighbors and reverses exactly.
use super::*;

#[test]
fn middle_splice_matches_the_declared_delta() {
    let policy = semio_framework_pack_json::JsonMemberPolicy::Reject;
    let before: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-rows/🔀️middle/📸️snapshot/⬅️before/🔣️.json"), policy).unwrap();
    let after: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-rows/🔀️middle/📸️snapshot/➡️after/🔣️.json"), policy).unwrap();
    let op: Puzzle5dMutation = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-rows/🔀️middle/🦠️mutation/🔣️.json"), policy).unwrap();
    let expected = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-rows/🔀️middle/🔺️diff/🔣️.json"), policy).unwrap();
    let outcome: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀️splice-scene-rows/🔀️middle/🎯️outcome/🔣️.json")).unwrap();
    assert_eq!(outcome["status"], "applied");
    let delta = ColdMutation::<Puzzle5dSnapshot>::diff(&op, &before);
    assert_eq!(delta.diff(), &expected);
    assert_eq!(protocol::apply_diff_cold(delta.diff(), &before).unwrap(), after);
    let inverse = inverse_puzzle5d_mutation(&before, &op).unwrap();
    assert_eq!(inverse.len(), 1);
    let undo = ColdMutation::<Puzzle5dSnapshot>::diff(&inverse[0], &after);
    assert_eq!(protocol::apply_diff_cold(undo.diff(), &after).unwrap(), before);
}
