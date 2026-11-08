//! 🏟️ Recursive native collection receivers borrow only the actual initialized lifetime arena and preserve donors across contention.
use super::*;
use semio_framework_value::native_decoding::NativeReceivingEffect;
use semio_framework_value::native_decoding::owned::{native_work_limit,tree::{NativeSequenceCollection,NativeSequenceReceiver,RecordEntries}};
fn arena_error(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,reason)}
fn storage_demand<T>(count:usize)->Result<RetirementDemand,ValueError>{if count>UI_VALUE_MAX_ITEMS{return Err(invalid())}Ok(fixed(size_of::<T>()+size_of::<collection_arena::UiCollectionSlot>()))}
fn admit<B>(out:&mut Option<B>,count:usize,frame:&mut NativeReceivingEffect,control:&mut NativeDecodeControl<'_>,maximum:u64,g:RetainedCloneGrant,empty:fn()->B,open:fn()->Result<Option<B>,&'static str>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
 let planned=(||->Result<Option<RetirementDemand>,OriginalReceivingPrimary>{if out.is_some(){return Err(invalid().into())}if native_work_limit(control,maximum)?==0{return Ok(None)}let d=storage_demand::<B>(count)?;Ok(retained_clone_grant_admits(g,d).then_some(d))})();
 let d=match planned{Ok(Some(d))=>d,Ok(None)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
 let turn=control.receive_allocation(d,frame,false,1,|_,_|{
  let opened=if count==0{Ok(Some(empty()))}else{open()};
  match opened{Ok(Some(value))=>{*out=Some(value);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}},Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(reason)=>RetainedWorkOutcome{result:Err(arena_error(reason)),work:0}}
 });
 RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}
}
fn push(input_present:bool,d:RetirementDemand,maximum:u64,g:RetainedCloneGrant,place:impl FnOnce()->Result<bool,&'static str>)->RetainedWorkOutcome<RetainedCloneStep>{
 if !input_present{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
 if maximum<1||!retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
 match place(){Ok(false)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Ok(true)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1},Err(reason)=>RetainedWorkOutcome{result:Err(arena_error(reason).with_retained_progress(receipt(d))),work:1}}
}
impl NativeSequenceCollection<UiValue> for UiList{
 type Storage=UiListBuilder;type Reservation=();
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{storage_demand::<UiListBuilder>(count)}
 fn storage_work_demand(count:usize)->Result<u64,ValueError>{Self::storage_demand(count)?;Ok(1)}
 fn reserve_demand(_:&UiListBuilder,_:usize,_:usize,_:&())->Result<Option<RetirementDemand>,ValueError>{Ok(None)}
 fn reserve_work_demand(_:&UiListBuilder,_:usize,_:usize,_:&(),_:u64)->Result<u64,ValueError>{Ok(0)}
 fn reserve_step(_:&mut UiListBuilder,_:usize,_:usize,_:&mut(),_:&mut NativeDecodeControl<'_>,_:u64,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Err(invalid().into()),work:0}}
 fn admit_storage(out:&mut Option<UiListBuilder>,count:usize,frame:&mut NativeReceivingEffect,control:&mut NativeDecodeControl<'_>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{admit(out,count,frame,control,maximum,g,||UiListBuilder{handle:None,len:0},UiListBuilder::try_new_initialized)}
 fn push_work_demand(_:&UiListBuilder,_:usize)->Result<u64,ValueError>{Ok(1)}
 fn push_copy_bytes(_:&UiListBuilder,_:usize)->Result<usize,ValueError>{Ok(2*size_of::<collection_arena::UiPageSlot>()+size_of::<collection_arena::UiCollectionSlot>())}
 fn push_step(storage:&mut UiListBuilder,index:usize,input:&mut Option<UiValue>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{let d=match Self::push_copy_bytes(storage,index){Ok(bytes)=>fixed(bytes),Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};push(input.is_some(),d,maximum,g,||storage.push_initialized(input))}
 fn finish(storage:UiListBuilder)->Self{storage.finish_initialized()}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiList{type Receiver=NativeSequenceReceiver<I,UiValue,Self>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
impl NativeSequenceCollection<(UiText,UiValue)> for UiMap{
 type Storage=UiMapBuilder;type Reservation=();
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{storage_demand::<UiMapBuilder>(count)}
 fn storage_work_demand(count:usize)->Result<u64,ValueError>{Self::storage_demand(count)?;Ok(1)}
 fn reserve_demand(_:&UiMapBuilder,_:usize,_:usize,_:&())->Result<Option<RetirementDemand>,ValueError>{Ok(None)}
 fn reserve_work_demand(_:&UiMapBuilder,_:usize,_:usize,_:&(),_:u64)->Result<u64,ValueError>{Ok(0)}
 fn reserve_step(_:&mut UiMapBuilder,_:usize,_:usize,_:&mut(),_:&mut NativeDecodeControl<'_>,_:u64,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Err(invalid().into()),work:0}}
 fn admit_storage(out:&mut Option<UiMapBuilder>,count:usize,frame:&mut NativeReceivingEffect,control:&mut NativeDecodeControl<'_>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{admit(out,count,frame,control,maximum,g,||UiMapBuilder{handle:None,len:0},UiMapBuilder::try_new_initialized)}
 fn push_work_demand(_:&UiMapBuilder,_:usize)->Result<u64,ValueError>{Ok(1)}
 fn push_copy_bytes(storage:&UiMapBuilder,_:usize)->Result<usize,ValueError>{storage.len.checked_mul(2*(UI_TEXT_MAX_BYTES+size_of::<usize>())).and_then(|n|n.checked_add(3*size_of::<collection_arena::UiPageSlot>()+size_of::<collection_arena::UiCollectionSlot>())).ok_or_else(invalid)}
 fn push_step(storage:&mut UiMapBuilder,index:usize,input:&mut Option<(UiText,UiValue)>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{let d=match Self::push_copy_bytes(storage,index){Ok(bytes)=>fixed(bytes),Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};push(input.is_some(),d,maximum,g,||storage.insert_initialized(input))}
 fn finish(storage:UiMapBuilder)->Self{storage.finish_initialized()}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiMap{type Receiver=NativeSequenceReceiver<I,(UiText,UiValue),Self,RecordEntries>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
