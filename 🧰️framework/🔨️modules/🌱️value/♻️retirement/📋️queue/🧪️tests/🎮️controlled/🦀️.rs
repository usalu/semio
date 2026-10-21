//! 🧪️ A failing controlled child keeps its actual failure receipt through the original retirement queue.
use crate::{RetirementQueue,allocation_testing};
use semio_framework_value_retirement::{RetirementDemand,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::controlled::ControlledRetirement};
use semio_framework_test_fixtures::fixture_text;
struct ExcessAllocation;
unsafe impl crate::list::PageAllocation for ExcessAllocation {
    fn reserve<T>(owner:&mut Vec<T>,slots:usize)->Result<(),std::collections::TryReserveError>{owner.try_reserve_exact(slots*2)}
}
#[test]
fn original_retirement_queue_keeps_actual_controlled_child_failure_receipt(){
    struct FailedChild { original:ControlledRetirement<String>,failed:bool,source_pointer:usize }
    impl ErasedSnapshotRetirement for FailedChild {
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.original.next_demand(body)}
        fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
            if !self.failed { self.failed=true;let quote=self.original.next_capacity_byte_demand(grant.maximum_copy_bytes)?;let result=self.original.reserve_cursor_slot(quote,|list,grant|list.reserve_one_using::<ExcessAllocation>(grant));assert_eq!(self.original.original().unwrap().as_ptr()as usize,self.source_pointer);return result; }
            self.original.step(grant)
        }
        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.original.next_copy_byte_demand()}
        fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.original.next_capacity_byte_demand(copy)}
        fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.original.next_release_byte_demand()}
        fn next_depth_demand(&self)->Result<usize,ValueError>{self.original.next_depth_demand()}
        fn terminal_is_empty(&self)->bool{self.original.terminal_is_empty()}
    }
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🎮️controlled/🧪️tests/🧫️fixtures/🔣️reservation.json")).unwrap();let text=law["original"].as_str().unwrap().to_owned();let pointer=text.as_ptr();
    let mut child=Some(Box::new(FailedChild{original:ControlledRetirement::new(text).unwrap(),failed:false,source_pointer:pointer as usize}));let mut queue=RetirementQueue::default();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
    while !queue.has_reserved_slot(){queue.reserve_step(grant).unwrap();}queue.admit_typed_retirement(&mut child,grant).unwrap().unwrap();assert!(child.is_none());
    let(result,heap)=allocation_testing::observe_retirement_allocations(||queue.step(grant));let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);let progress=queue.step_progress();assert_eq!(progress,error.retained_progress());assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);assert!(progress.retained_capacity_bytes>0);assert_eq!(error.under("original.queue").retained_progress(),progress);
    let mut turns=0;while !queue.terminal_is_empty(){turns+=1;assert!(turns<1024);let(step,heap)=allocation_testing::observe_retirement_allocations(||queue.step(grant).unwrap());assert_eq!(queue.step_progress(),step.progress());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);}
    let(_,heap)=allocation_testing::observe_retirement_allocations(||drop(queue));assert_eq!(heap,(0,0));
}
