//! 🧵️ The original StepContext lends its already-held return-budget guard to the real origin producer.
use super::*;
use semio_framework_job_return_budget::{OriginalReturnOriginProducer,OriginalReturnOriginContext,OriginalReturnOriginCommitProducer,OriginalReturnOriginCommitContext};
use semio_framework_value_control_contract::{OriginalEffectAction,OriginalEffectAuthority,RetainedAllocationEffect,RetainedCloneStep};
use semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding;
use semio_framework_value_native_outcome::OriginalReceivingPrimary;

impl OriginalCallerSourceParent{
 /// 🏠️ Borrows the Source-owned return budget without issuing a replacement owner.
 pub fn return_origin_source(&self)->Result<&semio_framework_job_return_budget::OriginalReturnSource<OriginalReturnSourceInput>,ValueError>{let roots=self.original.as_ref().ok_or_else(||refused("original return Source entered retirement"))?.get();Ok(&roots.returns)}
}
impl<'source> StepContext<'source>{
 /// 🪑️ Admits the actual Source producer into its resident output under the original effect.
 pub fn admit_return_origin_producer<'owner>(&mut self,source:&'owner semio_framework_job_return_budget::OriginalReturnSource<OriginalReturnSourceInput>,input:&mut Option<OriginalReturnSourceInput>,output:&mut Option<OriginalEffectAuthority<OriginalReturnOriginProducer<'owner,OriginalReturnSourceInput>>>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};let Some(parent)=self.source_parent else{return fail(refused("original return producer Source parent is absent"))};if let Err(error)=parent.validate_borrowed_context(self){return fail(error)}if self.original_source_turn_identity()!=binding.source()||!binding.matches_effect(effect){return fail(refused("original return producer changed its live Source"))}
  let Some(budget)=self.return_budget.as_ref()else{return fail(refused("original return budget loan is absent"))};OriginalReturnOriginProducer::admit(source,budget,input,output,binding,effect)
 }
 /// 👁️ Borrows the committed origin only through its exact accepted publication proof.
 pub fn accepted_return_origin(&self,proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>)->Result<Option<OriginalReturnSourceInput>,ValueError>{let parent=self.source_parent.ok_or_else(||refused("original return Source parent is absent"))?;parent.validate_borrowed_context(self)?;let budget=self.return_budget.as_ref().ok_or_else(||refused("original return budget loan is absent"))?;Ok(budget.get().origin_with_post(proof))}
 /// 📏️ Quotes the pending original Source's actual paid commit producer.
 pub fn return_origin_commit_demand(&self)->Result<RetirementDemand,ValueError>{let budget=self.return_budget.as_ref().ok_or_else(||refused("original return budget loan is absent"))?;OriginalReturnOriginCommitProducer::<OriginalReturnSourceInput>::demand(budget.get())}
 /// 🪑️ Admits the original commit producer only under its already authenticated accepted proof.
 pub fn admit_return_origin_commit_producer<'owner>(&mut self,source:&'owner semio_framework_job_return_budget::OriginalReturnSource<OriginalReturnSourceInput>,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,output:&mut Option<OriginalEffectAuthority<OriginalReturnOriginCommitProducer<'owner,OriginalReturnSourceInput>>>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};let Some(parent)=self.source_parent else{return fail(refused("original return producer Source parent is absent"))};if let Err(error)=parent.validate_borrowed_context(self){return fail(error)}if self.original_source_turn_identity()!=binding.source()||!binding.matches_effect(effect){return fail(refused("original return commit changed its live Source"))}
  let Some(budget)=self.return_budget.as_ref()else{return fail(refused("original return budget loan is absent"))};OriginalReturnOriginCommitProducer::admit(source,budget,accepted,output,binding,effect)
 }
 /// 📨️ Runs the original commit leaf with its real guard and already authenticated Post.
 pub fn drive_return_origin_commit_producer(&mut self,producer:&mut OriginalEffectAuthority<OriginalReturnOriginCommitProducer<'_,OriginalReturnSourceInput>>,action:&mut Option<OriginalEffectAction<RetainedCloneStep,OriginalReceivingPrimary>>,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let parent=self.source_parent.ok_or_else(||refused("original return producer Source parent is absent"))?;parent.validate_borrowed_context(self)?;if self.original_source_turn_identity()!=binding.source()||!binding.matches_effect(effect){return Err(refused("original return commit changed its live Source"))}
  let budget=self.return_budget.as_mut().ok_or_else(||refused("original return budget loan is absent"))?;let mut context=OriginalReturnOriginCommitContext{binding,budget,accepted};producer.drive(&mut context,effect,action)
 }
 /// 🔐️ Validates the actual held Source before the origin action is quoted.
 pub fn return_origin_producer_matches(&self,producer:&OriginalReturnOriginProducer<'_,OriginalReturnSourceInput>)->Result<bool,ValueError>{let parent=self.source_parent.ok_or_else(||refused("original return Source parent is absent"))?;parent.validate_borrowed_context(self)?;let budget=self.return_budget.as_ref().ok_or_else(||refused("original return budget loan is absent"))?;Ok(producer.authenticates(budget))}
 /// 🔐️ Validates the actual held Source before the commit action is quoted.
 pub fn return_origin_commit_producer_matches(&self,producer:&OriginalReturnOriginCommitProducer<'_,OriginalReturnSourceInput>)->Result<bool,ValueError>{let parent=self.source_parent.ok_or_else(||refused("original return Source parent is absent"))?;parent.validate_borrowed_context(self)?;let budget=self.return_budget.as_ref().ok_or_else(||refused("original return budget loan is absent"))?;Ok(producer.authenticates(budget))}
 /// 📨️ Runs only the retained first-party origin leaf with the same already-held Source guard.
 pub fn drive_return_origin_producer(&mut self,producer:&mut OriginalEffectAuthority<OriginalReturnOriginProducer<'_,OriginalReturnSourceInput>>,action:&mut Option<OriginalEffectAction<RetainedCloneStep,OriginalReceivingPrimary>>,binding:&NativeReceivingConstructionBinding,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let parent=self.source_parent.ok_or_else(||refused("original return producer Source parent is absent"))?;parent.validate_borrowed_context(self)?;if self.original_source_turn_identity()!=binding.source()||!binding.matches_effect(effect){return Err(refused("original return producer changed its live Source"))}
  let budget=self.return_budget.as_mut().ok_or_else(||refused("original return budget loan is absent"))?;let mut context=OriginalReturnOriginContext{binding,budget};producer.drive(&mut context,effect,action)
 }
}
