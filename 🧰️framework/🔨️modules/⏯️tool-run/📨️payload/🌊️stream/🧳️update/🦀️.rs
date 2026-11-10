//! 🧳️ Original chunk vectors and immutable directory paths remain owned until funded publication or physical close.
use super::{ToolRunPayloadStream,ToolRunPayloadUpdate,invalid};
use crate::ToolRunIdentity;
use semio_framework_tool_run_entities::{EntityComplete as Complete,EntityReply as Reply,denied,funded,local_step,route_fault,source_hold::SourceHold};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure,RetirementDemand,ValueError,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalProjectionReply,OriginalSourceProjection,RetainedCloneSource,RetainedCloneClose,StagedSourceAdmission,RetainedAllocationEffect,RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,retained_clone_grant_admits},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use semio_framework_value::shared_issuance::SealedShared;
use semio_framework_value_persistent::{list::persistent::PersistentList, list::persistent::PersistentSplice, list::persistent::PersistentSpliceKind};
use std::{mem::{ManuallyDrop,size_of},ptr::NonNull};
const CONTROL:usize=8*size_of::<usize>()+size_of::<ToolRunIdentity>()+size_of::<ToolRunPayloadUpdate>()+2*size_of::<bool>();
type Authored=RetainedCloneSource<SealedShared<Vec<u8>>>;
type AuthoredAdmission=StagedSourceAdmission<SealedShared<Vec<u8>>,SealedShared<Vec<u8>>>;
type Pages=PersistentList<Vec<u8>>;
type Hold=SourceHold<Pages,PayloadOriginal>;
type ValueReply<'r>=OriginalEffectReply<'r,Step,ValueError>;
const HOLD:u8=2;const SPLICE_CLOSE:u8=20;const AUTHORED_CLOSE:u8=21;const RETIREMENT:u8=22;
fn scalar(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes+CONTROL,depth:1,..Default::default()}}
fn item(d:RetirementDemand)->Progress{Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn source_close<T:Sync+'static,A:RetireOwned+Sync>(source:&RetainedCloneSource<T,A>,body:usize)->Result<RetirementDemand,ValueError>{source.next_close_demand(body)}
fn settled<'r>(reply:Reply<'r>,result:Result<Step,ValueError>)->Result<Complete<'r>,ValueError>{Ok(reply.complete(result.map_err(OriginalRetirementFailure::from)))}

/// 🪪️ Projects the page directory out of the published original stream owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct PayloadOriginal(SealedShared<ToolRunPayloadStream>);
impl OriginalSourceProjection<Pages> for PayloadOriginal{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,Pages>)->Result<OriginalEffectReplyComplete<'reply,NonNull<Pages>,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(invalid("payload directory projection exceeded its original work"))));}Ok(reply.complete(Ok(&self.0.get().pages)))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,Pages>)->Result<OriginalEffectReplyComplete<'reply,NonNull<Pages>,ValueError>,ValueError>{Ok(reply.pending())}
}

macro_rules! forward{($owner:ident,$key:expr,$target:expr,$loan:expr,$recover:expr,$body:expr,$effect:expr,$reply:expr,$plain:ident,$rplain:ident,$orig:ident,$rorig:ident)=>{{
 let mut scope=$reply.scope(&mut $owner.route,$key)?;
 let done=match($loan,$recover){(None,false)=>$target.$plain($body,$effect,scope.borrow()?)?,(None,true)=>$target.$rplain($body,$effect,scope.borrow()?)?,(Some(loan),false)=>$target.$orig(loan,$body,$effect,scope.borrow()?)?,(Some(loan),true)=>$target.$rorig(loan,$body,$effect,scope.borrow()?)?};
 drop(done);Ok(scope.forwarded())
}}}

/// 📬️ A rejected frontier preserves the same original vector, directory root, initialized path and publication identity.
pub struct ToolRunPayloadStreamUpdate{
 input:ManuallyDrop<Option<Vec<u8>>>,previous:ManuallyDrop<Option<SealedShared<ToolRunPayloadStream>>>,displaced:ManuallyDrop<Option<SealedShared<ToolRunPayloadStream>>>,
 seed:ManuallyDrop<Option<ToolRunPayloadStream>>,chunk:ManuallyDrop<Option<SealedShared<Vec<u8>>>>,candidate:ManuallyDrop<Option<ToolRunPayloadStream>>,output:ManuallyDrop<Option<SealedShared<ToolRunPayloadStream>>>,
 hold:Option<Hold>,adm_authored:Option<AuthoredAdmission>,authored:Option<Authored>,splice:Option<PersistentSplice<Vec<u8>>>,retirement:RetainedCloneClose,
 identity:ToolRunIdentity,update:ToolRunPayloadUpdate,length:usize,index:usize,phase:u8,closing:bool,route:Option<u8>,
}
impl ToolRunPayloadStreamUpdate{
 pub fn original_input_identity(&self)->Option<usize>{self.input.as_ref().map(|input|input.as_ptr()as usize)}
 pub fn empty()->Self{Self{input:ManuallyDrop::new(None),previous:ManuallyDrop::new(None),displaced:ManuallyDrop::new(None),seed:ManuallyDrop::new(None),chunk:ManuallyDrop::new(None),candidate:ManuallyDrop::new(None),output:ManuallyDrop::new(None),hold:None,adm_authored:None,authored:None,splice:None,retirement:Default::default(),identity:ToolRunIdentity::new(crate::ToolRunId{app_instance_id:0,run:0},[0;32]),update:ToolRunPayloadUpdate::Replace,length:0,index:0,phase:10,closing:true,route:None}}
 pub fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Vec<u8>>>()+4*size_of::<Option<SealedShared<ToolRunPayloadStream>>>()+size_of::<ToolRunIdentity>()+size_of::<ToolRunPayloadUpdate>()+size_of::<usize>()+size_of::<u8>()+size_of::<bool>(),depth:1,..Default::default()}}
 pub fn admit(&mut self,previous:&mut Option<SealedShared<ToolRunPayloadStream>>,input:&mut Option<Vec<u8>>,identity:ToolRunIdentity,update:ToolRunPayloadUpdate,g:Grant)->Result<Option<Progress>,ValueError>{
  let d=Self::birth_demand();if !retained_clone_grant_admits(g,d){return Ok(None);}if !self.terminal_is_empty(){return Err(invalid("payload admission requires its original empty owner"));}let length=input.as_ref().ok_or_else(||invalid("payload update has no original vector"))?.len();update.validate(length)?;
  let restart=match update{ToolRunPayloadUpdate::Replace=>true,ToolRunPayloadUpdate::Range{publication,offset,..}=>{if offset==0{if previous.as_ref().is_some_and(|p|p.get().identity==identity&&publication<=p.get().publication){return Err(invalid("payload publication replays an original identity"));}true}else{previous.as_ref().ok_or_else(||invalid("payload continuation has no original directory"))?.get().continuation(identity,update,length)?;false}}};
  let original=previous.take();let(previous,displaced)=if restart{(None,original)}else{(original,None)};*self.input=input.take();*self.previous=previous;*self.displaced=displaced;self.identity=identity;self.update=update;self.length=length;self.index=0;self.phase=if restart{0}else{2};self.closing=false;
  Ok(Some(Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 fn phase_now(&self)->u8{if self.phase==HOLD&&self.hold.as_ref().is_some_and(Hold::ready){3}else{self.phase}}
 pub fn complete(&self)->bool{!self.closing&&self.phase==9}
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>9{return Err(invalid("payload preparation is closing or transferred"));}
  Ok(match self.phase_now(){
   0=>scalar(2*size_of::<ToolRunPayloadStream>()),
   1=>SealedShared::<ToolRunPayloadStream>::source_admission_demand()?,
   2=>match self.hold.as_ref(){None=>scalar(size_of::<Hold>()),Some(hold)=>hold.quote(body)?},
   3=>SealedShared::<Vec<u8>>::source_admission_demand()?,
   4=>match self.adm_authored.as_ref(){Some(admission)=>if admission.is_projecting(){admission.owned_projection_demand()?}else{admission.next_demand()?},None=>scalar(size_of::<AuthoredAdmission>())},
   5=>scalar(2*size_of::<PersistentSplice<Vec<u8>>>()),
   6=>self.splice.as_ref().ok_or_else(||invalid("payload splice is absent"))?.demand(self.hold.as_ref().ok_or_else(||invalid("payload original hold is absent"))?.original()?,Some(self.authored.as_ref().ok_or_else(||invalid("payload authored source is absent"))?.borrow()))?,
   7=>self.splice.as_ref().ok_or_else(||invalid("payload splice is absent"))?.take_demand(),
   8=>SealedShared::<ToolRunPayloadStream>::source_admission_demand()?,
   9=>Default::default(),_=>return Err(invalid("payload preparation phase is invalid")),
  })
 }
 pub fn advance_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(body,effect,reply,false)}
 pub fn recover_advance_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(body,effect,reply,true)}
 fn finish(&self,progress:Progress)->Step{if self.complete(){Step::Complete(progress)}else{Step::Progress(progress)}}
 fn drive<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  if self.route.is_some()!=recover{return Err(route_fault("payload update changed its original recovery route"));}
  if self.complete(){return Ok(reply.complete(Ok(Step::Complete(Default::default()))));}
  self.phase=self.phase_now();
  let d=if recover{Default::default()}else{self.demand(body)?};
  if !recover&&!retained_clone_grant_admits(effect.grant(),d){return denied(reply);}
  let phase=self.phase;
  match phase{
   0=>local_step(effect,d,1,reply,|_|{let(publication,total_bytes,chunk_bytes)=match self.update{ToolRunPayloadUpdate::Replace=>(0,self.length,self.length.max(1)),ToolRunPayloadUpdate::Range{publication,total_bytes,chunk_bytes,..}=>(publication,usize::try_from(total_bytes).map_err(|_|invalid("payload extent exceeds native address range"))?,chunk_bytes as usize)};*self.seed=Some(ToolRunPayloadStream{identity:self.identity,publication,total_bytes,received_bytes:0,chunk_bytes,pages:PersistentList::default()});self.phase=1;Ok(item(d))}),
   1=>{
    if !funded(effect,d,SealedShared::<ToolRunPayloadStream>::source_admission_work_demand())?{return denied(reply);}
    let seed=self.seed.take().ok_or_else(||ValueError::invariant("seed absent"))?;
    match crate::component::admit_sealed(seed,effect){Ok((source,progress))=>{*self.previous=Some(source);self.phase=2;settled(reply,Ok(self.finish(progress)))},Err((error,seed))=>{*self.seed=Some(seed);settled(reply,Err(error))}}
   },
   2=>if self.hold.is_none(){
    local_step(effect,d,1,reply,|_|{let source=self.previous.take().ok_or_else(||ValueError::invariant("previous absent"))?;self.index=source.get().pages.len();self.hold=Some(SourceHold::new(PayloadOriginal(source)));Ok(item(d))})
   }else{
    let hold=self.hold.as_mut().ok_or_else(||route_fault("payload hold is absent"))?;
    let mut scope=reply.scope(&mut self.route,HOLD)?;drop(hold.step_into(body,effect,scope.borrow()?,recover)?);Ok(scope.forwarded())
   },
   3=>{
    if !funded(effect,d,SealedShared::<Vec<u8>>::source_admission_work_demand())?{return denied(reply);}
    let input=self.input.take().ok_or_else(||ValueError::invariant("input absent"))?;
    match crate::component::admit_sealed(input,effect){Ok((chunk,progress))=>{*self.chunk=Some(chunk);self.phase=4;settled(reply,Ok(self.finish(progress)))},Err((error,input))=>{*self.input=Some(input);settled(reply,Err(error))}}
   },
   4=>match self.adm_authored.as_mut(){
    None=>local_step(effect,d,1,reply,|_|{let chunk=self.chunk.take().ok_or_else(||ValueError::invariant("chunk absent"))?;self.adm_authored=Some(StagedSourceAdmission::owned(chunk));Ok(item(d))}),
    Some(admission)=>{let turn=if admission.is_projecting(){admission.project_owned(effect)}else{admission.advance(&mut self.authored,effect)};let done=admission.terminal_is_empty();match turn.result{Ok(step)=>{if done{self.adm_authored=None;self.phase=5;}settled(reply,Ok(self.finish(step.progress())))},Err(error)=>settled(reply,Err(error))}}
   },
   5=>local_step(effect,d,1,reply,|_|{self.splice=Some(PersistentSplice::new(PersistentSpliceKind::Insert,self.index));self.phase=6;Ok(item(d))}),
   6=>{
    let splice=self.splice.as_mut().ok_or_else(||ValueError::invariant("splice absent"))?;
    let original=self.hold.as_ref().ok_or_else(||ValueError::invariant("original absent"))?.original()?;
    let authored=self.authored.as_ref().ok_or_else(||ValueError::invariant("authored absent"))?.borrow();
    let mut slot=None;drop(splice.advance_into(original,Some(authored),effect,ValueReply::borrow(&mut slot)?)?);
    let result=slot.ok_or_else(||route_fault("payload splice advance suspended"))?;
    if matches!(result,Ok(Step::Complete(_))){self.phase=7;}
    settled(reply,result.map(|step|Step::Progress(step.progress())))
   },
   7=>{
    let splice=self.splice.as_mut().ok_or_else(||ValueError::invariant("splice absent"))?;
    let mut pages=None;let mut slot=None;drop(splice.take_into(&mut pages,effect,ValueReply::borrow(&mut slot)?)?);
    let result=slot.ok_or_else(||route_fault("payload splice take suspended"))?;
    match result{Ok(step)=>{if let Some(pages)=pages{
     let(publication,total_bytes,chunk_bytes,received_bytes)=match self.update{ToolRunPayloadUpdate::Replace=>(0,self.length,self.length.max(1),self.length),ToolRunPayloadUpdate::Range{publication,total_bytes,offset,chunk_bytes,..}=>(publication,usize::try_from(total_bytes).map_err(|_|invalid("payload extent exceeds native range"))?,chunk_bytes as usize,usize::try_from(offset).map_err(|_|invalid("payload offset exceeds native range"))?.checked_add(self.length).ok_or_else(||invalid("payload received extent overflow"))?)};
     *self.candidate=Some(ToolRunPayloadStream{identity:self.identity,publication,total_bytes,received_bytes,chunk_bytes,pages});self.phase=8;
    }settled(reply,Ok(Step::Progress(step.progress())))},Err(error)=>settled(reply,Err(error))}
   },
   8=>{
    if !funded(effect,d,SealedShared::<ToolRunPayloadStream>::source_admission_work_demand())?{return denied(reply);}
    let candidate=self.candidate.take().ok_or_else(||ValueError::invariant("candidate absent"))?;
    match crate::component::admit_sealed(candidate,effect){Ok((output,progress))=>{*self.output=Some(output);self.phase=9;settled(reply,Ok(self.finish(progress)))},Err((error,candidate))=>{*self.candidate=Some(candidate);settled(reply,Err(error))}}
   },
   _=>Err(invalid("payload preparation lost its admitted phase")),
  }
 }
 pub fn take_demand(&self)->RetirementDemand{scalar(2*size_of::<SealedShared<ToolRunPayloadStream>>())}
 pub fn take(&mut self,g:Grant)->Result<Option<(SealedShared<ToolRunPayloadStream>,Progress)>,ValueError>{if !self.complete(){return Err(invalid("payload directory is not ready"));}let d=self.take_demand();if !retained_clone_grant_admits(g,d){return Ok(None);}let result=self.output.take().ok_or_else(||invalid("payload directory output was already transferred"))?;self.phase=10;Ok(Some((result,item(d))))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(splice)=self.splice.as_ref(){return Ok(if splice.terminal_is_empty(){scalar(size_of::<PersistentSplice<Vec<u8>>>())}else if !splice.is_closing(){splice.begin_close_demand()}else{splice.close_demand(body)?});}
  if let Some(source)=self.authored.as_ref(){return Ok(if source.terminal_is_empty(){scalar(size_of::<Authored>())}else{source_close(source,body)?});}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Vec<u8>>(false,&None,body);}
  if self.hold.is_some(){return self.retirement.next_owner_demands::<Hold>(true,&None,body);}
  if let Some(admission)=self.adm_authored.as_ref(){return if admission.projection_result().is_some(){admission.projection_result_demand()}else{admission.close_demand()};}
  if self.input.is_some(){return self.retirement.next_owner_demands::<Vec<u8>>(true,&None,body);}
  if self.chunk.is_some(){return self.retirement.next_owner_demands::<SealedShared<Vec<u8>>>(true,&None,body);}
  if self.previous.is_some()||self.displaced.is_some()||self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<ToolRunPayloadStream>>(true,&None,body);}
  if self.seed.is_some()||self.candidate.is_some(){return self.retirement.next_owner_demands::<ToolRunPayloadStream>(true,&None,body);}
  Ok(Default::default())
 }
 fn close_work(&self,body:usize)->Result<u64,ValueError>{
  if let Some(splice)=self.splice.as_ref(){return if splice.terminal_is_empty(){Ok(1)}else if !splice.is_closing(){Ok(splice.begin_close_work_demand())}else{splice.close_work_demand(body)};}
  if let Some(source)=self.authored.as_ref(){return if source.terminal_is_empty(){Ok(1)}else{source.next_close_work_demand(body)};}
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<Vec<u8>>(false,&None,body);}
  if let Some(admission)=self.adm_authored.as_ref(){return if admission.projection_result().is_some(){admission.projection_result_work_demand()}else{admission.close_work_demand()};}
  Ok(u64::from(!self.terminal_is_empty()))
 }
 fn close_drive<'r>(&mut self,body:usize,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  if !self.closing{return Err(invalid("payload directory closure was not started"));}
  if self.route.is_some()!=recover{return Err(route_fault("payload update changed its original recovery route"));}
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(Step::Complete(Default::default()))));}
  let d=if recover{Default::default()}else{self.close_demand(body)?};
  if !recover&&!retained_clone_grant_admits(effect.grant(),d){return denied(reply);}
  if self.splice.is_some(){
   if self.splice.as_ref().is_some_and(PersistentSplice::terminal_is_empty){return local_step(effect,d,1,reply,|_|{self.splice=None;Ok(item(d))});}
   let splice=self.splice.as_mut().ok_or_else(||route_fault("payload splice is absent"))?;
   if !splice.is_closing(){let turn=splice.begin_close_paid(effect);return settled(reply,turn.result);}
   return forward!(self,SPLICE_CLOSE,splice,loan,recover,body,effect,reply,close_into,recover_close_into,close_original_into,recover_close_original_into);
  }
  if self.authored.is_some(){
   if self.authored.as_ref().is_some_and(RetainedCloneSource::terminal_is_empty){return local_step(effect,d,1,reply,|_|{self.authored=None;Ok(item(d))});}
   let source=self.authored.as_mut().ok_or_else(||route_fault("payload authored source is absent"))?;
   return forward!(self,AUTHORED_CLOSE,source,loan,recover,body,effect,reply,close_into,recover_close_into,close_original_into,recover_close_original_into);
  }
  if !self.retirement.is_empty(){let retirement=&mut self.retirement;return forward!(self,RETIREMENT,retirement,loan,recover,body,effect,reply,step_into,recover_step_into,step_original_into,recover_step_original_into);}
  macro_rules! begin{($slot:expr)=>{{let turn=self.retirement.begin_granted($slot,effect);return settled(reply,turn.result.map(|step|Step::Progress(step.map_or(Default::default(),|step|step.progress()))));}}}
  if self.hold.is_some(){begin!(&mut self.hold)}
  if let Some(admission)=self.adm_authored.as_mut(){
   let turn=match admission.projection_result(){Some(true)=>admission.discard_projection(effect),Some(false)=>{let mut refusal=None;admission.take_projection_failure(&mut refusal,effect)},None=>admission.close_into(&mut self.chunk,&mut self.authored,body,effect)};let done=admission.terminal_is_empty();
   return match turn.result{Ok(step)=>{if done{self.adm_authored=None;}settled(reply,Ok(Step::Progress(step.progress())))},Err(error)=>settled(reply,Err(error))};
  }
  if self.input.is_some(){begin!(&mut *self.input)}
  if self.chunk.is_some(){begin!(&mut *self.chunk)}
  if self.previous.is_some(){begin!(&mut *self.previous)}
  if self.displaced.is_some(){begin!(&mut *self.displaced)}
  if self.output.is_some(){begin!(&mut *self.output)}
  if self.seed.is_some(){begin!(&mut *self.seed)}
  if self.candidate.is_some(){begin!(&mut *self.candidate)}
  Ok(reply.complete(Ok(Step::Complete(Default::default()))))
 }
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close_drive(body,None,effect,reply,false)}
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close_drive(body,None,effect,reply,true)}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.input.is_none()&&self.previous.is_none()&&self.displaced.is_none()&&self.seed.is_none()&&self.chunk.is_none()&&self.candidate.is_none()&&self.output.is_none()&&self.hold.is_none()&&self.adm_authored.is_none()&&self.authored.is_none()&&self.splice.is_none()&&self.retirement.is_empty()&&self.route.is_none()}
}
impl Drop for ToolRunPayloadStreamUpdate{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"payload publication retains every original vector and path until paid close");}}
pub struct Close{owner:ToolRunPayloadStreamUpdate,reply:Option<Result<Step,OriginalRetirementFailure>>,route:Option<u128>}
impl Close{
 fn relay<'r>(&mut self,body:usize,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{
  let lost=||route_fault("payload close lost its retained reply");
  if recover{if self.route.is_none(){return Err(lost());}}else{if self.route.is_some()||self.reply.is_some(){return Err(lost());}self.route=Some(effect.performed_work());}
  if self.reply.is_none(){let done=self.owner.close_drive(body,loan,effect,OriginalEffectReply::borrow(&mut self.reply)?,recover)?;drop(done);}
  let Some(result)=self.reply.take()else{return Ok(reply.pending());};
  let start=self.route.take().ok_or_else(lost)?;let work=effect.performed_work().checked_sub(start).ok_or_else(lost)?;
  Ok(reply.complete(result.map(|step|RetirementStep::Progress(step.progress(),work))))
 }
}
impl RetireOwned for ToolRunPayloadStreamUpdate{
 type RetirementCursor=Close;
 fn retirement_seed(mut self)->Close{self.begin_close();Close{owner:self,reply:None,route:None}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Close>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for Close{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owner.close_demand(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.owner.close_work(body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(body,None,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(body,None,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(body,Some(original),effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(body,Some(original),effect,reply,true)}
 fn requires_original_control(&self)->bool{self.owner.splice.as_ref().is_some_and(|splice|!splice.terminal_is_empty()&&splice.requires_original_control())||self.owner.retirement.requires_original_control()||self.owner.authored.as_ref().is_some_and(|source|!source.terminal_is_empty()&&source.requires_original_control())}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&(self.owner.splice.as_ref().is_some_and(|splice|!splice.terminal_is_empty()&&splice.requires_original_birth())||self.owner.retirement.requires_original_birth()||self.owner.authored.as_ref().is_some_and(|source|!source.terminal_is_empty()&&source.requires_original_birth()))}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:Grant)->RetirementStep{
  let poll=if self.owner.splice.as_ref().is_some_and(|splice|!splice.terminal_is_empty()){self.owner.splice.as_mut().expect("selected splice").close_original_birth_step(original,grant)}else if !self.owner.retirement.is_empty(){self.owner.retirement.step_original_birth(original,grant)}else{self.owner.authored.as_mut().expect("selected authored source").close_original_birth_step(original,grant)};
  match poll{semio_framework_value::RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),semio_framework_value::RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
 }
 fn terminal_is_empty(&self)->bool{self.route.is_none()&&self.reply.is_none()&&self.owner.terminal_is_empty()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.next_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
