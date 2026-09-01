use super::*;
use super::component_compare::ValueFrame;

//#region 🧪️TypedComparison
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture.json")).unwrap() }

#[test]
fn retained_component_compare_frame_storage_matches_exact_bounded_domains() {
    let fixture = fixture();
    let frame = &fixture["frame"];
    assert_eq!(UI_VALUE_ADMISSION_SLOTS, frame["logicalDepth"].as_u64().unwrap() as usize);
    assert_eq!(UI_VALUE_AGGREGATE_ITEMS, frame["pageCount"].as_u64().unwrap() as usize);
    assert_eq!(UI_TEXT_MAX_BYTES, frame["maximumTextBytes"].as_u64().unwrap() as usize);
    assert_eq!(std::mem::size_of::<ValueFrame>(), frame["bytes"].as_u64().unwrap() as usize);
    assert!(std::mem::size_of::<UiComponentComparisonCursor>() <= frame["maximumCursorBytes"].as_u64().unwrap() as usize);
    assert_eq!(ValueFrame::checked_page(UI_VALUE_NONE).unwrap(), u16::MAX);
    assert_eq!(ValueFrame::checked_page(UI_VALUE_AGGREGATE_ITEMS - 1).unwrap(), 255);
    for index in [UI_VALUE_AGGREGATE_ITEMS, usize::from(u16::MAX), usize::from(u16::MAX) + 1] { assert!(ValueFrame::checked_page(index).is_err()); }
    assert_eq!(ValueFrame::checked_position(2 * UI_TEXT_MAX_BYTES).unwrap(), 1024);
    let mut bytes = [0u8; 8];
    bytes[..2].copy_from_slice(&ValueFrame::checked_page(UI_VALUE_AGGREGATE_ITEMS - 1).unwrap().to_le_bytes());
    bytes[4..6].copy_from_slice(&ValueFrame::checked_position(2 * UI_TEXT_MAX_BYTES).unwrap().to_le_bytes());
    assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), frame["littleEndian"].as_str().unwrap());
    for position in [2 * UI_TEXT_MAX_BYTES + 1, usize::from(u16::MAX) + 1, usize::MAX] { assert!(ValueFrame::checked_position(position).is_err()); }
    eprintln!("[DEBUG] comparison-frame bytes={} depth={} cursor={}", std::mem::size_of::<ValueFrame>(), UI_VALUE_ADMISSION_SLOTS, std::mem::size_of::<UiComponentComparisonCursor>());
}
fn close(owner: &mut UiComponentCompare) {
    for _ in 0..500_000 {
        let step = owner.close_step(1, 64).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= 64);
        if step.complete { assert!(owner.terminal_is_empty()); return; }
    }
    panic!("typed comparison did not return exact roots");
}
fn compare(left: crate::Component, right: crate::Component, grant: usize) -> bool {
    let mut owner = UiComponentCompare::new(left, right);
    assert!(!owner.advance(0, grant).unwrap().progressed);
    assert!(!owner.advance(1, 0).unwrap().progressed);
    for _ in 0..500_000 {
        let step = owner.advance(1, grant).unwrap();
        assert!(step.compared_bytes <= grant);
        if step.complete { break; }
    }
    let result = owner.result().expect("retained comparison completes");
    close(&mut owner);
    result
}

#[test]
fn retained_component_compare_matches_all_native_variants_and_hostile_values() {
    let fixture = fixture();
    let components: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/🌳️typed/🧪️components.json")).unwrap();
    for grant in fixture["grants"].as_array().unwrap() {
        for row in components["cases"].as_array().unwrap() {
            let left: crate::Component = serde_json::from_value(row["component"].clone()).unwrap();
            let right: crate::Component = serde_json::from_value(row["component"].clone()).unwrap();
            assert!(compare(left, right, grant.as_u64().unwrap() as usize));
        }
        for row in fixture["cases"].as_array().unwrap() {
            let left: crate::Component = serde_json::from_value(row["left"].clone()).unwrap();
            let right: crate::Component = serde_json::from_value(row["right"].clone()).unwrap();
            let expected = serde_json::to_value(&left).unwrap() == serde_json::to_value(&right).unwrap();
            assert_eq!(expected, row["equal"].as_bool().unwrap());
            assert_eq!(compare(left, right, grant.as_u64().unwrap() as usize), expected);
        }
    }
    eprintln!("[DEBUG] retained-component-compare variants=18 hostile-values=7 byte-grants=1,64,4096 exact-serde=true");
}

#[test]
fn retained_component_compare_cancellation_and_arena_contention_keep_both_roots() {
    let fixture = fixture();
    let text = fixture["text"].as_str().unwrap().repeat(fixture["textRepeats"].as_u64().unwrap() as usize);
    let data = serde_json::json!({"type":"extension","extension":"nested","props":[text.clone(), {"key":[text]}]});
    for frontier in fixture["cancelFrontiers"].as_array().unwrap() {
        let mut owner = UiComponentCompare::new(serde_json::from_value(data.clone()).unwrap(), serde_json::from_value(data.clone()).unwrap());
        for _ in 0..frontier.as_u64().unwrap() { owner.advance(1, 1).unwrap(); }
        close(&mut owner);
    }
    let mut owner = UiComponentCompare::new(serde_json::from_value(data.clone()).unwrap(), serde_json::from_value(data).unwrap());
    let (entered_tx, entered_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let holder = std::thread::spawn(move || { let _arena = UI_VALUE_ARENA.lock().unwrap(); entered_tx.send(()).unwrap(); release_rx.recv().unwrap(); });
    entered_rx.recv().unwrap();
    let mut blocked = false;
    for _ in 0..100 { if !owner.advance(1, 4096).unwrap().progressed { blocked = true; break; } }
    release_tx.send(()).unwrap(); holder.join().unwrap();
    close(&mut owner);
    assert!(blocked);
    eprintln!("[DEBUG] retained-component-compare cancel-frontiers=7 contended-owner-preserved=true no-wait=true");
}
//#endregion 🧪️TypedComparison
