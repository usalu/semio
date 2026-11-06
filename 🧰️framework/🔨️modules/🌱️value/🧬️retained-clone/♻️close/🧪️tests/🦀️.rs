//! ♻️ Actual clone close-frame private custody preserves original terminal release laws.
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::observe_retirement_allocations;
#[test]
fn clone_close_keeps_compound_original_failure_and_work(){
 struct Owner{turn:u8}
 impl crate::ErasedSnapshotRetirement for Owner{
  fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
  fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if self.turn==0{self.turn=1;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};let failure=crate::OriginalRetirementFailure::from_causes(Some(ValueError::literal(ValueRefusalKind::Canceled,"original clone close failed")),Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original work post refused")),Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original physical receipt refused"))).with_retained_progress(progress);return crate::RetainedWorkOutcome{result:Err(failure),work:2}}self.turn=2;crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:1}}
  fn terminal_is_empty(&self)->bool{self.turn==2}
  fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
  fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
  fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
  fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.turn!=2))}
 }
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🪢️compound-failure/🔣️.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1024,maximum_capacity_bytes:1024,maximum_release_bytes:1024,maximum_depth:16};let mut close=RetainedCloneClose::from_admitted_original(Box::new(Owner{turn:0}));let(turn,heap)=observe_retirement_allocations(||close.step_granted(grant));assert_eq!(heap,(0,0));assert_eq!(turn.work,law["work"].as_u64().unwrap());let failure=turn.result.unwrap_err();assert_eq!(failure.primary().unwrap().message,law["primary"].as_str().unwrap());assert_eq!(failure.work_refusal().unwrap().message,law["workRefusal"].as_str().unwrap());assert_eq!(failure.receipt_refusal().unwrap().message,law["receiptRefusal"].as_str().unwrap());assert_eq!(failure.retained_progress().copied_items,1);assert_eq!(failure.primary().unwrap().message.as_ptr(),"original clone close failed".as_ptr());assert_eq!(failure.work_refusal().unwrap().message.as_ptr(),"original work post refused".as_ptr());assert_eq!(failure.receipt_refusal().unwrap().message.as_ptr(),"original physical receipt refused".as_ptr());assert!(!close.is_empty());let actual=serde_json::json!({"primary":failure.primary().unwrap().message,"workRefusal":failure.work_refusal().unwrap().message,"receiptRefusal":failure.receipt_refusal().unwrap().message,"work":turn.work});assert_eq!(actual,law);while !close.is_empty(){close.step_granted(grant).result.unwrap();}
}
#[test]
fn cold_clone_terminal_frame_denies_partial_funding_and_reports_actual_same_turn_release() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🗺️ordered-map/🧪️tests/🧫️fixtures/📏️close/🔣️.json")).unwrap();
    let admission = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    let mut mismatches = 0;
    for row in fixture["cases"].as_array().unwrap() {
        let mut text = String::with_capacity(row["capacity"].as_u64().unwrap() as usize);
        text.push_str(row["text"].as_str().unwrap());
        let mut close = RetainedCloneClose::default();
        let mut text=Some(text);
        let birth=crate::owned_retirement_birth_bytes::<String>();
        assert!(birth<=admission);
        close.begin_granted(&mut text,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth,maximum_depth:1,..Default::default()}).result.unwrap();
        assert!(text.is_none());
        for _ in 0..10_000 {
            if close.retirement.as_ref().unwrap().terminal_is_empty() { break; }
            let capacity=close.next_capacity_byte_demand(7).unwrap();
            let release=close.next_release_byte_demand().unwrap();
            assert!(capacity<=admission&&release<=admission);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:close.next_depth_demand().unwrap()};
            let (step,events)=crate::observe_retirement_allocations(||close.step_granted(grant).result.unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(events,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        }
        assert!(close.retirement.as_ref().unwrap().terminal_is_empty());
        let pointer = close.retirement.as_ref().unwrap().as_ref() as *const dyn crate::ErasedSnapshotRetirement;
        let (physical, query_events) = crate::observe_retirement_allocations(|| close.next_release_byte_demand().unwrap());
        assert_eq!(query_events, (0, 0));
        assert!(physical > 0 && physical <= admission);
        for (items, bytes) in [(0, physical), (1, physical - 1)] {
            let (step, events) = crate::observe_retirement_allocations(|| close.step_granted(RetainedCloneGrant{maximum_items:items,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()}).result.unwrap());
            let retained = close.retirement.as_ref().is_some_and(|owner| std::ptr::addr_eq(pointer, owner.as_ref() as *const dyn crate::ErasedSnapshotRetirement));
            if step != (RetainedCloneStep::Progress(Default::default())) || events != (0, 0) || !retained {
                mismatches += 1;
            }
            if close.is_empty() { break; }
        }
        if !close.is_empty() {
            let (step, events) = crate::observe_retirement_allocations(|| close.step_granted(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:physical,maximum_depth:1,..Default::default()}).result.unwrap());
            if step != (RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:1,released_bytes:physical,..Default::default()})) || events != (0, physical) { mismatches += 1; }
        }
        assert!(close.is_empty());
        assert_eq!(close.step_granted(Default::default()).result.unwrap(), RetainedCloneStep::Complete(Default::default()));
    }
    assert_eq!(mismatches, 0, "cold terminal retirement Box requires its complete same-turn physical grant");
}



#[path="../🎟️demand/🧪️tests/🧫️owner/🦀️.rs"]
mod refusing_owner;
use refusing_owner::RefusingCopyDemand;

#[test]
fn retained_clone_terminal_ticket_quotes_and_funds_the_original_header(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🪜️close-demands/🔣️.json")).unwrap();let law=&law["terminalTicket"];
 let(mut close,heap)=observe_retirement_allocations(||RetainedCloneClose::from_admitted_original(Box::new(RefusingCopyDemand{owner:None})));let original=close.retirement.as_ref().unwrap().as_ref()as*const dyn crate::ErasedSnapshotRetirement as*const ();
 let expected=law["headerWidths"].as_array().unwrap().iter().find(|row|row["wordBytes"].as_u64().unwrap()as usize==size_of::<usize>()).unwrap()["copyBytes"].as_u64().unwrap()as usize;let bytes=heap.0;assert_eq!(heap.1,0);
 let((copy,capacity,release,depth),heap)=observe_retirement_allocations(||(close.next_copy_byte_demand().unwrap(),close.next_capacity_byte_demand(expected).unwrap(),close.next_release_byte_demand().unwrap(),close.next_depth_demand().unwrap()));assert_eq!(heap,(0,0));assert_eq!(copy,expected);assert_eq!(capacity,law["capacityBytes"].as_u64().unwrap()as usize);assert_eq!(release,bytes);assert_eq!(depth,law["depth"].as_u64().unwrap()as usize);
 let grant=RetainedCloneGrant{maximum_items:law["items"].as_u64().unwrap()as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
 for axis in law["refused"].as_array().unwrap(){let axis=axis.as_str().unwrap();let denied=match axis{"items"=>RetainedCloneGrant{maximum_items:0,..grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:copy-1,..grant},"release"=>RetainedCloneGrant{maximum_release_bytes:release-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};let(result,heap)=observe_retirement_allocations(||close.step_granted(denied));assert_eq!(heap,(0,0));if axis=="depth"{assert_eq!(result.result.unwrap_err().primary().unwrap().kind,crate::ValueRefusalKind::DepthLimit);}else{assert_eq!(result.result.unwrap().progress(),Default::default());}assert!(!close.is_empty());assert_eq!(close.retirement.as_ref().unwrap().as_ref()as*const dyn crate::ErasedSnapshotRetirement as*const (),original);}
 let(step,heap)=observe_retirement_allocations(||close.step_granted(grant).result.unwrap());assert!(step.progress().fits(grant));assert_eq!(step.progress().copied_bytes,copy);assert_eq!(heap,(0,release));assert_eq!(step.progress().released_bytes,release);assert!(close.is_empty());assert_eq!(observe_retirement_allocations(||drop(close)).1,(0,0));
}
