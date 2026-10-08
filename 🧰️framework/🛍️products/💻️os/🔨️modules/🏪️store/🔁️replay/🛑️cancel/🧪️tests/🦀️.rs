use super::*;
use super::super::tests::DemoSnapshot;
use super::super::fixture_mutations::demo::{DemoMutation, SetN};

type Raw = EditReplay<DemoSnapshot, DemoMutation>;

fn original() -> Raw {
    let (mut owner, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| Raw::retain(ReplayOwnedState::terminal()));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    owner.original.finished = false;
    owner.original.state = Some(Arc::new(DemoSnapshot { n: Some(7) }));
    owner.original.schema = String::with_capacity(65536);
    owner.original.schema.push_str("demo/v1");
    owner.original.committed = Vec::with_capacity(8192);
    owner.original.committed.push(String::with_capacity(65536));
    owner.original.edit_inverse.try_push(DemoMutation::SetN(SetN { n: 7 })).unwrap();
    owner.original.drafts.insert(MutationId("original-draft".into()), protocol::InputReplacement::Input { schema: String::with_capacity(65536), payload: Vec::with_capacity(65536) });
    let mut target = Vec::with_capacity(8192);
    target.push(String::with_capacity(65536));
    owner.original.edit_messages.rows_mut().try_push(crate::os_spr::MutationMessage { level: semio_framework_diagnostic::Severity::Warning, code: "original-row".into(), message: String::with_capacity(65536), target, op_index: Some(0) }).unwrap();
    owner.with_retirement_factories(Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<DemoSnapshot>::default()), Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<DemoMutation>::default()))
}

pub(crate) fn close(owner: &mut dyn ErasedSnapshotRetirement) -> usize {
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let (demands, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| RetirementDemand {
            copy_bytes: owner.next_copy_byte_demand().unwrap(),
            capacity_bytes: owner.next_capacity_byte_demand(owner.next_copy_byte_demand().unwrap()).unwrap(),
            release_bytes: owner.next_release_byte_demand().unwrap(), depth: owner.next_depth_demand().unwrap(),
        });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demands.copy_bytes, maximum_capacity_bytes: demands.capacity_bytes, maximum_release_bytes: demands.release_bytes, maximum_depth: demands.depth };
        for axis in 0..5 {
            let mut below = grant;
            match axis {
                0 => below.maximum_items = 0,
                1 if demands.copy_bytes > 0 => below.maximum_copy_bytes -= 1,
                2 if demands.capacity_bytes > 0 => below.maximum_capacity_bytes -= 1,
                3 if demands.release_bytes > 0 => below.maximum_release_bytes -= 1,
                4 if demands.depth > 0 => below.maximum_depth -= 1,
                _ => continue,
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(below).unwrap());
            assert_eq!(step.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes);
        assert_eq!(heap.released_bytes, step.progress().released_bytes);
        turns += 1;
        assert!(turns < 100000);
    }
    turns
}

#[test]
fn raw_replay_custody_refusal_cancel_and_original_capacities_are_independently_granted() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(law["cancellation"]["retained"], 5);
    let mut owner = original();
    let state = Arc::as_ptr(owner.original.state.as_ref().unwrap());
    let schema = owner.original.schema.as_ptr();
    let committed = owner.original.committed.as_ptr();
    let (refused, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.finish());
    assert!(matches!(refused, Err(VcsError::HistoryReplaying)));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(Arc::as_ptr(owner.original.state.as_ref().unwrap()), state);
    assert_eq!(owner.original.schema.as_ptr(), schema);
    assert_eq!(owner.original.committed.as_ptr(), committed);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.cancel());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let turns = close(&mut owner);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] raw replay original five-family custody pauses at every zero/one-below lane and physically closes in {turns} granted turns");
}

#[test]
fn raw_replay_custody_finished_output_preserves_original_residue_and_zero_heap_transfer() {
    let mut owner = original();
    owner.original.finished = true;
    let state = Arc::as_ptr(owner.original.state.as_ref().unwrap());
    let committed = owner.original.committed.as_ptr();
    let schema = owner.original.schema.as_ptr();
    let (mut result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.finish().unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert!(owner.terminal_is_empty());
    assert_eq!(Arc::as_ptr(result.original.state.as_ref().unwrap()), state);
    assert_eq!(result.original.committed.as_ptr(), committed);
    assert_eq!(result.residual.as_ref().unwrap().original.schema.as_ptr(), schema);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    result.begin_retirement();
    let turns = close(&mut result);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(result));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] completed raw replay moves its original publication outputs and residual without allocation; both remain granted through {turns} terminal turns");
}

#[test]
fn raw_replay_custody_cancelled_ready_replay_refuses_finish_without_original_transfer() {
    let mut owner = original();
    owner.original.finished = true;
    owner.cancel();
    let state = Arc::as_ptr(owner.original.state.as_ref().unwrap());
    let schema = owner.original.schema.as_ptr();
    let (refused, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.finish());
    assert!(matches!(refused, Err(VcsError::HistoryReplaying)));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(Arc::as_ptr(owner.original.state.as_ref().unwrap()), state);
    assert_eq!(owner.original.schema.as_ptr(), schema);
    let turns = close(&mut owner);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] cancellation preserves the ready replay's original finish refusal and {turns} separately granted terminal turns");
}

#[test]
fn raw_replay_custody_prepared_owners_keep_independent_refusals_and_unused_capacities(){
 for refusal in [false,true]{
  let mut owner=original();let inverse=if refusal{Err(ValueError::new(ValueRefusalKind::InvalidValue,String::with_capacity(65536)))}else{let mut inverse=semio_framework_value::list::PagedList::default();inverse.try_push(DemoMutation::SetN(SetN{n:9})).unwrap();Ok(inverse)};
  let mut messages=Vec::with_capacity(8192);let mut target=Vec::with_capacity(8192);target.push(String::with_capacity(65536));messages.push(crate::os_spr::MutationMessage{level:semio_framework_diagnostic::Severity::Fatal,code:String::with_capacity(65536).into(),message:String::with_capacity(65536),target,op_index:Some(0)});
  let mut target=Vec::with_capacity(8192);target.push(String::with_capacity(65536));let apply_refusal=Some(crate::os_spr::MutationApplyError{code:String::with_capacity(65536),message:String::with_capacity(65536),target});
  owner.original.prepared_operation=Some(ArtifactReplayPrepared{next:Some(Arc::new(DemoSnapshot{n:Some(9)})),inverse,messages,apply_refusal,input_refusal:Some(String::with_capacity(65536)),foreign_steps:true});let prepared=owner.original.prepared_operation.as_ref().unwrap();let pointer=Arc::as_ptr(prepared.next.as_ref().unwrap());let rows=prepared.messages.as_ptr();let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(Arc::as_ptr(owner.original.prepared_operation.as_ref().unwrap().next.as_ref().unwrap()),pointer);assert_eq!(owner.original.prepared_operation.as_ref().unwrap().messages.as_ptr(),rows);
  let turns=close(&mut owner);let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Raw prepared original snapshot/inverse/refusal/messages/apply/input backing retained inverseRefused={refusal}; exact {turns} granted turns and terminalDrop=0");
 }
}
