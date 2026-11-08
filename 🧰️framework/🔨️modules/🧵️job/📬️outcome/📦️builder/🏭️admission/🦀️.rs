//! 🏗️ The original payload ledger and actual shared backing stay in resident paid birth slots.
use super::*;
use semio_framework_value_retirement::retirement::allocation_return::{shared_uninitialized_demand,shared_uninitialized_work_demand,try_uninitialized_arc_into,shared_arc_publication_demand,shared_arc_publication_work_demand,publish_uninitialized_arc_from,uninitialized_arc_release_demand,release_uninitialized_arc_from};
/// 🧳️ Retains the original ledger, uninitialized backing and published Arc without guessed rollback.
pub struct OriginalJobPayloadAdmission{input:ManuallyDrop<Option<JobPayloadOperationLedger>>,backing:ManuallyDrop<Option<Arc<std::mem::MaybeUninit<JobPayloadOperationLedger>>>>,published:ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>}
impl OriginalJobPayloadAdmission{
 /// 🪹️ Creates only empty inline slots.
 pub fn new()->Self{Self{input:ManuallyDrop::new(None),backing:ManuallyDrop::new(None),published:ManuallyDrop::new(None)}}
 pub fn terminal_is_empty(&self)->bool{self.input.is_none()&&self.backing.is_none()&&self.published.is_none()}
 /// 🪪️ Observes the same original operation identity without transferring its ledger.
 pub fn input_identity(&self)->Option<(OperationId,Generation)>{self.input.as_ref().map(|input|(input.operation,input.generation))}
 /// 👁️ Reports the original backing and publication slots without lending them.
 pub fn has_backing(&self)->bool{self.backing.is_some()}
 /// 🪪️ Observes the actual allocated backing identity without lending it.
 pub fn backing_identity(&self)->Option<OriginalPayloadSourceIdentity>{self.backing.as_ref().map(|value|OriginalPayloadSourceIdentity(Arc::as_ptr(value).cast::<()>() as usize))}
 pub fn is_published(&self)->bool{self.published.is_some()}
 /// 📏️ Quotes the fixed original ledger metadata before it is constructed.
 pub fn prepare_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<JobPayloadOperationLedger>>(),depth:1,..Default::default()}}
 /// 🌱️ Starts the original operation ledger after its actual performed-work receipt.
 pub fn prepare(&mut self,operation:OperationId,generation:Generation,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let before=effect.performed_boundary();let result=(||{if !self.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"payload admission already holds its original owner"))}let demand=Self::prepare_demand();if effect.remaining_work_units()?<1||!semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"payload preparation exceeded original work"))}*self.input=Some(JobPayloadOperationLedger::new(operation,generation));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same payload preparation receipt")}}
 /// 📏️ Quotes one real allocator attempt, including its resident backing slot.
 pub fn backing_demand()->Result<RetirementDemand,ValueError>{shared_uninitialized_demand::<JobPayloadOperationLedger>()}
 pub fn backing_work_demand()->u64{shared_uninitialized_work_demand()}
 /// 👶️ Retains successful backing and records zero when the actual allocator refuses.
 pub fn admit_backing(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{if self.input.is_none()||self.published.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"payload backing has no eligible original input")),work:0}}try_uninitialized_arc_into(&mut self.backing,effect)}
 pub fn publication_demand()->Result<RetirementDemand,ValueError>{shared_arc_publication_demand::<JobPayloadOperationLedger>()}
 pub fn publication_work_demand()->u64{shared_arc_publication_work_demand()}
 /// 📦️ Publishes into the same retained Arc slot after uniqueness and transfer work.
 pub fn publish(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{publish_uninitialized_arc_from(&mut self.backing,&mut self.input,&mut self.published,effect)}
 /// 📏️ Quotes the original Arc and destination authority slots.
 pub fn take_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Arc<JobPayloadOperationLedger>>>()+size_of::<Option<JobPayloadAuthority>>(),depth:1,..Default::default()}}
 /// 📤️ Transfers the same published ledger into the caller-owned authority slot.
 pub fn take_authority(&mut self,output:&mut Option<JobPayloadAuthority>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let before=effect.performed_boundary();let result=(||{if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"payload authority output is occupied"))}if self.published.is_none(){return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=Self::take_demand();if effect.remaining_work_units()?<1||!semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"payload authority transfer exceeded original work"))}*output=Some(JobPayloadAuthority{original:ManuallyDrop::new(self.published.take())});Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same payload authority receipt")}}
 pub fn backing_release_demand(&self)->RetirementDemand{uninitialized_arc_release_demand(&self.backing)}
 /// ♻️ Releases only the same uninitialized backing after an actual paid cancellation.
 pub fn release_backing(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{release_uninitialized_arc_from(&mut self.backing,effect)}
 /// 📤️ Returns the untouched operation and generation from an unbacked original input.
 pub fn return_input(&mut self,output:&mut Option<(OperationId,Generation)>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let before=effect.performed_boundary();let result=(||{if output.is_some()||self.backing.is_some()||self.published.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"payload input return conflicts with resident backing or output"))}if self.input.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=RetirementDemand{copy_bytes:size_of::<Option<JobPayloadOperationLedger>>()+size_of::<Option<(OperationId,Generation)>>(),depth:1,..Default::default()};if effect.remaining_work_units()?<1||!semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"payload input return exceeded original work"))}let input=self.input.take().expect("original payload input checked");*output=Some((input.operation,input.generation));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same payload input return receipt")}}
}
impl Default for OriginalJobPayloadAdmission{fn default()->Self{Self::new()}}
impl Drop for OriginalJobPayloadAdmission{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"payload admission retains original owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.input);ManuallyDrop::drop(&mut self.backing);ManuallyDrop::drop(&mut self.published)}}}}
