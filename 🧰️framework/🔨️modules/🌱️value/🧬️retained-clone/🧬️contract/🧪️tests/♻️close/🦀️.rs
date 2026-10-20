//! ♻️ Actual clone close-frame private custody preserves original terminal release laws.
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::observe_retirement_allocations;
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
        close.begin_granted(&mut text,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth,maximum_depth:1,..Default::default()}).unwrap();
        assert!(text.is_none());
        for _ in 0..10_000 {
            if close.retirement.as_ref().unwrap().terminal_is_empty() { break; }
            let capacity=close.next_capacity_byte_demand(7).unwrap();
            let release=close.next_release_byte_demand().unwrap();
            assert!(capacity<=admission&&release<=admission);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:close.next_depth_demand().unwrap()};
            let (step,events)=crate::observe_retirement_allocations(||close.step_granted(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(events,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        }
        assert!(close.retirement.as_ref().unwrap().terminal_is_empty());
        let pointer = close.retirement.as_ref().unwrap().as_ref() as *const dyn crate::ErasedSnapshotRetirement;
        let (physical, query_events) = crate::observe_retirement_allocations(|| close.next_release_byte_demand().unwrap());
        assert_eq!(query_events, (0, 0));
        assert!(physical > 0 && physical <= admission);
        for (items, bytes) in [(0, physical), (1, physical - 1)] {
            let (step, events) = crate::observe_retirement_allocations(|| close.step_granted(RetainedCloneGrant{maximum_items:items,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()}).unwrap());
            let retained = close.retirement.as_ref().is_some_and(|owner| std::ptr::addr_eq(pointer, owner.as_ref() as *const dyn crate::ErasedSnapshotRetirement));
            if step != (RetainedCloneStep::Progress(Default::default())) || events != (0, 0) || !retained {
                mismatches += 1;
            }
            if close.is_empty() { break; }
        }
        if !close.is_empty() {
            let (step, events) = crate::observe_retirement_allocations(|| close.step_granted(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:physical,maximum_depth:1,..Default::default()}).unwrap());
            if step != (RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:1,released_bytes:physical,..Default::default()})) || events != (0, physical) { mismatches += 1; }
        }
        assert!(close.is_empty());
        assert_eq!(close.step_granted(Default::default()).unwrap(), RetainedCloneStep::Complete(Default::default()));
    }
    assert_eq!(mismatches, 0, "cold terminal retirement Box requires its complete same-turn physical grant");
}



struct RefusingCopyDemand { owner: Option<u64> }

impl crate::ErasedSnapshotRetirement for RefusingCopyDemand {
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"copy demand has no authority"))}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let items=usize::from(self.owner.take().is_some());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:items,..Default::default()}))}
    fn terminal_is_empty(&self)->bool{self.owner.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"copy demand has no authority"))}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,crate::ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
    fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
}

#[test]
fn retained_clone_erased_copy_demand_propagates_original_owner_refusal(){
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🪜️close-demands/🔣️.json")).unwrap();
    let owner=RefusingCopyDemand{owner:Some(law["owner"].as_u64().unwrap())};
    let(result,heap)=observe_retirement_allocations(||crate::ErasedSnapshotRetirement::next_copy_byte_demand(&owner));
    assert_eq!(heap,(0,0));assert_eq!(format!("{:?}",result.unwrap_err().kind),law["copyDemandRefusal"].as_str().unwrap());assert_eq!(owner.owner,Some(law["owner"].as_u64().unwrap()));
    let mut ticket:Option<Box<dyn crate::ErasedSnapshotRetirement>>=Some(Box::new(owner));
    let(result,heap)=observe_retirement_allocations(||crate::factory_ticket_demands(ticket.as_ref().unwrap(),0));assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind,crate::ValueRefusalKind::UnsupportedOwner);assert!(!ticket.as_ref().unwrap().terminal_is_empty());
    let step=crate::close_factory_ticket(&mut ticket,RetainedCloneGrant{maximum_items:1,..Default::default()}).unwrap();assert!(matches!(step,RetainedCloneStep::Progress(_)));assert!(ticket.as_ref().unwrap().terminal_is_empty());
    let demand=crate::factory_ticket_demands(ticket.as_ref().unwrap(),0).unwrap();assert_eq!(demand.depth,1);
    let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(step,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut ticket,grant).unwrap());assert_eq!(heap,(0,step.progress().released_bytes));assert!(ticket.is_none());assert!(matches!(step,RetainedCloneStep::Progress(_)));assert!(matches!(crate::close_factory_ticket(&mut ticket,Default::default()).unwrap(),RetainedCloneStep::Complete(_)));
}

#[test]
fn retained_clone_close_depth_observes_pending_birth_and_preserves_owner_on_refusal() {
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🪜️close-demands/🔣️.json")).unwrap();
    let mut cursor=crate::retained_clone::ScalarCursor::<u64>::default();
    cursor.value=Some(law["owner"].as_u64().unwrap());
    assert_eq!(cursor.next_close_depth_demand().unwrap(),law["idleDepth"].as_u64().unwrap()as usize);
    cursor.begin_close();
    let ((depth,capacity),heap)=observe_retirement_allocations(||(cursor.next_close_depth_demand().unwrap(),cursor.next_close_capacity_byte_demand(0).unwrap()));
    assert_eq!(heap,(0,0));assert_eq!(depth,law["pendingBirthDepth"].as_u64().unwrap()as usize);
    let denied=RetainedCloneGrant{maximum_items:law["refusedBirth"]["maximumItems"].as_u64().unwrap()as usize,maximum_capacity_bytes:capacity,maximum_depth:law["refusedBirth"]["maximumDepth"].as_u64().unwrap()as usize,..Default::default()};
    let(result,heap)=observe_retirement_allocations(||cursor.close_step(denied));
    assert_eq!(heap,(0,0));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!(cursor.value,Some(law["owner"].as_u64().unwrap()));
    let birth=RetainedCloneGrant{maximum_items:law["fundedBirth"]["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:cursor.next_close_copy_byte_demand().unwrap(),maximum_capacity_bytes:capacity,maximum_depth:depth,..Default::default()};
    let(step,heap)=observe_retirement_allocations(||cursor.close_step(birth).unwrap());
    assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(birth));assert!(cursor.value.is_none());
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){
        if cursor.terminal_is_empty(){break;}
        let copy=cursor.next_close_copy_byte_demand().unwrap();let release=cursor.next_close_release_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap(),maximum_release_bytes:release,maximum_depth:cursor.next_close_depth_demand().unwrap()};
        let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
    }
    assert!(cursor.terminal_is_empty());assert_eq!(cursor.next_close_depth_demand().unwrap(),law["terminalDepth"].as_u64().unwrap()as usize);
}

#[test]
fn retained_clone_terminal_ticket_quotes_and_funds_the_original_header(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🪜️close-demands/🔣️.json")).unwrap();let law=&law["terminalTicket"];
 let(mut close,heap)=observe_retirement_allocations(||crate::retained_clone::RetainedCloneClose::from_admitted_original(Box::new(RefusingCopyDemand{owner:None})));let original=close.retirement.as_ref().unwrap().as_ref()as*const dyn crate::ErasedSnapshotRetirement as*const ();
 let expected=law["headerWidths"].as_array().unwrap().iter().find(|row|row["wordBytes"].as_u64().unwrap()as usize==size_of::<usize>()).unwrap()["copyBytes"].as_u64().unwrap()as usize;let bytes=heap.0;assert_eq!(heap.1,0);
 let((copy,capacity,release,depth),heap)=observe_retirement_allocations(||(close.next_copy_byte_demand().unwrap(),close.next_capacity_byte_demand(expected).unwrap(),close.next_release_byte_demand().unwrap(),close.next_depth_demand().unwrap()));assert_eq!(heap,(0,0));assert_eq!(copy,expected);assert_eq!(capacity,law["capacityBytes"].as_u64().unwrap()as usize);assert_eq!(release,bytes);assert_eq!(depth,law["depth"].as_u64().unwrap()as usize);
 let grant=RetainedCloneGrant{maximum_items:law["items"].as_u64().unwrap()as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
 for axis in law["refused"].as_array().unwrap(){let axis=axis.as_str().unwrap();let denied=match axis{"items"=>RetainedCloneGrant{maximum_items:0,..grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:copy-1,..grant},"release"=>RetainedCloneGrant{maximum_release_bytes:release-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};let(result,heap)=observe_retirement_allocations(||close.step_granted(denied));assert_eq!(heap,(0,0));if axis=="depth"{assert_eq!(result.unwrap_err().kind,crate::ValueRefusalKind::DepthLimit);}else{assert_eq!(result.unwrap().progress(),Default::default());}assert!(!close.is_empty());assert_eq!(close.retirement.as_ref().unwrap().as_ref()as*const dyn crate::ErasedSnapshotRetirement as*const (),original);}
 let(step,heap)=observe_retirement_allocations(||close.step_granted(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(step.progress().copied_bytes,copy);assert_eq!(heap,(0,release));assert_eq!(step.progress().released_bytes,release);assert!(close.is_empty());assert_eq!(observe_retirement_allocations(||drop(close)).1,(0,0));
}
