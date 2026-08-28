use super::*;

//#region 🧪️BindingCopyLaws
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture.json")).unwrap() }

fn source(count: usize) -> crate::UiNodeBindings {
    let scope = fixture()["scope"].as_str().unwrap().to_owned();
    let mut result = crate::UiNodeBindings::default();
    for index in 0..count { result.try_push(ActionBinding { action: ActionId::try_v1(&scope, &format!("action-{index}")).unwrap(), ..Default::default() }).unwrap(); }
    result
}

fn close(owner: &mut UiBindingsCopy, grant: usize) {
    for _ in 0..100_000 {
        let step = owner.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        if step.complete { assert!(owner.terminal_is_empty()); return; }
    }
    panic!("binding copy did not retire exact owners");
}

#[test]
fn retained_binding_copy_partial_returns_keep_the_other_exact_root() {
    for order in fixture()["returnOrders"].as_array().unwrap() {
        let mut owner = UiBindingsCopy::new(source(32));
        let expected = serde_json::to_value(owner.source().unwrap()).unwrap();
        while !owner.advance(1, 4096, 4096).unwrap().complete {}
        let source_bytes = owner.source_allocated_bytes();
        let candidate_bytes = owner.candidate_allocated_bytes();
        assert!(owner.take_completed_source_with_grant(size_of::<crate::UiNodeBindings>() - 1).is_none());
        assert!(owner.take_completed_candidate_with_grant(size_of::<crate::UiNodeBindings>() - 1).is_none());
        assert_eq!(owner.source_allocated_bytes(), source_bytes);
        assert_eq!(owner.candidate_allocated_bytes(), candidate_bytes);
        for name in order.as_array().unwrap() {
            let returned = match name.as_str().unwrap() {
                "source" => owner.take_completed_source_with_grant(4096).unwrap(),
                "candidate" => owner.take_completed_candidate_with_grant(4096).unwrap(),
                _ => unreachable!(),
            };
            assert_eq!(serde_json::to_value(&returned).unwrap(), expected);
            assert!(owner.take_completed().is_none());
            let mut returned = UiBindingsCopy::new(returned);
            close(&mut returned, 64);
        }
        assert!(owner.terminal_is_empty());
    }
    eprintln!("[DEBUG] binding-copy-return orders=2 refused-grant-preserves=true exact-other-root=true");
}

#[test]
fn retained_binding_copy_separates_allocation_clone_and_placement() {
    let fixture = fixture();
    let source = source(fixture["count"].as_u64().unwrap() as usize);
    let expected = serde_json::to_value(&source).unwrap();
    let mut owner = UiBindingsCopy::new(source);
    let grant = fixture["grantBytes"].as_u64().unwrap() as usize;
    assert!(!owner.advance(0, grant, grant).unwrap().progressed);
    let mut copied = 0;
    let mut placed = 0;
    for _ in 0..1000 {
        let before = owner.candidate_allocated_bytes();
        let step = owner.advance(1, grant, grant).unwrap();
        assert_eq!(owner.candidate_allocated_bytes() - before, step.allocated_bytes);
        assert!(step.allocated_bytes + step.copied_bytes + step.placed_bytes <= grant);
        copied += step.copied_bytes;
        placed += step.placed_bytes;
        if step.complete { break; }
    }
    assert_eq!(copied, 32 * std::mem::size_of::<ActionBinding>());
    assert_eq!(placed, copied);
    assert_eq!(serde_json::to_value(owner.candidate().unwrap()).unwrap(), expected);
    let (source, candidate) = owner.take_completed().unwrap();
    assert_eq!(serde_json::to_value(&source).unwrap(), expected);
    let mut source = UiBindingsCopy::new(source);
    let mut candidate = UiBindingsCopy::new(candidate);
    close(&mut source, 64);
    close(&mut candidate, 64);
    assert!(owner.terminal_is_empty());
    eprintln!("[DEBUG] retained-binding-copy count=32 copied={copied} placed={placed} maximum-turn={grant} ordered=true");
}

#[test]
fn retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases() {
    let fixture = fixture();
    for frontier in fixture["cancelFrontiers"].as_array().unwrap() {
        for grant in fixture["closeGrants"].as_array().unwrap() {
            let grant = grant.as_u64().unwrap() as usize;
            let mut source = source(3);
            let mut builder = UiListBuilder::try_new().unwrap();
            builder.push(UiValue::Text(UiText::try_from_str(&"é".repeat(256)).unwrap())).unwrap();
            let value = UiValue::List(builder.finish());
            let mut reader = UiValueRetirement::new(value.credited_clone().unwrap());
            source.get_mut(0).unwrap().args = Some(value);
            let mut owner = UiBindingsCopy::new(source);
            for _ in 0..frontier.as_u64().unwrap() { owner.advance(1, 4096, 4096).unwrap(); }
            close(&mut owner, grant);
            assert!(!reader.terminal_is_empty());
            for _ in 0..2000 { if reader.close_step(1, grant).unwrap().complete { break; } }
            assert!(reader.terminal_is_empty());
        }
    }
    let mut source = source(1);
    let mut builder = UiListBuilder::try_new().unwrap();
    builder.push(UiValue::Bool(true)).unwrap();
    source.get_mut(0).unwrap().args = Some(UiValue::List(builder.finish()));
    let mut owner = UiBindingsCopy::new(source);
    while owner.next_allocation_bytes().unwrap() != 0 { owner.advance(1, 4096, 4096).unwrap(); }
    assert!(!owner.advance(1, 4096, fixture["smallGrantBytes"].as_u64().unwrap() as usize).unwrap().progressed);
    let guard = UI_VALUE_ARENA.lock().unwrap();
    let step = owner.advance(1, 4096, 4096).unwrap();
    assert_eq!(step.progressed, fixture["contendedCopyProgresses"].as_bool().unwrap());
    drop(guard);
    assert!(owner.advance(1, 4096, 4096).unwrap().progressed);
    close(&mut owner, 64);
    eprintln!("[DEBUG] retained-binding-cancel frontiers=13 close-grants=3 shared-reader=true arena-contention=blocked");
}
//#endregion 🧪️BindingCopyLaws
