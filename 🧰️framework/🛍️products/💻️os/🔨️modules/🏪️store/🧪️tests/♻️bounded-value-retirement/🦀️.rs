//! ♻️ A bounded value frees on the first positive grant and reports exactly one page, in grant-sized
//! instalments, never idling under a sub-page grant.
use super::*;

#[test]
fn bounded_value_retirement_is_live_and_conserves_one_page_under_every_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️bounded-value-retirement/🔣️.json")).unwrap();
    assert_eq!(fixture["pageBytes"].as_u64().unwrap() as usize, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let grant = case["grant"].as_u64().unwrap() as usize;
        let value = Arc::new(7u64);
        let mut retirement = SnapshotRetirementFactory::retire(&BoundedArtifactRetirementFactory::<u64>::new(), Arc::clone(&value));
        assert_eq!(retirement.close_step(0, grant).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }, "{name}");
        assert_eq!(retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }, "{name}");
        assert_eq!(Arc::strong_count(&value), 2, "{name}");
        let (mut turns, mut items, mut bytes) = (0usize, 0usize, 0usize);
        loop {
            let owing = !retirement.terminal_is_empty();
            match retirement.close_step(1, grant).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(owing, "{name}: a terminal retirement never reports more release");
                    assert!(released_items <= 1 && (1..=grant).contains(&released_bytes), "{name}");
                    if turns == 0 {
                        assert_eq!((released_items, released_bytes), (1, case["firstTurnBytes"].as_u64().unwrap() as usize), "{name}");
                        assert_eq!(Arc::strong_count(&value), 1, "{name}: the value is freed on the first positive grant");
                    }
                    (turns, items, bytes) = (turns + 1, items + released_items, bytes + released_bytes);
                }
                SnapshotRetirementStep::Complete => {
                    assert!(!owing, "{name}: Complete only after the last page byte was reported");
                    break;
                }
                SnapshotRetirementStep::Blocked => panic!("{name}: a bounded value never blocks"),
            }
        }
        assert_eq!((turns, items, bytes), (case["turns"].as_u64().unwrap() as usize, 1, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), "{name}");
        assert!(retirement.terminal_is_empty(), "{name}");
        drop(retirement);
    }
    eprintln!("[DEBUG] bounded value retirement frees on the first positive grant and reports exactly one page under 1/64/4096/10000-byte grants");
}
