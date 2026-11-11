use semio_framework_job::*;
const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};
use semio_framework_value_testing_allocator as _;
use semio_framework_job_payload::{JobPayloadStream,RetainedPayloadBuilder,RetainedWorkOutcome};
use semio_framework_job_testing_caller::bank_run;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
fn funded(cx:&mut StepContext<'_>,run:impl FnOnce(&mut StepContext<'_>)->RetainedWorkOutcome){let step=run(cx).result.unwrap();cx.consume_retained(step.progress()).unwrap();}
fn close_builder(builder:&mut RetainedPayloadBuilder){let mut bank=EffectBank::new(Wallet::from_profile(&serde_json::json!({"items":1<<16,"copyBytes":1<<28,"capacityBytes":1<<28,"releaseBytes":1<<28,"depth":1<<16,"fuel":1<<16})));while !builder.terminal_is_empty(){let d=builder.retirement_demands().unwrap();let issued=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let work=builder.retirement_work().unwrap();let(_,admitted)=bank_run(&mut bank,issued,work,|effect|semio_framework_value::RetainedWorkPoll::Complete(builder.close_step(effect)));assert_eq!(admitted,Some(true));}}

#[test]
fn retained_step_context_owner_turns_use_original_ledger_without_heap_birth() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let extent = StepContextOwner::birth_bytes();
    for row in fixture["cases"].as_array().unwrap() {
        let operation = OperationId(row["operation"].as_u64().unwrap());
        let generation = Generation(row["generation"].as_u64().unwrap());
        for (items, bytes) in [(0, extent), (1, 0), (1, extent - 1)] {
            let (denied, heap) = observe_heap(|| StepContextOwner::new(operation,generation,RetainedCloneGrant{maximum_items:items,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()}));
            assert!(denied.is_err());
            assert_eq!((heap.0, heap.1), (0, 0));
        }
        let (denied,heap)=observe_heap(||StepContextOwner::new(operation,generation,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:extent,maximum_depth:0,..Default::default()}));assert!(denied.is_err());assert_eq!((heap.0,heap.1),(0,0));
        let (owner, heap) = observe_heap(|| StepContextOwner::new(operation,generation,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:extent,maximum_depth:1,..Default::default()}));
        assert_eq!((heap.0, heap.1), (extent, 0));
        let (mut owner,receipt)=owner.unwrap();assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.0,heap.1));
        let cancel = root_cancel_token();
        let mut sequence = 0;
        for _ in 0..row["turns"].as_u64().unwrap() {
            let (_, heap) = observe_heap(|| {
                let mut actual_retained_progress=RetainedCloneProgress::default();let mut context = owner.context(StepBudget::new(1, u64::MAX,TEST_RETAINED_POLICY), cancel.clone(), default_now_us, &mut sequence,&mut actual_retained_progress).unwrap();
                assert_eq!(context.operation(), operation);
                assert_eq!(context.generation(), generation);
                assert_eq!(context.fuel_remaining(), 1);
                context.consume_fuel(1);
                assert!(context.fuel_exhausted());
                context.next_preview_sequence().unwrap();
                drop(context);
            });
            assert_eq!((heap.0, heap.1), (0, 0));
        }
        assert_eq!(sequence, row["turns"].as_u64().unwrap());
        for (items, bytes) in [(0, extent), (1, 0), (1, extent - 1)] {
            let (step, heap) = observe_heap(|| owner.close_step(RetainedCloneGrant{maximum_items:items,maximum_release_bytes:bytes,maximum_depth:64,..RetainedCloneGrant::default()}));
            assert_eq!(step, InteractiveJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
            assert_eq!((heap.0, heap.1), (0, 0));
            assert_eq!(owner.next_close_release_byte_demand().unwrap(), extent);
        }
        let (step, heap) = observe_heap(|| owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:extent,maximum_depth:64,..RetainedCloneGrant::default()}));
        assert_eq!(step, InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress{copied_items:1,released_bytes:extent,..RetainedCloneProgress::default()}});
        assert_eq!((heap.0, heap.1), (0, extent));
        assert!(owner.terminal_is_empty());
        assert_eq!(owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()}), InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress::default()});
        assert_eq!(observe_heap(|| drop(owner)).1.1, 0);
    }
}

#[test]
fn retained_step_context_owner_preserves_original_ledger_until_context_and_writer_return() {
    let extent = StepContextOwner::birth_bytes();
    let mut policy=TEST_RETAINED_POLICY;
    for kind in [0, 1] {
        let mut builder = RetainedPayloadBuilder::new(JobPayloadStream::Preview);
        let page = builder.staged_page_birth_demands().unwrap_or_default();
        policy = RetainedCloneGrant{maximum_copy_bytes:policy.maximum_copy_bytes.max(page.copy_bytes),maximum_capacity_bytes:policy.maximum_capacity_bytes.max(page.capacity_bytes),maximum_release_bytes:policy.maximum_release_bytes.max(page.release_bytes),maximum_depth:policy.maximum_depth.max(page.depth),..policy};
        let mut owner = StepContextOwner::new(OperationId(91004+kind),Generation(6),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:extent,maximum_depth:1,..Default::default()}).unwrap().0;
        let mut sequence = 0;
        let mut actual_retained_progress=RetainedCloneProgress::default();let context = owner.context(StepBudget::new(64, u64::MAX,policy), root_cancel_token(), default_now_us, &mut sequence,&mut actual_retained_progress).unwrap();
        let mut context = Some(context);
        if kind != 0 {
            let cx = context.as_mut().unwrap();
            for _ in 0..1024 { if builder.is_initialized() { break } funded(cx, |cx| builder.advance_initialization(cx)); }
            assert!(builder.is_initialized());
        }
        if kind != 0 {
            let cx = context.as_mut().unwrap();
            for _ in 0..16 { if builder.staged_page_len().is_some() { break } funded(cx, |cx| builder.begin_staged_page_funded(cx)); }
            assert!(builder.staged_page_len().is_some());
        }
        if kind != 0 { drop(context.take()); }
        let (step, heap) = observe_heap(|| owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:extent,maximum_depth:64,..RetainedCloneGrant::default()}));
        assert_eq!(step, InteractiveJobCloseStep::Blocked);
        assert_eq!((heap.0, heap.1), (0, 0));
        assert_eq!(owner.next_close_release_byte_demand().unwrap(), extent);
        let mut refused_sequence = 0;
        let mut actual_retained_progress=RetainedCloneProgress::default();assert!(owner.context(StepBudget::new(1, u64::MAX,TEST_RETAINED_POLICY), root_cancel_token(), default_now_us, &mut refused_sequence,&mut actual_retained_progress).is_none());
        if kind == 0 { drop(context.take()); }
        close_builder(&mut builder);
        let (step, heap) = observe_heap(|| owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:extent,maximum_depth:64,..RetainedCloneGrant::default()}));
        assert_eq!(step, InteractiveJobCloseStep::Complete {progress:RetainedCloneProgress{copied_items:1,released_bytes:extent,..RetainedCloneProgress::default()}});
        assert_eq!((heap.0, heap.1), (0, extent));
        assert!(owner.terminal_is_empty());
    }
}
fn observe_heap<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
