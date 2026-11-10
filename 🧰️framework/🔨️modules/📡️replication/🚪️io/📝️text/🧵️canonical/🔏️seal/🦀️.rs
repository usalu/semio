//! 🔏️ Two native tree passes seal one original boxed edit and return its exact owned fields.
use crate::Edit;
use semio_framework_hash::Sha256;
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneBinding,RetainedCloneSource,OriginalEffectReply,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;

fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn copy(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn admitted(demand:RetirementDemand,grant:RetainedCloneGrant)->bool{grant.maximum_items>0&&demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes&&demand.depth<=grant.maximum_depth}
fn child(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||refusal("seal child depth overflow"))?;Ok(demand)}
fn child_grant(mut grant:RetainedCloneGrant)->RetainedCloneGrant{grant.maximum_depth-=1;grant}

/// 🧾️ Borrows the original canonical edit seal framing independently of source ownership.
pub fn canonical_edit_seal_header_byte(id:&str,length:u64,mut offset:usize)->Option<u8>{for bytes in [b"semio.artifact.cursor.v2".as_slice(),&4u64.to_be_bytes(),b"edit".as_slice(),&(id.len()as u64).to_be_bytes(),id.as_bytes(),&length.to_be_bytes()]{if offset<bytes.len(){return Some(bytes[offset]);}offset-=bytes.len();}None}

/// 📄️ The window one canonical turn fills and one hash turn consumes; it lives inline in the cursor, so it is bounded by the Store contract that a sealer constructor needs at most 4096 bytes of capacity.
pub const SEAL_PAGE_BYTES:usize=1024;
/// 📃️ The free window below which a partly filled page is hashed instead of starving the next canonical turn.
const SEAL_FLUSH_MARGIN_BYTES:usize=64;
const SEAL_TURN_BYTES:usize=256;

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{CountStart,Count,Header,HashStart,Hash,TakeOwner,SourceClose,Finalize,Ready}

/// 📦️ Retains the actual original edit through count, framed hash, source closure and paid return.
pub struct ArtifactCanonicalEditSealCursor<M:RetireOwned+ArtifactCanonicalJsonTree>{
 source:Option<RetainedCloneSource<Box<Edit<M>>>>,
 tree:Option<ArtifactCanonicalJsonTreeCursor>,
 original:Option<Box<Edit<M>>>,
 retirement:Option<ControlledRetirement<Box<Edit<M>>>>,
 phase:Phase,
 closing:bool,
 length:u64,
 hashed:u64,
 header:usize,
 hash:Option<Sha256>,
 page:[u8;SEAL_PAGE_BYTES],
 page_length:usize,
 hash_pending:bool,
 buffered:bool,
 digest:Option<[u8;32]>,
 last_work:u128,
 hash_offset:usize,
 take_reply:Option<Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>>,
 take_pending:bool,
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree> ArtifactCanonicalEditSealCursor<M>{
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<Box<Edit<M>>>(),capacity_bytes:RetainedCloneSource::<Box<Edit<M>>>::owned_constructor_capacity_bytes::<()>(),depth:2,release_bytes:0}}
 fn from_source(source:RetainedCloneSource<Box<Edit<M>>>)->Self{Self{source:Some(source),tree:None,original:None,retirement:None,phase:Phase::CountStart,closing:false,length:0,hashed:0,header:0,hash:Some(Sha256::new()),page:[0;SEAL_PAGE_BYTES],page_length:0,hash_pending:false,buffered:true,digest:None,last_work:0,hash_offset:0,take_reply:None,take_pending:false}}
 /// 🎟️ Quotes only the actual native controller and original admitted source handoff, with no second birth.
 pub fn source_constructor_demand()->RetirementDemand{copy(size_of::<Self>()+size_of::<RetainedCloneSource<Box<Edit<M>>>>())}
 /// 🧳️ Adopts the genuine previously admitted immutable source and preserves it unchanged on refusal.
 pub fn admit_source(original:&mut Option<RetainedCloneSource<Box<Edit<M>>>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  if original.is_none(){return Ok(None);}let demand=Self::source_constructor_demand();if !admitted(demand,grant){return Ok(None);}let source=original.as_ref().unwrap();source.try_borrow()?;
  Ok(Some((Self::from_source(original.take().unwrap()),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
 }
 fn source_demands(&self)->Result<RetirementDemand,ValueError>{let source=self.source.as_ref().ok_or_else(||refusal("seal original source is absent"))?;let work=source.next_close_copy_byte_demand()?;child(RetirementDemand{copy_bytes:work,capacity_bytes:source.next_close_capacity_byte_demand(work)?,release_bytes:source.next_close_release_byte_demand()?,depth:source.next_close_depth_demand()?})}
 fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let owner=self.retirement.as_ref().unwrap();let work=owner.next_copy_byte_demand()?;child(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_capacity_byte_demand(work)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
 fn take_demands(&self)->Result<RetirementDemand,ValueError>{let source=self.source.as_ref().ok_or_else(||refusal("seal original source is absent"))?;let work=source.next_take_copy_byte_demand()?;child(RetirementDemand{copy_bytes:work,capacity_bytes:source.next_take_capacity_byte_demand(work)?,release_bytes:source.next_take_release_byte_demand()?,depth:source.next_take_depth_demand()?})}
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{
  if self.hash_pending{return Ok(copy((self.page_length-self.hash_offset).min(SEAL_TURN_BYTES)));}
  if let Some(tree)=self.tree.as_ref(){return if tree.terminal_is_empty(){Ok(copy(size_of::<Option<ArtifactCanonicalJsonTreeCursor>>()))}else{child(tree.next_demand()?)};}
  if self.closing{
   if self.source.is_some(){return if self.source.as_ref().unwrap().terminal_is_empty(){Ok(copy(size_of::<Option<RetainedCloneSource<Box<Edit<M>>>>>()))}else{self.source_demands()};}
   if self.original.is_some(){return Ok(copy(size_of::<Box<Edit<M>>>()+size_of::<ControlledRetirement<Box<Edit<M>>>>()));}
   if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(copy(size_of::<Option<ControlledRetirement<Box<Edit<M>>>>>() ))}else{self.retirement_demands()};}
   return Ok(Default::default());
  }
  Ok(match self.phase{
   Phase::CountStart|Phase::HashStart=>child(copy(RetainedCloneBinding::alias_copy_bytes()+ArtifactCanonicalJsonTreeCursor::constructor_demand().copy_bytes))?,
   Phase::Header=>copy(1),
   Phase::TakeOwner=>self.take_demands()?,
   Phase::SourceClose=>if self.source.as_ref().unwrap().terminal_is_empty(){copy(size_of::<Option<RetainedCloneSource<Box<Edit<M>>>>>() )}else{return self.source_demands();},
   Phase::Finalize=>copy(size_of::<Sha256>()+32),
   Phase::Ready=>Default::default(),
   _=>return Err(refusal("seal traversal owner is absent")),
  })
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(tree)=self.tree.as_mut(){tree.begin_close();}}
 pub fn terminal_is_empty(&self)->bool{!self.hash_pending&&self.source.is_none()&&self.tree.is_none()&&self.original.is_none()&&self.retirement.is_none()&&!self.take_pending}
 pub fn is_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.original.is_some()}
 pub fn canonical_length(&self)->u64{self.length}
 pub fn digest(&self)->Option<[u8;32]>{if self.is_ready(){self.digest}else{None}}
 pub fn next_take_demand(&self)->RetirementDemand{if self.is_ready(){copy(size_of::<Option<Box<Edit<M>>>>()+32)}else{Default::default()}}
 /// ⛽️ Quotes the bounded byte window or fixed ownership transitions before original caller reservation.
 pub fn next_work_demand(&self)->Result<u64,ValueError>{
  Ok(if self.terminal_is_empty()||self.is_ready(){0}else if self.hash_pending{(self.page_length-self.hash_offset).min(SEAL_TURN_BYTES).max(1)as u64}else if self.tree.is_some()||!self.closing&&self.phase==Phase::Header{SEAL_TURN_BYTES as u64}else{2})
 }
 /// 🧮️ Reports the complete most recent call, including failed child work and zero on admission denial.
 pub fn last_turn_work(&self)->u128{self.last_work}
 fn header_byte(&self)->Result<Option<u8>,ValueError>{
  let reference=self.source.as_ref().ok_or_else(||refusal("seal header source is absent"))?.borrow();let id=&reference.get().id;
  Ok(canonical_edit_seal_header_byte(id,self.length,self.header))
 }
 pub fn advance(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
  use semio_framework_value::RetirementPollExit;let grant=effect.grant();let base=effect.performed_work();
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||->Result<RetainedCloneStep,RetirementPollExit>{
  self.last_work=0;
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if self.is_ready(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let demand=self.next_demand()?;if !admitted(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  self.last_work=1;
  if self.hash_pending{if !self.closing{let end=(self.hash_offset+SEAL_TURN_BYTES).min(self.page_length);let length=end-self.hash_offset;self.last_work=length.max(1)as u128;self.hash.as_mut().unwrap().update(&self.page[self.hash_offset..end]);self.hash_offset=end;self.hashed=self.hashed.checked_add(length as u64).ok_or_else(||refusal("seal hash length overflow"))?;}progress.copied_bytes=demand.copy_bytes;if self.closing||self.hash_offset==self.page_length{self.hash_pending=false;self.page_length=0;self.hash_offset=0;if self.buffered&&!self.closing{self.buffered=false;if self.hashed!=self.length{return Err(refusal("seal buffered second pass length changed").into());}}}}
  else if let Some(tree)=self.tree.as_mut(){
   if tree.terminal_is_empty(){self.tree=None;progress.copied_bytes=demand.copy_bytes;if !self.closing{self.phase=if self.phase==Phase::Count{Phase::Header}else{if self.hashed!=self.length{return Err(refusal("seal canonical second pass length changed").into());}Phase::TakeOwner};}}
   else{let window=if self.phase==Phase::Count&&!self.buffered{0}else{self.page_length};let end=(window+SEAL_TURN_BYTES).min(SEAL_PAGE_BYTES);let turn=tree.advance(&mut self.page[window..end],&mut effect.nested(child_grant(grant)));self.last_work=u128::from(tree.last_turn_work());let step=turn?;progress=step.ownership.progress();if !self.closing&&step.written_bytes!=0{if self.phase==Phase::Count{self.length=self.length.checked_add(step.written_bytes as u64).ok_or_else(||refusal("seal canonical length overflow"))?;if self.buffered{self.page_length+=step.written_bytes;if !tree.terminal_is_empty()&&SEAL_PAGE_BYTES-self.page_length<SEAL_FLUSH_MARGIN_BYTES{self.buffered=false;self.page_length=0;}}}else{self.page_length+=step.written_bytes;if SEAL_PAGE_BYTES-self.page_length<SEAL_FLUSH_MARGIN_BYTES{self.hash_pending=true;}}}if !self.closing&&self.phase!=Phase::Count&&self.page_length!=0&&tree.terminal_is_empty(){self.hash_pending=true;}}
  }else if self.closing{
   if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source=None;progress.copied_bytes=demand.copy_bytes;}else{let turn=RetirementPollExit::settled(source.close_step(effect.grant().maximum_copy_bytes,&mut effect.nested(child_grant(grant))))?;self.last_work=turn.work;progress=turn.result?.progress();}}
   else if self.original.is_some(){let original=self.original.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>self.retirement=Some(owner),Err((error,original))=>{self.original=Some(original);return Err(error.into());}}progress.copied_bytes=demand.copy_bytes;}
   else if let Some(owner)=self.retirement.as_mut(){if owner.terminal_is_empty(){self.retirement=None;progress.copied_bytes=demand.copy_bytes;}else{let turn=RetirementPollExit::settled(owner.step(effect.grant().maximum_copy_bytes,&mut effect.nested(child_grant(grant))))?;self.last_work=turn.work;progress=turn.result?.progress();}}
  }else{match self.phase{
   Phase::CountStart|Phase::HashStart=>{
    let reference=self.source.as_ref().ok_or_else(||ValueError::invariant("source absent"))?.borrow().try_project(0,|edit|Ok::<&dyn ArtifactCanonicalJsonTree,ValueError>(edit as &dyn ArtifactCanonicalJsonTree))?;
    let before=effect.performed_work();
    match semio_framework_pack_json::retained_alias(reference,&mut effect.nested(child_grant(grant)))?{
     None=>{self.last_work=effect.performed_work()-before;progress=Default::default();}
     Some((projection,bound))=>{self.last_work=effect.performed_work()-before+1;let(tree,built)=ArtifactCanonicalJsonTreeCursor::admit(projection,child_grant(grant)).map_err(|_|ValueError::invariant("seal preadmitted original native tree"))?;self.tree=Some(tree);self.phase=if self.phase==Phase::CountStart{Phase::Count}else{Phase::Hash};progress=bound.checked_add(built)?;progress.copied_items=1;progress.copied_bytes=demand.copy_bytes;}
    }
   },
   Phase::Header=>{let limit=grant.maximum_copy_bytes.min(SEAL_TURN_BYTES);let mut taken=0usize;self.last_work=0;while taken<limit{self.last_work+=1;if let Some(byte)=self.header_byte()?{self.hash.as_mut().unwrap().update(&[byte]);self.header+=1;taken+=1;}else{if self.buffered{if self.page_length as u64!=self.length{return Err(refusal("seal buffered canonical bytes differ from their counted length").into());}self.hash_pending=true;self.phase=Phase::TakeOwner;}else{self.phase=Phase::HashStart;}break;}}progress.copied_bytes=taken.max(demand.copy_bytes);},
   Phase::TakeOwner=>{
    let body=grant.maximum_copy_bytes;let before=effect.performed_work();let source=self.source.as_mut().ok_or_else(||ValueError::invariant("source absent"))?;
    if self.take_reply.is_none(){let reply=OriginalEffectReply::borrow(&mut self.take_reply)?;let done=if self.take_pending{source.recover_take_authority_into(&mut self.original,body,effect,reply)?}else{source.take_authority_into(&mut self.original,body,effect,reply)?};drop(done);}
    self.last_work=effect.performed_work()-before;
    match self.take_reply.take(){
     None=>{self.take_pending=true;progress=Default::default();}
     Some(result)=>{self.take_pending=false;match result?{RetainedCloneStep::Progress(step)=>progress=step,RetainedCloneStep::Complete(step)=>{self.phase=Phase::SourceClose;progress=step;}}}
    }
   },
   Phase::SourceClose=>{let source=self.source.as_mut().ok_or_else(||ValueError::invariant("source absent"))?;if source.terminal_is_empty(){self.source=None;self.phase=Phase::Finalize;progress.copied_bytes=demand.copy_bytes;}else{let turn=RetirementPollExit::settled(source.close_step(effect.grant().maximum_copy_bytes,&mut effect.nested(child_grant(grant))))?;self.last_work=turn.work;progress=turn.result?.progress();}},
   Phase::Finalize=>{self.digest=Some(self.hash.take().unwrap().finalize());self.phase=Phase::Ready;progress.copied_bytes=demand.copy_bytes;},
   _=>return Err(refusal("seal phase cannot advance without original traversal").into()),
  }}
  Ok(if self.is_ready()||self.closing&&self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 
  })();let work=self.last_work;crate::canonical_fields_settle(effect,base,result,work)
 }
 pub fn take_edit(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,[u8;32],RetainedCloneProgress)>,ValueError>{self.last_work=0;if !self.is_ready(){return Ok(None);}let demand=self.next_take_demand();if !admitted(demand,grant){return Ok(None);}self.last_work=1;Ok(Some((self.original.take().unwrap(),self.digest.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{self.begin_close();self.advance(effect)}
}
impl<M:RetireOwned+ArtifactCanonicalJsonTree> Drop for ArtifactCanonicalEditSealCursor<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"canonical edit seal abandoned its original native owner");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
