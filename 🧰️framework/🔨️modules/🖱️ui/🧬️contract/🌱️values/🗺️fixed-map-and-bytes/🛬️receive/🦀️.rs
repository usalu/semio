//! 🗃️ Original bounded UI maps and byte arrays retain their real backing through construction and cleanup.
use super::*;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError};
use semio_framework_value::native_decoding::{NativeReceive,NativeDecodeControl,OriginalReceivingPrimary,owned::{NativeDefault,NativeDefaultValue,NativeDefaultValueCursor,tree::{NativeSequenceCollection,NativeSequenceReceiver,RecordEntries,NativeValueSource,fixed,invalid,receipt}}};
type Rows<V>=fixed_list_storage::PagedList<(UiText,V),UI_FIXED_LIST_ITEMS>;
impl<V:RetireOwned> RetireOwned for UiFixedMap<V>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{self.entries.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.entries.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl<V:RetireOwned> NativeSequenceCollection<(UiText,V)> for UiFixedMap<V>{
 type Storage=Self;
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{<Rows<V> as NativeSequenceCollection<(UiText,V)>>::storage_demand(count)}
 fn admit_storage(output:&mut Option<Self>,count:usize,c:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let mut rows=None;let turn=<Rows<V> as NativeSequenceCollection<(UiText,V)>>::admit_storage(&mut rows,count,c,g);if let Some(storage)=rows{*output=Some(Self{entries:UiFixedList{storage}})}turn}
 fn reserve_demand(storage:&Self,_:usize)->Result<Option<RetirementDemand>,ValueError>{<Rows<V> as NativeSequenceCollection<(UiText,V)>>::reserve_demand(&storage.entries.storage,storage.len())}
 fn reserve_step(storage:&mut Self,index:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{<Rows<V> as NativeSequenceCollection<(UiText,V)>>::reserve_step(&mut storage.entries.storage,index,g)}
 fn push_depth(storage:&Self,index:usize)->Result<usize,ValueError>{<Rows<V> as NativeSequenceCollection<(UiText,V)>>::push_depth(&storage.entries.storage,index)}
 fn push_copy_bytes(storage:&Self,_:usize)->Result<usize,ValueError>{storage.len().checked_add(2).and_then(|n|n.checked_mul(4*size_of::<(UiText,V)>())).and_then(|n|n.checked_add(8*UI_TEXT_MAX_BYTES)).ok_or_else(invalid)}
 fn push_step(storage:&mut Self,index:usize,input:&mut Option<(UiText,V)>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{let d=match (Self::push_copy_bytes(storage,index),Self::push_depth(storage,index)){(Ok(copy_bytes),Ok(depth))=>RetirementDemand{copy_bytes,depth,..Default::default()},(Err(error),_)|(_,Err(error))=>return RetainedWorkOutcome{result:Err(error),work:0}};semio_framework_value::native_decoding::owned::tree::push_native_sequence_value(input,g,d,|(key,value)|storage.try_insert(key,value))}
 fn finish(storage:Self)->Self{storage}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>> NativeReceive<I> for UiFixedMap<V>{type Receiver=NativeSequenceReceiver<I,(UiText,V),Self,RecordEntries>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
impl RetireOwned for UiFixedBytes{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{self.bytes.into_vec().retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::owned_retirement_birth_bytes::<Vec<u8>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl NativeSequenceCollection<u8> for UiFixedBytes{
 type Storage=Vec<u8>;
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{if count>UI_FIXED_BYTES{return Err(invalid())}<Vec<u8> as NativeSequenceCollection<u8>>::storage_demand(count)}
 fn admit_storage(output:&mut Option<Vec<u8>>,count:usize,c:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{if let Err(error)=Self::storage_demand(count){return RetainedWorkOutcome{result:Err(error.into()),work:0}}c.admit_vec(output,count,g)}
 fn push_step(storage:&mut Vec<u8>,index:usize,input:&mut Option<u8>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{<Vec<u8> as NativeSequenceCollection<u8>>::push_step(storage,index,input,g)}
 fn finish_copy_bytes(storage:&Vec<u8>)->Result<usize,ValueError>{storage.capacity().checked_sub(storage.len()).ok_or_else(invalid)}
 fn finish(mut storage:Vec<u8>)->Self{let len=storage.len()as u16;storage.resize(storage.capacity(),0);Self{bytes:storage.into_boxed_slice(),len}}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for UiFixedBytes{type Receiver=NativeSequenceReceiver<I,u8,Self>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
impl NativeDefaultValue for UiFixedBytes{
 fn default_demand()->RetirementDemand{fixed(size_of::<Self>())}
 fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{empty(output,g,Self{bytes:Box::default(),len:0})}
}
impl NativeDefault for UiFixedBytes{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{Self::Cursor::new()}}
fn empty<T>(output:&mut Option<T>,g:RetainedCloneGrant,value:T)->RetainedWorkOutcome{let d=fixed(size_of::<T>());if output.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*output=Some(value);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}}
impl<V:RetireOwned> NativeDefaultValue for UiFixedMap<V>{
 fn default_demand()->RetirementDemand{fixed(size_of::<Self>())}
 fn admit_default(output:&mut Option<Self>,g:RetainedCloneGrant)->RetainedWorkOutcome{empty(output,g,Self::default())}
}
impl<V:RetireOwned> NativeDefault for UiFixedMap<V>{type Cursor=NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{Self::Cursor::new()}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]pub(super)mod tests;
