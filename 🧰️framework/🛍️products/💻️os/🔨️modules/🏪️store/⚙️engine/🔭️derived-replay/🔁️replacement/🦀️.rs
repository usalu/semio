//! 🔁️ One replaced history operation: the Report replay and the draft preview select, receive, verify and close a draft or supersession input through the same retained owners.
use super::*;
use semio_framework_value::{RetirementDemand,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use semio_os_store_operation_wire::{OriginalOperationWireComparison,OriginalReplacementReceiving};
use semio_os_store_replay::EffectiveSupersessions;
use semio_os_store_replay_operation::native::history::{OriginalHistoryInput,OriginalHistorySource};

/// 🧾️ A pending poll keeps its owner and reports no progress; a complete poll settles to its step progress.
fn settled(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneProgress,ValueError>{match poll{semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(Default::default()),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|step.progress()).map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)}}

/// 🤲️ The borrowed owners of one operation's replacement: its phase, its retained inputs and the draft and supersession rows that supply it.
pub(crate) struct Replaced<'a,M:Sync>{
 pub wire:&'a mut OriginalOperationWireComparison<M>,
 pub receiving:&'a mut OriginalReplacementReceiving<M>,
 pub foreign:&'a mut bool,
 pub operation:&'a mut Option<(usize,usize,bool)>,
 pub phase:&'a mut u8,
 pub input:&'a mut OriginalHistoryInput<M>,
 pub source:Option<&'a OriginalHistorySource<Edit<M>>>,
 pub drafts:&'a mut protocol::HistoryInputDrafts,
 pub supersessions:&'a mut EffectiveSupersessions,
}

/// 🚦️ What one replacement step did: it advanced a phase, or the operation is withdrawn and needs no input.
pub(crate) enum ReplacementTurn{Pending(RetainedCloneProgress),Withdrawn(RetainedCloneProgress)}

pub(crate) fn replacement_reset_demand<M:Sync>()->RetirementDemand{
 let copy_bytes=size_of::<Option<(usize,usize,bool)>>()+size_of::<u8>()+size_of::<bool>()
  +size_of::<OriginalOperationWireComparison<M>>()
  +size_of::<OriginalReplacementReceiving<M>>();
 RetirementDemand{copy_bytes,depth:1,..Default::default()}
}

fn replacement_row<P,M,R>(source:&OriginalHistorySource<Edit<M>>,drafts:&mut protocol::HistoryInputDrafts,supersessions:&mut EffectiveSupersessions,operation:usize,draft:bool,work:impl FnOnce(&mut protocol::InputReplacement)->Result<R,ValueError>)->Result<R,ValueError>where M:Mutation<P>+Send+'static{
 let original=source.borrow_original()?;semio_os_store_ledger::native_digest::with_operation_identity::<P,M,_>(original.get(),operation,|id|{let row=if draft{drafts.get_mut(id)}else{supersessions.get_mut(id).map(|row|&mut row.replacement)}.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original replay replacement row is absent"))?;work(row)})
}

/// 🔎️ The draft (`true`) or supersession row of operation `operation` and whether it withdraws the operation.
pub(crate) fn select_replacement<P,M>(source:&OriginalHistorySource<Edit<M>>,drafts:&protocol::HistoryInputDrafts,supersessions:&EffectiveSupersessions,operation:usize)->Result<Option<(bool,bool)>,ValueError>where M:Mutation<P>+Send+'static{
 let original=source.borrow_original()?;
 semio_os_store_ledger::native_digest::with_operation_identity::<P,M,_>(original.get(),operation,|id|Ok(drafts.get(id).map(|input|(true,input)).or_else(||supersessions.get(id).map(|row|(false,&row.replacement))).map(|(draft,input)|(draft,matches!(input,protocol::InputReplacement::Withdrawn)))))
}

fn verify_replacement_wire<P,M>(r:&mut Replaced<'_,M>,wire:ArtifactOperationWire<M>,authored:bool,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,maximum_work:u64)->Result<RetainedCloneProgress,ValueError>where M:Mutation<P>+Send+'static{
 let input=r.input.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical replacement comparison lost its genuine input"))?.borrow()?;let mut compare=|bytes:&[u8]|{let maximum=control.maximum_bytes();let d=r.wire.demands(input,bytes,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(Default::default())}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;Ok(control.with_effect(grant,maximum_work.saturating_sub(semio_framework_value::ORIGINAL_INVOCATION_WORK),|native,effect|{native.charge(d.capacity_bytes)?;r.wire.advance(input,bytes,wire,maximum,effect)})?.transpose()?.unwrap_or_default())};
 if authored{let source=r.source.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical replacement comparison lost its original history"))?;let(_,operation,draft)=(*r.operation).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical replacement comparison lost its coordinates"))?;replacement_row::<P,M,_>(source,r.drafts,r.supersessions,operation,draft,|row|{let protocol::InputReplacement::Input{payload,..}=row else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical replacement input changed to withdrawal"))};compare(payload)})}else{let bytes=r.receiving.borrow_payload()?;compare(bytes.get())}
}

/// 🧹️ Closes one frontier of a replaced operation's owners and resets them once none is left.
pub(crate) fn close_replacement<P,M>(r:&mut Replaced<'_,M>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<RetainedCloneProgress,ValueError>where M:Mutation<P>+Send+'static{
 let grant=effect.grant();
 if !r.wire.terminal_is_empty(){let d=r.wire.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(Default::default())}control.charge(d.capacity_bytes)?;return r.wire.close_step(effect).map(|step|step.progress())}
 if let Some(input)=r.input.as_mut(){if !input.terminal_is_empty(){let demand=input.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(Default::default())}control.charge(demand.capacity_bytes)?;return settled(input.close_step(effect))}let copy=size_of::<Option<semio_framework_value::RetainedOwnedProjection<M>>>();let d=RetirementDemand{copy_bytes:copy,depth:1,..Default::default()};if !retained_clone_grant_admits(grant,d){return Ok(Default::default())}r.input.vacate();return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})}
 if !r.receiving.terminal_is_empty(){let(_,operation,draft)=(*r.operation).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original replay replacement coordinates are absent"))?;let source=r.source.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original replay replacement history source is absent"))?;let receiving=&mut *r.receiving;return replacement_row::<P,M,_>(source,r.drafts,r.supersessions,operation,draft,|row|settled(receiving.close_native(row,&mut *control,effect)))}
 let demand=replacement_reset_demand::<M>();if !retained_clone_grant_admits(grant,demand){return Ok(Default::default())}
 *r.operation=None;
 *r.phase=0;
 *r.wire=Default::default();
 *r.receiving=Default::default();
 *r.foreign=false;
 Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
}

impl<'a, P, M, Log> StoreDerived<'a, P, M, Log>
where
    P: semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>,
    Log: std::ops::Deref<Target = StoreLog<P, M>>,
{
 /// 🌱️ Binds operation `coordinate` to its selected replacement after the cross-artifact and foreign-planning guards; an unfunded grant leaves it unbound.
 pub(crate) fn begin_replacement(&self,r:&mut Replaced<'_,M>,coordinate:(usize,usize),selected:(bool,bool),grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,VcsError>{
  let (draft,withdrawn)=selected;let (cursor,operation)=coordinate;
  let copy=size_of::<Option<(usize,usize,bool)>>()+size_of::<u8>()+size_of::<bool>()+32;let d=RetirementDemand{copy_bytes:copy,depth:1,..Default::default()};if !retained_clone_grant_admits(grant,d){return Ok(None)}
  let source=r.source.ok_or(VcsError::HistoryReplaying)?.borrow_original().map_err(VcsError::InverseRefused)?;let original=source.get();let digest=CursorRevisionAccumulator::hash_record(b"edit-id",&[original.id.as_bytes()]);let presence=self.cursor.revision_accumulator.unit_flags.get(&(digest,operation));
  if presence==Some(&true)||matches!(original.mutation_meta.get(operation).map(|meta|&meta.origin),Some(protocol::MutationOrigin::Transaction{..})){return Err(VcsError::InverseRefused(ValueError::literal(ValueRefusalKind::InvalidValue,"one document cannot supersede an operation of a cross-artifact unit")))}
  if original.forwards[operation].may_emit_foreign_steps()&&presence.is_none(){return Err(VcsError::InverseRefused(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"foreign-planning history supersession requires its original recorded foreign-presence fact")))}
  *r.foreign=!withdrawn&&r.input.as_ref().ok_or(VcsError::HistoryReplaying)?.borrow().map_err(VcsError::InverseRefused)?.get().may_emit_foreign_steps();
  *r.operation=Some((cursor,operation,draft));*r.phase=if withdrawn{4}else{1};
  Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
 }

 /// ⏭️ Advances the bound replacement one funded step through verify, recorded-input close, native receive and replacement verify; `None` once the replacement input stands ready or no operation is bound.
 pub(crate) fn advance_replacement(&self,r:&mut Replaced<'_,M>,withdrawn_copy:usize,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,maximum_work:u64)->Result<Option<ReplacementTurn>,VcsError>{
  let idle=||->Result<Option<ReplacementTurn>,VcsError>{Ok(Some(ReplacementTurn::Pending(Default::default())))};
  if *r.phase==1&&*r.foreign&&!r.wire.prepared(){let wire=self.log.operation_wire.ok_or(VcsError::HistoryReplaying)?;let p=verify_replacement_wire::<P,M>(r,wire,true,grant,control,maximum_work).map_err(VcsError::InverseRefused)?;return Ok(Some(ReplacementTurn::Pending(p)))}
  if matches!(*r.phase,1|4){
   if let Some(input)=r.input.as_mut(){let d=if input.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::RetainedOwnedProjection<M>>>(),depth:1,..Default::default()}}else{input.close_demand(grant.maximum_copy_bytes).map_err(VcsError::InverseRefused)?};if !retained_clone_grant_admits(grant,d){return idle()}
    if input.terminal_is_empty(){r.input.vacate();return Ok(Some(ReplacementTurn::Pending(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
    let p=funded_plan_retirement(control,grant,|effect|input.close_step(effect))?;return Ok(Some(ReplacementTurn::Pending(p)))}
   let copy=size_of::<u8>()+if *r.phase==4{withdrawn_copy}else{0};if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return idle()}
   let progress=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};
   if *r.phase==4{*r.phase=5;return Ok(Some(ReplacementTurn::Withdrawn(progress)))}*r.phase=2;return Ok(Some(ReplacementTurn::Pending(progress)))
  }
  if *r.phase==2{let(_,operation,draft)=(*r.operation).ok_or(VcsError::HistoryReplaying)?;let source=r.source.ok_or(VcsError::HistoryReplaying)?;let factory=self.owners.mutation_retirement_factory.as_ref().ok_or(VcsError::HistoryReplaying)?;
   if r.receiving.ready(){let copy=size_of::<u8>();let d=RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::RetainedOwnedProjection<M>>>()+copy,depth:1,..Default::default()};if !retained_clone_grant_admits(grant,d){return idle()}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-copy,..grant};let mut p=r.receiving.take_input_native(r.input.slot(),control,child).map_err(VcsError::InverseRefused)?;if r.input.occupied(){*r.phase=3;p.copied_bytes+=copy;}return Ok(Some(ReplacementTurn::Pending(p)))}
   let wire=self.log.operation_wire.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original report requires its installed operation wire")).map_err(VcsError::InverseRefused)?;let receiving=&mut *r.receiving;let schema=&self.log.envelope.schema;
   let p=replacement_row::<P,M,_>(source,r.drafts,r.supersessions,operation,draft,|row|{if let protocol::InputReplacement::Input{schema:row_schema,..}=row{if row_schema!=schema{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original replay replacement schema differs from its artifact"))}}receiving.advance_native(row,wire,factory,control,grant,maximum_work.saturating_sub(semio_framework_value::ORIGINAL_INVOCATION_WORK))}).map_err(VcsError::InverseRefused)?;
   return Ok(Some(ReplacementTurn::Pending(p)))
  }
  if *r.phase==3&&!*r.foreign{if r.input.as_ref().ok_or(VcsError::HistoryReplaying)?.borrow().map_err(VcsError::InverseRefused)?.get().may_emit_foreign_steps(){return Err(VcsError::InverseRefused(ValueError::literal(ValueRefusalKind::InvalidValue,"replacement of a local operation cannot plan foreign steps")))}if !r.wire.prepared(){let wire=self.log.operation_wire.ok_or(VcsError::HistoryReplaying)?;let p=verify_replacement_wire::<P,M>(r,wire,false,grant,control,maximum_work).map_err(VcsError::InverseRefused)?;return Ok(Some(ReplacementTurn::Pending(p)))}}
  Ok(None)
 }
}
