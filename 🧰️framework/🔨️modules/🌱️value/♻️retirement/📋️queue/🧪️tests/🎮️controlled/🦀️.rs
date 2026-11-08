//! 🧪️ A failing controlled child keeps its actual failure receipt through the original retirement queue.
use crate::{RetirementQueue,allocation_testing,queue_bank::{bank,poll,sync}};
use crate::{RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_retirement::{RetirementDemand,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,OriginalRetirementPoll,retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
use semio_framework_test_fixtures::fixture_text;
struct ExcessAllocation;
unsafe impl crate::list::PageAllocation for ExcessAllocation {
    fn reserve<T>(owner:&mut Vec<T>,slots:usize,actual:Option<&mut Option<usize>>)->Result<(),std::collections::TryReserveError>{let allocating=size_of::<T>()!=0&&owner.capacity()<owner.len().saturating_add(slots*2);let result=owner.try_reserve_exact(slots*2);if allocating{if let Some(actual)=actual{*actual=Some(if result.is_ok(){owner.capacity()*size_of::<T>()}else{0})}}result}
}
#[test]
fn original_retirement_queue_keeps_actual_controlled_child_failure_receipt(){
    struct FailedChild { original:ControlledRetirement<String>,failed:bool,source_pointer:usize }
    impl ErasedSnapshotRetirement for FailedChild {
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.original.next_demand(body)}
        fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.original.next_computational_work(body)}
        fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();
            if !self.failed { self.failed=true;let quote=match self.original.next_capacity_byte_demand(grant.maximum_copy_bytes){Ok(quote)=>quote,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};let mut quoted=effect.nested(RetainedCloneGrant{maximum_capacity_bytes:quote,..grant});let result=self.original.reserve_cursor_slot(&mut quoted,|list,effect|list.reserve_one_using::<ExcessAllocation>(effect));assert_eq!(self.original.original().unwrap().as_ptr()as usize,self.source_pointer);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map(|step|step.expect("reservation settles a step")).map_err(Into::into),work:self.original.last_turn_work()}); }
            self.original.step(effect)
        }
        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.original.next_copy_byte_demand()}
        fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.original.next_capacity_byte_demand(copy)}
        fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.original.next_release_byte_demand()}
        fn next_depth_demand(&self)->Result<usize,ValueError>{self.original.next_depth_demand()}
        fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()}
    }
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🎮️controlled/🧪️tests/🧫️fixtures/🔣️reservation.json")).unwrap();let text=law["original"].as_str().unwrap().to_owned();let pointer=text.as_ptr();let mut bank=bank();
    let mut child=Some(Box::new(FailedChild{original:ControlledRetirement::new(text).unwrap(),failed:false,source_pointer:pointer as usize}));let mut queue=RetirementQueue::default();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
    while !queue.has_reserved_slot(){sync(&mut bank,grant,1,|effect|queue.reserve_step(effect)).0.result.unwrap();}sync(&mut bank,grant,1,|effect|queue.admit_typed_retirement(&mut child,effect)).0.result.unwrap().unwrap();assert!(child.is_none());
    let units=queue.next_computational_work(grant.maximum_copy_bytes).unwrap();let((result,_),heap)=allocation_testing::observe_retirement_allocations(||poll(&mut bank,grant,units,|effect|queue.step(effect)));assert!(result.work>0);let error=result.result.unwrap_err();assert_eq!(crate::queue_bank::kind(&error),ValueRefusalKind::InvariantViolated);let progress=queue.step_progress();assert_eq!(progress,error.retained_progress());assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);assert!(progress.retained_capacity_bytes>0);let(primary,..)=error.into_parts();assert_eq!(primary.unwrap().under("original.queue").retained_progress(),progress);
    let mut turns=0;while !queue.terminal_is_empty(){turns+=1;assert!(turns<1024);let units=queue.next_computational_work(grant.maximum_copy_bytes).unwrap();let((step,_),heap)=allocation_testing::observe_retirement_allocations(||poll(&mut bank,grant,units,|effect|queue.step(effect)));let step=step.result.unwrap();assert_eq!(queue.step_progress(),step.progress());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);}
    let(_,heap)=allocation_testing::observe_retirement_allocations(||drop(queue));assert_eq!(heap,(0,0));
}
