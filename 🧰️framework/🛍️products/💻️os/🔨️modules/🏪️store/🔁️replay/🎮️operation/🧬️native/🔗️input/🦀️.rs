//! 🔗️ Decoded and pressed native operations keep the original owner and captured retirement issuer.
use super::*;
use semio_framework_value::{OriginalRetirementFailure,retained_clone::{OriginalEffectAction,OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,OriginalSourceProjection}};
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
use semio_framework_value_source_hold::SourceHold;
use semio_framework_pack_json::{retained_alias,retained_alias_demand};
use std::marker::PhantomData;
type Hold<T,A>=SourceHold<T,A>;
type Effect<'a,'b>=&'a mut semio_framework_value::RetainedAllocationEffect<'b>;
type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
struct Call<'a,T:Send+Sync+'static,A:OriginalSourceProjection<T>+Send>{hold:&'a mut Hold<T,A>,body:usize,turn:OriginalSourceTurnIdentity}
struct Producer<T,A>{turn:OriginalSourceTurnIdentity,marker:PhantomData<fn()->(T,A)>}
impl<T:Send+Sync+'static,A:OriginalSourceProjection<T>+Send> OriginalEffectProducer for Producer<T,A>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Call<'a,T,A> where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if c.turn!=self.turn{return Err(invalid("original mutation source changed its original Source"))}Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.turn)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:Effect<'_,'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{c.hold.step_into(c.body,e,r,false)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:Effect<'_,'_>,r:Reply<'r>)->Result<Complete<'r>,ValueError>where 'b:'a{c.hold.step_into(c.body,e,r,true)}
}
/// 🧲️ Keeps one original Source hold with its resident projection frame, pending quote and exact handback closure across turns and cancellation.
pub(crate) struct HoldCustody<T:Send+Sync+'static,A:OriginalSourceProjection<T>+Send>{
 hold:ManuallyDrop<Option<Hold<T,A>>>,
 frame:Option<OriginalEffectAuthority<Producer<T,A>>>,
 action:Option<OriginalEffectAction<RetainedCloneStep,OriginalRetirementFailure>>,
 pending:Option<(RetirementDemand,usize)>,
 retirement:RetainedCloneClose,
}
impl<T:Send+Sync+'static,A:OriginalSourceProjection<T>+Send> HoldCustody<T,A>{
 pub(crate) fn vacant()->Self{Self{hold:ManuallyDrop::new(None),frame:None,action:None,pending:None,retirement:Default::default()}}
 pub(crate) fn fill(&mut self,authority:A){*self.hold=Some(SourceHold::new(authority));}
 fn hold(&self)->Result<&Hold<T,A>,ValueError>{self.hold.as_ref().ok_or_else(||invalid("original source hold absent"))}
 pub(crate) fn ready(&self)->bool{self.pending.is_none()&&self.hold.as_ref().is_some_and(Hold::ready)}
 pub(crate) fn original(&self)->Result<RetainedCloneRef<'_,T>,ValueError>{self.hold()?.original()}
 pub(crate) fn pending_body(&self)->Option<usize>{self.pending.map(|(_,body)|body)}
 pub(crate) fn busy(&self)->bool{self.pending.is_some()||!self.retirement.is_empty()}
 pub(crate) fn delegates(&self)->bool{self.pending.is_none()&&!self.retirement.is_empty()}
 pub(crate) fn quote(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some((demand,_))=self.pending{return Ok(demand);}self.hold()?.quote(body)}
 pub(crate) fn drive(&mut self,body:usize,demand:RetirementDemand,effect:Effect<'_,'_>)->Result<RetainedCloneStep,ValueError>{
  let turn=effect.work_source_turn().ok_or_else(||invalid("original source hold requires its original Source"))?;
  let(_,body)=*self.pending.get_or_insert((demand,body));
  let hold=self.hold.as_mut().ok_or_else(||invalid("original source hold absent"))?;let authority=self.frame.get_or_insert_with(||OriginalEffectAuthority::new(Producer{turn,marker:PhantomData}));
  let mut call=Call{hold,body,turn};
  if !authority.drive(&mut call,effect,&mut self.action)?{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut answer=None;if !self.action.as_mut().ok_or_else(||invalid("original source hold action absent"))?.transfer_reply(&mut answer)?{return Ok(RetainedCloneStep::Progress(Default::default()));}
  authority.finish(&mut self.action)?;self.pending=None;
  if let Some(frame)=self.frame.take(){frame.into_producer().map_err(|_|invalid("original source hold producer retains an action"))?;}
  let step=answer.ok_or_else(||invalid("original source hold reply absent"))?.map_err(OriginalRetirementFailure::into_value_error)?;Ok(RetainedCloneStep::Progress(step.progress()))
 }
 pub(crate) fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some((demand,_))=self.pending{return Ok(demand);}
  if !self.retirement.is_empty(){return Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?});}
  self.retirement.next_owner_demands::<Hold<T,A>>(self.hold.is_some(),&None,body)
 }
 pub(crate) fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if let Some((_,pending))=self.pending{return self.hold()?.work_quote(pending);}
  self.retirement.next_owner_work::<Hold<T,A>>(self.hold.is_some(),&None,body)
 }
 pub(crate) fn close_step(&mut self,body:usize,effect:Effect<'_,'_>)->Result<RetainedCloneStep,ValueError>{
  if let Some((demand,pending))=self.pending{return self.drive(pending,demand,effect);}
  if !self.retirement.is_empty(){return settled(self.retirement.step_granted(body,effect));}
  if let Some(step)=self.retirement.begin_granted(&mut self.hold,effect).result?{return Ok(step);}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub(crate) fn close_step_controlled(&mut self,body:usize,effect:Effect<'_,'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<RetainedCloneStep,Exit>{
  if self.delegates(){let selected=self.retirement.select(body)?;return Exit::resume(selected.step_controlled(effect,control)).map(|step|RetainedCloneStep::Progress(step.progress()));}
  Ok(self.close_step(body,effect)?)
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.hold.is_none()&&self.frame.is_none()&&self.action.is_none()&&self.pending.is_none()&&self.retirement.is_empty()}
}
impl<T:Send+Sync+'static,A:OriginalSourceProjection<T>+Send> Drop for HoldCustody<T,A>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original source hold requires funded closure");}}
/// 🧳️ Retains the same operation through source birth, projection handoff, and exact issuer closure.
pub struct ArtifactMutationSource<M:Send+Sync+'static>{
 custody:HoldCustody<M,FactoryOwnedSourceAuthority<M>>,
 projection:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,
 closing:bool,
}
fn one(effect:Effect<'_,'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original mutation source action exceeds its work ceiling"))}}
fn settled(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError>{match poll{semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}}
impl<M:Send+Sync+'static> ArtifactMutationSource<M>{
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<M>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>()+size_of::<Self>(),depth:1,..Default::default()}}
 pub fn admit(original:&mut Option<M>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}let original=original.take().ok_or_else(||invalid("original mutation source owner absent"))?;
  let mut custody=HoldCustody::vacant();custody.fill(FactoryOwnedSourceAuthority::new(original,Arc::clone(factory)));
  Ok(Some((Self{custody,projection:ManuallyDrop::new(None),closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 }
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demand(body);}if self.projection.is_some(){return Ok(header::<Option<RetainedOwnedProjection<M>>>());}
  if self.custody.ready(){retained_alias_demand(self.custody.original()?)}else{self.custody.quote(body)}
 }
 pub(crate) fn advance(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneStep,ValueError>{
  let grant=effect.grant();let body=self.custody.pending_body().unwrap_or(grant.maximum_copy_bytes);let d=self.demands(body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.closing{return self.close_step(effect);}
  if self.projection.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if self.custody.ready(){let Some((projection,p))=retained_alias(self.custody.original()?,effect)? else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.projection=Some(projection);return Ok(RetainedCloneStep::Complete(p));}
  self.custody.drive(body,d,effect)
 }
 /// 🔗️ The one public advance: opens one original effect per action over the grant a wide turn still holds, so a wide grant finishes the source in few turns while a quote-exact grant stays at one action; every Pending keeps its action in the custody and resumes on the next turn.
 pub fn advance_chained(&mut self,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let mut total=RetainedCloneProgress::default();
  loop{
   let rest=RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(total.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(total.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(total.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(total.released_bytes),..grant};
   let Some(step)=probe_effect(control,rest,|_,effect|self.advance(effect))?else{break};let step=step?;
   let progress=step.progress();total=total.checked_add(progress)?;
   if matches!(step,RetainedCloneStep::Complete(_)){return Ok(RetainedCloneStep::Complete(total))}
   if progress==RetainedCloneProgress::default(){break}
  }
  Ok(RetainedCloneStep::Progress(total))
 }
 pub fn take(&mut self,output:&mut Option<RetainedOwnedProjection<M>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{if output.is_some(){return Err(invalid("original mutation projection output already occupied"));}if self.closing||self.projection.is_none(){return Ok(None);}let d=header::<Option<RetainedOwnedProjection<M>>>();if !retained_clone_grant_admits(grant,d){return Ok(None);}*output=self.projection.take();Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn ready(&self)->bool{!self.closing&&self.projection.is_some()}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.custody.busy(){return self.custody.close_demand(body);}
  if let Some(projection)=self.projection.as_ref(){return if projection.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{projection.close_demand(body)};}
  self.custody.close_demand(body)
 }
 pub fn close_step(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneStep,ValueError>{
  let grant=effect.grant();if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let body=self.custody.pending_body().unwrap_or(grant.maximum_copy_bytes);let d=self.close_demand(body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.custody.busy(){return self.custody.close_step(body,effect);}
  if let Some(projection)=self.projection.as_mut(){if !projection.terminal_is_empty(){return settled(projection.close_step(grant.maximum_copy_bytes,effect));}self.projection.take();one(effect)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  self.custody.close_step(grant.maximum_copy_bytes,effect)
 }
}
impl<M:Send+Sync+'static> ArtifactMutationSource<M>{pub fn terminal_is_empty(&self)->bool{self.closing&&self.projection.is_none()&&self.custody.terminal_is_empty()}}
impl<M:Send+Sync+'static> Drop for ArtifactMutationSource<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original mutation source requires funded closure");}}
