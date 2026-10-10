//! 🔏️ Two native tree passes seal one original boxed edit and return its exact owned fields.
use crate::Edit;
use semio_framework_hash::Sha256;
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneSource,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::size_of,sync::Arc};

fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn copy(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn admitted(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items>0&&demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes&&demand.depth<=grant.maximum_depth}
fn child(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||refusal("seal child depth overflow"))?;Ok(demand)}
fn child_grant(mut grant:RetainedCloneGrant)->RetainedCloneGrant{grant.maximum_depth-=1;grant}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{CountStart,Count,Header,HashStart,Hash,TakeOwner,SourceClose,Unwrap,Finalize,Ready}

/// 📦️ Retains the actual original edit through count, framed hash, source closure and paid return.
pub struct ArtifactCanonicalEditSealCursor<M:RetireOwned+ArtifactCanonicalJsonTree>{
 source:Option<RetainedCloneSource<Box<Edit<M>>>>,
 tree:Option<ArtifactCanonicalJsonTreeCursor>,
 returned:Option<Arc<Box<Edit<M>>>>,
 original:Option<Box<Edit<M>>>,
 retirement:Option<ControlledRetirement<Box<Edit<M>>>>,
 phase:Phase,
 closing:bool,
 length:u64,
 hashed:u64,
 header:usize,
 hash:Option<Sha256>,
 last_byte:[u8;1],
 hash_pending:bool,
 digest:Option<[u8;32]>,
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree> ArtifactCanonicalEditSealCursor<M>{
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<Box<Edit<M>>>(),capacity_bytes:RetainedCloneSource::<Box<Edit<M>>>::owned_constructor_capacity_bytes::<()>(),depth:2,release_bytes:0}}
 pub fn admit_original(original:&mut Option<Box<Edit<M>>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  if original.is_none(){return Ok(None);}
  let demand=Self::constructor_demand();if !admitted(demand,grant){return Ok(None);}
  let owner=original.take().unwrap();
  match RetainedCloneSource::admit_owned(owner,(),child_grant(grant)){
   Ok((source,mut progress))=>{progress.copied_bytes=demand.copy_bytes;Ok(Some((Self::from_source(source),progress)))},
   Err((error,owner,_))=>{*original=Some(owner);Err(error)},
  }
 }
 fn from_source(source:RetainedCloneSource<Box<Edit<M>>>)->Self{Self{source:Some(source),tree:None,returned:None,original:None,retirement:None,phase:Phase::CountStart,closing:false,length:0,hashed:0,header:0,hash:Some(Sha256::new()),last_byte:[0],hash_pending:false,digest:None}}
 /// 🎟️ Quotes only the actual native controller and original admitted source handoff, with no second birth.
 pub fn source_constructor_demand()->RetirementDemand{copy(size_of::<Self>()+size_of::<RetainedCloneSource<Box<Edit<M>>>>())}
 /// 🧳️ Adopts the genuine previously admitted immutable source and preserves it unchanged on refusal.
 pub fn admit_source(original:&mut Option<RetainedCloneSource<Box<Edit<M>>>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  if original.is_none(){return Ok(None);}let demand=Self::source_constructor_demand();if !admitted(demand,grant){return Ok(None);}let source=original.as_ref().unwrap();source.try_borrow()?;
  Ok(Some((Self::from_source(original.take().unwrap()),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
 }
 fn source_demands(&self)->Result<RetirementDemand,ValueError>{let source=self.source.as_ref().ok_or_else(||refusal("seal original source is absent"))?;let work=source.next_close_copy_byte_demand()?;child(RetirementDemand{copy_bytes:work,capacity_bytes:source.next_close_capacity_byte_demand(work)?,release_bytes:source.next_close_release_byte_demand()?,depth:source.next_close_depth_demand()?})}
 fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let owner=self.retirement.as_ref().unwrap();let work=owner.next_copy_byte_demand()?;child(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_capacity_byte_demand(work)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
 fn unwrap_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Arc<Box<Edit<M>>>>>()+size_of::<Box<Edit<M>>>(),release_bytes:semio_framework_value::shared_retirement_allocation_bytes::<Box<Edit<M>>>(),depth:1,capacity_bytes:0}}
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{
  if self.hash_pending{return Ok(copy(1));}
  if let Some(tree)=self.tree.as_ref(){return if tree.terminal_is_empty(){Ok(copy(size_of::<Option<ArtifactCanonicalJsonTreeCursor>>()))}else{child(tree.next_demand()?)};}
  if self.closing{
   if self.source.is_some(){return if self.source.as_ref().unwrap().terminal_is_empty(){Ok(copy(size_of::<Option<RetainedCloneSource<Box<Edit<M>>>>>()))}else{self.source_demands()};}
   if self.returned.is_some(){return Ok(Self::unwrap_demand());}
   if self.original.is_some(){return Ok(copy(size_of::<Box<Edit<M>>>()+size_of::<ControlledRetirement<Box<Edit<M>>>>()));}
   if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(copy(size_of::<Option<ControlledRetirement<Box<Edit<M>>>>>() ))}else{self.retirement_demands()};}
   return Ok(Default::default());
  }
  Ok(match self.phase{
   Phase::CountStart|Phase::HashStart=>child(copy(ArtifactCanonicalJsonTreeCursor::constructor_demand().copy_bytes*2))?,
   Phase::Header=>copy(1),
   Phase::TakeOwner=>copy(size_of::<Option<Arc<Box<Edit<M>>>>>()),
   Phase::SourceClose=>if self.source.as_ref().unwrap().terminal_is_empty(){copy(size_of::<Option<RetainedCloneSource<Box<Edit<M>>>>>() )}else{return self.source_demands();},
   Phase::Unwrap=>Self::unwrap_demand(),
   Phase::Finalize=>copy(size_of::<Sha256>()+32),
   Phase::Ready=>Default::default(),
   _=>return Err(refusal("seal traversal owner is absent")),
  })
 }
 fn unwrap_original(&mut self)->Result<(),ValueError>{let owner=self.returned.take().unwrap();match Arc::try_unwrap(owner){Ok(original)=>{self.original=Some(original);Ok(())},Err(owner)=>{self.returned=Some(owner);Err(refusal("seal original retains an unexplained immutable alias"))}}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(tree)=self.tree.as_mut(){tree.begin_close();}}
 pub fn terminal_is_empty(&self)->bool{!self.hash_pending&&self.source.is_none()&&self.tree.is_none()&&self.returned.is_none()&&self.original.is_none()&&self.retirement.is_none()}
 pub fn is_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.original.is_some()}
 pub fn canonical_length(&self)->u64{self.length}
 pub fn digest(&self)->Option<[u8;32]>{if self.is_ready(){self.digest}else{None}}
 pub fn next_take_demand(&self)->RetirementDemand{if self.is_ready(){copy(size_of::<Option<Box<Edit<M>>>>()+32)}else{Default::default()}}
 fn header_byte(&self)->Result<Option<u8>,ValueError>{
  let reference=self.source.as_ref().ok_or_else(||refusal("seal header source is absent"))?.borrow();let id=&reference.get().id;
  let mut ordinal=self.header;
  for bytes in [b"semio.artifact.cursor.v2".as_slice(),&4u64.to_be_bytes(),b"edit".as_slice(),&(id.len()as u64).to_be_bytes(),id.as_bytes(),&self.length.to_be_bytes()]{if ordinal<bytes.len(){return Ok(Some(bytes[ordinal]));}ordinal-=bytes.len();}
  Ok(None)
 }
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.closing&&self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if self.is_ready(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let demand=self.next_demand()?;if !admitted(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.hash_pending{if !self.closing{self.hash.as_mut().unwrap().update(&self.last_byte);self.hashed=self.hashed.checked_add(1).ok_or_else(||refusal("seal hash length overflow"))?;}self.hash_pending=false;progress.copied_bytes=demand.copy_bytes;}
  else if let Some(tree)=self.tree.as_mut(){
   if tree.terminal_is_empty(){self.tree=None;progress.copied_bytes=demand.copy_bytes;if !self.closing{self.phase=if self.phase==Phase::Count{Phase::Header}else{if self.hashed!=self.length{return Err(refusal("seal canonical second pass length changed"));}Phase::TakeOwner};}}
   else{let step=tree.advance(&mut self.last_byte,child_grant(grant))?;progress=step.ownership.progress();if !self.closing&&step.written_bytes!=0{if self.phase==Phase::Count{self.length=self.length.checked_add(1).ok_or_else(||refusal("seal canonical length overflow"))?;}else{self.hash_pending=true;}}}
  }else if self.closing{
   if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source=None;progress.copied_bytes=demand.copy_bytes;}else{progress=source.close_step(child_grant(grant))?.progress();}}
   else if self.returned.is_some(){self.unwrap_original()?;progress.copied_bytes=demand.copy_bytes;progress.released_bytes=demand.release_bytes;}
   else if self.original.is_some(){let original=self.original.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>self.retirement=Some(owner),Err((error,original))=>{self.original=Some(original);return Err(error);}}progress.copied_bytes=demand.copy_bytes;}
   else if let Some(owner)=self.retirement.as_mut(){if owner.terminal_is_empty(){self.retirement=None;progress.copied_bytes=demand.copy_bytes;}else{progress=owner.step(child_grant(grant))?.progress();}}
  }else{match self.phase{
   Phase::CountStart|Phase::HashStart=>{let projection=self.source.as_ref().unwrap().project_owned(0,|edit|edit as&dyn ArtifactCanonicalJsonTree);let(tree,_)=ArtifactCanonicalJsonTreeCursor::admit(projection,child_grant(grant)).unwrap_or_else(|_|unreachable!("seal preadmitted original native tree"));self.tree=Some(tree);self.phase=if self.phase==Phase::CountStart{Phase::Count}else{Phase::Hash};progress.copied_bytes=demand.copy_bytes;},
   Phase::Header=>{if let Some(byte)=self.header_byte()?{self.hash.as_mut().unwrap().update(&[byte]);self.header+=1;}else{self.phase=Phase::HashStart;}progress.copied_bytes=demand.copy_bytes;},
   Phase::TakeOwner=>{self.returned=self.source.as_mut().unwrap().take_owner();if self.returned.is_none(){return Err(refusal("seal original source could not transfer owner"));}self.phase=Phase::SourceClose;progress.copied_bytes=demand.copy_bytes;},
   Phase::SourceClose=>{let source=self.source.as_mut().unwrap();if source.terminal_is_empty(){self.source=None;self.phase=Phase::Unwrap;progress.copied_bytes=demand.copy_bytes;}else{progress=source.close_step(child_grant(grant))?.progress();}},
   Phase::Unwrap=>{self.unwrap_original()?;self.phase=Phase::Finalize;progress.copied_bytes=demand.copy_bytes;progress.released_bytes=demand.release_bytes;},
   Phase::Finalize=>{self.digest=Some(self.hash.take().unwrap().finalize());self.phase=Phase::Ready;progress.copied_bytes=demand.copy_bytes;},
   _=>return Err(refusal("seal phase cannot advance without original traversal")),
  }}
  Ok(if self.is_ready()||self.closing&&self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 pub fn take_edit(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,[u8;32],RetainedCloneProgress)>,ValueError>{if !self.is_ready(){return Ok(None);}let demand=self.next_take_demand();if !admitted(demand,grant){return Ok(None);}Ok(Some((self.original.take().unwrap(),self.digest.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.begin_close();self.advance(grant)}
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree> Drop for ArtifactCanonicalEditSealCursor<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"canonical edit seal abandoned its original native owner");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
