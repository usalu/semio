use super::*;

//#region 🧪️TypedComparison
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../⚖️compare/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()
}

#[test]
fn retained_component_compare_cursor_stays_bounded(){let fixture=fixture();assert!(std::mem::size_of::<UiComponentComparisonCursor>()<=fixture["frame"]["maximumCursorBytes"].as_u64().unwrap()as usize);}

fn close(owner: &mut UiComponentCompare) {
    for _ in 0..500_000 {
        let step = owner.close_step(1, 64).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= 64);
        if step.complete {
            assert!(owner.terminal_is_empty());
            return;
        }
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
        if step.complete {
            break;
        }
    }
    let result = owner.result().expect("retained comparison completes");
    close(&mut owner);
    result
}

#[test]
fn retained_component_compare_matches_all_native_variants_and_hostile_values() {
    let fixture = fixture();
    let components: serde_json::Value = serde_json::from_str(include_str!("../../../../♻️retirement/🌳️typed/🧩️components.json")).unwrap();
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
}

#[test]
fn retained_component_compare_cancellation_and_arena_contention_keep_both_roots() {
    let fixture = fixture();
    let text = fixture["text"].as_str().unwrap().repeat(fixture["textRepeats"].as_u64().unwrap() as usize);
    let data = serde_json::json!({"type":"extension","extension":"nested","props":[text.clone(), {"key":[text]}]});
    for frontier in fixture["cancelFrontiers"].as_array().unwrap() {
        let mut owner = UiComponentCompare::new(serde_json::from_value(data.clone()).unwrap(), serde_json::from_value(data.clone()).unwrap());
        for _ in 0..frontier.as_u64().unwrap() {
            owner.advance(1, 1).unwrap();
        }
        close(&mut owner);
    }
    let mut owner = UiComponentCompare::new(serde_json::from_value(data.clone()).unwrap(), serde_json::from_value(data).unwrap());
    let (entered_tx, entered_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let holder = std::thread::spawn(move || {
        semio_framework_ui_contract_values::test_observer::with_value_arena_held(||{
        entered_tx.send(()).unwrap();
        release_rx.recv().unwrap();
        });
    });
    entered_rx.recv().unwrap();
    let mut blocked = false;
    for _ in 0..100 {
        if !owner.advance(1, 4096).unwrap().progressed {
            blocked = true;
            break;
        }
    }
    release_tx.send(()).unwrap();
    holder.join().unwrap();
    close(&mut owner);
    assert!(blocked);
}
//#endregion 🧪️TypedComparison
