//! 🧪️ Original staged job source enters native authority through paid initialized fields and bounded byte ranges.
use super::*;
#[repr(transparent)]
struct OriginalSource([u8;8192]);
impl InteractiveJob for OriginalSource{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{JobOutcomeBorrow::admit_yield(cx)}
 fn borrow_outcome<'a>(&'a self,d:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{d.yielded()}
 fn begin_close(&mut self){}
 fn close_step(&mut self,_:RetainedCloneGrant)->InteractiveJobCloseStep{InteractiveJobCloseStep::Complete{progress:Default::default()}}
 fn terminal_is_empty(&self)->bool{true}
}
struct OriginalInitializer{written:usize}
unsafe impl WorkerJobInitializer<OriginalSource> for OriginalInitializer{
 fn initialization_demand(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:(8192-self.written).min(maximum_copy_bytes.max(1)),depth:1,..Default::default()})}
 unsafe fn initialize_step(&mut self,target:*mut MaybeUninit<OriginalSource>,grant:RetainedCloneGrant)->Result<WorkerJobInitializationStep,ValueError>{let bytes=(8192-self.written).min(grant.maximum_copy_bytes);unsafe{std::ptr::write_bytes(target.cast::<u8>().add(self.written),71,bytes);}self.written+=bytes;Ok(WorkerJobInitializationStep{progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()},complete:self.written==8192})}
 fn initialization_input_is_empty(&self)->bool{true}
 fn partial_retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 unsafe fn close_partial_step(&mut self,_:*mut MaybeUninit<OriginalSource>,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}
 fn partial_terminal_is_empty(&self)->bool{true}

}
fn now()->Option<u64>{Some(1)}
fn observed<T>(f:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);(value,(heap.requested_bytes,heap.released_bytes))}
fn turn<T>(grant:RetainedCloneGrant,f:impl FnOnce(&mut WorkerJobAdmissionContext<'_>)->T)->(T,RetainedCloneProgress){let mut receipt=Default::default();let mut cx=WorkerJobAdmissionContext::new(OperationId(99927),Generation(4),StepBudget::new(1,2,grant),now,&mut receipt).unwrap();let value=f(&mut cx);drop(cx);assert!(receipt.fits(grant));(value,receipt)}
#[test]
fn original_staged_source_preserves_metadata_zero_exact_ranges_and_native_custody(){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let metadata=RetainedCloneGrant{maximum_copy_bytes:0,..grant};let mut storage=MaybeUninit::<WorkerJobSource<OriginalSource>>::uninit();
 let((denied,receipt),heap)=observed(||turn(RetainedCloneGrant{maximum_items:0,..metadata},|cx|WorkerJobSource::start_preparation(&mut storage,cx).unwrap().is_none()));assert!(denied);assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));let((source,bind),heap)=observed(||turn(metadata,|cx|WorkerJobSource::start_preparation(&mut storage,cx).unwrap().unwrap().0));assert_eq!(heap,(0,0));assert_eq!(bind,RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()});let pointer=source as*const _;let mut initializer=OriginalInitializer{written:0};
 for denied in[metadata,RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let((step,receipt),heap)=observed(||turn(denied,|cx|source.prepare_step(&mut initializer,cx).unwrap()));assert_eq!(heap,(0,0));assert_eq!(step.progress,Default::default());assert_eq!(receipt,Default::default());assert_eq!(initializer.written,0);assert_eq!(source as*const _,pointer);}
 let expected=[4096,4096];for (index,bytes) in expected.into_iter().enumerate(){let((step,receipt),heap)=observed(||turn(grant,|cx|source.prepare_step(&mut initializer,cx).unwrap()));assert_eq!(heap,(0,0));assert_eq!(receipt,step.progress);assert_eq!(receipt.copied_bytes,bytes);assert_eq!(step.complete,index==1);}
 assert_eq!(source.source().unwrap().0,[71;8192]);let original=source.source().unwrap()as*const _;let((cancel,cancel_birth),heap)=observed(||CancelToken::admit_root(metadata).unwrap().unwrap());assert_eq!((cancel_birth.retained_capacity_bytes,cancel_birth.released_bytes),heap);assert_eq!(cancel_birth.copied_bytes,0);let mut born=heap.0;
 let mut params=Some(BatchJobParams{operation:OperationId(99927),generation:Generation(4),cancel,config:BatchDriveConfig{site:"test.original-staged",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:1,retained:grant},now_us:now});let demand=WorkerJobAdmission::<OriginalSource>::birth_demand();assert_eq!(demand.copy_bytes,26);assert!(demand.copy_bytes<=grant.maximum_copy_bytes);
 let((admission,receipt),heap)=observed(||turn(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant},|cx|WorkerJobAdmission::try_begin(source,&mut params,cx).unwrap().is_some()));assert!(!admission);assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));assert!(params.is_some());assert_eq!(source.source().unwrap()as*const _,original);
 let((admission,receipt),heap)=observed(||turn(grant,|cx|WorkerJobAdmission::try_begin(source,&mut params,cx).unwrap().unwrap()));let(mut admission,step)=admission;assert_eq!(receipt,step);assert_eq!(receipt.copied_bytes,demand.copy_bytes);assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),heap);born+=heap.0;
 let mut initialization_turns=0;while !admission.destination_is_initialized(){let((output,receipt),heap)=observed(||turn(metadata,|cx|admission.advance(cx).unwrap()));assert!(output.0.is_none());assert_eq!(receipt,output.1);assert!(receipt.fits(metadata));assert_eq!(receipt.copied_bytes,0);assert_eq!(heap,(0,0));assert_eq!(admission.source()as*const _,original);initialization_turns+=1;assert!(initialization_turns<64);}for bytes in[4096,4096]{let((output,receipt),heap)=observed(||turn(grant,|cx|admission.advance(cx).unwrap()));assert!(output.0.is_none());assert_eq!(receipt,output.1);assert_eq!(receipt.copied_bytes,bytes);assert_eq!(heap,(0,0));assert_eq!(admission.source()as*const _,original);}
 let((denied,receipt),heap)=observed(||turn(RetainedCloneGrant{maximum_items:0,..metadata},|cx|admission.advance(cx).unwrap()));assert!(denied.0.is_none());assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));let((output,receipt),heap)=observed(||turn(metadata,|cx|admission.advance(cx).unwrap()));let(mut session,step)=(output.0.unwrap(),output.1);assert_eq!(receipt,step);assert_eq!(receipt.copied_bytes,0);assert_eq!(heap,(0,0));drop(admission);assert!(params.is_none());assert!(source.source().is_none());
 session.begin_close();let mut released=0;let mut turns=0;while !session.terminal_is_empty(){let(step,heap)=observed(||session.close_step(metadata));assert!(step.progress().fits(metadata));assert_eq!(step.progress().copied_bytes,0);assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);released+=heap.1;turns+=1;assert!(turns<64);}assert_eq!(born,released);let((),heap)=observed(||{drop(session);drop(storage);drop(params);});assert_eq!(heap,(0,0));println!("[DEBUG] original staged source8192 paid preparation/destination ranges with original4096 sourceBind0 cursor0 presence0 same fault page exactNativeBornReleased={born} closeTurns={turns} terminalDrop0");
}
