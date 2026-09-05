use super::*;

//#region 🧪️ByteCandidateOwnership
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).unwrap() }
fn source() -> Component { serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":[1,2,3]},"bindings":[]})).unwrap() }

#[test]
fn retained_component_copy_separates_allocation_child_work_and_exact_root_returns() {
    assert!(fixture()["separateAllocationAndRootTransfers"].as_bool().unwrap());
    let mut owner = UiComponentCopy::new(source());
    let before = serde_json::to_value(owner.source().unwrap()).unwrap();
    let mut allocated = 0;
    for _ in 0..1000 {
        let requested = owner.next_allocation_bytes().unwrap();
        let step = if requested != 0 {
            let refused = owner.reserve_next(requested - 1).unwrap();
            assert_eq!(refused.allocated_bytes, 0);
            let step = owner.reserve_next(requested).unwrap();
            assert_eq!(step.copied_bytes, 0);
            step
        } else { owner.advance(1, 0, 4096).unwrap() };
        allocated += step.allocated_bytes;
        assert!(step.allocated_bytes <= 32768 && step.copied_bytes <= 4096);
        if step.complete { break; }
    }
    assert!(owner.take_completed_source_with_grant(size_of::<Component>() - 1).is_none());
    assert!(owner.take_completed_candidate_with_grant(size_of::<Component>() - 1).is_none());
    assert_eq!(serde_json::to_value(owner.source().unwrap()).unwrap(), before);
    let source = owner.take_completed_source_with_grant(4096).unwrap();
    assert!(owner.source().is_none() && owner.candidate().is_some());
    let candidate = owner.take_completed_candidate_with_grant(4096).unwrap();
    assert_eq!(serde_json::to_value(&candidate).unwrap(), before);
    assert!(owner.terminal_is_empty());
    let mut returned = UiComponentCopy::new(source);
    close(&mut returned, 64);
    let mut returned = UiComponentCopy::new(candidate);
    close(&mut returned, 64);
    assert_eq!(allocated, 32768);
    eprintln!("[DEBUG] component-copy-transfers allocation=32768 child-work<=4096 exact-root-grant={} separately-returned=true", size_of::<Component>());
}
fn semantic(component: &Option<Component>) -> usize { match component { Some(Component::Surface(props)) => props.doc_schema.len() + props.doc.bytes.len(), None => 0, _ => unreachable!() } }
fn close(owner: &mut UiComponentCopy, grant: usize) -> usize {
    let mut released = 0;
    for _ in 0..200_000 {
        let before = owner.owned.byte_candidate.capacity();
        let step = owner.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        assert!(owner.owned.byte_candidate.capacity() <= before);
        released += step.released_bytes;
        if step.complete { assert!(owner.terminal_is_empty()); return released; }
    }
    panic!("byte candidate did not retire");
}

#[test]
fn retained_component_copy_byte_candidate_cancel_reports_every_initialized_byte() {
    let fixture = fixture();
    for frontier in fixture["byteCancelFrontiers"].as_array().unwrap() {
        for grant in fixture["closeGrants"].as_array().unwrap() {
            let mut owner = UiComponentCopy::new(source());
            for _ in 0..frontier.as_u64().unwrap() {
                let step = owner.advance(1, 32768, fixture["runtimeWorkGrant"].as_u64().unwrap() as usize).unwrap();
                assert!(step.copied_bytes <= 4096 && step.allocated_bytes <= 32768);
            }
            let expected = semantic(&owner.owned.source) + semantic(&owner.owned.candidate) + owner.owned.byte_candidate.len();
            let actual = close(&mut owner, grant.as_u64().unwrap() as usize);
            assert_eq!(actual, expected);
        }
    }
    eprintln!("[DEBUG] component-byte-cancel frontiers=10 grants=1,64,4096 initialized-prefix-exact=true");
}

#[test]
fn retained_component_copy_overallocated_backing_error_retains_exact_owner() {
    let fixture = fixture();
    let mut owner = UiComponentCopy::new(source());
    let factor = fixture["allocatorMultiplier"].as_u64().unwrap() as usize;
    let fault = reserve_byte_candidate(&mut owner.owned.byte_candidate, 32768, |candidate, request| candidate.try_reserve_exact(request * factor).map_err(|_| ())).unwrap_err();
    assert_eq!(fault.allocated_bytes, owner.owned.byte_candidate.capacity());
    assert!(fault.allocated_bytes >= 65536);
    assert_eq!(owner.owned.byte_candidate.len(), 0);
    assert_eq!(serde_json::to_value(owner.source().unwrap()).unwrap(), serde_json::to_value(source()).unwrap());
    assert_eq!(close(&mut owner, 1), 7);
    eprintln!("[DEBUG] component-byte-overcapacity actual={} initialized=0 retained-error=true exact-close=true", fault.allocated_bytes);
}
//#endregion 🧪️ByteCandidateOwnership
