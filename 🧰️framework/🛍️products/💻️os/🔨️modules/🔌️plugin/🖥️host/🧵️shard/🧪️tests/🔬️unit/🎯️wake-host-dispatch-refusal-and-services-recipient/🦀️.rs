/// 🎯️ Wake, host dispatch refusal and services recipient: the wake of a mid-flight turn, the original host dispatch refusal and the issued shard active spans of the services recipient.

/// ⏯️ A `Wake` granted to a mid-flight owned turn is its resume (no events); a settled instance gets
/// the `Wake` itself, and every other event reaches the runtime unchanged so a mid-flight turn
/// refuses it loudly. Measured cause: the wgpu native kernel's first block2d poll overran its 100 ms
/// grant, the shard answered `MoreWork`, and the host's next ordinary event trapped the guest for good
/// (ticket 26/09/18 slice G7w).
#[test]
fn a_wake_to_a_mid_flight_turn_resumes_it_with_no_events() {
    assert!(turn_events(&Event::Wake, true).is_empty());
    assert!(matches!(turn_events(&Event::Wake, false), [Event::Wake]));
    let job = Event::JobCompleted { job: 7, result: RequestOutcome::Ok(Vec::new()) };
    assert!(matches!(turn_events(&job, true), [Event::JobCompleted { job: 7, .. }]));
    assert!(matches!(turn_events(&job, false), [Event::JobCompleted { job: 7, .. }]));
}

#[semio_framework_async_macros::async_test]
async fn original_host_dispatch_refusal_keeps_original_decision_and_typed_cause_without_heap() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🎟️grant/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut retained: semio_framework_actor::RetainedTurnInput = serde_json::from_value(law["input"].clone()).unwrap();
    retained.operation = 0;
    let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
    let decision = semio_framework_actor::Decision { run: vec![semio_framework_actor::TurnGrant { actor: ActorId(71), shard: semio_framework_actor::ShardId(0), budget, retained, loan: serde_json::from_value(law["loan"].clone()).unwrap(), envelopes: Vec::new() }], wake_at: Some(19) };
    let pointer = decision.run.as_ptr();
    let mut input = Some(decision);
    let mut dispatch = semio_framework_plugin_host_shard::grant::OriginalShardDispatch::default();
    let (result, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| dispatch.admit(&mut input));
    let error = result.unwrap_err();
    assert_eq!(error, semio_framework_actor::pack::PackError::InvalidRetainedTurn("operation identity is absent"));
    assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
    assert!(input.is_none());
    assert_eq!(dispatch.decision().unwrap().run.as_ptr(), pointer);
    assert_eq!(*dispatch.decision().unwrap().run[0].original_input(), retained);
    assert_eq!(dispatch.cursor(), law["dispatch"]["cursor"].as_u64().unwrap() as usize);
    assert_eq!(dispatch.refusal(), Some(&error));
    let mut bytes = Vec::with_capacity(256);
    bytes.extend_from_slice(&[11, 13]);
    let wire = bytes.as_ptr();
    let (packed, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_async::poll::resolve_ready(ShardFrame::pack_encode_grant(dispatch.next().unwrap(), semio_framework_plugin_host_shard::grant::ShardResourceBudget::from_original(&budget), &mut bytes)));
    assert_eq!(packed.unwrap_err(), error);
    assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
    assert_eq!(bytes.as_ptr(), wire);
    assert_eq!(bytes, [11, 13]);
    let mut next = Some(semio_framework_actor::Decision { run: Vec::new(), wake_at: Some(20) });
    let (refused, physical) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| dispatch.admit(&mut next));
    assert_eq!(refused.unwrap_err(), error);
    assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
    assert_eq!(next.as_ref().unwrap().wake_at, Some(20));
    assert!(dispatch.finish().is_none());
    assert_eq!(dispatch.decision().unwrap().run.as_ptr(), pointer);
}

#[test]
fn issued_shard_active_spans_shrink_the_original_services_recipient_before_envelope_return(){
 use semio_framework_os_services::ComputeRetainedRecipient;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/📦️batch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let input=serde_json::from_value(law["input"].clone()).unwrap();let resources=semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);let mut turn=IssuedShardTurn::new(resources,input,law["envelopes"].as_u64().unwrap()as usize).unwrap();let mut source=Vec::new();let mut returned=0;
 for(index,spent)in law["spent"].as_array().unwrap().iter().enumerate(){
  let mut original=turn.envelope_recipient(index).unwrap();let progress=serde_json::from_value(spent.clone()).unwrap();let(child,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{match index{0=>{source=Vec::with_capacity(128);source.extend_from_slice("文".as_bytes());},1=>{source.extend_from_slice(&[81;9]);drop(std::mem::take(&mut source));},_=>{}}ComputeRetainedRecipient::record_progress(&mut original,progress).unwrap();progress});assert_eq!((heap.requested_bytes,heap.released_bytes),(child.retained_capacity_bytes,child.released_bytes));assert_eq!(serde_json::to_value(ComputeRetainedRecipient::remaining_grant(&original).unwrap()).unwrap(),law["activeRemaining"][index]);assert_eq!(serde_json::to_value(original.parent_remaining_input().unwrap().grant).unwrap(),law["activeRemaining"][index]);
  if let Some(receipt)=original.finish().unwrap(){returned+=1;receipt.validate_for(input).unwrap();assert_eq!(serde_json::to_value(receipt.remaining).unwrap(),law["remaining"])}
 }
 assert_eq!(serde_json::to_value(returned).unwrap(),law["completionCount"]);assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(source)).1.released_bytes,0);
}

#[test]
fn issued_shard_closed_services_recipient_cannot_checkout_or_spend_again(){
 use semio_framework_os_services::ComputeRetainedRecipient;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/📦️batch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let input=serde_json::from_value(law["input"].clone()).unwrap();let resources=semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);let mut turn=IssuedShardTurn::new(resources,input,1).unwrap();let mut original=turn.envelope_recipient(0).unwrap();let receipt=original.finish().unwrap().unwrap();receipt.validate_for(input).unwrap();let remaining=receipt.remaining;
 let(refused,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(ComputeRetainedRecipient::remaining_grant(&original).is_err(),ComputeRetainedRecipient::record_progress(&mut original,Default::default()).is_err(),original.finish().is_err()));let expected=&law["closedRecipient"];assert_eq!(serde_json::to_value(refused).unwrap(),serde_json::json!([expected["grantRefused"],expected["recordRefused"],expected["finishRefused"]]));assert_eq!(serde_json::to_value([heap.requested_bytes,heap.released_bytes]).unwrap(),expected["heap"]);assert_eq!(serde_json::to_value(*original.original_input()==input).unwrap(),expected["sameOriginalIdentity"]);assert_eq!(receipt.remaining,remaining);assert_eq!(original.settled_progress(),Default::default());assert!(original.parent_remaining_input().is_err());
}
