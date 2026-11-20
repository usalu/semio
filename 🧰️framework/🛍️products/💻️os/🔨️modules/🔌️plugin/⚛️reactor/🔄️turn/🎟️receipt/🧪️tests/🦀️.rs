//! 🧪️ Receipt collection preserves the original output backing and borrowed recipient.
use super::collect_original_publication_receipts;
use semio_framework_job::{close_original_effect_step, Generation, OperationId, OriginalJobFixtureCaller, StepBudget, StepContext};
use semio_framework_job_payload::JobPayloadAuthority;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedWorkPoll};
use semio_framework_value_testing_allocator as _;
use semio_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};
use semio_plugin_runtime_ops::ops_ingress::{OriginalPublicationFamily, OriginalPublicationReceiving, PluginExchangeOutput};

fn observe<T>(body: impl FnOnce() -> T) -> (T, (usize, usize)) {
    let (value, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(body);
    assert!(!heap.overflowed);
    (value, (heap.requested_bytes, heap.released_bytes))
}

fn amounts(value: &serde_json::Value) -> [usize; 4] {
    std::array::from_fn(|index| usize::try_from(value[index].as_u64().unwrap()).unwrap())
}

fn progress([copied_items, copied_bytes, retained_capacity_bytes, released_bytes]: [usize; 4]) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items, copied_bytes, retained_capacity_bytes, released_bytes }
}

fn grant([maximum_items, maximum_copy_bytes, maximum_capacity_bytes, maximum_release_bytes]: [usize; 4]) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items, maximum_copy_bytes, maximum_capacity_bytes, maximum_release_bytes, maximum_depth: 64 }
}

fn collect(output: &mut PluginExchangeOutput, cx: &mut StepContext<'_>, expected: RetainedCloneProgress, pointer: *const u8) {
    let (result, heap) = observe(|| collect_original_publication_receipts(output, cx));
    assert!(result.is_ok());
    assert_eq!(heap, (0, 0));
    assert_eq!(cx.retained_progress(), expected);
    assert_eq!(output.frames[0].as_ptr(), pointer);
}

#[test]
fn original_reactor_publication_receipts_preserve_all_five_tuples_and_same_recipient_once() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let issued = grant(amounts(&law["grant"]));
    let (operation, generation) = (OperationId(99210), Generation(1));
    let cancel = semio_framework_job::root_cancel_token();
    let mut actual_birth = None;
    let ((mut authority, birth), heap) = observe(|| {
        JobPayloadAuthority::admit(operation, generation, issued, &mut actual_birth).result.unwrap().unwrap()
    });
    assert_eq!(actual_birth, Some(heap.0));
    assert_eq!(heap, (birth.retained_capacity_bytes, 0));
    let mut sequence = 0;
    let initial = RetainedCloneProgress { copied_bytes: 23, ..Default::default() };
    let mut recipient = initial;
    let pointer = &mut recipient as *mut _;
    let mut cx = StepContext::with_payload_authority(
        operation, generation, StepBudget::new(9, u64::MAX, issued), &cancel,
        semio_framework_job::logical_now_us, &mut sequence, &mut recipient, &authority,
    ).unwrap();
    let mut output = PluginExchangeOutput { frames: vec![vec![83u8; 41]], ..Default::default() };
    let page = output.frames[0].as_ptr();
    let receipt = |index: usize| {
        let amounts = amounts(&law["receipts"][index]["spent"]);
        (grant(amounts), progress(amounts))
    };

    let (g, p) = receipt(0);
    output.constructor_receipt = Some((ArtifactStoreConstructorKind::Recipient, ArtifactStoreConstructorPhase::Preparation, g, p));
    let (g, p) = receipt(1);
    output.preparation_receipt = Some((operation.0, g, p));
    output.history_command_receipt = Some(receipt(2));
    output.tool_run_receipt = Some(receipt(3));
    let (g, p) = receipt(4);
    output.retirement_receipt = Some((operation.0, g, p));
    let spent = initial.checked_add(progress(amounts(&law["expected"]))).unwrap();
    collect(&mut output, &mut cx, spent, page);
    collect(&mut output, &mut cx, spent, page);

    let before_post = cx.retained_grant();
    let posted = progress([1, 7, 0, 0]);
    let source = [81u8; 7];
    let mut destination = [0u8; 7];
    let ((), heap) = observe(|| {
        destination.copy_from_slice(&source);
        cx.consume_retained(posted).unwrap();
    });
    assert_eq!(heap, (0, 0));
    assert_eq!(destination, source);
    let received = cx.retained_progress();
    collect(&mut output, &mut cx, received, page);
    output.tool_run_receipt = Some((before_post, posted));
    output.original_receiving = Some(OriginalPublicationReceiving {
        family: Some(OriginalPublicationFamily::Tool), before: spent, posted, grant: before_post,
        recipient: cx.original_recipient_identity(),
    });
    collect(&mut output, &mut cx, received, page);
    collect(&mut output, &mut cx, received, page);

    let remaining = cx.retained_grant();
    let bad = RetainedCloneGrant { maximum_depth: remaining.maximum_depth + 1, ..remaining };
    let actual = progress([1, 41, 0, 0]);
    output.preparation_receipt = Some((operation.0, bad, actual));
    let (result, heap) = observe(|| collect_original_publication_receipts(&mut output, &mut cx));
    let refusal = result.err().expect("forged original depth must refuse");
    assert_eq!(heap, (0, 0));
    assert_eq!(refusal.receipts.preparation, Some((operation, bad, actual)));
    assert_eq!(refusal.error.primary_value().unwrap().kind, semio_framework_value::ValueRefusalKind::InvariantViolated);
    assert_eq!(cx.retained_progress().copied_bytes, received.copied_bytes + 41);
    assert_eq!(output.frames[0].as_ptr(), page);
    drop(cx);
    assert_eq!(&mut recipient as *mut _, pointer);

    let stock = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 20, ..issued };
    let mut caller = OriginalJobFixtureCaller::admit(operation, generation, stock, || Some(1000), 10000, 10000);
    let demand = authority.retirement_demands();
    let (closed, heap) = observe(|| caller.bare_turn(issued, |cx| {
        close_original_effect_step(cx, demand, |effect| RetainedWorkPoll::Complete(authority.close_step(effect)))
            .map(|step| step.progress())
    }));
    assert_eq!(closed.unwrap().released_bytes, birth.retained_capacity_bytes);
    assert_eq!(heap, (0, birth.retained_capacity_bytes));
    assert!(authority.terminal_is_empty());
    drop(authority);
    caller.retire(stock);
}
