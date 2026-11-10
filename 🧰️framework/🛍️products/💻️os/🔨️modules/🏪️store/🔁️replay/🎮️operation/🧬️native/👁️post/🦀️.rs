//! 👁️ One resident producer drives the captured candidate through original Source reads and returns it unchanged.
use super::Failure;
use semio_framework_value::{RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{OriginalEffectAction,OriginalEffectAuthority,OriginalEffectCallbackMode,OriginalEffectInvocationIdentity,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_snapshot_projection::snapshot_projection::post::{OriginalPostRead,PagedOneItemPost,PagedOneItemPostMode,PagedOneItemPostRead};
use std::marker::PhantomData;
/// 🎯️ Selects the exact owner action behind one granted post turn.
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum PostOp{Capture,Prepare(PagedOneItemPostMode),Drive(PagedOneItemPostMode),Return}
/// 🧭️ Names the caller stage; the custody picks Drive or Prepare from its actual frontier.
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum PostWant{Capture,Edit(PagedOneItemPostMode),Close}
type Key=(OriginalEffectInvocationIdentity,PostOp,usize);
#[derive(Default)]
struct PostDrive{inner:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>,reply:Option<Result<RetainedCloneStep,ValueError>>,selected:Option<Key>}
struct PostCall<'a,P:RetireOwned+Sync+'static>{read:&'a mut OriginalPostRead<P>,post:&'a mut Option<P>,drive:&'a mut PostDrive,turn:OriginalSourceTurnIdentity,op:PostOp,body:usize}
struct PostProducer<P>{turn:OriginalSourceTurnIdentity,marker:PhantomData<fn()->P>}
fn fault(reason:&'static str)->ValueError{ValueError::invariant(reason)}
fn forward<'r>(resident:&mut Option<Result<RetainedCloneStep,ValueError>>,reply:OriginalEffectReply<'r,RetainedCloneStep,Failure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,Failure>,ValueError>{match resident.take(){Some(result)=>Ok(reply.complete(result.map_err(Failure::from_primary))),None=>Ok(reply.pending())}}
fn dispatch<'r,P:RetireOwned+Sync+'static>(c:&mut PostCall<'_,P>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,Failure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,Failure>,ValueError>{
 let invocation=effect.invocation().ok_or_else(||fault("post custody invocation absent"))?;
 let mode=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive};
 if invocation.mode()!=mode{return Err(fault("post custody invocation changed"))}
 let key=(invocation.identity(),c.op,c.body);
 let PostDrive{inner,reply:answer,selected}=&mut *c.drive;
 if recover{if *selected!=Some(key){return Err(fault("post custody recovery changed"))}}else if selected.is_some(){return Err(fault("post custody action occupied"))}
 let mut scope=reply.progress().scope(selected,key)?;
 match c.op{
  PostOp::Capture|PostOp::Return=>{
   if recover{return Err(fault("post custody transfer cannot recover without a reply"))}
   let mut result=None;let child=OriginalEffectReply::borrow(&mut result)?;
   let done=if c.op==PostOp::Capture{c.read.capture_into(c.post,effect,child)?}else{c.read.return_into(c.post,effect,child)?};drop(done);
   let done=forward(&mut result,scope.borrow()?)?;drop(done)
  },
  PostOp::Prepare(mode)=>{
   let done=if recover{c.read.recover_prepare_into(mode,c.body,effect,scope.borrow()?)?}else{c.read.prepare_into(mode,c.body,effect,scope.borrow()?)?};drop(done)
  },
  PostOp::Drive(_)=>{
   let frame=c.read.projection_frame(effect)?;let mut context=frame.context()?;
   let ready=frame.authority()?.drive(&mut context,effect,inner)?;
   if recover&&ready{
    if !inner.as_mut().ok_or_else(||fault("post custody nested action absent"))?.transfer_reply(answer)?{return Ok(scope.forwarded())}
    frame.authority()?.finish(inner)?;
    let done=forward(answer,scope.borrow()?)?;drop(done)
   }
  },
 }
 Ok(scope.forwarded())
}
impl<P:RetireOwned+Sync+'static> OriginalEffectProducer for PostProducer<P>{
 type Output=RetainedCloneStep;type Failure=Failure;type Context<'a,'b>=PostCall<'a,P> where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if c.turn!=self.turn{return Err(fault("post custody changed its original Source"))}Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.turn)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{dispatch(c,e,r,false)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{dispatch(c,e,r,true)}
}
/// 🧳️ Retains the post owner, its resident producer and the selected action until the candidate returns.
pub struct PostCustody<P:RetireOwned+Sync+'static>{read:OriginalPostRead<P>,authority:Option<OriginalEffectAuthority<PostProducer<P>>>,action:Option<OriginalEffectAction<RetainedCloneStep,Failure>>,drive:PostDrive,pending:Option<(PostOp,RetirementDemand,u64)>}
impl<P:RetireOwned+Sync+'static> Default for PostCustody<P>{fn default()->Self{Self{read:OriginalPostRead::default(),authority:None,action:None,drive:PostDrive::default(),pending:None}}}
impl<P:RetireOwned+Sync+'static> PostCustody<P>{
 pub fn captured(&self)->bool{!self.read.terminal_is_empty()}
 pub fn capture_quote()->(RetirementDemand,u64){(OriginalPostRead::<P>::capture_demand(),1)}
 pub fn read(&self,mode:PagedOneItemPostMode)->Result<PagedOneItemPostRead<'_,P>,ValueError>{self.read.read(mode)}
 pub fn access(&mut self,mode:PagedOneItemPostMode)->Result<PagedOneItemPost<'_,P>,ValueError>{self.read.access(mode)}
 pub fn terminal_is_empty(&self)->bool{self.read.terminal_is_empty()&&self.authority.is_none()&&self.action.is_none()&&self.pending.is_none()&&self.drive.inner.is_none()&&self.drive.reply.is_none()&&self.drive.selected.is_none()}
 pub fn due(&self,mode:PagedOneItemPostMode,body:usize)->Result<Option<(RetirementDemand,u64)>,ValueError>{
  if let Some((_,demand,work))=self.pending{return Ok(Some((demand,work)))}
  match self.read.demand(mode,body)?{Some(demand)=>Ok(Some((demand,self.read.work_demand(mode,body)?))),None=>Ok(None)}
 }
 pub fn closing_quote(&self,body:usize)->Result<(RetirementDemand,u64),ValueError>{
  if let Some((_,demand,work))=self.pending{return Ok((demand,work))}
  match self.read.demand(PagedOneItemPostMode::Mutable,body)?{Some(demand)=>Ok((demand,self.read.work_demand(PagedOneItemPostMode::Mutable,body)?)),None=>Ok((self.read.return_demand(),1))}
 }
 fn select(&self,want:PostWant,body:usize)->Result<PostOp,ValueError>{
  let prepare=|mode|Ok::<_,ValueError>(if self.read.projection_drive_required()?{PostOp::Drive(mode)}else{PostOp::Prepare(mode)});
  match want{PostWant::Capture=>Ok(PostOp::Capture),PostWant::Edit(mode)=>prepare(mode),PostWant::Close=>if self.read.demand(PagedOneItemPostMode::Mutable,body)?.is_some(){prepare(PagedOneItemPostMode::Mutable)}else{Ok(PostOp::Return)}}
 }
 fn quote(&self,op:PostOp,body:usize)->Result<(RetirementDemand,u64),ValueError>{
  match op{
   PostOp::Capture=>Ok(Self::capture_quote()),
   PostOp::Prepare(mode)|PostOp::Drive(mode)=>Ok((self.read.demand(mode,body)?.ok_or_else(||fault("post custody has no pending read frontier"))?,self.read.work_demand(mode,body)?)),
   PostOp::Return=>Ok((self.read.return_demand(),1)),
  }
 }
 /// ⏯️ Runs the selected action once; `None` means the recorded reply is still pending and the same action recovers next turn.
 pub fn run(&mut self,want:PostWant,body:usize,post:&mut Option<P>,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,Failure>{
  let op=match self.pending{Some((op,..))=>op,None=>{let op=self.select(want,body)?;let(demand,work)=self.quote(op,body)?;self.pending=Some((op,demand,work));op}};
  let turn=effect.work_source_turn().ok_or_else(||fault("post custody requires its original Source"))?;
  let authority=self.authority.get_or_insert_with(||OriginalEffectAuthority::new(PostProducer{turn,marker:PhantomData}));
  let mut call=PostCall{read:&mut self.read,post,drive:&mut self.drive,turn,op,body};
  if !authority.drive(&mut call,effect,&mut self.action)?{return Ok(None)}
  let mut answer=None;
  if !self.action.as_mut().ok_or_else(||fault("post custody action absent"))?.transfer_reply(&mut answer)?{return Ok(None)}
  authority.finish(&mut self.action)?;self.pending=None;
  if op==PostOp::Return&&self.read.terminal_is_empty(){if let Some(authority)=self.authority.take(){authority.into_producer().map_err(|_|fault("post custody producer retains an action"))?;}}
  let step=answer.ok_or_else(||fault("post custody reply absent"))??;
  Ok(Some(step.progress()))
 }
}
