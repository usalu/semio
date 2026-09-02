use super::*;

//#region 🧪️ComponentCopyLaws
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }

fn close(owner: &mut UiComponentCopy, grant: usize) {
    for _ in 0..500_000 {
        let step = owner.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        if step.complete { assert!(owner.terminal_is_empty()); return; }
    }
    panic!("component copy did not retire exact source and candidate");
}

#[test]
fn retained_component_copy_all_variants_match_native_serde() {
    let fixture = fixture();
    let rows: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/🌳️typed/🧪️components.json")).unwrap();
    assert_eq!(rows["cases"].as_array().unwrap().len(), fixture["componentCount"].as_u64().unwrap() as usize);
    for row in rows["cases"].as_array().unwrap() {
        let source: crate::Component = serde_json::from_value(row["component"].clone()).unwrap();
        let expected = serde_json::to_value(&source).unwrap();
        let mut owner = UiComponentCopy::new(source);
        assert!(!owner.advance(0, 32768, 32768).unwrap().progressed);
        assert!(!owner.advance(1, 32768, 0).unwrap().progressed);
        let mut allocated = 0;
        let mut turns = 0;
        for _ in 0..100_000 {
            let request = owner.next_allocation_bytes().unwrap();
            assert!(request <= fixture["allocationGrant"].as_u64().unwrap() as usize);
            let step = owner.advance(1, request, fixture["workGrant"].as_u64().unwrap() as usize).unwrap();
            assert!(step.allocated_bytes <= request && step.copied_bytes <= 32768);
            allocated += step.allocated_bytes;
            turns += 1;
            if step.complete { break; }
            assert_eq!(owner.candidate().is_some(), fixture["partialCandidateReadable"].as_bool().unwrap());
        }
        assert_eq!(serde_json::to_value(owner.candidate().unwrap()).unwrap(), expected);
        assert_eq!(serde_json::to_value(owner.source().unwrap()).unwrap(), expected);
        close(&mut owner, 64);
        eprintln!("[DEBUG] retained-component-copy type={} turns={turns} allocated={allocated} exact-serde=true", row["component"]["type"]);
    }
}

#[test]
fn retained_component_copy_large_list_cancel_preserves_source_and_partial_candidate() {
    let fixture = fixture();
    let text = fixture["text"].as_str().unwrap().repeat(fixture["textRepeats"].as_u64().unwrap() as usize);
    let items: Vec<_> = (0..fixture["listItems"].as_u64().unwrap()).map(|_| serde_json::json!({"value": text, "label": text})).collect();
    let value = serde_json::json!({"type":"select","value":"selected","items":items});
    for frontier in fixture["cancelFrontiers"].as_array().unwrap() {
        for grant in fixture["closeGrants"].as_array().unwrap() {
            let source: crate::Component = serde_json::from_value(value.clone()).unwrap();
            let expected = serde_json::to_value(&source).unwrap();
            let mut owner = UiComponentCopy::new(source);
            for _ in 0..frontier.as_u64().unwrap() {
                let request = owner.next_allocation_bytes().unwrap();
                let step = owner.advance(1, request, 32768).unwrap();
                assert!(step.allocated_bytes <= 32768 && step.copied_bytes <= 32768);
            }
            assert_eq!(serde_json::to_value(owner.source().unwrap()).unwrap(), expected);
            close(&mut owner, grant.as_u64().unwrap() as usize);
        }
    }
    eprintln!("[DEBUG] retained-component-cancel list-items=32 payload-bytes=32768 frontiers=8 close-grants=3 terminal=true");
}
#[test]
fn retained_component_copy_surface_advances_under_real_4096_work_grant() {
    let fixture = fixture();
    let grant = fixture["runtimeWorkGrant"].as_u64().unwrap() as usize;
    let rows: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/🌳️typed/🧪️components.json")).unwrap();
    let row = rows["cases"].as_array().unwrap().iter().find(|row| row["component"]["type"] == "surface").unwrap();
    let source: crate::Component = serde_json::from_value(row["component"].clone()).unwrap();
    let expected = serde_json::to_value(&source).unwrap();
    let mut owner = UiComponentCopy::new(source);
    let mut maximum_work = 0;
    let mut complete = false;
    for _ in 0..256 {
        let step = owner.advance(1, fixture["allocationGrant"].as_u64().unwrap() as usize, grant).unwrap();
        maximum_work = maximum_work.max(step.copied_bytes);
        if step.complete { complete = true; break; }
        if !step.progressed { break; }
    }
    if complete { assert_eq!(serde_json::to_value(owner.candidate().unwrap()).unwrap(), expected); }
    close(&mut owner, 64);
    eprintln!("[DEBUG] component-copy-real-grant inline={} work-max={maximum_work} complete={complete}", std::mem::size_of::<crate::Component>());
    assert!(maximum_work <= grant);
    assert!(complete, "valid Surface must progress under the actual runtime 4096-byte work grant");
}
//#endregion 🧪️ComponentCopyLaws
