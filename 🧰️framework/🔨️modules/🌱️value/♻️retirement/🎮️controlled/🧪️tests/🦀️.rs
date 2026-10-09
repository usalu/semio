use super::*;
struct ExcessAllocation;
impl crate::list::PageAllocation for ExcessAllocation {
    fn reserve<T>(owner:&mut Vec<T>,slots:usize)->Result<(),std::collections::TryReserveError>{owner.try_reserve_exact(slots*2)}
}
#[test]
fn original_controlled_failed_reservation_retains_actual_receipt_and_same_owner(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️reservation.json")).unwrap();
    let original=law["original"].as_str().unwrap().to_owned();let pointer=original.as_ptr();let mut owner=ControlledRetirement::new(original).unwrap();
    let quote=owner.next_capacity_byte_demand(4096).unwrap();
    let(result,heap)=crate::value::observe_retirement_allocations(||owner.reserve_cursor_slot(quote,|list,grant|list.reserve_one_using::<ExcessAllocation>(grant)));
    let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);let progress=owner.step_progress();assert_eq!(error.retained_progress(),progress);assert_eq!(progress.copied_items,1);assert_eq!(progress.copied_bytes,0);assert_eq!(progress.released_bytes,0);assert!(progress.retained_capacity_bytes>quote);assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);assert_eq!(owner.original().unwrap().as_ptr(),pointer);assert_eq!(owner.original().unwrap(),law["original"].as_str().unwrap());
    let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:quote,maximum_release_bytes:262144,maximum_depth:64};
    let(error,heap)=crate::value::observe_retirement_allocations(||crate::retained_clone::admit_retained_clone_progress(denied,progress,"original failed reservation").unwrap_err());assert_eq!(error.retained_progress(),progress);assert_eq!(heap,(0,0));
    let admitted=RetainedCloneGrant{maximum_capacity_bytes:progress.retained_capacity_bytes,..denied};let(error,heap)=crate::value::observe_retirement_allocations(||crate::retained_clone::admit_retained_clone_close(admitted,RetainedCloneStep::Complete(progress),false,"original failed terminal").unwrap_err());assert_eq!(error.retained_progress(),progress);assert_eq!(heap,(0,0));assert_eq!(owner.original().unwrap().as_ptr(),pointer);
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let mut turns=0;
    while !owner.terminal_is_empty(){turns+=1;assert!(turns<1024);let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(grant).unwrap());assert_eq!(owner.step_progress(),step.progress());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);}
    let(_,heap)=crate::value::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));println!("[DEBUG] actual original ControlledRetirement failed metadata reserve retains source pointer and born{} receipt before Err; same cursor physical close and terminal0/0",progress.retained_capacity_bytes);
}

#[test]
fn original_retirement_queue_keeps_actual_controlled_child_failure_receipt(){
    struct FailedChild { original:ControlledRetirement<String>,failed:bool,source_pointer:usize }
    impl crate::ErasedSnapshotRetirement for FailedChild {
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
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️reservation.json")).unwrap();let text=law["original"].as_str().unwrap().to_owned();let pointer=text.as_ptr();
    let mut child=Some(Box::new(FailedChild{original:ControlledRetirement::new(text).unwrap(),failed:false,source_pointer:pointer as usize}));let mut queue=crate::retirement::queue::RetirementQueue::default();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
    while !queue.has_reserved_slot(){queue.reserve_step(grant).unwrap();}queue.admit_typed_retirement(&mut child,grant).unwrap().unwrap();assert!(child.is_none());
    let(result,heap)=crate::value::observe_retirement_allocations(||queue.step(grant));let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);let progress=queue.step_progress();assert_eq!(progress,error.retained_progress());assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);assert!(progress.retained_capacity_bytes>0);assert_eq!(error.under("original.queue").retained_progress(),progress);
    let mut turns=0;while !queue.terminal_is_empty(){turns+=1;assert!(turns<1024);let(step,heap)=crate::value::observe_retirement_allocations(||queue.step(grant).unwrap());assert_eq!(queue.step_progress(),step.progress());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);}
    let(_,heap)=crate::value::observe_retirement_allocations(||drop(queue));assert_eq!(heap,(0,0));println!("[DEBUG] actual original RetirementQueue preserves failed ControlledRetirement allocated{} receipt through erased Result and under; same typed child closes physically and terminal0/0",progress.retained_capacity_bytes);
}

#[test]
fn original_controlled_root_transfer_preserves_same_source_with_narrow_copy_policy(){
 struct Original{value:String,pointer:usize,metadata:[u8;256]}
 impl RetireOwned for Original{fn retirement(self)->Box<dyn RetirementCursor>{assert_eq!(self.value.as_ptr()as usize,self.pointer);assert_eq!(self.metadata,[17;256]);self.value.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.value.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📥️root/🔣️.json")).unwrap();
 for copy in [1,3,64]{let(value,born)=crate::value::observe_retirement_allocations(||law["source"].as_str().unwrap().to_owned());let pointer=value.as_ptr();let(mut owner,heap)=crate::value::observe_retirement_allocations(||ControlledRetirement::new(Original{pointer:pointer as usize,value,metadata:[17;256]}).ok().unwrap());assert_eq!(heap,(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let mut births=born.0;let mut releases=born.1;
  while !owner.cursors.has_reserved_slot(){let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(grant).unwrap());assert_eq!(step.progress().copied_bytes,0);assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);births+=heap.0;releases+=heap.1;assert_eq!(owner.original().unwrap().value.as_ptr(),pointer);}
  assert_eq!(owner.next_copy_byte_demand().unwrap(),0);let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(grant).unwrap());assert_eq!(step.progress().copied_items,1);assert_eq!(step.progress().copied_bytes,law["rootCopyBytes"].as_u64().unwrap()as usize);assert!(step.progress().fits(grant));assert_eq!(owner.step_progress(),step.progress());assert!(owner.original().is_none());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);births+=heap.0;releases+=heap.1;
  let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000);let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(owner.step_progress(),step.progress());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);births+=heap.0;releases+=heap.1;}
  let(_,heap)=crate::value::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));assert_eq!(births,releases);println!("[DEBUG] Original Controlled rootcopy0 metadata256 moved same source{:?} undercopy{copy}, ctor0/0, exactbirth{} release{}, childwork preserved, terminalDrop0",pointer,births,releases);
 }
}

#[test]
fn original_controlled_narrow_child_keeps_real_semantic_minimum_and_same_grant(){
 for copy in [1,3,64]{let(original,born)=crate::value::observe_retirement_allocations(||vec![u128::MAX;3]);let pointer=original.as_ptr();let mut owner=ControlledRetirement::new(original).unwrap();assert_eq!(owner.original().unwrap().as_ptr(),pointer);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let(mut births,mut releases,mut narrow,mut turns)=(born.0,born.1,false,0);
  while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000);let demand=owner.next_copy_byte_demand().unwrap();let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(owner.step_progress(),progress);assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);if demand>copy&&progress.retained_capacity_bytes>0{narrow=true;}births+=heap.0;releases+=heap.1;
  }
  if copy<16{assert!(narrow);}let(_,heap)=crate::value::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));assert_eq!(births,releases);println!("[DEBUG] Original Vec<u128> same source{:?} genuine typedchild undercopy{copy} narrow{narrow}, grants unchanged, actualbirth{} release{}, turns{turns}, zeroitems0/0 terminalDrop0",pointer,births,releases);
 }
}
