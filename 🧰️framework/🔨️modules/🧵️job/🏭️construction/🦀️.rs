//! 🏗️ The original Job decoder owns generic construction, candidate custody and complete physical receipts.
use super::{StepContext,OriginalTurnFailure,Generation,OperationId,OriginalPayloadSourceIdentity};
use semio_framework_value_retained_clone::{ValueError, ValueRefusalKind, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
/// 📏️ One defining owner declares all five independent currencies before a native birth.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalConstructionDemand{pub items:usize,pub copy_bytes:usize,pub capacity_bytes:usize,pub release_bytes:usize,pub depth:usize}
impl OriginalConstructionDemand{
 pub fn fits(self,grant:RetainedCloneGrant)->bool{self.items>0&&self.depth>0&&self.items<=grant.maximum_items&&self.copy_bytes<=grant.maximum_copy_bytes&&self.capacity_bytes<=grant.maximum_capacity_bytes&&self.release_bytes<=grant.maximum_release_bytes&&self.depth<=grant.maximum_depth}
 fn physical(self)->RetirementDemand{RetirementDemand{copy_bytes:self.copy_bytes,capacity_bytes:self.capacity_bytes,release_bytes:self.release_bytes,depth:self.depth}}
}
/// 🚦️ Constructor progress never substitutes a summary for its retained complete causes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalConstructionStatus{Pending,Ready,Failed}
/// 🧳️ The actual child and independent original Source refusals move together.
#[derive(Debug,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub struct OriginalConstructionFailure{pub source:super::OriginalNativeReceivingOutcome,pub child:Option<Result<RetainedCloneProgress,super::OriginalReceivingPrimary>>}

enum ConstructionCause<'a>{Native(&'a super::OriginalReceivingPrimary),Value(&'a ValueError)}
impl OriginalConstructionFailure{
 fn cause(&self,index:usize)->Result<(&'static str,ConstructionCause<'_>),ValueError>{let call=self.source.call.as_ref().err();[self.child.as_ref().and_then(|child|child.as_ref().err()).map(|e|("native",ConstructionCause::Native(e))),call.and_then(|e|e.operation.as_ref()).map(|e|("operation",ConstructionCause::Value(e))),call.and_then(|e|e.context.as_ref()).map(|e|("context",ConstructionCause::Value(e))),call.and_then(|e|e.receipt_refusal.as_ref()).map(|e|("receiptRefusal",ConstructionCause::Value(e))),self.source.source_refusal.as_ref().map(|e|("sourcePosting",ConstructionCause::Value(e)))].into_iter().flatten().nth(index).ok_or_else(||ValueError::invariant("construction cause ordinal changed"))}
 fn cause_count(&self)->usize{usize::from(self.child.as_ref().is_some_and(Result::is_err))+self.source.call.as_ref().err().map_or(0,|e|usize::from(e.operation.is_some())+usize::from(e.context.is_some())+usize::from(e.receipt_refusal.is_some()))+usize::from(self.source.source_refusal.is_some())}
}
impl super::OriginalDomainProjection for OriginalConstructionFailure{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<super::OriginalDomainView<'_>,ValueError>{let Some((&index,rest))=path.split_first()else{return Ok(super::OriginalDomainView::IntrinsicObject(self.cause_count()))};match self.cause(index)?.1{ConstructionCause::Native(error)=>super::project_original_receiving_primary(error,rest),ConstructionCause::Value(error)=>super::project_original_value_error(error,rest)}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let Some((&field,rest))=path.split_first()else{return self.cause(index).map(|(name,_)|name)};match self.cause(field)?.1{ConstructionCause::Native(error)=>super::key_original_receiving_primary(error,rest,index),ConstructionCause::Value(error)=>super::key_original_value_error(error,rest,index)}}
}
impl super::OriginalDomainFailure for OriginalConstructionFailure{
 fn domain_code(&self)->&str{"original-construction"}
 fn domain_message(&self)->&str{"Original native construction failed"}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.child.as_ref().map_or_else(Default::default,|result|match result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()})}
}
/// 🏭️ Resident producer identity; each turn lends only the one selected construction call.
#[derive(Default,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub struct ConstructionProducer{claimed:bool}
/// 🧵️ Lends the genuine Source witness and the selected construction call for one original effect.
pub struct ConstructionRun<'turn,'source>{witness:semio_framework_value_native_owned::native_decoding::owned::OriginalSourceTurnWitness<'source>,run:&'turn mut dyn FnMut(&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>}
impl semio_framework_value_native_owned::native_decoding::OriginalEffectProducer for ConstructionProducer{
 type Output=RetainedCloneStep;type Failure=ValueError;type Context<'turn,'source>=ConstructionRun<'turn,'source> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<super::OriginalSourceTurnIdentity>where 'source:'turn{Some(context.witness.identity())}
 fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_owned::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<semio_framework_value_native_owned::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{self.claimed=true;let result=(context.run)(effect);Ok(reply.complete(result))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_owned::native_decoding::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<semio_framework_value_native_owned::native_decoding::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{Ok(reply.pending())}
}
fn child_primary(failure:semio_framework_value_native_owned::native_decoding::NativeReceivingEffectFailure<ValueError>)->super::OriginalReceivingPrimary{let mut receiving=failure.receiving;receiving.allocation_refusal=failure.child.or(receiving.allocation_refusal.take());receiving.primary=receiving.primary.take().or(failure.producer_refusal);receiving.work_refusal=receiving.work_refusal.take().or(failure.work_refusal);receiving}
/// 🏠️ One caller-owned frame survives callback interruption and every failed publication.
#[derive(Default,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub struct OriginalConstructionAdmission{binding:Option<super::OriginalSourceBinding>,authority:Option<semio_framework_value_native_owned::native_decoding::OriginalEffectAuthority<ConstructionProducer>>,frame:semio_framework_value_native_owned::native_decoding::NativeReceivingEffect,source:Option<super::OriginalNativeReceivingOutcome>,child:Option<Result<RetainedCloneProgress,super::OriginalReceivingPrimary>>,failed_taken:bool,complete:bool,captured:bool}
impl OriginalConstructionAdmission{
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(ValueError::invariant("constructor changed its actual Source").into())}Ok(())}
 pub fn has_failure(&self)->bool{self.failed_taken||self.source.as_ref().is_some_and(|source|source.call.is_err()||source.source_refusal.is_some())||self.child.as_ref().is_some_and(Result::is_err)}
 /// 🔁️ Explicitly retires a completed attempt before its caller advances to another construction.
 pub fn reset_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.has_failure()||self.source.is_some()||self.child.is_some()||self.frame.phase()!=semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty{return Err(ValueError::invariant("constructor still owns an unfinished attempt").into())}if !self.complete{return Ok(true)}if construction_metadata(cx,std::mem::size_of::<bool>(),1)?{self.complete=false;return Ok(true)}Ok(false)}
 fn release_authority(&mut self){if let Some(authority)=self.authority.take(){if let Err(authority)=authority.into_producer().map(drop){self.authority=Some(authority)}}}
 pub fn pending_recovery(&mut self)->Option<&mut semio_framework_value_native_owned::native_decoding::NativeReceivingEffect>{self.frame.pending_recovery()}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.authority.is_none()&&self.source.is_none()&&self.child.is_none()&&!self.complete&&!self.failed_taken&&self.frame.phase()==semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty}
 /// ♻️ Retires settled control metadata without starting another semantic constructor.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.frame.phase()!=semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPhase::Empty||self.has_failure()&&!self.failed_taken{return Ok(false)}if self.source.is_some()||self.child.is_some(){let bytes=std::mem::size_of_val(&self.source)+std::mem::size_of_val(&self.child);if construction_metadata(cx,bytes,2)?{self.source=None;self.child=None}return Ok(false)}if self.authority.is_some(){if !construction_metadata(cx,std::mem::size_of_val(&self.authority),1)?{return Ok(false)}self.release_authority();if self.authority.is_some(){return Err(ValueError::invariant("constructor authority still owns an action").into())}}if self.binding.is_some()||self.complete||self.failed_taken{if !construction_metadata(cx,std::mem::size_of_val(&self.binding)+2*std::mem::size_of::<bool>(),3)?{return Ok(false)}self.binding=None;self.complete=false;self.failed_taken=false}Ok(true)}
 /// 📤️ Transfers all original causes only to the caller's actual empty destination.
 pub fn take_failure_into_step(&mut self,output:&mut Option<OriginalConstructionFailure>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if output.is_some(){return Err(ValueError::invariant("construction failure destination is occupied").into())}if !self.has_failure()||self.failed_taken{return Ok(false)}let bytes=std::mem::size_of::<Option<OriginalConstructionFailure>>()+std::mem::size_of::<Option<super::OriginalNativeReceivingOutcome>>()+std::mem::size_of_val(&self.child)+std::mem::size_of::<bool>();if !construction_metadata(cx,bytes,4)?{return Ok(false)}*output=Some(OriginalConstructionFailure{source:self.source.take().expect("failed constructor retains its Source outcome"),child:self.child.take()});self.failed_taken=true;Ok(true)}
}
fn construction_metadata(cx:&mut StepContext<'_>,bytes:usize,items:usize)->Result<bool,OriginalTurnFailure>{let progress=RetainedCloneProgress{copied_items:items,copied_bytes:bytes,..Default::default()};if cx.retirement_should_yield()||cx.fuel_remaining()<items as u64||cx.retained_grant().maximum_depth==0||!progress.fits(cx.retained_grant()){return Ok(false)}cx.consume_retained(progress)?;cx.consume_fuel(items as u64);Ok(true)}
/// 🧾️ Quotes exactly the control metadata that settles one finished construction after its constructor ran (bind, release of the Source outcome and child, reset): items, copy bytes and fuel.
pub fn original_construction_settle_overhead()->(usize,usize,u64){
 let release=std::mem::size_of::<Option<super::OriginalNativeReceivingOutcome>>()+std::mem::size_of::<Option<Result<RetainedCloneProgress,super::OriginalReceivingPrimary>>>()+std::mem::size_of::<Option<semio_framework_value_native_owned::native_decoding::OriginalEffectAuthority<ConstructionProducer>>>()+std::mem::size_of::<bool>();
 let copy=std::mem::size_of::<Option<super::OriginalSourceBinding>>()+release+std::mem::size_of::<bool>();
 (5,copy,5)
}
/// 📏️ Quotes the complete same-turn constructor, candidate, capture and Source result plan.
pub fn original_construction_reserve<O>(demand:OriginalConstructionDemand,work:u64)->Result<(OriginalConstructionDemand,u64),ValueError>{
 let source=super::OriginalNativeReceivingOutcome::<()>::placement();let capture=std::mem::size_of::<Option<Result<RetainedCloneProgress,super::OriginalReceivingPrimary>>>();
 let items=demand.items.checked_add(2).and_then(|n|n.checked_add(source.copied_items)).ok_or_else(||ValueError::invariant("construction item quote overflow"))?;
 let copy=demand.copy_bytes.checked_add(std::mem::size_of::<Option<O>>()).and_then(|n|n.checked_add(capture)).and_then(|n|n.checked_add(source.copied_bytes)).ok_or_else(||ValueError::invariant("construction copy quote overflow"))?;
 let work=work.checked_add(4).ok_or_else(||ValueError::invariant("construction work quote overflow"))?;
 Ok((OriginalConstructionDemand{items,copy_bytes:copy,..demand},work))
}
/// 🎟️ The actual caller retains candidate, allocation frame, complete native child and Source outcome.
pub fn admit_original_construction<O>(context:&mut StepContext<'_>,output:&mut Option<O>,admission:&mut OriginalConstructionAdmission,demand:OriginalConstructionDemand,work:u64,construct:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope<'_, '_>,&mut Option<usize>)->semio_framework_value_retained_clone::RetainedWorkOutcome<(O,RetainedCloneProgress)>)->Result<OriginalConstructionStatus,OriginalTurnFailure>{
 admission.validate(context)?;
 if admission.has_failure(){return Ok(OriginalConstructionStatus::Failed)}
 if admission.complete{return Ok(OriginalConstructionStatus::Ready)}
 if admission.source.is_some(){let ready=output.is_some();let bytes=std::mem::size_of_val(&admission.source)+std::mem::size_of_val(&admission.child)+std::mem::size_of_val(&admission.authority)+std::mem::size_of::<bool>();if !construction_metadata(context,bytes,3)?{return Ok(OriginalConstructionStatus::Pending)}admission.source=None;admission.child=None;admission.release_authority();admission.complete=ready;return Ok(if ready{OriginalConstructionStatus::Ready}else{OriginalConstructionStatus::Pending})}
 if output.is_some(){return Err(ValueError::invariant("construction requires its original empty candidate").into())}
 if context.is_cancelled()||context.should_yield(){return Ok(OriginalConstructionStatus::Pending)}
 if admission.binding.is_none(){if construction_metadata(context,std::mem::size_of_val(&admission.binding),1)?{admission.binding=Some(super::OriginalSourceBinding::borrow(context))}return Ok(OriginalConstructionStatus::Pending)}
 let candidate=std::mem::size_of::<Option<O>>();let capture=RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of_val(&admission.child),..Default::default()};let(complete,total_work)=original_construction_reserve::<O>(demand,work)?;
 if work==0||!complete.fits(context.retained_grant())||context.fuel_remaining()<total_work{return Ok(OriginalConstructionStatus::Pending)}
 if !admission.captured{context.consume_retained(capture)?;context.consume_fuel(1);admission.captured=true}
 let mut physical=demand.physical();physical.copy_bytes+=candidate;let body_grant=RetainedCloneGrant{maximum_items:demand.items,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
 let OriginalConstructionAdmission{frame,source,child,authority,..}=admission;let authority=authority.get_or_insert_with(||semio_framework_value_native_owned::native_decoding::OriginalEffectAuthority::new(ConstructionProducer::default()));
 let quote=RetainedCloneGrant{maximum_items:complete.items,maximum_copy_bytes:complete.copy_bytes,maximum_capacity_bytes:complete.capacity_bytes,maximum_release_bytes:complete.release_bytes,maximum_depth:complete.depth};
 context.with_native_receiving_quoted(source,(quote,total_work),|native,_body_grant|{
  let witness=native.source_witness()?;let mut construct=Some(construct);
  let poll=native.receive_prepared_construction(physical,frame,work+1,authority,|scope,effect,authority,action|{
   let mut run=|effect:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>|->Result<RetainedCloneStep,ValueError>{
    let construct=construct.take().ok_or_else(||ValueError::invariant("construction call already consumed"))?;
    let turn=scope.with_limits(body_grant,work,|scope|construct(scope,effect.actual()))?;
    let step=turn.result.and_then(|(candidate,progress)|{*output=Some(candidate);progress.checked_add(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<O>>(),..Default::default()}).map(RetainedCloneStep::Complete)})?;
    scope.checkpoint().map_err(|error|error.with_retained_progress(step.progress()))?;
    let total=turn.work.checked_add(u128::from(output.is_some())).expect("primitive work is bounded by its held construction scope");
    let owed=u64::try_from(total.saturating_sub(effect.performed_work())).map_err(|_|ValueError::invariant("construction work overflow"))?;
    if owed!=0&&!effect.performed(owed)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"construction exceeded its original work"))}
    Ok(step)
   };
   authority.drive(&mut ConstructionRun{witness,run:&mut run},effect,action)
  })?;
  if let semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPoll::Complete(turn)=poll{*child=Some(turn.result.map(|step|step.progress()).map_err(child_primary));}
  Ok(())
 })?;
 Ok(if admission.has_failure(){OriginalConstructionStatus::Failed}else{OriginalConstructionStatus::Pending})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
