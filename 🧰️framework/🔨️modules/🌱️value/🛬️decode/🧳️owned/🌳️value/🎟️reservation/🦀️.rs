//! 🎟️ Sequence reservation retains paid selection and the actual original allocation callback.
pub use semio_framework_value_native_schema_engine::reservation::{NativeSequenceReservation,NativePagedReservation};
pub use semio_framework_value_control_contract::{RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
pub use semio_framework_value_retirement::{retirement,OriginalRetirementFailure,OriginalRetirementOutcome};
use retirement::RetireOwned;
pub use semio_framework_value_derive::RetireOwned;
use semio_framework_value_native_decoding::native_decoding::{self,NativeDecodeControl,OriginalReceivingPrimary};
use semio_framework_value_native_receiving_contract::{native_work_limit,work_sum,selected_native_work,NativeRecoveryLoan};
use semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits;
use std::mem::size_of;
fn invalid()->ValueError{ValueError::invariant("native reservation state is invalid")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
use native_decoding::construction::{NativeReceivingEffect,NativeReceivingEffectPhase};
use semio_framework_value_paged_list::PagedListReservationSelection;

use semio_framework_value_native_source::{NativeValueSource,NativeSourceAccess};
fn backing_turn(turn:native_decoding::construction::NativeBackingTurn)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{work:turn.work,result:turn.result.map_err(|failure|{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child;receiving.work_refusal=failure.work_refusal;receiving})}}
pub trait NativeSequenceCollection<T:RetireOwned>:RetireOwned{
 type Reservation:NativeSequenceReservation;
 type Storage:RetireOwned;
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>;
 fn reserve_demand(storage:&Self::Storage,index:usize,limit:usize,reservation:&Self::Reservation)->Result<Option<RetirementDemand>,ValueError>;
 fn reserve_work_demand(storage:&Self::Storage,index:usize,limit:usize,reservation:&Self::Reservation,maximum:u64)->Result<u64,ValueError>;
 fn reserve_step(storage:&mut Self::Storage,index:usize,limit:usize,reservation:&mut Self::Reservation,control:&mut NativeDecodeControl<'_>,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>;
 fn storage_work_demand(count:usize)->Result<u64,ValueError>;
 fn push_work_demand(storage:&Self::Storage,index:usize)->Result<u64,ValueError>;
 fn push_depth(_storage:&Self::Storage,_index:usize)->Result<usize,ValueError>{Ok(1)}
 fn push_copy_bytes(_storage:&Self::Storage,_index:usize)->Result<usize,ValueError>{Ok(size_of::<T>())}
 fn finish_copy_bytes(_storage:&Self::Storage)->Result<usize,ValueError>{Ok(0)}
 fn admit_storage(output:&mut Option<Self::Storage>,count:usize,frame:&mut native_decoding::NativeReceivingEffect,control:&mut NativeDecodeControl<'_>,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>;

 fn push_step(storage:&mut Self::Storage,index:usize,input:&mut Option<T>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>;
 fn finish(storage:Self::Storage)->Self;
}
pub fn push_native_sequence_value<T>(input:&mut Option<T>,g:RetainedCloneGrant,d:RetirementDemand,push:impl FnOnce(T)->Result<(),T>)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(None),work:0}}let Some(value)=input.take()else{return RetainedWorkOutcome{result:Err(invalid()),work:0}};let result=match push(value){Ok(())=>Ok(Some(receipt(d))),Err(value)=>{*input=Some(value);Err(invalid().with_retained_progress(receipt(d)))}};RetainedWorkOutcome{result,work:1}}
impl<T:RetireOwned> NativeSequenceCollection<T> for Vec<T>{
 type Storage=Vec<T>;type Reservation=();
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Vec<T>>(),capacity_bytes:count.checked_mul(size_of::<T>()).filter(|n|*n<=isize::MAX as usize).ok_or_else(invalid)?,depth:1,..Default::default()})}
 fn storage_work_demand(count:usize)->Result<u64,ValueError>{Self::storage_demand(count)?;Ok(1)}
 fn reserve_demand(_:&Self::Storage,_:usize,_:usize,_:&())->Result<Option<RetirementDemand>,ValueError>{Ok(None)}
 fn reserve_work_demand(_:&Self::Storage,_:usize,_:usize,_:&(),_:u64)->Result<u64,ValueError>{Ok(0)}
 fn reserve_step(_:&mut Self::Storage,_:usize,_:usize,_:&mut(),_:&mut NativeDecodeControl<'_>,_:u64,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Err(invalid().into()),work:0}}
 fn admit_storage(output:&mut Option<Self::Storage>,count:usize,frame:&mut native_decoding::NativeReceivingEffect,control:&mut NativeDecodeControl<'_>,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{if maximum==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}backing_turn(control.admit_vec(output,count,grant,frame))}
 fn push_work_demand(storage:&Self::Storage,index:usize)->Result<u64,ValueError>{if storage.len()!=index||storage.len()==usize::MAX||size_of::<T>()!=0&&storage.len()==storage.capacity(){return Err(invalid())}Ok(3)}
 fn push_copy_bytes(_:&Self::Storage,_:usize)->Result<usize,ValueError>{Ok(size_of::<T>()+size_of::<Option<T>>()+size_of::<usize>())}
 fn push_step(storage:&mut Self::Storage,index:usize,input:&mut Option<T>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{let mut work=0;let result=(||{let q=Self::push_work_demand(storage,index)?;let d=fixed(Self::push_copy_bytes(storage,index)?);if maximum<q||!retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if input.is_none(){return Err(invalid())}let value=input.take().expect("validated retained vector donor");storage.push(value);work=u128::from(q);Ok(RetainedCloneStep::Complete(receipt(d)))})();RetainedWorkOutcome{result,work}}
 fn finish(storage:Self::Storage)->Self{storage}
}
pub trait NativeSequenceInput<I:NativeValueSource+?Sized>:Send+'static{
 fn length_work_demand(input:&I)->Result<u64,ValueError>;
 fn child_work_demand(input:&I,index:usize)->Result<u64,ValueError>;
 fn length(input:&I)->Result<usize,ValueError>;
 fn child(input:&I,index:usize)->Result<&dyn NativeValueSource,ValueError>;
}
pub struct SequenceRows;
impl<I:NativeValueSource+?Sized> NativeSequenceInput<I> for SequenceRows{fn length_work_demand(input:&I)->Result<u64,ValueError>{input.accessor_work_demand(NativeSourceAccess::SequenceLen)}fn child_work_demand(input:&I,index:usize)->Result<u64,ValueError>{input.accessor_work_demand(NativeSourceAccess::SequenceChild(index))}fn length(input:&I)->Result<usize,ValueError>{input.sequence_len()}fn child(input:&I,index:usize)->Result<&dyn NativeValueSource,ValueError>{input.sequence_child(index)}}
pub struct RecordEntries;
impl<I:NativeValueSource+?Sized> NativeSequenceInput<I> for RecordEntries{fn length_work_demand(input:&I)->Result<u64,ValueError>{input.accessor_work_demand(NativeSourceAccess::RecordLen)}fn child_work_demand(input:&I,index:usize)->Result<u64,ValueError>{input.accessor_work_demand(NativeSourceAccess::RecordEntry(index))}fn length(input:&I)->Result<usize,ValueError>{input.record_len()}fn child(input:&I,index:usize)->Result<&dyn NativeValueSource,ValueError>{input.record_entry(index)}}
impl<T:RetireOwned,const N:usize> NativeSequenceCollection<T> for semio_framework_value_paged_list::PagedList<T,N>{
 type Storage=Self;type Reservation=NativePagedReservation;
 fn storage_demand(count:usize)->Result<RetirementDemand,ValueError>{if count>N{return Err(invalid())}Ok(fixed(size_of::<Option<Self>>()))}
 fn storage_work_demand(count:usize)->Result<u64,ValueError>{Self::storage_demand(count)?;Ok(1)}
 fn admit_storage(output:&mut Option<Self>,count:usize,frame:&mut native_decoding::NativeReceivingEffect,control:&mut NativeDecodeControl<'_>,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{let result=(||{let maximum=native_work_limit(control,maximum)?;if maximum==0{return Ok(None)}let d=<Self as NativeSequenceCollection<T>>::storage_demand(count)?;if !retained_clone_grant_admits(grant,d){return Ok(None)}let turn=control.receive_allocation(d,frame,false,1,|_,_|{if output.is_some(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}*output=Some(Self::new());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(d))),work:1}});Ok(Some(turn))})();match result{Ok(Some(turn))=>RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work},Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>RetainedWorkOutcome{result:Err(error),work:0}}}
 fn reserve_demand(storage:&Self,index:usize,limit:usize,state:&NativePagedReservation)->Result<Option<RetirementDemand>,ValueError>{state.demand(storage,index,limit)}
 fn reserve_work_demand(storage:&Self,index:usize,limit:usize,state:&NativePagedReservation,maximum:u64)->Result<u64,ValueError>{state.work_demand(storage,index,limit,maximum)}
 fn reserve_step(storage:&mut Self,index:usize,limit:usize,state:&mut NativePagedReservation,control:&mut NativeDecodeControl<'_>,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{state.advance(storage,index,limit,control,maximum,grant)}
 fn push_depth(storage:&Self,_:usize)->Result<usize,ValueError>{storage.next_push_depth_demand().map_err(|e|ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason))}
 fn push_copy_bytes(_:&Self,_:usize)->Result<usize,ValueError>{size_of::<T>().checked_add(size_of::<Option<T>>()).and_then(|n|n.checked_add(2*size_of::<usize>())).ok_or_else(invalid)}
 fn push_work_demand(storage:&Self,index:usize)->Result<u64,ValueError>{if storage.len()!=index{return Err(invalid())}storage.push_reserved_source_work_demand().map_err(|e|ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason))}
 fn push_step(storage:&mut Self,index:usize,input:&mut Option<T>,maximum:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{if storage.len()!=index{return RetainedWorkOutcome{result:Err(invalid()),work:0}}storage.push_reserved_source_step(input,maximum,g)}
 fn finish(storage:Self)->Self{storage}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod parent_tests;
