//! 🧳️ Native receiving constructs retained owners from genuine original input under explicit physical grants.
use crate::{RetainedWorkOutcome,OriginalReceivingPrimary,NativeDecodeControl,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
#[path="🪡️field/🦀️.rs"]mod field;
pub use field::NativeIntrinsicSharedField;
#[path="🧩️record/🦀️.rs"]pub mod record;
pub use record::{NativeRecordKey,NativeRecordField,NativeSchemaLeaf,NativeSchemaKeySource,NativeRecordSchema,NativeFieldMatch,NativeRecordMatcher,NativeRecordStorage};
#[path="🧳️original/🦀️.rs"]mod original;
pub use original::NativeReceivingOriginal;
#[path="📋️list/🦀️.rs"]mod list;
pub use list::{NativeListSource,NativePersistentListReceiver};

#[path="🔗️sealed-shared-receiver/🦀️.rs"]mod shared;
pub use shared::NativeSharedReceiver;
pub use semio_framework_value_native_default::{NativeDefault,NativeDefaultCursor,NativeDefaultValue,NativeDefaultValueCursor,NativeDefaultSlot,NativeArrayDefaultCursor,NativePairDefaultCursor};

#[path="🌳️value/🦀️.rs"]pub mod tree;
#[path="🗂️map/🦀️.rs"]pub mod map;
#[path="🧺️set/🦀️.rs"]pub mod set;

fn receiving_child_body_budget(body:usize,metadata:usize)->usize{body.saturating_sub(metadata)}
/// 📏️ Checks the current decoder allocation ceiling before any retained backing effect.
pub fn receiving_capacity(control:&NativeDecodeControl<'_>,bytes:usize)->Result<(),ValueError>{if control.owned_bytes().checked_add(bytes).is_none_or(|next|next>control.maximum_bytes()){Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decoding ownership exceeds caller limit"))}else{Ok(())}}

/// 🏭️ Native construction declares a required receiving owner independently of synchronous wire decoding.
pub trait NativeReceive<I:?Sized>:RetireOwned+Sized{
 type Receiver:NativeReceiver<I,Self>;
 /// 🌱️ Initializes only an allocation-free fixed receiver header without probing input or constructing defaults.
 fn receiver()->Self::Receiver;
 /// 🪡️ Declares an allocation-free flattened receiver capability before any original input is read.
 fn record_subset_receiver()->Option<Self::Receiver>{None}
}

/// 🧮️ Linear composition retains actual work and every physical effect; it creates no CPU authority or callback.
pub struct NativeReceivingBatch{left:RetainedCloneGrant,progress:RetainedCloneProgress,work:u64}
impl NativeReceivingBatch{
 pub fn new(left:RetainedCloneGrant)->Self{Self{left,progress:Default::default(),work:0}}
 pub fn grant(&self)->RetainedCloneGrant{self.left}
 pub fn finish(self)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(self.progress)),work:self.work}}
 pub fn refuse(self,e:ValueError)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Err(e.with_retained_progress(self.progress).into()),work:self.work}}
 /// 📥️ Ok retains the same remaining batch; Err publishes its sole completed outcome, including a genuine child refusal.
 pub fn record(mut self,turn:RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>,unposted:bool,complete:bool)->Result<Self,RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>>{
  self.work+=turn.work;let step=match turn.result{Ok(s)=>s,Err(mut e)=>{e.progress=self.progress.checked_add(e.progress).expect("actual native progress fits the same grant");return Err(RetainedWorkOutcome{result:Err(e),work:self.work})}};let p=step.progress();self.progress=self.progress.checked_add(p).expect("actual native progress fits the same grant");if complete{return Err(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(self.progress)),work:self.work})}if !unposted||p==RetainedCloneProgress::default(){return Err(self.finish())}
  self.left.maximum_items-=p.copied_items;self.left.maximum_copy_bytes-=p.copied_bytes;self.left.maximum_capacity_bytes-=p.retained_capacity_bytes;self.left.maximum_release_bytes-=p.released_bytes;if self.left.maximum_items==0{Err(self.finish())}else{Ok(self)}
 }
}

/// 🧬️ Schema membership quotes every fixed key comparison before a flattened original record read.
pub trait NativeRecordFields{
 fn native_schema_count()->usize;
 fn native_schema_field(index:usize)->Option<NativeRecordSchema>;
}

/// 🫴️ The original input remains held until staged output and every alias complete funded transfer or closure.
pub trait NativeReceiver<I:?Sized,T>:Send+'static{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>;
 fn output_ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome;
 fn terminal_is_empty(&self)->bool;
}




#[path="🔄️map/🦀️.rs"]
pub mod mapped;
pub use mapped::{NativeReceiveMap,NativeMappedReceiver};



fn native_receiver_backing_demands<I:?Sized,T,C:NativeReceiver<I,T>>(storage:&semio_framework_value_native_storage::NativeOwnedStorage<C>,retired:&Option<Box<C>>,body:usize)->Result<RetirementDemand,ValueError>{
 if let Some(cursor)=retired.as_ref(){if !cursor.terminal_is_empty(){return Err(tree::invalid())}return Ok(RetirementDemand{release_bytes:std::alloc::Layout::new::<C>().size(),depth:1,..Default::default()})}if let Some(cursor)=storage.get(){if !cursor.terminal_is_empty(){return tree::nested(cursor.close_demands(body)?,0)}return Ok(semio_framework_value_native_storage::NativeOwnedStorage::<C>::move_demand())}Ok(storage.release_demand())
}
/// ♻️ Releases only a proven empty cursor's existing allocation and executes no header Drop callbacks.
fn close_native_receiver_backing<I:?Sized,T,C:NativeReceiver<I,T>>(storage:&mut semio_framework_value_native_storage::NativeOwnedStorage<C>,retired:&mut Option<Box<C>>,g:RetainedCloneGrant)->RetainedWorkOutcome{
 let d=match native_receiver_backing_demands::<I,T,C>(storage,retired,g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !crate::retained_clone::retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if retired.is_some(){let layout=std::alloc::Layout::new::<C>();let pointer=Box::into_raw(match retired.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("retired absent").into()),work:0}});if layout.size()!=0{unsafe{std::alloc::dealloc(pointer.cast(),layout)}}return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}}if let Some(cursor)=storage.get_mut(){if !cursor.terminal_is_empty(){let turn=cursor.close_step(tree::child_grant(g,0));return RetainedWorkOutcome{result:turn.result.map(|s|RetainedCloneStep::Progress(s.progress())),work:turn.work}}let turn=storage.take_box(g);let work=turn.work;let result=turn.result.map(|value|if let Some((owner,p))=value{*retired=Some(owner);RetainedCloneStep::Progress(p)}else{RetainedCloneStep::Progress(Default::default())});return RetainedWorkOutcome{result,work}}storage.release(g)
}

#[cfg(test)]#[path="♻️header/🎟️work/🧪️tests/🦀️.rs"]mod header_retirement_tests;
