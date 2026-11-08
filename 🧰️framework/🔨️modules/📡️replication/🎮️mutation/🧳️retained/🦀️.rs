//! 🧳️ Required immutable decision and inverse owners yield typed diffs for the central applier.
use super::{ColdMutation, Mutation,MutationMessage,MutationMessageLedgerRetirement};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use std::sync::Arc;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,admit_retained_clone_progress,admit_retained_clone_close};
use std::mem::{size_of,ManuallyDrop};
pub use semio_framework_replication_retained_field::{owe,settle,settle_exit,left,RetainedFieldPreparation,RetainedFieldChange,RetainedRecordChange,RetainedRecordChangeField,RetainedRecordFieldProjection,RetainedRecordFieldChange,RetainedRecordChangeOwner,boxed_record_field,retained_mutation_demand_fits};
#[path="✂️span/🦀️.rs"] pub mod span;
pub use span::RetainedByteSpan;
pub use semio_framework_replication_retained_key as key;
pub use key::{RetainedKeyLookup, RetainedKeyedRow,RetainedKeyMatch};
#[path="🩹️patch/🦀️.rs"] pub mod patch;
pub use patch::{RetainedRowPatch,RetainedRowPatchFields,RetainedPatchSource,RetainedRowPatchApply};
#[path="⚠️diagnostic/🦀️.rs"] pub mod diagnostic;
pub use diagnostic::{RetainedDiagnosticSpec,RetainedDiagnosticTarget,RetainedDiagnosticPreparation};
#[path="🎛️set-field/🦀️.rs"] pub mod set_field;
pub use set_field::{RetainedSetField,RetainedSetFieldLeaf,RetainedPatchDiffApply};
#[path="🔺️diff/🦀️.rs"] pub mod diff;
pub use diff::{RetainedDiff,RetainedDiffFragment,RetainedDiffSequence,RetainedDiffApply,RetainedDiffOwner,RetainedDiffAdmission,RetainedDiffPreflightAdvance,RetainedDiffPreflightStep,retained_diff_birth_demand,begin_retained_diff,preflight_retained_diff,commit_retained_diff};

/// 🏭️ The exact original mutation issuer remains borrowed through every inverse cancellation.
pub struct RetainedMutationCloseContext<'a,M>{pub mutations:&'a Arc<dyn ArtifactOwnedValueRetirementFactory<M>>}
impl<M> Copy for RetainedMutationCloseContext<'_,M>{}
impl<M> Clone for RetainedMutationCloseContext<'_,M>{fn clone(&self)->Self{*self}}
/// 🪪️ Data and concrete type are compared without retaining or dereferencing the borrowed issuer.
struct FactoryIssuerIdentity<M>{data:usize,implementation:std::any::TypeId,marker:std::marker::PhantomData<fn(M)>}
impl<M> Copy for FactoryIssuerIdentity<M>{}
impl<M> Clone for FactoryIssuerIdentity<M>{fn clone(&self)->Self{*self}}
impl<M:'static> FactoryIssuerIdentity<M>{
 fn capture_copy_bytes()->usize{size_of::<Self>()+size_of::<Option<Self>>()}
 fn capture(original:&dyn ArtifactOwnedValueRetirementFactory<M>)->Self{Self{data:std::ptr::from_ref(original).cast::<()>() as usize,implementation:std::any::Any::type_id(original),marker:std::marker::PhantomData}}
 fn matches(self,original:&dyn ArtifactOwnedValueRetirementFactory<M>)->bool{let current=Self::capture(original);self.data==current.data&&self.implementation==current.implementation}
}
/// 🚦️ A receipt precedes every original result transfer.
pub enum RetainedMutationAdvance{Progress(RetainedCloneProgress),Ready(RetainedCloneProgress)}
impl RetainedMutationAdvance{pub fn progress(&self)->RetainedCloneProgress{match self{Self::Progress(p)|Self::Ready(p)=>*p}}}
/// 🔺️ The original decision yields a real serializable typed diff and its original diagnostics.
pub enum RetainedMutationDecision<D>{Admitted(D,Vec<MutationMessage>),Refused(Vec<MutationMessage>)}
/// 📨️ Central decision admission scans diagnostics before lending a typed diff.
pub enum RetainedMutationDecisionStep<D>{Progress(RetainedCloneProgress),Admitted(RetainedCloneProgress,D,Vec<MutationMessage>),Refused(RetainedCloneProgress,Vec<MutationMessage>)}
impl<D> RetainedMutationDecisionStep<D>{pub fn progress(&self)->RetainedCloneProgress{match self{Self::Progress(p)|Self::Admitted(p,..)|Self::Refused(p,..)=>*p}}}
/// ↩️ Inverse rows transfer only after the exact central diff preflight witness.
pub enum RetainedMutationInverseStep<M>{Progress(RetainedCloneProgress),Ready(RetainedCloneProgress,Vec<M>)}
impl<M> RetainedMutationInverseStep<M>{pub fn progress(&self)->RetainedCloneProgress{match self{Self::Progress(p)|Self::Ready(p,..)=>*p}}}

/// 🪡️ Technology semantics inspect only genuine immutable payload, base and yielded diff.
/// Decision owns typed D and original diagnostics; inversion follows admitted central preflight.
/// Neither port receives mutable post or an apply capability. Every child stays closeable.
pub trait RetainedMutationPreparation<P,M:Mutation<P,Diff:RetainedDiff<P>>>:Send+'static{
 fn new()->Self;
 fn birth_work()->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained mutation preparation lacks a bounded constructor"))}
 fn decision_demand(&self,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn decision_work_demand(&self,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn decide(&mut self,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedMutationAdvance,OriginalRetirementFailure>;
 fn take_decision(&mut self)->RetainedMutationDecision<M::Diff>;
 fn inverse_demand(&self,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,diff:RetainedCloneRef<'_,M::Diff>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn inverse_work_demand(&self,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,diff:RetainedCloneRef<'_,M::Diff>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn inverse(&mut self,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,diff:RetainedCloneRef<'_,M::Diff>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedMutationAdvance,OriginalRetirementFailure>;
 fn take_inverse(&mut self)->Vec<M>;
 fn begin_close(&mut self);
 fn close_started(&self)->bool;
 fn begin_close_demand(&self)->RetirementDemand;
 fn begin_close_work_demand(&self)->u64;
 fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>;
 fn close_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,context:RetainedMutationCloseContext<'_,M>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
}
/// 🧬️ Retained semantics and central typed diff application are both mandatory.
pub trait RetainedMutation<P>:Mutation<P,Diff:RetainedDiff<P>>+semio_framework_value::retirement::RetireOwned+Send+Sync+'static{type Preparation:RetainedMutationPreparation<P,Self>;}
/// 🧩️ Every declared aggregate leaf supplies its explicit bounded semantic owner.
pub trait RetainedMutationLeaf<P,M:Mutation<P,Diff:RetainedDiff<P>>>:Send+Sync+Sized+'static{type Preparation:RetainedMutationLeafPreparation<P,M,Self>;}
/// 🪆️ Required leaf ports preserve the aggregate's original diff and exact inverse issuer.
pub trait RetainedMutationLeafPreparation<P,M:Mutation<P,Diff:RetainedDiff<P>>,L>:Send+'static{
 fn new()->Self;
 fn birth_work()->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained leaf lacks a bounded constructor"))}
 fn decision_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn decision_work_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn decide(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedMutationAdvance,OriginalRetirementFailure>;
 fn take_decision(&mut self)->RetainedMutationDecision<M::Diff>;
 fn inverse_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,P>,diff:RetainedCloneRef<'_,M::Diff>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn inverse_work_demand(&self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,P>,diff:RetainedCloneRef<'_,M::Diff>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn inverse(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,P>,diff:RetainedCloneRef<'_,M::Diff>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedMutationAdvance,OriginalRetirementFailure>;
 fn take_inverse(&mut self)->Vec<M>;
 fn begin_close(&mut self);
 fn close_started(&self)->bool;
 fn begin_close_demand(&self)->RetirementDemand;
 fn begin_close_work_demand(&self)->u64;
 fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>;
 fn close_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,context:RetainedMutationCloseContext<'_,M>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
}

/// ⌨️ Original aliases, typed decision custody and exact inverse child survive cancellation.
pub struct RetainedMutationOwner<P,M:RetainedMutation<P>>{
 preparation:ManuallyDrop<Option<Box<M::Preparation>>>,
 mutation_source:Option<RetainedCloneBinding>,
 original_source:Option<RetainedCloneBinding>,
 diff_source:Option<RetainedCloneBinding>,
 decision:ManuallyDrop<Option<M::Diff>>,
 messages:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,
 retirement:RetainedCloneClose,
 phase:u8,
 message_index:usize,
 refusal:bool,
 decision_admitted:bool,
 closing:bool,
 inverse_issuer:Option<FactoryIssuerIdentity<M>>,
 projection:std::marker::PhantomData<fn()->P>,
}
/// 📦️ Fixed semantic owner birth inspects no payload or projection.
pub fn retained_mutation_birth_demand<P,M:RetainedMutation<P>>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<RetainedMutationOwner<P,M>>().saturating_mul(2).saturating_add(size_of::<M::Preparation>()),capacity_bytes:size_of::<M::Preparation>(),depth:1,..Default::default()}}
/// 🌱️ Insufficient grants create no child or alias.
pub fn retained_mutation_birth_work<P,M:RetainedMutation<P>>()->Result<u64,ValueError>{
 let constructor=M::Preparation::birth_work()?;
 let bytes=size_of::<RetainedMutationOwner<P,M>>().checked_mul(2).and_then(|n|n.checked_add(size_of::<M::Preparation>())).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained mutation birth extent overflow"))?;
 constructor.checked_add(u64::try_from(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"retained mutation birth work overflow"))?).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained mutation birth work overflow"))
}
pub fn begin_retained_mutation<P,M:RetainedMutation<P>>(maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<(RetainedMutationOwner<P,M>,RetainedCloneProgress)>>{
 let grant=effect.grant();
 let work=match retained_mutation_birth_work::<P,M>(){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
 let d=retained_mutation_birth_demand::<P,M>();if maximum_work_units<work||effect.maximum_work_units()<work||!retained_mutation_demand_fits(d,grant){return RetainedWorkOutcome{result:Ok(None),work:0}}
 if effect.actual().is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("original mutation birth allocation marker is occupied")),work:0}}
 match effect.performed(work){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retained mutation birth exceeded its original action")),work:u128::from(work)},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:u128::from(work)}}
 let layout=std::alloc::Layout::new::<M::Preparation>();let pointer=if layout.size()==0{std::ptr::NonNull::<M::Preparation>::dangling().as_ptr()}else{*effect.actual()=Some(0);unsafe{std::alloc::alloc(layout)}.cast::<M::Preparation>()};
 if pointer.is_null(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"retained mutation preparation allocation refused")),work:u128::from(work)}}
 if layout.size()!=0{*effect.actual()=Some(layout.size());}unsafe{pointer.write(M::Preparation::new());}
 let preparation=unsafe{Box::from_raw(pointer)};
 let owner=RetainedMutationOwner{preparation:ManuallyDrop::new(Some(preparation)),mutation_source:None,original_source:None,diff_source:None,decision:ManuallyDrop::new(None),messages:ManuallyDrop::new(None),retirement:Default::default(),phase:0,message_index:0,refusal:false,decision_admitted:false,closing:false,inverse_issuer:None,projection:std::marker::PhantomData};
 RetainedWorkOutcome{result:Ok(Some((owner,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))),work:u128::from(work)}
}
/// 🧾️ Settles a typed turn against its effect once; a work refusal never drops the primary failure.
pub fn settle_value<T>(effect:&mut RetainedAllocationEffect<'_>,base:u128,result:Result<T,OriginalRetirementFailure>,work:u128,progress:impl FnOnce(&T)->RetainedCloneProgress)->RetainedWorkOutcome<T,OriginalRetirementFailure>{
 let result=match(result,owe(effect,base,work)){
  (result,Ok(()))=>result,
  (Ok(step),Err(refusal))=>Err(OriginalRetirementFailure::from_work_refusal(refusal.with_retained_progress(progress(&step)))),
  (Err(primary),Err(refusal))=>Err(OriginalRetirementFailure::from_owned_parts(None,Some(refusal),None,None,Some(Box::new(primary)),None)),
 };
 RetainedWorkOutcome{result,work}
}
/// 🧾️ Settles one mutation advance against its effect.
pub fn settle_advance(effect:&mut RetainedAllocationEffect<'_>,base:u128,result:Result<RetainedMutationAdvance,OriginalRetirementFailure>,work:u128)->RetainedWorkOutcome<RetainedMutationAdvance,OriginalRetirementFailure>{settle_value(effect,base,result,work,RetainedMutationAdvance::progress)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(size_of::<u8>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"semantic parent copy overflow"))?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"semantic parent depth overflow"))?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<u8>(),maximum_depth:g.maximum_depth-1,..g}}
fn receipt(g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{let mut p=p;p.copied_bytes=p.copied_bytes.checked_add(size_of::<u8>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"semantic parent receipt overflow"))?;admit_retained_clone_progress(g,p,"retained immutable semantics")}
impl<P,M:RetainedMutation<P>> RetainedMutationOwner<P,M>{
 /// 📏️ Each read-only decision turn or original result movement has its own quote.
 pub fn decision_demand(&self,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>2{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic decision already settled"));}
  if self.mutation_source.is_none(){return Ok(scalar(mutation.binding_copy_bytes()));}
  if self.original_source.is_none(){return Ok(scalar(original.binding_copy_bytes()));}
  match self.phase{0=>nested(self.preparation.as_ref().ok_or_else(||ValueError::invariant("original preparation is absent"))?.decision_demand(mutation,original,body,maximum_work_units)?),1=>Ok(scalar(size_of::<RetainedMutationDecision<M::Diff>>()+size_of::<Option<M::Diff>>()+size_of::<MutationMessageLedgerRetirement>()+size_of::<u8>())),2=>Ok(scalar(if self.message_index<self.messages.as_ref().ok_or_else(||ValueError::invariant("original diagnostic ledger is absent"))?.retained_messages().len(){size_of::<semio_framework_diagnostic::Severity>()+size_of::<usize>()+size_of::<bool>()}else{size_of::<M::Diff>()+size_of::<Vec<MutationMessage>>()+size_of::<u8>()+size_of::<bool>()})),_=>unreachable!()}
 }
 /// ⛽️ Quotes the actual bound source, child or decision ledger step.
 pub fn decision_work_demand(&self,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.decision_demand(mutation,original,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.mutation_source.is_none()||self.original_source.is_none(){return Ok(1)}if self.phase==0{self.preparation.as_ref().ok_or_else(||ValueError::invariant("original preparation is absent"))?.decision_work_demand(mutation,original,body,maximum_work_units)}else{Ok(1)}}
 /// 🔏️ Only an actual admitted central diff preflight permits original inverse preparation.
 pub fn inverse_demand(&self,mutation:RetainedCloneRef<'_,M>,admission:&RetainedDiffAdmission<'_,P,M::Diff>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||!self.decision_admitted||self.phase<3||self.phase>4{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"inverse lacks admitted original decision"));}
  if !mutation.check_binding(&self.mutation_source)?||!admission.original().check_binding(&self.original_source)?{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"inverse lost original source custody"));}
  if self.diff_source.is_none(){return Ok(scalar(admission.diff().binding_copy_bytes()));}
  if self.phase==4{return Ok(scalar(size_of::<Vec<M>>()*2+size_of::<u8>()));}
  nested(self.preparation.as_ref().ok_or_else(||ValueError::invariant("original preparation is absent"))?.inverse_demand(mutation,admission.original(),admission.diff(),body,maximum_work_units)?)
 }
 /// ⛽️ Quotes the exact inverse binding, child or result handoff.
 pub fn inverse_work_demand(&self,mutation:RetainedCloneRef<'_,M>,admission:&RetainedDiffAdmission<'_,P,M::Diff>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.inverse_demand(mutation,admission,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.diff_source.is_none()||self.phase==4{return Ok(1)}self.preparation.as_ref().ok_or_else(||ValueError::invariant("original inverse preparation is absent"))?.inverse_work_demand(mutation,admission.original(),admission.diff(),body,maximum_work_units)}
 /// 🛑️ Typed child closure precedes source and fixed body release.
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_started(&self)->bool{self.closing}
 pub fn begin_close_demand(&self)->RetirementDemand{if self.closing{RetirementDemand::default()}else{RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}}}
 pub fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 pub fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{let d=self.begin_close_demand();if self.closing||effect.maximum_work_units()==0||!retained_mutation_demand_fits(d,effect.grant()){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let work=match effect.performed(1){Ok(true)=>1,Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retained mutation close start exceeded its original action")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:1}};self.closing=true;RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work}}
 /// ♻️ Untransferred D uses its explicitly registered intrinsic issuer; inverse M uses exact caller context.
 pub fn close_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.inverse_issuer.is_some_and(|issuer|!issuer.matches(&**context.mutations)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse issuer changed"));}
  let capture=self.issuer_capture_bytes();if capture>0{return Ok(RetirementDemand{copy_bytes:capture,depth:1,..Default::default()})}self.owner_close_demand(context,body)
 }
 fn issuer_capture_bytes(&self)->usize{if self.inverse_issuer.is_none()&&!self.terminal_is_empty(){FactoryIssuerIdentity::<M>::capture_copy_bytes()}else{0}}
 fn owner_close_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{
  let Some(p)=self.preparation.as_ref()else{return Ok(Default::default());};
  if !p.terminal_is_empty(){return if p.close_started(){p.close_demand(context,body)}else{Ok(p.begin_close_demand())};}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<M::Diff>(false,&None,body);}
  if self.decision.is_some(){return Ok(self.retirement.next_owner_demands::<M::Diff>(true,&None,body)?);}
  if let Some(m)=self.messages.as_ref(){return if m.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.messages),depth:1,..Default::default()})}else{semio_framework_value::ErasedSnapshotRetirement::next_demand(m,body)};}
  if self.diff_source.is_some(){return binding_demand(&self.diff_source,body);}
  if self.original_source.is_some(){return binding_demand(&self.original_source,body);}
  if self.mutation_source.is_some(){return binding_demand(&self.mutation_source,body);}
  Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<M::Preparation>>>(),release_bytes:size_of::<M::Preparation>(),depth:1,..Default::default()})
 }
 /// ⛽️ Quotes the selected original child closure and issuer capture.
 pub fn close_work_demand(&self,context:RetainedMutationCloseContext<'_,M>,body:usize)->Result<u64,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic closure not started"));}
  self.close_demand(context,body)?;let Some(p)=self.preparation.as_ref()else{return Ok(0)};if self.issuer_capture_bytes()>0{return Ok(1)}
  let child=if !p.terminal_is_empty(){if p.close_started(){p.close_work_demand(context,body)?}else{p.begin_close_work_demand()}}
  else if !self.retirement.is_empty(){self.retirement.next_owner_work::<M::Diff>(false,&None,body)?}
  else if self.decision.is_some(){self.retirement.next_owner_work::<M::Diff>(true,&None,body)?}
  else if self.messages.is_some(){1}
  else if self.diff_source.is_some(){RetainedCloneBinding::close_work_demand(&self.diff_source,body)?}
  else if self.original_source.is_some(){RetainedCloneBinding::close_work_demand(&self.original_source,body)?}
  else if self.mutation_source.is_some(){RetainedCloneBinding::close_work_demand(&self.mutation_source,body)?}
  else{1};Ok(child)
 }
 /// 🪜️ Original inverse provider identity is latched before any child closure.
 pub fn close_step(&mut self,context:RetainedMutationCloseContext<'_,M>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic closure not started").into());}
  let d=self.close_demand(context,grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let capture=self.issuer_capture_bytes();if capture>0{work=1;self.inverse_issuer=Some(FactoryIssuerIdentity::capture(&**context.mutations));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:capture,..Default::default()}))}
  self.owner_close_step(context,effect,&mut work)
 })();settle_exit(effect,base,result,work)}
 fn owner_close_step(&mut self,context:RetainedMutationCloseContext<'_,M>,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
  let grant=effect.grant();
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic closure not started").into());}
  if self.inverse_issuer.is_some_and(|i|!i.matches(&**context.mutations)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse issuer changed").into());}
  let d=self.owner_close_demand(context,grant.maximum_copy_bytes)?;if self.preparation.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let p=self.preparation.as_mut().ok_or_else(||ValueError::invariant("original preparation is absent"))?;if !p.terminal_is_empty()&&!p.close_started(){let turn=p.begin_close_paid(effect);*work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?))}if !p.terminal_is_empty(){let turn=RetirementPollExit::settled(p.close_step(context,effect))?;*work+=turn.work;let step=admit_retained_clone_close(grant,turn.result?,p.terminal_is_empty(),"original semantic child closure")?;return Ok(RetainedCloneStep::Progress(step.progress()));}
  if !self.retirement.is_empty(){let turn=RetirementPollExit::settled(self.retirement.step_granted(effect))?;*work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  let turn=self.retirement.begin_granted(&mut self.decision,effect);*work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.messages.as_ref().is_some_and(MutationMessageLedgerRetirement::terminal_is_empty){self.messages.take();*work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
  if let Some(m)=self.messages.as_mut(){let turn=m.close_step(grant);*work+=u128::from(m.last_turn_work());let s=turn?;if m.terminal_is_empty(){*self.messages=None;}return Ok(RetainedCloneStep::Progress(s.progress()));}
  if self.diff_source.is_some(){let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(&mut self.diff_source,effect))?;*work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  if self.original_source.is_some(){let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(&mut self.original_source,effect))?;*work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  if self.mutation_source.is_some(){let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(&mut self.mutation_source,effect))?;*work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  *work+=1;drop(self.preparation.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))
 }
 /// 🫙️ All fixed and original ownership has terminal custody.
 pub fn terminal_is_empty(&self)->bool{self.preparation.is_none()&&self.mutation_source.is_none()&&self.original_source.is_none()&&self.diff_source.is_none()&&self.decision.is_none()&&self.messages.is_none()&&self.retirement.is_empty()}
}
/// 🛡️ Original decision diagnostics are scanned before D can leave the central semantic owner.
pub fn decide_retained_mutation<P,M:RetainedMutation<P>>(owner:&mut RetainedMutationOwner<P,M>,mutation:RetainedCloneRef<'_,M>,original:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedMutationDecisionStep<M::Diff>,OriginalRetirementFailure>{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedMutationDecisionStep<M::Diff>,OriginalRetirementFailure>=(||{
 let selected=owner.decision_work_demand(mutation,original,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedMutationDecisionStep::Progress(Default::default()))}let d=owner.decision_demand(mutation,original,body,maximum_work_units)?;if !retained_mutation_demand_fits(d,grant){return Ok(RetainedMutationDecisionStep::Progress(Default::default()));}
 let turn=mutation.bind(&mut owner.mutation_source,effect);work+=turn.work;if let Some(p)=turn.result?{return Ok(RetainedMutationDecisionStep::Progress(p));}
 let turn=original.bind(&mut owner.original_source,effect);work+=turn.work;if let Some(p)=turn.result?{return Ok(RetainedMutationDecisionStep::Progress(p));}
 let invariant=|message:&'static str|ValueError::invariant(message);
 if owner.phase==0{let turn=owner.preparation.as_mut().ok_or_else(||invariant("original preparation is absent"))?.decide(mutation,original,body,maximum_work_units,&mut effect.nested(child(grant)));work+=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.saturating_add(size_of::<u8>());error.with_retained_progress(p)})?;let p=receipt(grant,step.progress())?;if matches!(step,RetainedMutationAdvance::Ready(_)){owner.phase=1;}return Ok(RetainedMutationDecisionStep::Progress(p));}
 work+=1;
 let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
 if owner.phase==1{let messages=match owner.preparation.as_mut().ok_or_else(||invariant("original preparation is absent"))?.take_decision(){RetainedMutationDecision::Admitted(diff,messages)=>{*owner.decision=Some(diff);messages},RetainedMutationDecision::Refused(messages)=>{owner.refusal=true;messages}};*owner.messages=Some(MutationMessageLedgerRetirement::new(String::new(),messages));owner.phase=2;return Ok(RetainedMutationDecisionStep::Progress(p));}
 let ledger=owner.messages.as_mut().ok_or_else(||invariant("original decision diagnostic ledger is absent"))?;if let Some(m)=ledger.retained_messages().get(owner.message_index){owner.refusal|=matches!(m.level,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal);owner.message_index+=1;return Ok(RetainedMutationDecisionStep::Progress(p));}
 let g=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<M::Diff>()-size_of::<u8>()-size_of::<bool>(),..grant};let(messages,_)=ledger.take_messages_controlled(g)?.ok_or_else(||invariant("funded original decision diagnostic transfer was denied"))?;owner.phase=3;owner.decision_admitted=!owner.refusal;
 Ok(if owner.refusal{RetainedMutationDecisionStep::Refused(p,messages)}else{RetainedMutationDecisionStep::Admitted(p,owner.decision.take().ok_or_else(||invariant("original admitted typed diff is absent"))?,messages)})
})();settle_value(effect,base,result,work,RetainedMutationDecisionStep::progress)}
/// 📖️ Exact preflight witness gates read-only inversion of the original semantic operation.
pub fn inverse_retained_mutation<P,M:RetainedMutation<P>>(owner:&mut RetainedMutationOwner<P,M>,mutation:RetainedCloneRef<'_,M>,admission:&RetainedDiffAdmission<'_,P,M::Diff>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedMutationInverseStep<M>,OriginalRetirementFailure>{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedMutationInverseStep<M>,OriginalRetirementFailure>=(||{
 let selected=owner.inverse_work_demand(mutation,admission,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedMutationInverseStep::Progress(Default::default()))}let d=owner.inverse_demand(mutation,admission,body,maximum_work_units)?;if !retained_mutation_demand_fits(d,grant){return Ok(RetainedMutationInverseStep::Progress(Default::default()));}
 let turn=admission.diff().bind(&mut owner.diff_source,effect);work+=turn.work;if let Some(p)=turn.result?{return Ok(RetainedMutationInverseStep::Progress(p));}
 admission.original().check_binding(&owner.original_source)?;mutation.check_binding(&owner.mutation_source)?;
 if owner.phase==4{work+=1;owner.phase=5;let rows=owner.preparation.as_mut().ok_or_else(||ValueError::invariant("original inverse preparation is absent"))?.take_inverse();return Ok(RetainedMutationInverseStep::Ready(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},rows));}
 let turn=owner.preparation.as_mut().ok_or_else(||ValueError::invariant("original inverse preparation is absent"))?.inverse(mutation,admission.original(),admission.diff(),body,maximum_work_units,&mut effect.nested(child(grant)));work+=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.saturating_add(size_of::<u8>());error.with_retained_progress(p)})?;let p=receipt(grant,step.progress())?;if matches!(step,RetainedMutationAdvance::Ready(_)){owner.phase=4;}Ok(RetainedMutationInverseStep::Progress(p))
})();settle_value(effect,base,result,work,RetainedMutationInverseStep::progress)}
fn binding_demand(binding:&Option<RetainedCloneBinding>,body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:RetainedCloneBinding::copy_demand(binding)?,capacity_bytes:RetainedCloneBinding::capacity_demand(binding,body)?,release_bytes:RetainedCloneBinding::release_demand(binding)?,depth:RetainedCloneBinding::depth_demand(binding)?})}
impl<P,M:RetainedMutation<P>> Drop for RetainedMutationOwner<P,M>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"retained semantic owner requires original funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.preparation);ManuallyDrop::drop(&mut self.decision);ManuallyDrop::drop(&mut self.messages);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"] mod tests;
