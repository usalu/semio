//! 🤝️ Actual Host input and guest authorities conserve one total source across bounded loans.
use semio_framework_actor::retained_turn::{ActorGuestPhase, OriginalActorInput, OriginalActorTurnWork, RetainedTurnInput, RetainedTurnReceipt};
use semio_framework_actor::retained_context as guest_context;
use semio_framework_job::{StepBudget, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};

fn clock() -> Option<u64> { Some(1) }
fn grant(row: &serde_json::Value) -> RetainedCloneGrant {
 RetainedCloneGrant {
  maximum_items: row[0].as_u64().unwrap() as usize,
  maximum_copy_bytes: row[1].as_u64().unwrap() as usize,
  maximum_capacity_bytes: row[2].as_u64().unwrap() as usize,
  maximum_release_bytes: row[3].as_u64().unwrap() as usize,
  maximum_depth: row[4].as_u64().unwrap() as usize,
 }
}
fn input(law: &serde_json::Value) -> RetainedTurnInput {
 RetainedTurnInput {
  operation: law["operation"].as_u64().unwrap(), generation: law["generation"].as_u64().unwrap(),
  epoch: law["firstEpoch"].as_u64().unwrap(), work: serde_json::from_value(law["work"].clone()).unwrap(),
  grant: grant(&law["originalGrant"]),
 }
}
fn next(receipt: RetainedTurnReceipt) -> RetainedTurnInput {
 RetainedTurnInput { epoch: receipt.input.epoch + 1, grant: receipt.remaining,
  work: OriginalActorTurnWork { fuel: receipt.remaining_fuel, ..receipt.input.work }, ..receipt.input }
}
fn budget(input: RetainedTurnInput, policy: RetainedCloneGrant) -> StepBudget {
 StepBudget::new(1, input.work.absolute_deadline_us, policy)
}
fn copy_input(cx: &mut StepContext<'_>, source: &[u8]) {
 cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: source.len(), ..Default::default() }).unwrap();
 cx.consume_fuel(1);
 let mut target = [0; 7]; target.copy_from_slice(source);
 assert_eq!(target, [81; 7]);
}
fn release_input(cx: &mut StepContext<'_>, source: &mut Vec<u8>) {
 cx.consume_retained(RetainedCloneProgress { copied_items: 1, released_bytes: source.capacity(), ..Default::default() }).unwrap();
 cx.consume_fuel(1);
 drop(std::mem::take(source));
}
fn observe<T>(body: impl FnOnce() -> T) -> (T, (usize, usize)) {
 let (value, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(body);
 (value, (heap.requested_bytes, heap.released_bytes))
}

#[test]
fn original_public_actor_paired_admission_retains_exact_host_input_and_native_guest_births() {
 let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut incoming = input(&law);
 let mut source = Vec::with_capacity(97); source.extend_from_slice(&[81; 7]);
 let pointer = source.as_ptr();
 let (mut host, heap) = observe(|| OriginalActorInput::new(source, incoming).ok().unwrap());
 assert_eq!(heap, (0, 0));
 let mut guest = guest_context::OriginalActorContext::new((), incoming).ok().unwrap();
 let mut births = 0;
 for index in 0..6 {
  if index > 0 { host.receive_next_issued(incoming).unwrap(); }
  let mut policy = grant(&law[if index == 5 { "bodyPolicy" } else { "guestPolicy" }]);
  if index == 0 { policy.maximum_capacity_bytes = 0; }
  let checked = host.checkout_guest(policy).unwrap().unwrap();
  let policy = host.guest_policy(checked).unwrap();
  assert_eq!(checked.grant, incoming.grant);
  assert!(host.checkout_guest(policy).is_err());
  let mut external = RetainedCloneProgress::default();
  let (progress, phase, heap) = if index == 5 {
   guest.begin_dispatch(checked).unwrap();
   let (_, heap) = observe(|| {
    let mut cx = guest.borrow_context(checked, budget(checked, policy), &mut external, clock).unwrap();
    copy_input(&mut cx, host.original().unwrap());
   });
   (external, ActorGuestPhase::Dispatched, heap)
  } else {
   let (admission, heap) = observe(|| guest.admit_step(checked, policy, clock).unwrap());
   let phase = if admission.ready { guest.take_input().unwrap(); ActorGuestPhase::Ready } else { ActorGuestPhase::Pending };
   (admission.progress, phase, heap)
  };
  let work = checked.work.fuel - guest.remaining_work().unwrap().fuel;
  assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes));
  births += heap.0;
  let receipt = checked.return_original(progress, work).unwrap();
  host.receive_guest(receipt, phase).unwrap();
  assert!(host.receive_guest(receipt, phase).is_err());
  assert_eq!(host.original().unwrap().as_ptr(), pointer);
  let expected = &law["spent"][index];
  assert_eq!(progress.copied_items, expected["items"].as_u64().unwrap() as usize);
  assert_eq!(progress.copied_bytes, expected["copy"].as_u64().unwrap() as usize);
  assert_eq!(progress.retained_capacity_bytes, expected["capacity"].as_u64().unwrap() as usize);
  incoming = next(host.return_actor_turn().unwrap());
  assert!(host.return_actor_turn().is_err());
 }
 assert_eq!(incoming.grant.maximum_items, law["remaining"][0].as_u64().unwrap() as usize);
 assert_eq!(incoming.grant.maximum_copy_bytes, 25);
 assert_eq!(incoming.grant.maximum_capacity_bytes, 3864);
 assert_eq!(incoming.work.fuel, law["remainingFuel"].as_u64().unwrap());
 assert_eq!(births, 232);
 let mut source = host.take_dispatched_input().unwrap().unwrap();
 assert_eq!(source.as_ptr(), pointer);
 let close = grant(&law["closePolicy"]);
 host.receive_next_issued(incoming).unwrap();
 let checked = host.checkout_guest(close).unwrap().unwrap();
 let policy = host.guest_policy(checked).unwrap();
 guest.begin_dispatch(checked).unwrap();
 let mut external = RetainedCloneProgress::default();
 let (_, heap) = observe(|| {
  let mut cx = guest.borrow_context(checked, budget(checked, policy), &mut external, clock).unwrap();
  release_input(&mut cx, &mut source);
 });
 assert_eq!(heap, (0, 97));
 host.receive_guest(checked.return_original(external, checked.work.fuel - guest.remaining_work().unwrap().fuel).unwrap(), ActorGuestPhase::Dispatched).unwrap();
 incoming = next(host.return_actor_turn().unwrap());
 let mut released = 0;
 let mut turns = 0;
 while !guest.terminal_is_empty() {
  host.receive_next_issued(incoming).unwrap();
  let checked = host.checkout_guest(close).unwrap().unwrap();
  let policy = host.guest_policy(checked).unwrap();
  let (step, heap) = observe(|| guest.close_step(checked, policy).unwrap());
  assert_eq!(heap, (0, step.progress().released_bytes));
  released += heap.1;
  let work = checked.work.fuel - guest.remaining_work().unwrap().fuel;
  host.receive_guest(checked.return_original(step.progress(), work).unwrap(), ActorGuestPhase::Dispatched).unwrap();
  incoming = next(host.return_actor_turn().unwrap());
  turns += 1; assert!(turns < 16);
 }
 assert_eq!(released, births);
 assert_eq!(observe(|| drop(guest)).1, (0, 0));
 assert_eq!(observe(|| drop(host)).1, (0, 0));
}

#[test]
fn original_public_actor_guest_call_returns_actual_failed_body_receipt_and_paid_same_context_close() {
 let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut incoming = input(&law);
 let mut source = Vec::with_capacity(97); source.extend_from_slice(&[81; 7]);
 let pointer = source.as_ptr();
 let mut host = OriginalActorInput::new(source, incoming).ok().unwrap();
 let mut guest = guest_context::OriginalActorContextSlot::empty();
 let mut births = 0;
 for index in 0..7 {
  if index > 0 { host.receive_next_issued(incoming).unwrap(); }
  let mut policy = grant(&law[if index == 6 { "bodyPolicy" } else { "guestPolicy" }]);
  if index < 2 { policy.maximum_capacity_bytes = 0; }
  let checked = host.checkout_guest(policy).unwrap().unwrap();
  let policy = host.guest_policy(checked).unwrap();
  let (receipt, phase, heap) = if index == 6 {
   let mut external = RetainedCloneProgress::default();
   let (call, heap) = observe(|| guest.dispatch_step(checked, budget(checked, policy), &mut external, clock, |cx| {
    copy_input(cx, host.original().unwrap()); Err::<(), _>("original semantic refusal")
   }));
   assert!(matches!(call.result, Err(guest_context::OriginalActorCallError::Semantic("original semantic refusal"))));
   assert_eq!(call.receipt_spent, external);
   assert_eq!(call.work_units_spent, 1);
   (call.receipt().unwrap(), ActorGuestPhase::Dispatched, heap)
  } else {
   let (call, heap) = observe(|| guest.admit_step(checked, policy, clock));
   let receipt = call.receipt().unwrap();
   let phase = if call.result.unwrap().ready { ActorGuestPhase::Ready } else { ActorGuestPhase::Pending };
   (receipt, phase, heap)
  };
  assert_eq!(heap, (receipt.spent.retained_capacity_bytes, receipt.spent.released_bytes));
  births += heap.0;
  host.receive_guest(receipt, phase).unwrap();
  assert_eq!(host.original().unwrap().as_ptr(), pointer);
  incoming = next(host.return_actor_turn().unwrap());
 }
 assert_eq!(births, 232);
 assert_eq!(incoming.grant.maximum_items, law["slotRemainingItems"].as_u64().unwrap() as usize);
 assert_eq!(incoming.grant.maximum_copy_bytes, 25);
 assert_eq!(incoming.grant.maximum_capacity_bytes, 3864);
 let mut source = host.take_dispatched_input().unwrap().unwrap();
 let close = grant(&law["closePolicy"]);
 host.receive_next_issued(incoming).unwrap();
 let checked = host.checkout_guest(close).unwrap().unwrap();
 let policy = host.guest_policy(checked).unwrap();
 let mut external = RetainedCloneProgress::default();
 let (call, heap) = observe(|| guest.dispatch_step(checked, budget(checked, policy), &mut external, clock, |cx| {
  release_input(cx, &mut source); Ok::<_, ()>(())
 }));
 assert!(call.result.is_ok());
 assert_eq!(heap, (0, 97));
 host.receive_guest(call.receipt().unwrap(), ActorGuestPhase::Dispatched).unwrap();
 incoming = next(host.return_actor_turn().unwrap());
 let mut released = 0;
 let mut turns = 0;
 while !guest.terminal_is_empty() {
  host.receive_next_issued(incoming).unwrap();
  let checked = host.checkout_guest(close).unwrap().unwrap();
  let policy = host.guest_policy(checked).unwrap();
  let (call, heap) = observe(|| guest.close_step(checked, policy));
  let receipt = call.receipt().unwrap();
  assert!(call.result.is_ok());
  assert_eq!(heap, (0, receipt.spent.released_bytes));
  released += heap.1;
  host.receive_guest(receipt, ActorGuestPhase::Dispatched).unwrap();
  incoming = next(host.return_actor_turn().unwrap());
  turns += 1; assert!(turns < 16);
 }
 assert_eq!(released, births);
 assert_eq!(observe(|| drop(guest)).1, (0, 0));
 assert_eq!(observe(|| drop(host)).1, (0, 0));
}
