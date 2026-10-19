//! 🧪️ One actual finite original source owns candidate, publication, acknowledgement and typed candidate close.

use super::*;
use serde_json::Value;


fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
#[test]
fn original_frame_outcome_keeps_candidate_and_absent_payload_through_funded_descriptor_ack(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧊️wgpu/🧵️frame-job/📬️outcome/🔣️.json")).unwrap();
    let grant:RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
    assert_eq!(grant.maximum_copy_bytes,size_of::<Option<FrameDirectives>>());assert_eq!(grant.maximum_copy_bytes,size_of::<FrameDirectives>());
    let mut caller=crate::fixture_original_turn::FixtureTurnV1::with_identity(grant,OperationId(71),Generation(3));
    let mut job=FrameBuildJob::new(FrameBuildInputs{wheel_zoom_deadline_ms:1.0,now_ms:2.0});let mut receipts=Vec::new();
    for funded in [false,true]{
        let current=RetainedCloneGrant{maximum_items:usize::from(funded),..grant};
        let(result,heap)=observe(||caller.limited(current,|cx|job.step(cx).map(|loan|loan.is_none())));
        assert!(result.unwrap());assert_eq!(heap,(0,0));assert_eq!(job.complete.is_some(),funded);receipts.push(caller.receipt());
    }
    let pointer=job.complete.as_ref().unwrap()as*const FrameDirectives;
    let(result,heap)=observe(||caller.limited(RetainedCloneGrant{maximum_items:0,..grant},|cx|job.step(cx).map(|loan|loan.is_none())));
    assert!(result.unwrap());assert_eq!(heap,(0,0));assert_eq!(job.complete.as_ref().unwrap()as*const _,pointer);receipts.push(caller.receipt());
    let(mut descriptor,heap)=observe(||caller.turn(|cx|job.step(cx).unwrap().unwrap().into_descriptor()));
    assert_eq!(heap,(0,0));assert!(matches!(job.borrow_outcome(&descriptor).unwrap(),JobOutcomeView::Complete{state:None,output:None,..}));assert_eq!(job.complete.as_ref().unwrap()as*const _,pointer);receipts.push(caller.receipt());
    let(step,heap)=observe(||caller.turn(|cx|semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{depth:1,..Default::default()},|grant|Ok(descriptor.acknowledge(grant)))).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress().copied_items,1);receipts.push(caller.receipt());drop(descriptor);
    job.begin_close();let(close,heap)=observe(||caller.turn(|cx|job.close_step(cx)).unwrap());assert_eq!(heap,(0,0));assert_eq!(close.progress().copied_bytes,size_of::<FrameDirectives>());receipts.push(caller.receipt());assert!(job.terminal_is_empty());
    let(close,heap)=observe(||caller.turn(|cx|job.close_step(cx)).unwrap());assert!(matches!(close,InteractiveJobCloseStep::Complete{..}));assert_eq!(heap,(0,0));receipts.push(caller.receipt());
    for(actual,expected)in receipts.iter().zip(fixture["progress"].as_array().unwrap()){assert_eq!(*actual,serde_json::from_value::<RetainedCloneProgress>(expected.clone()).unwrap())}
    assert_eq!(observe(||drop(job)).1,(0,0));caller.finish();
    eprintln!("[DEBUG] same finite original frame71/3: candidate ABI copy1 + publication/ACK metadata + candidate close copy1, four items, no synthetic contexts, absent semantic payload, denied turns zero heap, terminal source paid");
}
