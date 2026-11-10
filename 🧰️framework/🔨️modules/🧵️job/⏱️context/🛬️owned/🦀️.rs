//! 🛬️ Source-owned decoder admission and loan return.
use super::{OriginalCallerSourceLoan as SourceLoan,OriginalTurnFailure as Failure,OriginalNativeReceivingOutcome as Outcome,StepContext};
use semio_framework_job_return_budget as stock;
use crate::original_job_continuation::source::OriginalReturnSourceInput as Input;
use semio_framework_job_receiving::{OriginalDecodeState as State,OriginalReceivingOperation as Operation};
use semio_framework_value_retained_clone::{ValueError,RetirementDemand,RetainedCloneGrant as G,RetainedCloneProgress as P};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeProgress,NativeRetirementDecodeControl as Recovery};
use semio_framework_job_receiving::OriginalJobWorkRecipient;
use semio_framework_value_call::NativeCallFailure;
type ScopeLoan=stock::OriginalReturnScopeLoan<Input>;
/// 🏦️ A constructed output that carries the scope credit its consumed payload held.
pub type OriginalCreditedOutput<O>=stock::CreditedOutput<Input,O>;
use stock::OriginalReturnEscrowAmounts as Amounts;
pub struct OriginalJobDecodeOwner{source:std::mem::ManuallyDrop<Option<SourceLoan>>,pub(crate) scope:Option<ScopeLoan>,state:State}
impl OriginalJobDecodeOwner{
 pub fn empty()->Self{Self{source:std::mem::ManuallyDrop::new(None),scope:None,state:State::empty()}}
 pub fn work_issuer_birth_copy_bytes()->usize{State::work_issuer_birth_copy_bytes()}
 pub fn source_loan_admitted(&self)->bool{self.source.is_some()}
 pub(crate) fn admission_complete(&self)->bool{self.source.is_some()&&self.state.work_issuer_admitted()}
 pub fn admit_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,Failure>{if self.state.is_closing(){return Err(ValueError::invariant("decoder closing").into())}if self.source.is_none(){cx.original_source_parent()?.admit_loan(&mut self.source,cx)?;return Ok(false)}self.source.as_ref().unwrap().validate_context(cx)?;if self.state.work_issuer_admitted(){return Ok(true)}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let turn=self.state.admit_work_issuer(cx.retained_grant(),cx.fuel_remaining());let ready=turn.result?;if ready{let progress=P{copied_items:1,copied_bytes:Self::work_issuer_birth_copy_bytes(),..Default::default()};let receipt=cx.consume_retained(progress);cx.consume_work(turn.work);receipt?;}Ok(ready)}
 pub(crate) fn validate_source(&self,cx:&StepContext<'_>,retirement:bool)->Result<(),Failure>{let source=self.source.as_ref().ok_or_else(||ValueError::invariant("decoder Source absent"))?;if retirement{source.validate_retirement_context(cx)}else{source.validate_context(cx)}?;if !self.state.work_issuer_admitted(){return Err(ValueError::invariant("work issuer absent").into())}Ok(())}
 pub(crate) fn retire<T,E:From<ValueError>>(&mut self,grant:G,callback:&mut dyn FnMut(NativeDecodeProgress)->bool,deadline:&mut dyn FnMut()->bool,work:&mut OriginalJobWorkRecipient<'_>,observed:&mut u64,operation:impl FnOnce(&mut Recovery<'_, '_>)->Result<T,E>)->Result<T,NativeCallFailure<T,E>>{self.state.retire(grant,callback,deadline,work,observed,operation)}
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.state.terminal_is_empty(){return self.state.retirement_demands(body)}Ok(RetirementDemand{copy_bytes:if self.source.is_some(){size_of::<Option<SourceLoan>>()}else{0},depth:usize::from(self.source.is_some()),..Default::default()})}
 /// 🧹️ Returns decoder state before its Source alias.
 pub fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,Failure>{if self.terminal_is_empty(){return Ok(true)}if self.scope.is_some(){let mut held=self.scope.take();let released=cx.release_original_scope(&mut held);self.scope=held;released?;return Ok(false)}if let Some(source)=self.source.as_ref().filter(|source|!source.terminal_is_empty()){source.validate_retirement_context(cx)?;}if !self.state.terminal_is_empty(){let demand=self.state.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;super::close_original_effect_step(cx,demand,|effect|self.state.close_step(effect))?;return Ok(false)}let Some(loan)=self.source.as_mut()else{return Ok(true)};let bytes=size_of::<Option<SourceLoan>>();let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}let parent=cx.original_source_parent()?;if !matches!(loan.close_step(parent,cx)?,super::InteractiveJobCloseStep::Complete{..}){return Ok(false)}cx.consume_retained(P{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.source.take();Ok(true)}
 pub fn terminal_is_empty(&self)->bool{self.state.terminal_is_empty()&&self.source.is_none()&&self.scope.is_none()}
}
impl std::ops::Deref for OriginalJobDecodeOwner{type Target=State;fn deref(&self)->&Self::Target{&self.state}}
impl Drop for OriginalJobDecodeOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"decoder needs Source retirement")}}

impl StepContext<'_>{

 /// 🏦️ Hands the decoder's retained scope credit to a carrier at the point its payload is consumed; `Ok(None)` when no retained credit is held.
 pub fn adopt_original_credit<O>(&mut self,output:O)->Result<std::result::Result<OriginalCreditedOutput<O>,O>,Failure>{
  let Self{original_decoder,return_budget,..}=self;let(Some(owner),Some(budget))=(original_decoder.as_deref_mut(),return_budget.as_mut())else{return Ok(Err(output))};
  let budget=budget.get_mut();let Some(loan)=owner.scope.as_mut()else{return Ok(Err(output))};if !budget.scope_requires_retirement(loan)?{return Ok(Err(output))}
  let credited=stock::CreditedOutput::adopt(output,budget,loan).map_err(|(error,_)|Failure::from(error))?;owner.scope.take();Ok(Ok(credited))
 }
 /// 🧾️ Settles an adopted credit once its output has retired.
 pub fn settle_original_credit<O>(&mut self,credited:&mut OriginalCreditedOutput<O>)->Result<bool,Failure>{let budget=self.return_budget.as_mut().ok_or_else(||ValueError::invariant("Source budget absent"))?.get_mut();Ok(credited.settle_credit(budget)?)}
 /// 💱️ Exact receiving quote of one cleanup effect: its item, its physical demand and its maximum work.
 pub fn native_retirement_quote(demand:RetirementDemand,work:u64)->(G,u64){(G{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth},work)}
 /// 🫴️ Stores, at exactly its quote, the full recovery outcome before Source posting.
 pub fn with_native_retirement_receiving(&mut self,out:&mut Option<Outcome>,quote:(G,u64),operation:impl FnOnce(&mut Recovery<'_, '_ >,G)->Result<(),ValueError>)->Result<bool,Failure>{let mut operation=Some(operation);self.original_receiving_outcome(out,Some(quote),Operation::Recover(&mut |native,grant|operation.take().unwrap()(native,grant)))}
 /// 🧳️ Retains typed output through original Source return.
 pub fn with_native_retirement_receiving_value<T>(&mut self,out:&mut Option<Outcome<T>>,quote:(G,u64),operation:impl FnOnce(&mut Recovery<'_, '_>,G)->Result<T,ValueError>)->Result<bool,Failure>{let mut operation=Some(operation);self.original_receiving_outcome(out,Some(quote),Operation::RecoverValue(&mut|native,grant|operation.take().unwrap()(native,grant)))}
 fn native_scope_quantum(&self)->G{stock::original_return_scope_quantum(self.retained_progress(),self.reserved_return_progress,self.retained,self.retained_policy)}
 fn observe_native_scope(&mut self)->Result<(),ValueError>{let Some(loan)=self.original_decoder.as_deref_mut().and_then(|owner|owner.scope.take())else{return Ok(())};let mut held=Some(loan);let result=self.observe_scope_loan(&mut held);if let Some(owner)=self.original_decoder.as_deref_mut(){owner.scope=held}result}
 fn observe_scope_loan(&mut self,held:&mut Option<ScopeLoan>)->Result<(),ValueError>{let grant=self.native_scope_quantum();let Self{return_budget,retained_progress,work_units_spent,fuel_remaining,reserved_return_progress,reserved_return_work,original_physical_recipient,original_work_recipient,reserved_observer,..}=self;let Some(loan)=held.as_mut()else{return Ok(())};let b=return_budget.as_mut().ok_or_else(||ValueError::invariant("Source scope budget absent"))?.get_mut();let counters=stock::OriginalReturnScopeCounters{physical:&mut**retained_progress,work:work_units_spent,fuel:fuel_remaining,reserved:reserved_return_progress,reserved_work:reserved_return_work,aggregate_physical:original_physical_recipient.as_ref().map(|(p,start)|(&**p,*start)),aggregate_work:original_work_recipient.as_deref(),reserved_observer:reserved_observer.as_mut().map(|(p,w)|(&mut**p,&mut**w)),grant};if b.observe_scope(loan,counters)?{held.take();}Ok(())}
 /// 🏁️ Settles the owner's one persistent escrow row when the owner retires; `true` once it is returned.
 fn release_original_scope(&mut self,held:&mut Option<ScopeLoan>)->Result<bool,Failure>{
  let Some(loan)=held.as_ref()else{return Ok(true)};let budget=self.return_budget.as_mut().ok_or_else(||ValueError::invariant("Source scope budget absent"))?.get_mut();
  if budget.scope_requires_retirement(loan)?{return Err(ValueError::invariant("decoder retains receiving stock").into())}
  if budget.scope_observation(loan)?.is_none(){budget.enter_scope(loan)?;budget.finish_scope(loan,Amounts::default(),true,Default::default(),Default::default(),0,false)?}
  self.observe_scope_loan(held)?;Ok(held.is_none())
 }
 /// ♻️ Owners retire one row each, so many owners on one Source leave tombstones; recycles one acknowledged tombstone before a new owner's claim refuses. `true` while a reclaimable slot remains to retry against.
 fn reclaim_original_escrow_slot(&mut self)->Result<bool,Failure>{
  let copy=semio_framework_job_payload::ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES;let grant=self.retained_grant();let funded=self.fuel_remaining()>0&&grant.maximum_items>0&&grant.maximum_depth>0&&grant.maximum_copy_bytes>=copy;let Some(table)=self.return_budget.as_mut().and_then(|b|b.get_mut().table_mut())else{return Ok(false)};
  if !table.has_reclaimable(){return Ok(false)}if !funded||!table.reclaim_one()?{return Ok(true)}
  let posted=self.consume_retained(P{copied_items:1,copied_bytes:copy,..Default::default()});self.consume_fuel(1);posted?;Ok(true)
 }
 pub(super) fn original_receiving_outcome<T>(&mut self,out:&mut Option<Outcome<T>>,quote:Option<(G,u64)>,operation:Operation<'_,T>)->Result<bool,Failure>{
  let retirement=operation.is_retirement();let typed=operation.is_typed();let placement=if typed{Outcome::<T>::typed_placement()?}else{Outcome::<T>::placement()};let placement_work=if typed{Outcome::<T>::typed_placement_work()?}else{0};if out.is_some(){return Err(ValueError::invariant("receiving destination occupied").into())}self.original_decoder.as_deref().ok_or_else(||ValueError::invariant("receiving decoder absent"))?;if !retirement&&!self.original_decoder.as_deref().is_some_and(|owner|owner.admission_complete()){let owner=self.original_decoder.take().ok_or_else(||ValueError::invariant("receiving decoder absent"))?;let admitted=owner.admit_step(self);self.original_decoder=Some(owner);admitted?;return Ok(false)}self.original_decoder.as_deref().unwrap().validate_source(self,retirement)?;
  if self.return_budget.as_ref().is_none_or(|b|b.get().table().is_none()){self.admit_original_return_table()?;return Ok(false)}
  self.observe_native_scope()?;if !retirement&&self.original_decoder.as_deref().unwrap().state.has_pending_receiving(){return Err(ValueError::invariant("pending decoder requires original recovery").into())}
  if (!retirement&&(self.is_cancelled()||self.deadline_exceeded()))||(retirement&&!self.original_retirement&&self.deadline_exceeded()){return Ok(false)}
  let local=self.return_budget.as_ref().unwrap().get().delivery==stock::OriginalReturnDelivery::LocalWitness;let owner=self.original_decoder.as_deref().unwrap();let existing=match owner.scope.as_ref(){Some(loan)=>self.return_budget.as_ref().unwrap().get().scope_remaining(loan)?,None=>Amounts::default()};let whole=self.native_scope_quantum();let(quantum,quantum_work)=match quote{Some((grant,work))=>(G{maximum_depth:whole.maximum_depth,..grant},work),None=>(G{maximum_items:whole.maximum_items.div_ceil(2),maximum_copy_bytes:whole.maximum_copy_bytes.div_ceil(2),maximum_capacity_bytes:whole.maximum_capacity_bytes.div_ceil(2),maximum_release_bytes:whole.maximum_release_bytes.div_ceil(2),maximum_depth:whole.maximum_depth},self.fuel_remaining.div_ceil(2))};let Some(plan)=stock::OriginalReturnScopePlan::quote::<Input>(self.retained_grant(),quantum,self.fuel_remaining(),quantum_work,existing,owner.scope.is_some(),owner.state.pending_work(),owner.state.has_pending_receiving(),retirement,local,placement,placement_work)?else{return Ok(false)};
  let epoch=self.original_source_epoch().unwrap_or(0);let admitted={let Self{original_decoder,return_budget,..}=self;return_budget.as_mut().unwrap().get_mut().admit_scope(&mut original_decoder.as_deref_mut().unwrap().scope,plan.wanted,epoch)?};let Some((progress,work))=admitted else{if self.reclaim_original_escrow_slot()?{return Ok(false)}return Err(ValueError::literal(semio_framework_value_retained_clone::ValueRefusalKind::OwnershipLimit,"original return escrow table has no free slot for a new receiving scope").into())};let posted=self.consume_retained(progress);self.consume_fuel(work);posted?;if !plan.ready{return Ok(false)}
  let before=self.retained_progress();let result={let Self{original_decoder,return_budget,retained_progress,receiving,deadline_us,now_us,original_retirement,cancel,source_turn,..}=self;let source=source_turn.as_ref().map(|turn|turn.token().witness());let mut ready=||if retirement{*original_retirement||now_us().is_some_and(|now|now<*deadline_us)}else{!cancel.is_cancelled_now()&&now_us().is_some_and(|now|now<*deadline_us)};let owner=original_decoder.as_deref_mut().unwrap();owner.state.run_scoped_on_caller(return_budget.as_mut().unwrap().get_mut(),owner.scope.as_ref().unwrap(),out,&plan,before,&mut**retained_progress,receiving.spent_mut(),&mut ready,source,operation)};if out.is_none(){result?;return Ok(false)}let kept=out.as_mut().unwrap().source_refusal.take();let refusal=match result{Err(error)=>Some(error),Ok(())=>self.observe_native_scope().err()};out.as_mut().unwrap().source_refusal=refusal.or(kept);Ok(true)
 }
}
