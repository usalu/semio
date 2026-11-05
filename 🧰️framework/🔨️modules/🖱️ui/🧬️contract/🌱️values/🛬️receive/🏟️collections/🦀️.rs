//! 🏟️ Recursive native collection receivers borrow only the actual initialized lifetime arena and preserve donors across contention.
use super::*;
use semio_framework_value::native_decoding::owned::tree::{NativeSequenceCollection,NativeSequenceReceiver,RecordEntries};
fn arena_error(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,reason)}
fn storage_demand<T>(count:usize)->Result<RetirementDemand,ValueError>{if count>UI_VALUE_MAX_ITEMS{return Err(invalid())}Ok(fixed(size_of::<T>()+size_of::<collection_arena::UiCollectionSlot>()))}
impl NativeSequenceCollection<UiValue> for UiList{
 type Storage=UiListBuilder;
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{storage_demand::<UiListBuilder>(count)}
 fn admit_storage(out:&mut Option<UiListBuilder>,count:usize,_:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let mut work=0;let result=(||{let d=Self::storage_demand(count)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if out.is_some(){return Err(invalid().into())}if count==0{*out=Some(UiListBuilder{handle:None,len:0})}else{let Some(value)=UiListBuilder::try_new_initialized().map_err(arena_error)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*out=Some(value)}work=1;Ok(RetainedCloneStep::Complete(receipt(d)))})();RetainedWorkOutcome{result,work}}
 fn push_copy_bytes(_:&UiListBuilder,_:usize)->Result<usize,ValueError>{Ok(2*size_of::<collection_arena::UiPageSlot>()+size_of::<collection_arena::UiCollectionSlot>())}
 fn push_step(storage:&mut UiListBuilder,index:usize,input:&mut Option<UiValue>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{let mut work=0;let result=(||{let d=fixed(Self::push_copy_bytes(storage,index)?);if !retained_clone_grant_admits(g,d){return Ok(None)}match storage.push_initialized(input){Ok(false)=>Ok(None),Ok(true)=>{work=1;Ok(Some(receipt(d)))},Err(reason)=>{work=1;Err(arena_error(reason).with_retained_progress(receipt(d)))}}})();RetainedWorkOutcome{result,work}}
 fn finish(storage:UiListBuilder)->Self{storage.finish_initialized()}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiList{type Receiver=NativeSequenceReceiver<I,UiValue,Self>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
impl NativeSequenceCollection<(UiText,UiValue)> for UiMap{
 type Storage=UiMapBuilder;
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{storage_demand::<UiMapBuilder>(count)}
 fn admit_storage(out:&mut Option<UiMapBuilder>,count:usize,_:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let mut work=0;let result=(||{let d=Self::storage_demand(count)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if out.is_some(){return Err(invalid().into())}if count==0{*out=Some(UiMapBuilder{handle:None,len:0})}else{let Some(value)=UiMapBuilder::try_new_initialized().map_err(arena_error)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*out=Some(value)}work=1;Ok(RetainedCloneStep::Complete(receipt(d)))})();RetainedWorkOutcome{result,work}}
 fn push_copy_bytes(storage:&UiMapBuilder,_:usize)->Result<usize,ValueError>{storage.len.checked_mul(2*(UI_TEXT_MAX_BYTES+size_of::<usize>())).and_then(|n|n.checked_add(3*size_of::<collection_arena::UiPageSlot>()+size_of::<collection_arena::UiCollectionSlot>())).ok_or_else(invalid)}
 fn push_step(storage:&mut UiMapBuilder,index:usize,input:&mut Option<(UiText,UiValue)>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{let mut work=0;let result=(||{let d=fixed(Self::push_copy_bytes(storage,index)?);if !retained_clone_grant_admits(g,d){return Ok(None)}match storage.insert_initialized(input){Ok(false)=>Ok(None),Ok(true)=>{work=1;Ok(Some(receipt(d)))},Err(reason)=>{work=1;Err(arena_error(reason).with_retained_progress(receipt(d)))}}})();RetainedWorkOutcome{result,work}}
 fn finish(storage:UiMapBuilder)->Self{storage.finish_initialized()}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiMap{type Receiver=NativeSequenceReceiver<I,(UiText,UiValue),Self,RecordEntries>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
