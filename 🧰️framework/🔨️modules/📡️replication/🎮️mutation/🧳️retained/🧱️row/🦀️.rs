//! 🧱️ Required typed row semantics compose bounded lookup, guard, fields, original inverse and central exchange.
use super::{RetainedMutationLeafPreparation,RetainedMutationCloseContext,RetainedMutationAdvance,RetainedMutationOutcome,RetainedFieldPreparation,RetainedKeyLookup,RetainedKeyMatch,RetainedRowPatch,RetainedRowPatchFields,RetainedPatchSource,RetainedDiagnosticSpec,RetainedDiagnosticPreparation,retained_mutation_demand_fits};
use super::super::{Mutation,ApplyCapability,MutationMessageLedgerRetirement};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,FactoryVectorRetirement,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose}};
use std::mem::{size_of,ManuallyDrop};

/// 🛡️ A guard resolves every fallible domain requirement before any inverse or row write.
#[derive(Clone,Copy)]
pub enum RetainedRowDecision{Admit,Noop,Refuse}
/// 🚦️ Guard completion may attach one original declared diagnostic before proceeding or refusing.
pub enum RetainedRowGuardStep{Progress(RetainedCloneProgress),Ready(RetainedCloneProgress,RetainedRowDecision,Option<RetainedDiagnosticSpec>)}
impl RetainedRowGuardStep{fn progress(&self)->RetainedCloneProgress{match self{Self::Progress(value)|Self::Ready(value,..)=>*value}}}

/// 🧬️ Domain checks supply a required bounded owner rather than a cold validation callback.
pub trait RetainedRowGuard<P,R,A>:Send+'static{
 fn new()->Self;
 fn demand(&self,original:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,patch:RetainedCloneRef<'_,A>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,original:RetainedCloneRef<'_,P>,row:RetainedCloneRef<'_,R>,patch:RetainedCloneRef<'_,A>,grant:RetainedCloneGrant)->Result<RetainedRowGuardStep,ValueError>;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// 🪡️ Technology declares only typed row access, authored field shape, required guard and allocation-free inverse assembly.
pub trait RetainedRowMutation<P,M:Mutation<P>,const N:usize>:Send+Sync+Sized+'static{
 type Row:'static;
 type Patch:RetainedRowPatch<Self::Row>;
 type Guard:RetainedRowGuard<P,Self::Row,Self::Patch>;
 fn rows(base:&P)->&Vec<Self::Row>;
 fn rows_mut(base:&mut P)->&mut Vec<Self::Row>;
 fn row_key(row:&Self::Row)->&str;
 fn key(&self)->&String;
 fn patch(&self)->&Self::Patch;
 fn target(&self,index:usize)->&String;
 fn from_inverse(target:[String;N],patch:Self::Patch)->M;
}

/// 🧳️ Original sources and each staged, displaced or inverse owner survive every bounded frontier.
pub struct RetainedRowPreparation<P:'static,M:Mutation<P>+Send+'static,L:RetainedRowMutation<P,M,N>,const N:usize>{
 lookup:RetainedKeyLookup,
 guard:L::Guard,
 authored:<L::Patch as RetainedRowPatch<L::Row>>::Fields,
 inverse:<L::Patch as RetainedRowPatch<L::Row>>::Fields,
 identities:[RetainedFieldPreparation<String>;N],
 identity:ManuallyDrop<[Option<String>;N]>,
 forward:ManuallyDrop<Option<L::Patch>>,
 undo:ManuallyDrop<Option<L::Patch>>,
 inverse_rows:ManuallyDrop<Option<Vec<M>>>,
 inverse_close:Option<FactoryVectorRetirement<M>>,
 diagnostic:RetainedDiagnosticPreparation<N>,
 ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,
 pending_messages:ManuallyDrop<Option<Vec<super::super::MutationMessage>>>,
 retirement:RetainedCloneClose,
 phase:usize,
 index:usize,
 field:usize,
 decision:RetainedRowDecision,
 notice:Option<RetainedDiagnosticSpec>,
 ready:bool,
 finished:bool,
 closing:bool,
 projected:std::marker::PhantomData<fn()->P>,
}
const META:usize=size_of::<usize>()*3+size_of::<Option<RetainedDiagnosticSpec>>();
fn child_grant(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-META,maximum_depth:grant.maximum_depth-1,..grant}}
fn child_demand(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(META).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained row parent copy extent overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retained row child depth overflow"))?;Ok(demand)}
fn progress(mut value:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{value.copied_bytes=value.copied_bytes.checked_add(META).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained row receipt extent overflow"))?;Ok(value)}

impl<P:'static,M:Mutation<P>+Send+'static,L:RetainedRowMutation<P,M,N>,const N:usize> RetainedRowPreparation<P,M,L,N>{
 fn refs<'a>(payload:RetainedCloneRef<'a,L>,original:RetainedCloneRef<'a,P>,index:usize)->(RetainedCloneRef<'a,L::Patch>,RetainedCloneRef<'a,L::Row>){(payload.project(N,|leaf|leaf.patch()),original.project(index+1,|base|&L::rows(base)[index]))}
 fn targets(payload:RetainedCloneRef<'_,L>)->[RetainedCloneRef<'_,String>;N]{std::array::from_fn(|index|payload.project(index,|leaf|leaf.target(index)))}
 fn scalar(copy_bytes:usize,capacity_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()}}
 fn retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?})}
}

impl<P:'static,M:Mutation<P>+Send+'static,L:RetainedRowMutation<P,M,N>,const N:usize> RetainedMutationLeafPreparation<P,M,L> for RetainedRowPreparation<P,M,L,N>{
 fn new()->Self{Self{lookup:RetainedKeyLookup::new(),guard:L::Guard::new(),authored:RetainedRowPatchFields::new(RetainedPatchSource::Authored),inverse:RetainedRowPatchFields::new(RetainedPatchSource::Inverse),identities:std::array::from_fn(|_|Default::default()),identity:ManuallyDrop::new(std::array::from_fn(|_|None)),forward:ManuallyDrop::new(None),undo:ManuallyDrop::new(None),inverse_rows:ManuallyDrop::new(None),inverse_close:None,diagnostic:Default::default(),ledger:ManuallyDrop::new(None),pending_messages:ManuallyDrop::new(None),retirement:Default::default(),phase:0,index:0,field:0,decision:RetainedRowDecision::Admit,notice:None,ready:false,finished:false,closing:false,projected:std::marker::PhantomData}}
 fn demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,P>,_:&P,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.finished{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained row cannot prepare after closure or ready transfer"));}
  let inner=body.saturating_sub(META);
  match self.phase{
   0=>child_demand(self.lookup.demand(original.project(0,L::rows),payload.project(N+1,|leaf|leaf.key()),L::row_key,inner)?),
   1=>{let(patch,row)=Self::refs(payload,original,self.index);child_demand(self.guard.demand(original,row,patch,inner)?)},
   2=>Ok(Self::scalar(RetainedDiagnosticPreparation::<N>::start_demand().copy_bytes+META,0)),
   3=>child_demand(self.diagnostic.demand(Self::targets(payload),inner)?),
   4=>Ok(Self::scalar(size_of::<Vec<super::super::MutationMessage>>()+size_of::<MutationMessageLedgerRetirement>()+META,0)),
   5=>{let(patch,row)=Self::refs(payload,original,self.index);child_demand(self.authored.demand(patch,row,inner)?)},
   6=>Ok(Self::scalar(size_of::<L::Patch>()*3+META,0)),
   7=>{let(patch,row)=Self::refs(payload,original,self.index);child_demand(self.inverse.demand(patch,row,inner)?)},
   8=>Ok(Self::scalar(size_of::<L::Patch>()*3+META,0)),
   9=>{let field=&self.identities[self.field];if field.value().is_some(){Ok(Self::scalar(field.exchange_demand()?.copy_bytes+size_of::<Option<String>>()+META,0))}else{child_demand(field.demand(Self::targets(payload)[self.field],inner)?) }},
   10=>Ok(Self::scalar(size_of::<M>()+size_of::<L::Patch>()+N*size_of::<String>()+size_of::<Vec<M>>()+META,size_of::<M>())),
   11=>{let mut demand=self.forward.as_ref().expect("owned authored fields").exchange_demand(self.field)?;demand.copy_bytes+=META;Ok(demand)},
   12=>if !self.retirement.is_empty(){self.retirement_demand(body)}else if self.forward.is_some(){Ok(Self::scalar(0,self.retirement.next_owner_capacity_byte_demand::<L::Patch>(true,body)?))}else{Ok(Self::scalar(META,0))},
   13=>Ok(Self::scalar(size_of::<Vec<M>>()+size_of::<Vec<super::super::MutationMessage>>()*2+META,0)),
   _=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained row phase outside declared contract")),
  }
 }
 fn advance(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,P>,unpublished:&mut P,grant:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedMutationAdvance,ValueError>{
  let demand=self.demand(payload,original,unpublished,grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedMutationAdvance::Progress(Default::default()));}
  let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()};
  match self.phase{
   0=>{let step=self.lookup.advance(original.project(0,L::rows),payload.project(N+1,|leaf|leaf.key()),L::row_key,child_grant(grant))?;receipt=progress(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){match self.lookup.result().expect("complete original keyed lookup"){RetainedKeyMatch::Unique(index)=>{self.index=index;self.phase=1;},RetainedKeyMatch::Missing=>{self.decision=RetainedRowDecision::Refuse;self.notice=Some(RetainedDiagnosticSpec{level:semio_framework_diagnostic::Severity::Error,code:"mutation.target-missing",message:"The addressed entity is missing."});self.phase=2;},RetainedKeyMatch::Duplicate(..)=>{self.decision=RetainedRowDecision::Refuse;self.notice=Some(RetainedDiagnosticSpec{level:semio_framework_diagnostic::Severity::Fatal,code:"mutation.invariant",message:"The addressed entity key is duplicated."});self.phase=2;}}}},
   1=>{let(patch,row)=Self::refs(payload,original,self.index);let step=self.guard.advance(original,row,patch,child_grant(grant))?;receipt=progress(step.progress())?;if let RetainedRowGuardStep::Ready(_,decision,notice)=step{self.decision=decision;self.notice=notice;self.phase=if notice.is_some(){2}else{match decision{RetainedRowDecision::Admit=>5,RetainedRowDecision::Noop=>13,RetainedRowDecision::Refuse=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic row refusal requires original diagnostic declaration"))}};}},
   2=>{let step=self.diagnostic.start(self.notice.expect("original semantic declaration"),RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-META,..grant})?;receipt=progress(step)?;self.phase=3;},
   3=>{let step=self.diagnostic.advance(Self::targets(payload),child_grant(grant))?;receipt=progress(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=4;}},
   4=>{let(rows,child)=self.diagnostic.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<MutationMessageLedgerRetirement>()-META,..grant})?.expect("paid semantic row transfer");*self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),rows));receipt=progress(child)?;receipt.copied_bytes+=size_of::<MutationMessageLedgerRetirement>();self.phase=match self.decision{RetainedRowDecision::Admit=>5,_=>13};},
   5=>{let(patch,row)=Self::refs(payload,original,self.index);let step=self.authored.advance(patch,row,child_grant(grant))?;receipt=progress(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=6;}},
   6=>{let(value,child)=self.authored.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<L::Patch>()-META,..grant})?;*self.forward=value;receipt=progress(child)?;receipt.copied_bytes+=size_of::<L::Patch>();self.phase=7;},
   7=>{let(patch,row)=Self::refs(payload,original,self.index);let step=self.inverse.advance(patch,row,child_grant(grant))?;receipt=progress(step.progress())?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=8;}},
   8=>{let(value,child)=self.inverse.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<L::Patch>()-META,..grant})?;*self.undo=value;receipt=progress(child)?;receipt.copied_bytes+=size_of::<L::Patch>();self.field=0;self.phase=if N==0{10}else{9};},
   9=>{let field=&mut self.identities[self.field];if field.value().is_some(){let(value,child)=field.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<String>>()-META,..grant})?;self.identity[self.field]=value;receipt=progress(child)?;receipt.copied_bytes+=size_of::<Option<String>>();self.field+=1;if self.field==N{self.phase=10;}}else{receipt=progress(field.advance(Self::targets(payload)[self.field],child_grant(grant))?.progress())?;}},
   10=>{let target=std::array::from_fn(|index|self.identity[index].take().expect("prepared original inverse identity"));let operation=L::from_inverse(target,self.undo.take().expect("prepared original inverse fields"));*self.inverse_rows=Some(Vec::with_capacity(1));self.inverse_rows.as_mut().unwrap().push(operation);receipt.retained_capacity_bytes=self.inverse_rows.as_ref().unwrap().capacity()*size_of::<M>();self.field=0;self.phase=11;},
   11=>{let child=self.forward.as_mut().expect("owned authored field plan").exchange_field(&mut L::rows_mut(unpublished)[self.index],self.field,capability,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-META,..grant})?;receipt=progress(child)?;self.field+=1;if self.field==L::Patch::FIELD_COUNT{self.phase=12;}},
   12=>{if !self.retirement.is_empty(){receipt=self.retirement.step_granted(grant)?.progress();}else if let Some(step)=self.retirement.begin_granted(&mut self.forward,grant)?{receipt=step.progress();}else{self.phase=13;}},
   13=>{if let Some(ledger)=self.ledger.as_mut(){let remaining=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Vec<M>>()-size_of::<Vec<super::super::MutationMessage>>()-META,..grant};let(messages,_)=ledger.take_messages_controlled(remaining)?.expect("admitted original diagnostic transfer");*self.pending_messages=Some(messages);}self.ready=true;self.finished=true;return Ok(RetainedMutationAdvance::Ready(receipt));},
   _=>unreachable!("quoted retained row phase"),
  }
  if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained row actual receipt exceeded original admission").with_retained_progress(receipt));}
  Ok(RetainedMutationAdvance::Progress(receipt))
 }
 fn take_outcome(&mut self)->RetainedMutationOutcome<M>{
  assert!(self.ready&&!self.closing,"retained row original outcome unavailable");self.ready=false;
  let messages=self.pending_messages.take().unwrap_or_default();
  match self.decision{RetainedRowDecision::Refuse=>RetainedMutationOutcome::Refused(messages),_=>RetainedMutationOutcome::Admitted(self.inverse_rows.take().unwrap_or_default(),messages)}
 }
 fn begin_close(&mut self){self.closing=true;self.lookup.begin_close();self.guard.begin_close();self.authored.begin_close();self.inverse.begin_close();for identity in &mut self.identities{identity.begin_close();}self.diagnostic.begin_close();}
 fn close_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.lookup.terminal_is_empty(){return self.lookup.close_demand(body);}
  if !self.guard.terminal_is_empty(){return self.guard.close_demand(body);}
  if !self.authored.terminal_is_empty(){return self.authored.close_demand(body);}
  if !self.inverse.terminal_is_empty(){return self.inverse.close_demand(body);}
  if let Some(identity)=self.identities.iter().find(|owner|!owner.terminal_is_empty()){return identity.close_demand(body);}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body);}
  if !self.retirement.is_empty(){return self.retirement_demand(body);}
  if self.forward.is_some()||self.undo.is_some(){return Ok(Self::scalar(0,self.retirement.next_owner_capacity_byte_demand::<L::Patch>(true,body)?));}
  if self.identity.iter().any(Option::is_some){return Ok(Self::scalar(0,self.retirement.next_owner_capacity_byte_demand::<String>(true,body)?));}
  if let Some(inverse)=self.inverse_close.as_ref(){return inverse.demands(context.mutations,body);}
  if self.inverse_rows.is_some(){return Ok(FactoryVectorRetirement::<M>::constructor_demand());}
  if let Some(ledger)=self.ledger.as_ref(){return Ok(RetirementDemand{copy_bytes:ledger.next_copy_byte_demand(),capacity_bytes:ledger.next_capacity_byte_demand(body)?,release_bytes:ledger.next_release_byte_demand()?,depth:ledger.next_depth_demand()?});}
  if self.pending_messages.is_some(){return Ok(Self::scalar(size_of::<Vec<super::super::MutationMessage>>()+size_of::<MutationMessageLedgerRetirement>(),0));}
  Ok(Default::default())
 }
 fn close_step(&mut self,context:RetainedMutationCloseContext<'_,M>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained row closure not started"));}
  if !retained_mutation_demand_fits(self.close_demand(context,grant.maximum_copy_bytes)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.lookup.terminal_is_empty(){return self.lookup.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.guard.terminal_is_empty(){return self.guard.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.authored.terminal_is_empty(){return self.authored.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.inverse.terminal_is_empty(){return self.inverse.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(identity)=self.identities.iter_mut().find(|owner|!owner.terminal_is_empty()){return identity.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(step)=self.retirement.begin_granted(&mut self.forward,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(step)=self.retirement.begin_granted(&mut self.undo,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(identity)=self.identity.iter_mut().find(|owner|owner.is_some()){if let Some(step)=self.retirement.begin_granted(identity,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}}
  if let Some(inverse)=self.inverse_close.as_mut(){let step=inverse.close_step(context.mutations,grant)?;if inverse.terminal_is_empty(){self.inverse_close=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(rows)=self.inverse_rows.take(){return match FactoryVectorRetirement::admit(rows,grant){Ok((owner,receipt))=>{self.inverse_close=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,rows))=>{*self.inverse_rows=Some(rows);Err(error)}};}
  if let Some(ledger)=self.ledger.as_mut(){let step=ledger.close_step(grant)?;if ledger.terminal_is_empty(){*self.ledger=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(messages)=self.pending_messages.take(){*self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),messages));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<super::super::MutationMessage>>()+size_of::<MutationMessageLedgerRetirement>(),..Default::default()}));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn terminal_is_empty(&self)->bool{self.lookup.terminal_is_empty()&&self.guard.terminal_is_empty()&&self.authored.terminal_is_empty()&&self.inverse.terminal_is_empty()&&self.identities.iter().all(RetainedFieldPreparation::terminal_is_empty)&&self.diagnostic.terminal_is_empty()&&self.retirement.is_empty()&&self.identity.iter().all(Option::is_none)&&self.forward.is_none()&&self.undo.is_none()&&self.inverse_rows.is_none()&&self.inverse_close.is_none()&&self.ledger.is_none()&&self.pending_messages.is_none()}
}
impl<P:'static,M:Mutation<P>+Send+'static,L:RetainedRowMutation<P,M,N>,const N:usize> Drop for RetainedRowPreparation<P,M,L,N>{fn drop(&mut self){let empty=<Self as RetainedMutationLeafPreparation<P,M,L>>::terminal_is_empty(self);assert!(std::thread::panicking()||empty,"retained row requires original complete closure before body release");if empty{unsafe{ManuallyDrop::drop(&mut self.identity);ManuallyDrop::drop(&mut self.forward);ManuallyDrop::drop(&mut self.undo);ManuallyDrop::drop(&mut self.inverse_rows);ManuallyDrop::drop(&mut self.ledger);ManuallyDrop::drop(&mut self.pending_messages);}}}}

/// 🎞️ A standalone row event uses the same required owner as its declared semantic leaf.
impl<P:'static,M:Mutation<P>+Send+Sync+RetainedRowMutation<P,M,N>+'static,const N:usize> super::RetainedMutationPreparation<P,M> for RetainedRowPreparation<P,M,M,N>{
 fn new()->Self{<Self as RetainedMutationLeafPreparation<P,M,M>>::new()}
 fn demand(&self,event:RetainedCloneRef<'_,M>,before:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>{<Self as RetainedMutationLeafPreparation<P,M,M>>::demand(self,event,before,post,body)}
 fn advance(&mut self,event:RetainedCloneRef<'_,M>,before:RetainedCloneRef<'_,P>,post:&mut P,grant:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedMutationAdvance,ValueError>{<Self as RetainedMutationLeafPreparation<P,M,M>>::advance(self,event,before,post,grant,capability)}
 fn take_outcome(&mut self)->RetainedMutationOutcome<M>{<Self as RetainedMutationLeafPreparation<P,M,M>>::take_outcome(self)}
 fn begin_close(&mut self){<Self as RetainedMutationLeafPreparation<P,M,M>>::begin_close(self)}
 fn close_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{<Self as RetainedMutationLeafPreparation<P,M,M>>::close_demand(self,context,body)}
 fn close_step(&mut self,context:RetainedMutationCloseContext<'_,M>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{<Self as RetainedMutationLeafPreparation<P,M,M>>::close_step(self,context,grant)}
 fn terminal_is_empty(&self)->bool{<Self as RetainedMutationLeafPreparation<P,M,M>>::terminal_is_empty(self)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
