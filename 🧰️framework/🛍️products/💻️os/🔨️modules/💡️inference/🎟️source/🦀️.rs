//! 🎟️ Explicit owned snapshots and actual Store reads seal once under their original native caller; neither kind manufactures the other.
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetireOwned,ControlledRetirement,RetainedCloneSource,RetainedCloneRef,RetainedCloneStep,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_snapshot_read::SnapshotRead;
/// 🧭️ Original source ownership remains explicit during admission, work and every partial close.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalInferenceSourceKind{Owned,StoreRead}
#[derive(semio_framework_value::RetireOwned)]
enum Input<P:RetireOwned+Send+Sync>{Owned(P),StoreRead(SnapshotRead<P>)}
impl<P:RetireOwned+Send+Sync> Input<P>{fn kind(&self)->OriginalInferenceSourceKind{match self{Self::Owned(_)=>OriginalInferenceSourceKind::Owned,Self::StoreRead(_)=>OriginalInferenceSourceKind::StoreRead}}}
enum Sealed<P:RetireOwned+Send+Sync>{Owned(RetainedCloneSource<P,P>),StoreRead(RetainedCloneSource<P,SnapshotRead<P>>)}
impl<P:RetireOwned+Send+Sync> Sealed<P>{
 fn kind(&self)->OriginalInferenceSourceKind{match self{Self::Owned(_)=>OriginalInferenceSourceKind::Owned,Self::StoreRead(_)=>OriginalInferenceSourceKind::StoreRead}}
 fn borrow(&self)->Option<RetainedCloneRef<'_,P>>{match self{Self::Owned(source)=>source.try_borrow().ok(),Self::StoreRead(source)=>source.try_borrow().ok()}}
 fn project(&self,grant:semio_framework_value::RetainedCloneGrant)->Result<(semio_framework_value::retained_clone::RetainedOwnedProjection<P>,RetainedCloneProgress),ValueError>{match self{Self::Owned(source)=>source.project_owned(0,|snapshot|snapshot,grant),Self::StoreRead(source)=>source.project_owned(0,|snapshot|snapshot,grant)}}
 fn store_lease_identity(&self)->Option<(usize,u16,u64)>{match self{Self::Owned(_)=>None,Self::StoreRead(source)=>source.original_authority().and_then(SnapshotRead::lease_identity)}}
 fn next_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self{Self::Owned(source)=>source.next_close_demand(body),Self::StoreRead(source)=>source.next_close_demand(body)}}
 fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match self{Self::Owned(source)=>source.close_step(grant),Self::StoreRead(source)=>source.close_step(grant)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Owned(source)=>source.terminal_is_empty(),Self::StoreRead(source)=>source.terminal_is_empty()}}
}
/// 🔗️ The actual owned value or original read, sealed projection source and field key retain exact typed custody.
pub struct OriginalInferenceSource<P:RetireOwned+Send+Sync>{
 original:Option<Input<P>>,source:Option<Sealed<P>>,key:Option<String>,key_text:&'static str,kind:OriginalInferenceSourceKind,
 binding:Option<OriginalSourceBinding>,original_close:Option<ControlledRetirement<Input<P>>>,key_close:Option<ControlledRetirement<String>>,closing:bool,
}
impl<P:RetireOwned+Send+Sync> OriginalInferenceSource<P>{
 /// 📥️ Captures a genuinely owned snapshot as metadata; the caller funds this frame before moving its donor.
 pub fn owned(snapshot:P,key:&'static str,binding:OriginalSourceBinding)->Self{Self::new(Input::Owned(snapshot),key,binding)}
 /// 🏪️ Captures the actual Store-issued read without cloning its snapshot or creating another registry.
 pub fn store_read(read:SnapshotRead<P>,key:&'static str,binding:OriginalSourceBinding)->Self{Self::new(Input::StoreRead(read),key,binding)}
 fn new(original:Input<P>,key:&'static str,binding:OriginalSourceBinding)->Self{let kind=original.kind();Self{original:Some(original),source:None,key:None,key_text:key,kind,binding:Some(binding),original_close:None,key_close:None,closing:false}}
 fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.binding.is_some_and(|binding|!binding.matches(cx))||self.original.as_ref().is_some_and(|input|input.kind()!=self.kind)||self.source.as_ref().is_some_and(|source|source.kind()!=self.kind){return Err(changed().into())}Ok(())}
 pub fn source_kind(&self)->OriginalInferenceSourceKind{self.kind}
 /// 📏️ Quotes the selected actual source header or key backing without constructing a substitute owner.
 pub fn admission_demands(&self)->Result<OriginalConstructionDemand,ValueError>{
  if self.closing||self.complete(){return Err(changed())}
  if self.source.is_none(){let capacity_bytes=match self.kind{OriginalInferenceSourceKind::Owned=>RetainedCloneSource::<P,P>::borrowed_constructor_capacity_bytes::<P>(),OriginalInferenceSourceKind::StoreRead=>RetainedCloneSource::<P,SnapshotRead<P>>::borrowed_constructor_capacity_bytes::<SnapshotRead<P>>()};return Ok(OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes,release_bytes:0,depth:1})}
  Ok(OriginalConstructionDemand{items:1,copy_bytes:self.key_text.len(),capacity_bytes:self.key_text.len(),release_bytes:0,depth:1})
 }
 /// 📐️ Quotes the actual original kind's next retirement event, including partial typed custody.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let metadata=||RetirementDemand{depth:1,..Default::default()};
  if let Some(owner)=self.key_close.as_ref(){return if owner.terminal_is_empty(){Ok(metadata())}else{owner.next_demand(body)}}
  if self.key.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<String>(),..metadata()})}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(metadata())}else{source.next_close_demand(body)}}
  if let Some(owner)=self.original_close.as_ref(){return if owner.terminal_is_empty(){Ok(metadata())}else{owner.next_demand(body)}}
  if self.original.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Input<P>>(),..metadata()})}
  Ok(if self.binding.is_some(){metadata()}else{Default::default()})
 }
 /// 🏗️ Seals exactly the original kind and preserves every refused input in its caller-owned source frame.
 pub fn admit_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_context(cx)?;if self.closing{return Err(changed().into())}if self.complete(){return Ok(true)}
  if self.source.is_none(){
   if self.original.as_ref().is_none_or(|input|matches!(input,Input::StoreRead(read)if read.lease_identity().is_none())){return Err(changed().into())}
   let demand=self.admission_demands()?;let grant=cx.retained_grant();let original=&mut self.original;
   admit_original_construction(cx,&mut self.source,demand,|_|match original.take().ok_or_else(changed)?{
    Input::Owned(snapshot)=>match RetainedCloneSource::<P,P>::admit_borrowed(snapshot,|snapshot|snapshot,grant){Ok((source,progress))=>Ok((Sealed::Owned(source),progress)),Err((error,snapshot))=>{*original=Some(Input::Owned(snapshot));Err(error)}},
    Input::StoreRead(read)=>match read.admit_retained_clone_source(grant){Ok((source,progress))=>Ok((Sealed::StoreRead(source),progress)),Err((error,read))=>{*original=Some(Input::StoreRead(read));Err(error)}},
   })?;return Ok(false)
  }
  let count=self.key_text.len();let text=self.key_text;
  admit_original_construction(cx,&mut self.key,OriginalConstructionDemand{items:1,copy_bytes:count,capacity_bytes:count,release_bytes:0,depth:1},|_|{
   let mut key=String::new();key.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original inferred key allocation refused"))?;key.push_str(text);let progress=RetainedCloneProgress{copied_items:1,copied_bytes:count,retained_capacity_bytes:key.capacity(),..Default::default()};Ok((key,progress))
  })?;Ok(self.complete())
 }
 pub fn complete(&self)->bool{!self.closing&&self.source.is_some()&&self.key.is_some()}
 pub fn snapshot(&self)->Option<&P>{self.borrow().map(RetainedCloneRef::get)}
 pub fn borrow(&self)->Option<RetainedCloneRef<'_,P>>{self.source.as_ref().and_then(Sealed::borrow)}
 pub fn key(&self)->Option<&String>{self.key.as_ref()}
 /// 🪪️ Observes the same admitted sealed source ID used by actual projection binding for either ownership kind.
 pub fn source_identity(&self)->Option<u64>{self.borrow().map(RetainedCloneRef::source_identity)}
 /// 🏷️ Store lease observation is explicitly absent for an owned value; no nominal read is manufactured.
 pub fn store_lease_identity(&self)->Option<(usize,u16,u64)>{self.source.as_ref().and_then(Sealed::store_lease_identity)}
 pub fn project_snapshot_step(&self,output:&mut Option<semio_framework_value::retained_clone::RetainedOwnedProjection<P>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_context(cx)?;if !self.complete(){return Err(changed().into())}let grant=cx.retained_grant();let demand=semio_framework_value::retained_clone::RetainedOwnedProjection::<P>::constructor_demand();let source=self.source.as_ref().ok_or_else(changed)?;
  admit_original_construction(cx,output,OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|_|source.project(grant))
 }
 pub fn validate_source(&self,snapshot:&P,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.validate_context(cx)?;if !self.complete()||self.snapshot().is_none_or(|original|!std::ptr::eq(original,snapshot)){return Err(changed().into())}Ok(())}
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Retires the key, actual sealed kind and any partial original before releasing its caller witness.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_context(cx)?;self.begin_close();if cx.retirement_should_yield(){return Ok(false)}
  if let Some(owner)=self.key_close.as_mut(){if !owner.terminal_is_empty(){let demand=owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}metadata(cx,||self.key_close=None)?;return Ok(false)}
  if self.key.is_some(){close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<String>(),depth:1,..Default::default()},|_|{let key=self.key.take().unwrap();match ControlledRetirement::new(key){Ok(owner)=>self.key_close=Some(owner),Err((error,key))=>{self.key=Some(key);return Err(error)}}Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<String>(),..Default::default()}))})?;return Ok(false)}
  if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){let demand=source.next_close_demand(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|source.close_step(grant))?;return Ok(false)}metadata(cx,||self.source=None)?;return Ok(false)}
  if let Some(owner)=self.original_close.as_mut(){if !owner.terminal_is_empty(){let demand=owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}metadata(cx,||self.original_close=None)?;return Ok(false)}
  if self.original.is_some(){close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<Input<P>>(),depth:1,..Default::default()},|_|{let original=self.original.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>self.original_close=Some(owner),Err((error,original))=>{self.original=Some(original);return Err(error)}}Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Input<P>>(),..Default::default()}))})?;return Ok(false)}
  if self.binding.is_some(){metadata(cx,||self.binding=None)?;return Ok(false)}Ok(true)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.original.is_none()&&self.source.is_none()&&self.key.is_none()&&self.original_close.is_none()&&self.key_close.is_none()&&self.binding.is_none()}
}
impl<P:RetireOwned+Send+Sync> Drop for OriginalInferenceSource<P>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original inference source requires paid typed closure")}}
fn changed()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native inference original kind, owner, key or physical caller changed")}
fn metadata(cx:&mut StepContext<'_>,apply:impl FnOnce())->Result<(),OriginalTurnFailure>{close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|{apply();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))})?;Ok(())}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
