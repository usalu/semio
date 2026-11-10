//! 🔗️ Original history source retains its exact ledger slot, captured issuer and projected forward owners.
use semio_os_vcs::{ArtifactHistoryLedger,ArtifactHistoryReadKey};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,FactoryAuthority,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,retained_clone_grant_admits}};
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
type Issuer<T>=Arc<dyn ArtifactOwnedValueRetirementFactory<T>>;
type Birth=semio_framework_value::OriginalRetirementFailure;
/// 🧷️ One granted source binds the original ledger and issuer until every retained projection returns.
pub struct OriginalHistorySource<T:Sync>{root:RetainedOwnedProjectionAdmission<T>,key:Option<ArtifactHistoryReadKey>,issuer:ManuallyDrop<Option<Issuer<T>>>,returned:ManuallyDrop<Option<Issuer<T>>>,closing_factory:[Option<FactoryAuthority>;2],ready:bool,closing:bool,recover:Option<Pending>}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Pending{Advance,Return}
/// 🧷️ One projected forward owner is admitted as an alias first and published into its retained owner second.
pub struct OriginalHistoryInput<M:Sync>{admission:RetainedOwnedProjectionAdmission<M>,projection:Option<RetainedOwnedProjection<M>>}
impl<M:Sync> Default for OriginalHistoryInput<M>{fn default()->Self{Self{admission:Default::default(),projection:None}}}
impl<M:Sync> OriginalHistoryInput<M>{
 /// 🫥️ True while neither an alias nor a published projection is held.
 pub fn is_none(&self)->bool{self.projection.is_none()&&self.admission.terminal_is_empty()}
 /// 🫙️ True once the projection is published and can lend or move its owner.
 pub fn is_some(&self)->bool{self.projection.is_some()}
 /// 🧲️ True while an alias, a published projection or its terminal shell is held.
 pub fn occupied(&self)->bool{self.projection.is_some()||!self.admission.terminal_is_empty()}
 /// 👁️ Observes the published projection without moving it.
 pub fn as_ref(&self)->Option<&RetainedOwnedProjection<M>>{self.projection.as_ref()}
 /// 📖️ Borrows the published projection.
 pub fn borrow(&self)->Result<RetainedCloneRef<'_,M>,ValueError>{self.projection.as_ref().ok_or_else(||invalid("history input is not published"))?.borrow()}
 /// 📤️ Moves the published projection to its single consumer.
 pub fn take(&mut self)->Option<RetainedOwnedProjection<M>>{self.projection.take()}
 /// ✅️ Confirms every alias and published projection is physically closed.
 pub fn terminal_is_empty(&self)->bool{self.admission.terminal_is_empty()&&self.projection.as_ref().is_none_or(RetainedOwnedProjection::terminal_is_empty)}
 /// 🧹️ Drops the closed projection shell.
 pub fn vacate(&mut self){assert!(self.terminal_is_empty(),"history input vacated before physical closure");self.projection=None;}
 /// 📏️ Quotes the next alias or projection close.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.projection.as_ref(){Some(projection) if self.admission.terminal_is_empty()=>projection.close_demand(body),_=>self.admission.close_demand(body)}}
 /// ♻️ Closes the alias before the published projection.
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let body=effect.grant().maximum_copy_bytes;match self.projection.as_mut(){Some(projection) if self.admission.terminal_is_empty()=>projection.close_step(body,effect),_=>self.admission.close_step(body,effect)}}
 /// 🧲️ Selects the occupied input for closure.
 pub fn as_mut(&mut self)->Option<&mut Self>{if self.occupied(){Some(self)}else{None}}
 /// 🕳️ Lends the publication slot to a producer that moves a finished projection in.
 pub fn slot(&mut self)->&mut Option<RetainedOwnedProjection<M>>{&mut self.projection}
}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn one(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original history source action exceeds its work ceiling"))}}
fn settled(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError>{match poll{semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(Birth::into_value_error)}}
fn drive<S,E>(call:impl for<'r> FnOnce(OriginalEffectReply<'r,S,E>)->Result<OriginalEffectReplyComplete<'r,S,E>,ValueError>)->Result<Option<Result<S,E>>,ValueError>{let mut slot=None;drop(call(OriginalEffectReply::borrow(&mut slot)?)?);Ok(slot)}
fn outcome(result:Result<RetainedCloneStep,ValueError>,effect:&RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{RetainedWorkOutcome{result,work:effect.performed_work()}}
impl<T:Send+Sync+'static> OriginalHistorySource<T>{
 /// 📏️ Captures only the original slot key and exact issuer alias under caller authority.
 pub fn constructor_demand()->RetirementDemand{header::<Self>()}
 /// 🧷️ Keeps the same indexed source throughout one original edit's operation projections.
 pub fn key(&self)->Option<ArtifactHistoryReadKey>{self.key}
 /// 🎟️ Admits source custody without reading, cloning or reconstructing any history operation.
 pub fn admit(key:ArtifactHistoryReadKey,issuer:&Issuer<T>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}Ok(Some((Self{root:Default::default(),key:Some(key),issuer:ManuallyDrop::new(Some(Arc::clone(issuer))),returned:ManuallyDrop::new(None),closing_factory:[None,None],ready:false,closing:false,recover:None},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 fn held(&self)->bool{self.root.borrow().is_ok()}
 fn birth(&mut self,ledger:&mut ArtifactHistoryLedger<T>,key:ArtifactHistoryReadKey,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let recover=self.recover==Some(Pending::Advance);let issuer=Arc::clone(self.issuer.as_ref().ok_or_else(||invalid("original history issuer absent"))?);
  let done=drive(|reply|if recover{ledger.recover_advance_retained_read_into(key,&issuer,effect,reply)}else{ledger.advance_retained_read_into(key,&issuer,effect,reply)})?;drop(issuer);
  self.recover=if done.is_none(){Some(Pending::Advance)}else{None};let Some(done)=done else{return Ok(RetainedCloneStep::Progress(Default::default()))};let step=done.map_err(Birth::into_value_error)?;self.ready=ledger.retained_read_demands(key,effect.grant().maximum_copy_bytes)?==RetirementDemand::default();Ok(RetainedCloneStep::Progress(step.progress()))
 }
 fn retain_root(&mut self,ledger:&ArtifactHistoryLedger<T>,key:ArtifactHistoryReadKey,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{drive(|reply|ledger.retain_read_into(key,&mut self.root,effect,reply))?.ok_or_else(||invalid("original history root admission suspended inside a native action"))?}
 /// 📏️ Quotes the original root birth or alias admission independently of optional child projections.
 pub fn root_demands(&self,ledger:&ArtifactHistoryLedger<T>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid("closing history source cannot retain its original root"))}if self.held(){return Ok(Default::default())}let key=self.key.ok_or_else(||invalid("original history key absent"))?;if self.ready{ledger.retain_read_demand(key,&self.root)}else{ledger.retained_read_demands(key,body)}}
 /// 🪪️ Retains the genuine ledger root for borrowed metadata codecs without creating a child alias.
 pub fn advance_root(&mut self,ledger:&mut ArtifactHistoryLedger<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{
  let grant=effect.grant();let result=(||{let d=self.root_demands(ledger,grant.maximum_copy_bytes)?;if self.held(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let key=self.key.unwrap();if !self.ready{return self.birth(ledger,key,effect)}self.retain_root(ledger,key,effect)})();outcome(result,effect)
 }
 /// 📏️ Quotes the next root birth, root alias admission, child alias admission or child publication.
 pub fn projection_demands<M:Sync>(&self,ledger:&ArtifactHistoryLedger<T>,body:usize,input:&OriginalHistoryInput<M>,projection:usize,project:impl FnOnce(&T)->&M+Copy)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid("closing history source cannot project another operation"))}if !self.held(){return self.root_demands(ledger,body)}if !input.admission.terminal_is_empty(){return input.admission.take_demand()}input.admission.demand(self.root.borrow()?.project(projection,project))}
 /// 📽️ Projects the actual original field only after independent ledger read births complete.
 pub fn advance<M:Sync>(&mut self,ledger:&mut ArtifactHistoryLedger<T>,input:&mut OriginalHistoryInput<M>,projection:usize,project:impl FnOnce(&T)->&M+Copy,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{
  let grant=effect.grant();let result=(||{if input.projection.is_some(){return Err(invalid("history projection output already occupied"));}let d=self.projection_demands(ledger,grant.maximum_copy_bytes,input,projection,project)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
   if !self.held(){let key=self.key.unwrap();if !self.ready{return self.birth(ledger,key,effect)}return self.retain_root(ledger,key,effect).map(|step|RetainedCloneStep::Progress(step.progress()))}
   if !input.admission.terminal_is_empty(){let step=drive(|reply|input.admission.take(&mut input.projection,effect,reply))?.ok_or_else(||invalid("history projection publication suspended inside a native action"))??;return Ok(step)}
   let source=self.root.borrow()?.project(projection,project);let step=drive(|reply|input.admission.admit(source,effect,reply))?.ok_or_else(||invalid("history projection admission suspended inside a native action"))??;Ok(RetainedCloneStep::Progress(step.progress()))
  })();outcome(result,effect)
 }
 /// 👁️ Borrows only the authenticated original ledger root already retained by this source.
 pub fn borrow_original(&self)->Result<RetainedCloneRef<'_,T>,ValueError>{if self.closing{return Err(invalid("closing original history source cannot lend metadata"))}self.root.borrow()}
 /// 📏️ Quotes the exact original return and independently controlled issuer aliases.
 pub fn close_demands(&self,ledger:&ArtifactHistoryLedger<T>,body:usize)->Result<RetirementDemand,ValueError>{if !self.root.terminal_is_empty(){return self.root.close_demand(body);}if let Some(key)=self.key{return ledger.retained_read_return_demands(key,body);}if self.returned.is_some()||self.issuer.is_some(){return Ok(header::<Issuer<T>>());}if let Some(factory)=self.closing_factory.iter().find_map(Option::as_ref){return factory.demands(body);}Ok(Default::default())}
 /// ↩️ Restores the same slot before closing the aliases of its exact captured issuer.
 pub fn close_step(&mut self,ledger:&mut ArtifactHistoryLedger<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{
  let grant=effect.grant();let result=(||{
  if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}let d=self.close_demands(ledger,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
   if !self.root.terminal_is_empty(){return settled(self.root.close_step(grant.maximum_copy_bytes,effect));}
   if let Some(key)=self.key{
    if self.recover==Some(Pending::Advance){return self.birth(ledger,key,effect);}
    let recover=self.recover==Some(Pending::Return);let done=match drive(|reply|if recover{ledger.recover_return_retained_read_into(key,&mut self.returned,effect,reply)}else{ledger.return_retained_read_into(key,&mut self.returned,effect,reply)}){Ok(done)=>done,Err(error)if error.kind==ValueRefusalKind::WorkLimit&&error.retained_progress()==RetainedCloneProgress::default()=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>return Err(error)};
    self.recover=if done.is_none(){Some(Pending::Return)}else{None};let Some(done)=done else{return Ok(RetainedCloneStep::Progress(Default::default()))};let step=match done{Ok(step)=>step,Err(failure)=>{let error=failure.into_value_error();if error.kind==ValueRefusalKind::WorkLimit&&error.retained_progress()==RetainedCloneProgress::default(){return Ok(RetainedCloneStep::Progress(Default::default()))}return Err(error)}};if matches!(step,RetainedCloneStep::Complete(_)){self.key=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
   if let Some(factory)=self.returned.take(){one(effect)?;self.closing_factory[0]=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
   if let Some(factory)=self.issuer.take(){one(effect)?;self.closing_factory[1]=Some(FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
   if let Some(index)=self.closing_factory.iter().position(Option::is_some){let Some(factory)=self.closing_factory[index].as_mut() else { return Err(invalid("history closing factory slot is absent"))};let step=settled(factory.step(grant.maximum_copy_bytes,effect))?;if factory.terminal_is_empty(){self.closing_factory[index]=None;}return Ok(step);}Ok(RetainedCloneStep::Complete(Default::default()))})();outcome(result,effect)
 }
}
impl<T:Sync> OriginalHistorySource<T>{
 /// 🛑️ Stops original reads before producer cancellation returns its projection aliases.
 pub fn begin_close(&mut self){self.closing=true;}

 /// 🫙️ Confirms original slot restoration and physical closure of every source issuer alias.
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.root.terminal_is_empty()&&self.key.is_none()&&self.issuer.is_none()&&self.returned.is_none()&&self.closing_factory.iter().all(Option::is_none)}
}
impl<T:Sync> Drop for OriginalHistorySource<T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original history source requires original ledger return and funded issuer closure");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
