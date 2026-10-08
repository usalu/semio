use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

#[test]
fn retained_step_context_owner_turns_use_original_ledger_without_heap_birth() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let extent = StepContextOwner::birth_bytes();
    for row in fixture["cases"].as_array().unwrap() {
        let operation = OperationId(row["operation"].as_u64().unwrap());
        let generation = Generation(row["generation"].as_u64().unwrap());
        for (items, bytes) in [(0, extent), (1, 0), (1, extent - 1)] {
            let (denied, heap) = observe(|| StepContextOwner::new(operation, generation, items, bytes));
            assert!(denied.is_none());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (owner, heap) = observe(|| StepContextOwner::new(operation, generation, 1, extent));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (extent, 0));
        let mut owner = owner.unwrap();
        let cancel = root_cancel_token();
        let mut sequence = 0;
        for _ in 0..row["turns"].as_u64().unwrap() {
            let (_, heap) = observe(|| {
                let mut context = owner.context(StepBudget::new(1, u64::MAX), cancel.clone(), default_now_us, &mut sequence).unwrap();
                assert_eq!(context.operation(), operation);
                assert_eq!(context.generation(), generation);
                assert_eq!(context.fuel_remaining(), 1);
                context.consume_fuel(1);
                assert!(context.fuel_exhausted());
                context.next_preview_sequence().unwrap();
                drop(context);
            });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        assert_eq!(sequence, row["turns"].as_u64().unwrap());
        for (items, bytes) in [(0, extent), (1, 0), (1, extent - 1)] {
            let (step, heap) = observe(|| owner.close_step(RetainedCloneGrant{maximum_items:items,maximum_release_bytes:bytes,maximum_depth:64,..RetainedCloneGrant::default()}));
            assert_eq!(step, InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(owner.next_close_release_byte_demand().unwrap(), extent);
        }
        let (step, heap) = observe(|| owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:extent,maximum_depth:64,..RetainedCloneGrant::default()}));
        assert_eq!(step, InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress{copied_items:1,released_bytes:extent,..RetainedCloneProgress::default()}});
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
        assert!(owner.terminal_is_empty());
        assert_eq!(owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()}), InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()});
        assert_eq!(observe(|| drop(owner)).1.released_bytes, 0);
        println!("[DEBUG] retained context original ledger op={} turns={} exact birth/release={extent} turn heap=0/0", operation.0, sequence);
    }
}

#[test]
fn retained_step_context_owner_preserves_original_ledger_until_context_and_writer_return() {
    let extent = StepContextOwner::birth_bytes();
    for kind in [0, 1, 2] {
        let mut owner = StepContextOwner::new(OperationId(91004 + kind), Generation(6), 1, extent).unwrap();
        let mut sequence = 0;
        let context = owner.context(StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut sequence).unwrap();
        let identity = Arc::as_ptr(&context.payload_ledger);
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::Preview);
        let mut context = Some(context);
        if kind != 0 {
            let mut page = writer.admit_page(context.as_mut().unwrap()).unwrap();
            page.write("original 雪\0".as_bytes()).unwrap();
            page.commit();
        }
        let mut payload = if kind == 2 { Some(std::mem::replace(&mut writer, RetainedJobPayloadWriter::new(JobPayloadStream::Preview)).finish().unwrap()) } else { None };
        if kind != 0 {
            let held = payload.as_ref().or_else(|| writer.payload.as_ref());
            assert_eq!(held.and_then(|payload| payload.ledger.as_ref()).map(Arc::as_ptr), Some(identity));
            drop(context.take());
        }
        let (step, heap) = observe(|| owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:extent,maximum_depth:64,..RetainedCloneGrant::default()}));
        assert_eq!(step, InteractiveJobCloseStep::Blocked);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(owner.next_close_release_byte_demand().unwrap(), extent);
        let mut refused_sequence = 0;
        assert!(owner.context(StepBudget::new(1, u64::MAX), root_cancel_token(), default_now_us, &mut refused_sequence).is_none());
        if kind == 0 { drop(context.take()); }
        else if let Some(payload) = payload.as_mut() {
            assert_eq!(payload.page(0).unwrap(), "original 雪\0".as_bytes());
            while !payload.terminal_is_empty() { payload.close_step(1, payload.next_close_byte_demand()); }
        }
        while !writer.terminal_is_empty() { writer.close_step(1, writer.next_close_byte_demand()); }
        let (step, heap) = observe(|| owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:extent,maximum_depth:64,..RetainedCloneGrant::default()}));
        assert_eq!(step, InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress{copied_items:1,released_bytes:extent,..RetainedCloneProgress::default()}});
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
        assert!(owner.terminal_is_empty());
        println!("[DEBUG] retained context ledger alias kind={kind} blocked then exact frame release={extent}");
    }
}
