//! 🩹️ Original operation receivers fund the directory, each one-operation page and each exact donor placement separately.
use super::*;
use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,native_decoding::{NativeReceive,NativeReceivingEffect,OriginalReceivingPrimary,owned::{native_work_limit,NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,tree::{NativeValueSource,NativeSequenceCollection,NativeSequenceReceiver,fixed,receipt,invalid}}},retirement::{RetireOwned,RetirementCursor},retained_clone::retained_clone_grant_admits};
type OriginalPages=(Vec<Vec<UiPatchOp>>,UiTypedRetirementCursor);
impl RetireOwned for UiPatchOps{
 type RetirementCursor=<OriginalPages as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{(self.storage.pages,self.retirement).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{(Vec::<Vec<UiPatchOp>>::new(),UiTypedRetirementCursor::empty()).retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{OriginalPages::controlled_retirement_supported()}
}
impl NativeSequenceCollection<UiPatchOp> for UiPatchOps{
 type Storage=Self;type Reservation=();
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{if count>UI_DOCUMENT_PATCH_OPS{return Err(invalid())}Ok(fixed(size_of::<Self>()))}
 fn storage_work_demand(count:usize)->Result<u64,ValueError>{Self::storage_demand(count)?;Ok(1)}
 fn admit_storage(output:&mut Option<Self>,count:usize,frame:&mut NativeReceivingEffect,control:&mut NativeDecodeControl<'_>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
  let planned=(||->Result<Option<RetirementDemand>,OriginalReceivingPrimary>{if output.is_some(){return Err(invalid().into())}if native_work_limit(control,maximum)?==0{return Ok(None)}let d=Self::storage_demand(count)?;Ok(retained_clone_grant_admits(g,d).then_some(d))})();
  let d=match planned{Ok(Some(d))=>d,Ok(None)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  let turn=control.receive_allocation(d,frame,false,1,|_,_|{*output=Some(Self::default());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}});
  RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}
 }
 fn reserve_demand(storage:&Self,index:usize,_:usize,_:&())->Result<Option<RetirementDemand>,ValueError>{if storage.len()!=index{return Err(invalid())}if storage.has_reserved_slot(){return Ok(None)}let bytes=storage.next_allocation_bytes().map_err(|reason|ValueError::literal(ValueRefusalKind::OwnershipLimit,reason))?;Ok(Some(RetirementDemand{capacity_bytes:bytes,..fixed(size_of::<PatchPages>()+size_of::<Vec<UiPatchOp>>())}))}
 fn reserve_work_demand(storage:&Self,index:usize,limit:usize,reservation:&(),_:u64)->Result<u64,ValueError>{Ok(u64::from(Self::reserve_demand(storage,index,limit,reservation)?.is_some()))}
 fn reserve_step(storage:&mut Self,index:usize,limit:usize,reservation:&mut (),control:&mut NativeDecodeControl<'_>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
  let planned=(||->Result<Option<RetirementDemand>,OriginalReceivingPrimary>{let Some(d)=Self::reserve_demand(storage,index,limit,reservation)?else{return Err(invalid().into())};if maximum<1||!retained_clone_grant_admits(g,d){return Ok(None)}control.checkpoint()?;Ok(if control.prepare_work(1)?{Some(d)}else{None})})();
  let d=match planned{Ok(Some(d))=>d,Ok(None)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  let receipt_of=|bytes:usize|RetainedCloneProgress{retained_capacity_bytes:bytes,..receipt(RetirementDemand{capacity_bytes:0,..d})};
  let result=match storage.try_reserve_one(d.capacity_bytes){Ok(bytes)=>Ok(RetainedCloneStep::Progress(receipt_of(bytes))),Err(error)=>Err(OriginalReceivingPrimary::from(ValueError::literal(ValueRefusalKind::AllocationFailed,error.reason).with_retained_progress(receipt_of(error.allocated_bytes))))};
  match control.commit_work(1){Ok(_)=>RetainedWorkOutcome{result,work:1},Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:1}}
 }
 fn push_work_demand(_:&Self,_:usize)->Result<u64,ValueError>{Ok(1)}
 fn push_copy_bytes(_:&Self,_:usize)->Result<usize,ValueError>{Ok(size_of::<UiPatchOp>()+size_of::<usize>())}
 fn push_step(storage:&mut Self,index:usize,input:&mut Option<UiPatchOp>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{let mut work=0;let result=(||{let d=fixed(Self::push_copy_bytes(storage,index)?);if maximum<1||!retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if storage.len()!=index||!storage.has_reserved_slot()||input.is_none(){return Err(invalid())}work=1;let bytes=storage.try_push_reserved(input,size_of::<UiPatchOp>()).map_err(|reason|ValueError::literal(ValueRefusalKind::InvariantViolated,reason).with_retained_progress(receipt(d)))?;if bytes==0{return Err(invalid().with_retained_progress(receipt(d)))}Ok(RetainedCloneStep::Complete(receipt(d)))})();RetainedWorkOutcome{result,work}}
 fn finish_copy_bytes(_:&Self)->Result<usize,ValueError>{Ok(size_of::<Self>())}
 fn finish(storage:Self)->Self{storage}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiPatchOps{type Receiver=NativeSequenceReceiver<I,UiPatchOp,Self>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
impl NativeDefaultValue for UiPatchOps{fn default_demand()->RetirementDemand{fixed(size_of::<Self>())}fn default_work_demand()->Result<u64,ValueError>{Ok(1)}fn default_allocates()->bool{false}fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant,_:Option<&mut Option<usize>>)->RetainedWorkOutcome{let d=Self::default_demand();if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some(Self::default());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}}
impl NativeDefault for UiPatchOps{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{Self::Cursor::new()}}
