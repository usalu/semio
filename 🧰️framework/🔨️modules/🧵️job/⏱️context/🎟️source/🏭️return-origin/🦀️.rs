//! 🧵️ The original StepContext lends its already-held return-budget guard to the real origin producer.
use super::*;
use semio_framework_job_context_step::original_return_source::OriginalReturnSourceInput;
use semio_framework_job_return_budget::{OriginalReturnOriginProducer,OriginalReturnOriginContext,OriginalReturnOriginCommitProducer,OriginalReturnOriginCommitContext,OriginalReturnBirthStage,OriginalReturnBirthPhase};
use semio_framework_value_control_contract::{OriginalEffectAction,OriginalEffectAuthority,RetainedAllocationEffect,RetainedCloneStep};
use semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding;
use semio_framework_value_native_outcome::{OriginalReceivingPrimary,OriginalConstructionAcceptedProof as Accepted};
/// 🌱️ The Source return birth surface of the step context.
pub trait StepContextReturnOrigin{
 /// 📏️ Quotes the pending original Source's actual paid commit producer.
 fn return_origin_commit_demand(&self)->Result<RetirementDemand,ValueError>;
 /// 🧾️ Quotes the commit action's exact charge to its Source turn, without the transient reply extents its admission also needs.
 fn return_origin_commit_cost(&self)->Result<RetirementDemand,ValueError>;
 /// 📨️ Runs the original commit leaf with its real guard and already authenticated Post.
 fn drive_return_origin_commit_producer(&mut self,producer:&mut OriginalEffectAuthority<OriginalReturnOriginCommitProducer<'_,OriginalReturnSourceInput>>,action:&mut Option<OriginalEffectAction<RetainedCloneStep,OriginalReceivingPrimary>>,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>;
 /// 🔐️ Validates the actual held Source before the origin action is quoted.
 fn return_origin_producer_matches(&self,producer:&OriginalReturnOriginProducer<'_,OriginalReturnSourceInput>)->Result<bool,ValueError>;
 /// 🔐️ Validates the actual held Source before the commit action is quoted.
 fn return_origin_commit_producer_matches(&self,producer:&OriginalReturnOriginCommitProducer<'_,OriginalReturnSourceInput>)->Result<bool,ValueError>;
 /// 📨️ Runs only the retained first-party origin leaf with the same already-held Source guard.
 fn drive_return_origin_producer(&mut self,producer:&mut OriginalEffectAuthority<OriginalReturnOriginProducer<'_,OriginalReturnSourceInput>>,action:&mut Option<OriginalEffectAction<RetainedCloneStep,OriginalReceivingPrimary>>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>;
 /// 🌱️ Reports whether a caller asked this Source for its return table or publisher.
 fn return_birth_requested(&self)->bool;
 /// ⛽️ Quotes the exact Work of one whole stage action from the actual held budget.
 fn return_birth_work(&self,stage:OriginalReturnBirthStage)->Result<u64,ValueError>;
 /// 📏️ Quotes the next birth action of the selected stage from the actual held budget.
 /// 🧮️ Quotes the whole stage's items and extents from the actual held budget.
 fn return_birth_total(&self,stage:OriginalReturnBirthStage)->Result<(usize,RetirementDemand),ValueError>;

 fn return_birth_demand(&self,stage:OriginalReturnBirthStage)->Result<RetirementDemand,ValueError>;
 /// 🌗️ Observes how far the hidden origin, table and publisher progressed for the live Source turn.
 fn return_birth_state(&self)->Result<OriginalReturnBirthState,ValueError>;
 /// 🌱️ Stages or refreshes the hidden origin inside one paid construction action.
 fn birth_origin_action(&mut self,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>;
 /// 🔒️ Commits the accepted hidden origin inside one paid construction action.
 fn birth_origin_commit_action(&mut self,accepted:&Accepted<'_>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>;
 /// 🌱️ Runs one table or publisher birth stage inside one paid construction action.
 #[allow(clippy::too_many_arguments)]
 fn birth_stage_action(&mut self,stage:OriginalReturnBirthStage,origin:&Accepted<'_>,table:Option<&Accepted<'_>>,birth:Option<&Accepted<'_>>,publisher:Option<&Accepted<'_>>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>;
 /// ↩️ Releases one partially filled table or publisher backing through the paid Source effect.
 fn cancel_return_birth(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>;
}
impl StepContextReturnOrigin for StepContext<'_>{
 /// 📏️ Quotes the pending original Source's actual paid commit producer.
 fn return_origin_commit_demand(&self)->Result<RetirementDemand,ValueError>{let budget=self.return_budget_ref().ok_or_else(||refused("original return budget loan is absent"))?;OriginalReturnOriginCommitProducer::<OriginalReturnSourceInput>::demand(budget.get())}
 /// 🧾️ Quotes the commit action's exact charge to its Source turn, without the transient reply extents its admission also needs.
 fn return_origin_commit_cost(&self)->Result<RetirementDemand,ValueError>{let budget=self.return_budget_ref().ok_or_else(||refused("original return budget loan is absent"))?;Ok(budget.get().origin_commit_demand())}
 /// 📨️ Runs the original commit leaf with its real guard and already authenticated Post.
 fn drive_return_origin_commit_producer(&mut self,producer:&mut OriginalEffectAuthority<OriginalReturnOriginCommitProducer<'_,OriginalReturnSourceInput>>,action:&mut Option<OriginalEffectAction<RetainedCloneStep,OriginalReceivingPrimary>>,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let parent=self.source_parent_ref().ok_or_else(||refused("original return producer Source parent is absent"))?;parent.validate_borrowed_context(self)?;if self.original_source_turn_identity()!=binding.source()||!binding.matches_effect(effect){return Err(refused("original return commit changed its live Source"))}
  let budget=self.return_budget_mut().ok_or_else(||refused("original return budget loan is absent"))?;let mut context=OriginalReturnOriginCommitContext{binding,budget,accepted};producer.drive(&mut context,effect,action)
 }
 /// 🔐️ Validates the actual held Source before the origin action is quoted.
 fn return_origin_producer_matches(&self,producer:&OriginalReturnOriginProducer<'_,OriginalReturnSourceInput>)->Result<bool,ValueError>{let parent=self.source_parent_ref().ok_or_else(||refused("original return Source parent is absent"))?;parent.validate_borrowed_context(self)?;let budget=self.return_budget_ref().ok_or_else(||refused("original return budget loan is absent"))?;Ok(producer.authenticates(budget))}
 /// 🔐️ Validates the actual held Source before the commit action is quoted.
 fn return_origin_commit_producer_matches(&self,producer:&OriginalReturnOriginCommitProducer<'_,OriginalReturnSourceInput>)->Result<bool,ValueError>{let parent=self.source_parent_ref().ok_or_else(||refused("original return Source parent is absent"))?;parent.validate_borrowed_context(self)?;let budget=self.return_budget_ref().ok_or_else(||refused("original return budget loan is absent"))?;Ok(producer.authenticates(budget))}
 /// 📨️ Runs only the retained first-party origin leaf with the same already-held Source guard.
 fn drive_return_origin_producer(&mut self,producer:&mut OriginalEffectAuthority<OriginalReturnOriginProducer<'_,OriginalReturnSourceInput>>,action:&mut Option<OriginalEffectAction<RetainedCloneStep,OriginalReceivingPrimary>>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let parent=self.source_parent_ref().ok_or_else(||refused("original return producer Source parent is absent"))?;parent.validate_borrowed_context(self)?;if self.original_source_turn_identity()!=binding.source()||!binding.matches_effect(effect){return Err(refused("original return producer changed its live Source"))}
  let budget=self.return_budget_mut().ok_or_else(||refused("original return budget loan is absent"))?;let mut context=OriginalReturnOriginContext{binding,budget};producer.drive(&mut context,effect,action)
 }
 /// 🌱️ Reports whether a caller asked this Source for its return table or publisher.
 fn return_birth_requested(&self)->bool{self.return_budget_ref().is_some_and(|budget|budget.get().birth_requested)}
 /// ⛽️ Quotes the exact Work of one whole stage action from the actual held budget.
 fn return_birth_work(&self,stage:OriginalReturnBirthStage)->Result<u64,ValueError>{Ok(self.return_budget_ref().ok_or_else(||refused("original return budget loan is absent"))?.get().birth_work(stage))}
 /// 📏️ Quotes the next birth action of the selected stage from the actual held budget.
 /// 🧮️ Quotes the whole stage's items and extents from the actual held budget.
 fn return_birth_total(&self,stage:OriginalReturnBirthStage)->Result<(usize,RetirementDemand),ValueError>{let budget=self.return_budget_ref().ok_or_else(||refused("original return budget loan is absent"))?;budget.get().birth_total(stage)}

 fn return_birth_demand(&self,stage:OriginalReturnBirthStage)->Result<RetirementDemand,ValueError>{let budget=self.return_budget_ref().ok_or_else(||refused("original return budget loan is absent"))?;budget.get().birth_demand(stage)}
 /// 🌗️ Observes how far the hidden origin, table and publisher progressed for the live Source turn.
 fn return_birth_state(&self)->Result<OriginalReturnBirthState,ValueError>{
  let parent=self.source_parent_ref().ok_or_else(||refused("original return Source parent is absent"))?;parent.validate_borrowed_context(self)?;let budget=self.return_budget_ref().ok_or_else(||refused("original return budget loan is absent"))?.get();
  let turn=self.original_source_turn_identity();let origin=budget.origin_phase();let table=budget.table_phase();let publisher=budget.publisher_phase();
  Ok(OriginalReturnBirthState{origin,table,publisher,publisher_wanted:budget.publisher_requested,origin_current:origin==OriginalReturnBirthPhase::Live&&turn.is_some()&&budget.origin().is_some_and(|origin|semio_framework_job_return_budget::OriginalReturnOrigin::source_turn(&origin)==turn)})
 }
 /// 🌱️ Stages or refreshes the hidden origin inside one paid construction action.
 fn birth_origin_action(&mut self,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  if let Err(error)=birth_guard(self,binding,effect){return RetainedWorkOutcome{result:Err(error),work:0}}let input=match OriginalReturnSourceInput::capture(self){Ok(input)=>input,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  match self.return_budget_mut(){Some(budget)=>budget.get_mut().origin_action(input,binding,effect),None=>RetainedWorkOutcome{result:Err(refused("original return budget loan is absent")),work:0}}
 }
 /// 🔒️ Commits the accepted hidden origin inside one paid construction action.
 fn birth_origin_commit_action(&mut self,accepted:&Accepted<'_>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  if let Err(error)=birth_guard(self,binding,effect){return RetainedWorkOutcome{result:Err(error),work:0}}
  match self.return_budget_mut(){Some(budget)=>budget.get_mut().origin_commit_action(accepted,binding,effect),None=>RetainedWorkOutcome{result:Err(refused("original return budget loan is absent")),work:0}}
 }
 /// 🌱️ Runs one table or publisher birth stage inside one paid construction action.
 #[allow(clippy::too_many_arguments)]
 fn birth_stage_action(&mut self,stage:OriginalReturnBirthStage,origin:&Accepted<'_>,table:Option<&Accepted<'_>>,birth:Option<&Accepted<'_>>,publisher:Option<&Accepted<'_>>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  birth_guard(self,binding,effect)?;self.return_budget_mut().ok_or_else(||refused("original return budget loan is absent"))?.get_mut().birth_action(stage,origin,table,birth,publisher,binding,effect)
 }
 /// ↩️ Releases one partially filled table or publisher backing through the paid Source effect.
 fn cancel_return_birth(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  use semio_framework_value_control_contract::OriginalEffectReply;let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};let Some(budget)=self.return_budget_mut()else{return fail(refused("original return budget loan is absent"))};let budget=budget.get_mut();let mut slot=None;
  let reply=match OriginalEffectReply::borrow(&mut slot){Ok(reply)=>reply,Err(error)=>return fail(error)};
  if budget.table_phase()==OriginalReturnBirthPhase::Building{drop(budget.cancel_table_birth(effect,reply))}else if budget.publisher_phase()==OriginalReturnBirthPhase::Building{drop(budget.cancel_publisher_birth(effect,reply))}else{return RetainedWorkOutcome{result:Ok(None),work:0}}
  match slot.take(){Some(Ok(step))=>RetainedWorkOutcome{result:Ok(Some(step.progress())),work:1},Some(Err(error))=>fail(error),None=>RetainedWorkOutcome{result:Ok(None),work:0}}
 }
}

 fn birth_guard(cx:&StepContext<'_>,binding:&NativeReceivingConstructionBinding,effect:&RetainedAllocationEffect<'_>)->Result<(),ValueError>{let parent=cx.source_parent_ref().ok_or_else(||refused("original return birth Source parent is absent"))?;parent.validate_borrowed_context(cx)?;if cx.original_source_turn_identity()!=binding.source()||!binding.matches_effect(effect){return Err(refused("original return birth changed its live Source"))}Ok(())}
/// 🌗️ Reports the hidden origin, table and publisher phases for the live Source turn.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalReturnBirthState{pub origin:OriginalReturnBirthPhase,pub table:OriginalReturnBirthPhase,pub publisher:OriginalReturnBirthPhase,pub publisher_wanted:bool,pub origin_current:bool}
