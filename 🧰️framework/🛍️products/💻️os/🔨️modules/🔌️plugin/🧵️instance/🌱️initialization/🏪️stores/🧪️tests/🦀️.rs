//! 🧪️ One Store lane finishes its original receipt before the next can begin.
use super::*;
use semio_framework_job::{default_now_us, Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;

#[test]
fn store_stage_waits_for_its_marker_to_settle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let mut stage = fixture["initialStage"].as_u64().unwrap() as u8;
    let maximum_turns = fixture["maximumTurnsPerStage"].as_u64().unwrap();
    let grant = RetainedCloneGrant {
        maximum_items: 1 << 20,
        maximum_copy_bytes: 1 << 24,
        maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20,
        maximum_depth: 64,
    };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(7901), Generation(47), grant, default_now_us, u64::MAX, 1 << 20);
    let mut decoder = OriginalJobDecodeOwner::empty();
    let mut birth = OriginalStoreBirth::default();
    let mut marker = None;
    for expected in fixture["settledStages"].as_array().unwrap() {
        let prior = stage;
        let mut candidate_seen = false;
        for _ in 0..maximum_turns {
            caller.turn_with_decoder(grant, &mut decoder, |cx| advance_stage(&mut stage, &mut marker, &mut birth, cx)).unwrap();
            if marker.is_some() {
                candidate_seen = true;
                assert_eq!(stage, prior);
            }
            if stage != prior {
                break;
            }
        }
        assert!(candidate_seen);
        assert!(marker.is_none());
        assert_eq!(u64::from(stage), expected.as_u64().unwrap());
    }
    for _ in 0..maximum_turns {
        if birth.terminal_is_empty() {
            break;
        }
        caller.turn_with_decoder(grant, &mut decoder, |cx| cx.with_original_retirement(|cx| birth.close_step(cx))).unwrap();
    }
    assert!(birth.terminal_is_empty());
    for _ in 0..maximum_turns {
        if decoder.terminal_is_empty() {
            break;
        }
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| decoder.close_on_source(cx).map(|_| ()))).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(grant);
}
