//! 🧪️ Original diagnostic ownership moves without heap copies and retires through exact physical grants.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread;

fn observed(step: impl FnOnce() -> Result<RetainedCloneProgress, ValueError>, grant: RetainedCloneGrant) -> RetainedCloneProgress {
    let (result, heap) = observe_heap_allocations_on_this_thread(step);
    let progress = result.unwrap();
    assert!(progress.fits(grant));
    assert!(progress.copied_bytes + progress.retained_capacity_bytes <= 4096);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
    progress
}

fn drain(cursor: &mut OperationMessageDrain, ledger: &mut ReplayMessageAccumulator, limit: usize) -> usize {
    for turn in 0..limit {
        if cursor.is_finished() { return turn; }
        let ((copy, capacity, release), heap) = observe_heap_allocations_on_this_thread(|| (cursor.next_copy_byte_demand(ledger), cursor.next_capacity_byte_demand(ledger).unwrap(), cursor.next_release_byte_demand()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(copy + capacity <= 4096);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
        let paused = RetainedCloneGrant { maximum_depth: 0, ..grant };
        assert_eq!(observed(|| cursor.step(ledger, paused), paused), RetainedCloneProgress::default());
        assert_eq!(observed(|| cursor.step(ledger, RetainedCloneGrant::default()), RetainedCloneGrant::default()), RetainedCloneProgress::default());
        for short in [RetainedCloneGrant { maximum_copy_bytes: copy.saturating_sub(1), ..grant }, RetainedCloneGrant { maximum_capacity_bytes: capacity.saturating_sub(1), ..grant }, RetainedCloneGrant { maximum_release_bytes: release.saturating_sub(1), ..grant }] {
            if short != grant { assert_eq!(observed(|| cursor.step(ledger, short), short), RetainedCloneProgress::default()); }
        }
        observed(|| cursor.step(ledger, grant), grant);
    }
    limit
}

fn close(ledger: &mut ReplayMessageAccumulator) {
    for turn in 0..100000 {
        if ledger.terminal_is_empty() { println!("[DEBUG] original diagnostic native pages/strings/targets exact three-axis terminal turns={turn}"); return; }
        let ((copy, release), heap) = observe_heap_allocations_on_this_thread(|| (ledger.next_close_copy_byte_demand(), ledger.next_close_release_byte_demand().unwrap()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: 0, maximum_release_bytes: release, maximum_depth: 64 };
        let paused = RetainedCloneGrant { maximum_depth: 0, ..grant };
        assert_eq!(observed(|| ledger.close_step(paused), paused), RetainedCloneProgress::default());
        assert_eq!(observed(|| ledger.close_step(RetainedCloneGrant::default()), RetainedCloneGrant::default()), RetainedCloneProgress::default());
        if release > 0 { let short = RetainedCloneGrant { maximum_release_bytes: release - 1, ..grant }; assert_eq!(observed(|| ledger.close_step(short), short), RetainedCloneProgress::default()); }
        observed(|| ledger.close_step(grant), grant);
    }
    panic!("original diagnostic owner did not reach terminal");
}

#[test]
fn replay_message_accumulator_native_original_rows_paid_vec_scaffold_and_partial_transfer() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let count = law["nativeRows"].as_u64().unwrap() as usize;
    for cancel_at in std::iter::once(None).chain(law["cancelAt"].as_array().unwrap().iter().map(|value| Some(value.as_u64().unwrap() as usize))) {
        let mut messages = Vec::with_capacity(count);
        for index in 0..count {
            let mut message = MutationMessage::fatal("mutation.invariant", format!("original-{index}"));
            message.target = vec![format!("id-{index}"), String::new()];
            if index == 0 { message.message = "🧬\0".repeat(2000); }
            messages.push(message);
        }
        let pointers: Vec<_> = messages.iter().map(|message| (message.message.as_ptr(), message.code.0.as_ptr(), message.target.as_ptr())).collect();
        let scaffold = messages.capacity() * size_of::<MutationMessage>();
        assert!(scaffold > 4096);
        let outcome = MutationReplayOutcome { mutation_id: crate::MutationId("original-mutation".into()), edit_id: "original-edit".into(), op_index: 7, worst: None, messages, superseded: false, withdrawn: false };
        let ((mut cursor, mut ledger), heap) = observe_heap_allocations_on_this_thread(|| (OperationMessageDrain::new(outcome), ReplayMessageAccumulator::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(size_of::<OperationMessageDrain>() <= 4096 && size_of::<ReplayMessageAccumulator>() <= 4096);
        let _turns = drain(&mut cursor, &mut ledger, cancel_at.unwrap_or(100000));
        if cancel_at.is_some() && !cursor.is_finished() {
            let ((outcome, mut pending), heap) = observe_heap_allocations_on_this_thread(|| cursor.into_parts());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            while pending.is_some() {
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: if ledger.rows.has_reserved_slot() { size_of::<MutationMessage>() } else { 0 }, maximum_capacity_bytes: ledger.next_capacity_byte_demand().unwrap(), maximum_release_bytes: 0, maximum_depth: 64 };
                observed(|| ledger.append(&mut pending, grant), grant);
            }
            let (next, heap) = observe_heap_allocations_on_this_thread(|| OperationMessageDrain::new(outcome));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            cursor = next;
            assert!(drain(&mut cursor, &mut ledger, 100000) < 100000);
        } else {
            assert!(cursor.is_finished());
            assert_eq!(ledger.rows().len(), count);
            for (index, message) in ledger.rows().iter().enumerate() {
                assert_eq!((message.message.as_ptr(), message.code.0.as_ptr(), message.target.as_ptr()), pointers[index]);
                assert_eq!(message.op_index, Some(7));
            }
        }
        let outcome = cursor.take().unwrap();
        assert_eq!(outcome.worst, Some(semio_framework_diagnostic::Severity::Fatal));
        assert_eq!(outcome.messages.capacity(), 0);
        let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(cursor));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] original message transfer cancel={cancel_at:?} rows={} exact original empty Vec release={scaffold}; body4096 and separately admitted structural release", ledger.rows().len());
        close(&mut ledger);
        let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(ledger));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    }
}
