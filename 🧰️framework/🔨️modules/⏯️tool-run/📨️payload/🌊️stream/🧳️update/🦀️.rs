//! 🧳️ Original chunk vectors and immutable directory paths remain owned until funded publication or physical close.
use super::{ToolRunPayloadStream,ToolRunPayloadUpdate,invalid};
use crate::ToolRunIdentity;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneSource, RetainedCloneClose, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step}, retirement::{RetireOwned, RetirementCursor, RetirementStep, shared::sealed::{SealedShared, SharedIssuer}}};
use semio_framework_value_persistent::{list::persistent::PersistentList, list::persistent::PersistentSplice, list::persistent::PersistentSpliceKind};
use std::mem::{ManuallyDrop,size_of};
const CONTROL:usize=8*size_of::<usize>()+size_of::<ToolRunIdentity>()+size_of::<ToolRunPayloadUpdate>()+2*size_of::<bool>();
type Original=RetainedCloneSource<PersistentList<Vec<u8>>,SealedShared<ToolRunPayloadStream>>;
type Authored=RetainedCloneSource<SealedShared<Vec<u8>>>;
fn fits(d:RetirementDemand,g:Grant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn scalar(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes+CONTROL,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=CONTROL;d.depth+=1;d}
fn child(g:Grant)->Grant{Grant{maximum_copy_bytes:g.maximum_copy_bytes-CONTROL,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:Progress)->Progress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=CONTROL;p}
fn source_close<T:Sync+'static,A:RetireOwned+Sync>(source:&RetainedCloneSource<T,A>,body:usize)->Result<RetirementDemand,ValueError>{source.next_close_demands(body)}

/// 📬️ A rejected frontier preserves the same original vector, directory root, initialized path and publication identity.
pub struct ToolRunPayloadStreamUpdate{
 input:ManuallyDrop<Option<Vec<u8>>>,previous:ManuallyDrop<Option<SealedShared<ToolRunPayloadStream>>>,displaced:ManuallyDrop<Option<SealedShared<ToolRunPayloadStream>>>,
 seed:ManuallyDrop<Option<ToolRunPayloadStream>>,chunk:ManuallyDrop<Option<SealedShared<Vec<u8>>>>,candidate:ManuallyDrop<Option<ToolRunPayloadStream>>,output:ManuallyDrop<Option<SealedShared<ToolRunPayloadStream>>>,
 original:Option<Original>,authored:Option<Authored>,splice:Option<PersistentSplice<Vec<u8>>>,retirement:RetainedCloneClose,
 identity:ToolRunIdentity,update:ToolRunPayloadUpdate,length:usize,index:usize,phase:u8,closing:bool,
}
impl ToolRunPayloadStreamUpdate{
 pub fn original_input_identity(&self)->Option<usize>{self.input.as_ref().map(|input|input.as_ptr()as usize)}
 pub fn empty()->Self{Self{input:ManuallyDrop::new(None),previous:ManuallyDrop::new(None),displaced:ManuallyDrop::new(None),seed:ManuallyDrop::new(None),chunk:ManuallyDrop::new(None),candidate:ManuallyDrop::new(None),output:ManuallyDrop::new(None),original:None,authored:None,splice:None,retirement:Default::default(),identity:ToolRunIdentity::new(crate::ToolRunId{app_instance_id:0,run:0},[0;32]),update:ToolRunPayloadUpdate::Replace,length:0,index:0,phase:10,closing:true}}
 pub fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Vec<u8>>>()+4*size_of::<Option<SealedShared<ToolRunPayloadStream>>>()+size_of::<ToolRunIdentity>()+size_of::<ToolRunPayloadUpdate>()+size_of::<usize>()+size_of::<u8>()+size_of::<bool>(),depth:1,..Default::default()}}
 pub fn admit(&mut self,previous:&mut Option<SealedShared<ToolRunPayloadStream>>,input:&mut Option<Vec<u8>>,identity:ToolRunIdentity,update:ToolRunPayloadUpdate,g:Grant)->Result<Option<Progress>,ValueError>{
  let d=Self::birth_demand();if !fits(d,g){return Ok(None);}if !self.terminal_is_empty(){return Err(invalid("payload admission requires its original empty owner"));}let length=input.as_ref().ok_or_else(||invalid("payload update has no original vector"))?.len();update.validate(length)?;
  let restart=match update{ToolRunPayloadUpdate::Replace=>true,ToolRunPayloadUpdate::Range{publication,offset,..}=>{if offset==0{if previous.as_ref().is_some_and(|p|p.get().identity==identity&&publication<=p.get().publication){return Err(invalid("payload publication replays an original identity"));}true}else{previous.as_ref().ok_or_else(||invalid("payload continuation has no original directory"))?.get().continuation(identity,update,length)?;false}}};
  let original=previous.take();let(previous,displaced)=if restart{(None,original)}else{(original,None)};*self.input=input.take();*self.previous=previous;*self.displaced=displaced;self.identity=identity;self.update=update;self.length=length;self.index=0;self.phase=if restart{0}else{2};self.closing=false;
  Ok(Some(Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 pub fn complete(&self)->bool{!self.closing&&self.phase==9}
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>9{return Err(invalid("payload preparation is closing or transferred"));}
  Ok(match self.phase{
   0=>scalar(2*size_of::<ToolRunPayloadStream>()),
   1=>RetirementDemand{copy_bytes:2*size_of::<ToolRunPayloadStream>()+CONTROL,capacity_bytes:SealedShared::<ToolRunPayloadStream>::birth_bytes(),depth:1,..Default::default()},
   2=>RetirementDemand{copy_bytes:Original::borrowed_constructor_copy_bytes::<SealedShared<ToolRunPayloadStream>>()+CONTROL,capacity_bytes:Original::borrowed_constructor_capacity_bytes::<SealedShared<ToolRunPayloadStream>>(),depth:2,..Default::default()},
   3=>RetirementDemand{copy_bytes:2*size_of::<Vec<u8>>()+CONTROL,capacity_bytes:SealedShared::<Vec<u8>>::birth_bytes(),depth:1,..Default::default()},
   4=>RetirementDemand{copy_bytes:Authored::constructor_copy_bytes()+CONTROL,capacity_bytes:Authored::owned_constructor_capacity_bytes::<()>(),depth:2,..Default::default()},
   5=>scalar(2*size_of::<PersistentSplice<Vec<u8>>>()),
   6=>nested(self.splice.as_ref().unwrap().demand(self.original.as_ref().unwrap().borrow(),Some(self.authored.as_ref().unwrap().borrow()))?),
   7=>{let mut d=nested(self.splice.as_ref().unwrap().take_demand());d.copy_bytes+=2*size_of::<ToolRunPayloadStream>();d},
   8=>RetirementDemand{copy_bytes:2*size_of::<ToolRunPayloadStream>()+CONTROL,capacity_bytes:SealedShared::<ToolRunPayloadStream>::birth_bytes(),depth:1,..Default::default()},
   9=>Default::default(),_=>return Err(invalid("payload preparation phase is invalid")),
  })
 }
 pub fn advance(&mut self,g:Grant)->Result<Step,ValueError>{
  let d=self.demand()?;if !fits(d,g){return Ok(Step::Progress(Default::default()));}if self.complete(){return Ok(Step::Complete(Default::default()));}
  let mut p=Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   0=>{let(publication,total_bytes,chunk_bytes)=match self.update{ToolRunPayloadUpdate::Replace=>(0,self.length,self.length.max(1)),ToolRunPayloadUpdate::Range{publication,total_bytes,chunk_bytes,..}=>(publication,usize::try_from(total_bytes).map_err(|_|invalid("payload extent exceeds native address range"))?,chunk_bytes as usize)};*self.seed=Some(ToolRunPayloadStream{identity:self.identity,publication,total_bytes,received_bytes:0,chunk_bytes,pages:PersistentList::default()});self.phase=1;},
   1=>{let seed=self.seed.take().unwrap();match SealedShared::admit(seed,SharedIssuer::owned(),g){Ok((source,q))=>{*self.previous=Some(source);p.retained_capacity_bytes=q.retained_capacity_bytes;self.phase=2;},Err((error,seed))=>{*self.seed=Some(seed);return Err(error);}}},
   2=>{let source=self.previous.take().unwrap();self.index=source.get().pages.len();match RetainedCloneSource::admit_borrowed(source,|source|&source.get().pages,child(g)){Ok((source,q))=>{self.original=Some(source);p.retained_capacity_bytes=q.retained_capacity_bytes;self.phase=3;},Err((error,source))=>{*self.previous=Some(source);return Err(error);}}},
   3=>{let input=self.input.take().unwrap();match SealedShared::admit(input,SharedIssuer::owned(),g){Ok((chunk,q))=>{*self.chunk=Some(chunk);p.retained_capacity_bytes=q.retained_capacity_bytes;self.phase=4;},Err((error,input))=>{*self.input=Some(input);return Err(error);}}},
   4=>{let chunk=self.chunk.take().unwrap();match Authored::admit_owned(chunk,(),child(g)){Ok((source,q))=>{self.authored=Some(source);p.retained_capacity_bytes=q.retained_capacity_bytes;self.phase=5;},Err((error,chunk,()))=>{*self.chunk=Some(chunk);return Err(error);}}},
   5=>{self.splice=Some(PersistentSplice::new(PersistentSpliceKind::Insert,self.index));self.phase=6;},
   6=>{let step=self.splice.as_mut().unwrap().advance(self.original.as_ref().unwrap().borrow(),Some(self.authored.as_ref().unwrap().borrow()),child(g))?;if matches!(step,Step::Complete(_)){self.phase=7;}return Ok(Step::Progress(receipt(step.progress())));},
   7=>{let(pages,q)=self.splice.as_mut().unwrap().take(child(g))?.ok_or_else(||invalid("admitted payload directory take was refused"))?;let(publication,total_bytes,chunk_bytes,received_bytes)=match self.update{ToolRunPayloadUpdate::Replace=>(0,self.length,self.length.max(1),self.length),ToolRunPayloadUpdate::Range{publication,total_bytes,offset,chunk_bytes,..}=>(publication,usize::try_from(total_bytes).map_err(|_|invalid("payload extent exceeds native range"))?,chunk_bytes as usize,usize::try_from(offset).map_err(|_|invalid("payload offset exceeds native range"))?.checked_add(self.length).ok_or_else(||invalid("payload received extent overflow"))?)};*self.candidate=Some(ToolRunPayloadStream{identity:self.identity,publication,total_bytes,received_bytes,chunk_bytes,pages});p.retained_capacity_bytes=q.retained_capacity_bytes;self.phase=8;},
   8=>{let candidate=self.candidate.take().unwrap();match SealedShared::admit(candidate,SharedIssuer::owned(),g){Ok((output,q))=>{*self.output=Some(output);p.retained_capacity_bytes=q.retained_capacity_bytes;self.phase=9;},Err((error,candidate))=>{*self.candidate=Some(candidate);return Err(error);}}},
   _=>return Err(invalid("payload preparation lost its admitted phase")),
  }
  Ok(if self.complete(){Step::Complete(p)}else{Step::Progress(p)})
 }
 pub fn take_demand(&self)->RetirementDemand{scalar(2*size_of::<SealedShared<ToolRunPayloadStream>>())}
 pub fn take(&mut self,g:Grant)->Result<Option<(SealedShared<ToolRunPayloadStream>,Progress)>,ValueError>{if !self.complete(){return Err(invalid("payload directory is not ready"));}let d=self.take_demand();if !fits(d,g){return Ok(None);}let result=self.output.take().ok_or_else(||invalid("payload directory output was already transferred"))?;self.phase=10;Ok(Some((result,Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(splice)=self.splice.as_mut(){splice.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(splice)=self.splice.as_ref(){return Ok(if splice.terminal_is_empty(){scalar(size_of::<PersistentSplice<Vec<u8>>>())}else{nested(splice.close_demand(body.saturating_sub(CONTROL))?)});}
  if let Some(source)=self.original.as_ref(){return Ok(if source.terminal_is_empty(){scalar(size_of::<Original>())}else{nested(source_close(source,body.saturating_sub(CONTROL))?)});}
  if let Some(source)=self.authored.as_ref(){return Ok(if source.terminal_is_empty(){scalar(size_of::<Authored>())}else{nested(source_close(source,body.saturating_sub(CONTROL))?)});}
  if !self.retirement.is_empty(){return Ok(nested(self.retirement.next_owner_demands::<Vec<u8>>(false,&None,body.saturating_sub(CONTROL))?));}
  if self.input.is_some(){return Ok(nested(self.retirement.next_owner_demands::<Vec<u8>>(true,&None,body.saturating_sub(CONTROL))?));}
  if self.chunk.is_some(){return Ok(nested(self.retirement.next_owner_demands::<SealedShared<Vec<u8>>>(true,&None,body.saturating_sub(CONTROL))?));}
  if self.previous.is_some()||self.displaced.is_some()||self.output.is_some(){return Ok(nested(self.retirement.next_owner_demands::<SealedShared<ToolRunPayloadStream>>(true,&None,body.saturating_sub(CONTROL))?));}
  if self.seed.is_some()||self.candidate.is_some(){return Ok(nested(self.retirement.next_owner_demands::<ToolRunPayloadStream>(true,&None,body.saturating_sub(CONTROL))?));}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,g:Grant)->Result<Step,ValueError>{
  if !self.closing{return Err(invalid("payload directory closure was not started"));}let d=self.close_demand(g.maximum_copy_bytes)?;if !fits(d,g){return Ok(Step::Progress(Default::default()));}if self.terminal_is_empty(){return Ok(Step::Complete(Default::default()));}
  let p=Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if let Some(splice)=self.splice.as_mut(){if !splice.terminal_is_empty(){return splice.close_step(child(g)).map(|step|Step::Progress(receipt(step.progress())));}self.splice=None;return Ok(Step::Progress(p));}
  if let Some(source)=self.original.as_mut(){if !source.terminal_is_empty(){return source.close_step(child(g)).map(|step|Step::Progress(receipt(step.progress())));}self.original=None;return Ok(Step::Progress(p));}
  if let Some(source)=self.authored.as_mut(){if !source.terminal_is_empty(){return source.close_step(child(g)).map(|step|Step::Progress(receipt(step.progress())));}self.authored=None;return Ok(Step::Progress(p));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(child(g)).map(|step|Step::Progress(receipt(step.progress())));}
  if let Some(step)=self.retirement.begin_granted(&mut self.input,child(g))?{return Ok(Step::Progress(receipt(step.progress())));}
  if let Some(step)=self.retirement.begin_granted(&mut self.chunk,child(g))?{return Ok(Step::Progress(receipt(step.progress())));}
  for slot in [&mut self.previous,&mut self.displaced,&mut self.output]{if let Some(step)=self.retirement.begin_granted(slot,child(g))?{return Ok(Step::Progress(receipt(step.progress())));}}
  for slot in [&mut self.seed,&mut self.candidate]{if let Some(step)=self.retirement.begin_granted(slot,child(g))?{return Ok(Step::Progress(receipt(step.progress())));}}
  Ok(Step::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.input.is_none()&&self.previous.is_none()&&self.displaced.is_none()&&self.seed.is_none()&&self.chunk.is_none()&&self.candidate.is_none()&&self.output.is_none()&&self.original.is_none()&&self.authored.is_none()&&self.splice.is_none()&&self.retirement.is_empty()}
}
impl Drop for ToolRunPayloadStreamUpdate{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"payload publication retains every original vector and path until paid close");}}
struct Close{owner:ToolRunPayloadStreamUpdate}
impl RetireOwned for ToolRunPayloadStreamUpdate{
 fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(Close{owner:self})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Close>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for Close{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owner.close_demand(body)}
 fn close_step(&mut self,g:Grant)->RetirementStep{match self.owner.close_step(g){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
