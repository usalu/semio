//! 📸️ ARC1 capture preserves original work and target bindings while writing one funded byte.
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use std::mem::MaybeUninit;
#[derive(Clone,Copy,PartialEq,Eq)]
struct OriginalCheckpointSource{work:usize,target:usize,extent:usize}
/// 🧳️ Original command admission owns the inline plan and target; app work stays immutable until capture and loan end.
pub struct ArtifactCommandCheckpointCursor{work_phase:bool,values:[u64;5],original:Option<OriginalCheckpointSource>,written:usize,complete:bool}
impl ArtifactCommandCheckpointCursor{
 pub fn new(work_phase:bool,values:[u64;5])->Self{Self{work_phase,values,original:None,written:0,complete:false}}
 pub fn written(&self)->usize{self.written}
 pub fn is_complete(&self)->bool{self.complete}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn binding<W:?Sized>(work:&W,target:&[MaybeUninit<u8>])->OriginalCheckpointSource{OriginalCheckpointSource{work:work as*const W as*const()as usize,target:target.as_ptr()as usize,extent:target.len()}}
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"checkpoint capture requires its same original work and initialized target extent")}
 fn byte<W:?Sized>(&self,work:&W,read:fn(&W,usize)->Option<u8>)->Option<u8>{match self.written{0..=3=>Some(b"ARC1"[self.written]),4=>Some(3),5=>Some(u8::from(self.work_phase)),6|7=>Some(0),8..=47=>Some((self.values[(self.written-8)/8]>>(((self.written-8)%8)*8))as u8),_=>read(work,self.written-48)}}
 pub fn advance_demands<W:?Sized>(&self,work:&W,target:&[MaybeUninit<u8>],read:fn(&W,usize)->Option<u8>)->Result<RetirementDemand,ValueError>{if self.original.is_some_and(|original|original!=Self::binding(work,target)){return Err(Self::invalid())}Ok(RetirementDemand{copy_bytes:usize::from(self.original.is_some()&&!self.complete&&self.byte(work,read).is_some()),depth:usize::from(!self.complete),..Default::default()})}
 /// ✍️ Admits one original binding, byte or completion marker; denied independent axes leave all source fields held.
 pub fn advance_one<W:?Sized>(&mut self,work:&W,target:&mut[MaybeUninit<u8>],read:fn(&W,usize)->Option<u8>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.advance_demands(work,target,read)?;if self.complete{return Ok(RetainedCloneStep::Complete(Default::default()))}
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.original.is_none(){self.original=Some(Self::binding(work,target));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if let Some(byte)=self.byte(work,read){target.get_mut(self.written).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"checkpoint capture exceeds its original fixed target extent"))?.write(byte);self.written+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}))}
  self.complete=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 /// 🔎️ Borrows only the exact initialized checkpoint prefix inside the same original target.
 pub fn bytes<'a>(&self,target:&'a[MaybeUninit<u8>])->Result<&'a[u8],ValueError>{if self.original.is_some_and(|original|original.target!=target.as_ptr()as usize||original.extent!=target.len())||self.written>target.len(){return Err(Self::invalid())}Ok(unsafe{std::slice::from_raw_parts(target.as_ptr().cast::<u8>(),self.written)})}
 pub fn retirement_demands(&self)->RetirementDemand{RetirementDemand{depth:usize::from(self.original.is_some()),..Default::default()}}
 /// ♻️ Removes only capture metadata after the caller ends its original checkpoint byte loan.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}self.original=None;self.written=0;self.complete=false;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
}
impl Drop for ArtifactCommandCheckpointCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"checkpoint capture reached Drop before its original metadata removal")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
