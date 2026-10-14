//! 🎟️ Independently authored fixture authority survives original settlement and terminal closure.
use semio_framework_job::{CancelToken,Generation,JobPayloadAuthority,OperationId,StepBudget,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;

#[derive(serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct CallerPolicy{operation:u64,generation:u64,fuel:u64,clock_us:u64,deadline_us:u64,maximum_turns:usize,grant:RetainedCloneGrant,retirement_loan:RetainedCloneGrant}

/// 🪙️ One original ledger, cancellation root, cursor and cumulative recipient own the fixture's whole financed body.
pub struct OriginalFixtureCaller{policy:CallerPolicy,budget:Option<StepBudget>,cancel:ManuallyDrop<Option<CancelToken>>,authority:JobPayloadAuthority,sequence:u64,receipt:RetainedCloneProgress}
impl OriginalFixtureCaller{
 /// 🚪️ Creates original authority from closed test input before semantic work begins.
 pub fn admit()->Result<Self,ValueError>{
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).expect("closed original fixture caller policy");assert_eq!(policy.clock_us,1000);let (cancel,root)=CancelToken::admit_root(policy.grant)?.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture cancellation birth is not funded"))?;
  let (authority,ledger)=JobPayloadAuthority::admit(OperationId(policy.operation),Generation(policy.generation),policy.grant)?.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture ledger birth is not funded"))?;
  let receipt=root.checked_add(ledger)?;assert!(receipt.fits(policy.grant));let budget=Some(StepBudget::new(policy.fuel,policy.deadline_us,policy.grant));Ok(Self{policy,budget,cancel:ManuallyDrop::new(Some(cancel)),authority,sequence:0,receipt})
 }
 /// 🪢️ Lends one original context; another call cannot recreate its original fuel or ceiling.
 pub fn context(&mut self)->StepContext<'_>{let budget=self.budget.take().expect("original fixture context can only be issued once");StepContext::with_payload_authority(OperationId(self.policy.operation),Generation(self.policy.generation),budget,self.cancel.as_ref().unwrap(),||Some(1000),&mut self.sequence,&mut self.receipt,&self.authority).unwrap()}
 /// 📏️ Bounds caller loops without deriving finance from an owner's descriptive demand.
 pub fn maximum_turns()->usize{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().maximum_turns}
 /// 🧹️ Returns the original caller ledger and root with its remaining authored authority after all child payloads close.
 pub fn retire(mut self,mut fuel:u64)->Result<RetainedCloneProgress,ValueError>{
  let mut cancel=None;
  for _ in 0..self.policy.maximum_turns{
   if self.authority.terminal_is_empty()&&cancel.as_ref().is_some_and(|owner:&semio_framework_async::CancelTokenRetirement|owner.terminal_is_empty()){return Ok(self.receipt)}
   if fuel==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original fixture caller retirement exhausted its original fuel"))}let supplied=self.policy.grant;let posted=self.receipt;let wanted=self.policy.retirement_loan;let grant=RetainedCloneGrant{maximum_items:wanted.maximum_items.min(supplied.maximum_items.saturating_sub(posted.copied_items)),maximum_copy_bytes:wanted.maximum_copy_bytes.min(supplied.maximum_copy_bytes.saturating_sub(posted.copied_bytes)),maximum_capacity_bytes:wanted.maximum_capacity_bytes.min(supplied.maximum_capacity_bytes.saturating_sub(posted.retained_capacity_bytes)),maximum_release_bytes:wanted.maximum_release_bytes.min(supplied.maximum_release_bytes.saturating_sub(posted.released_bytes)),maximum_depth:wanted.maximum_depth.min(supplied.maximum_depth)};
   let step=if !self.authority.terminal_is_empty(){self.authority.close_step(grant)?}else{let owner=cancel.get_or_insert_with(||semio_framework_async::CancelTokenRetirement::from_token(self.cancel.take().unwrap()));match owner.close_step(grant){Ok(step)=>step,Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>return Err(error),Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>{fuel=fuel.saturating_sub(1);std::thread::yield_now();continue}}};let progress=step.progress();fuel=fuel.saturating_sub(progress.copied_items as u64);self.receipt=self.receipt.checked_add(progress)?;if progress==Default::default(){std::thread::yield_now()}
  }
  Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original fixture caller retirement exceeded its supplied turn bound"))
 }
}
impl Drop for OriginalFixtureCaller{fn drop(&mut self){assert!(std::thread::panicking()||self.authority.terminal_is_empty()&&self.cancel.is_none(),"original fixture caller requires paid terminal retirement")}}

#[cfg(test)]
mod original_physical_laws{
 use super::*;
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 use semio_framework_job::close_original_retained_step;
 #[test]
 fn original_guest_failure_debit_fixed_page_original_context_denial_and_paid_physical_release(){
  let input:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/📄️retirement.json")).unwrap();let bytes:Vec<u8>=serde_json::from_value(input["sourceBytes"].clone()).unwrap();let block=input["blockBytes"].as_u64().unwrap()as usize;let phase=serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().retirement_loan;let mut caller=OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let recipient=cx.original_recipient_identity();let(page,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||kernel::FixedCommandPage::try_copy_from(&bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(block,0));let pointer=page.as_slice().as_ptr();cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:block,..Default::default()}).unwrap();let mut owner=ControlledRetirement::new(page).unwrap_or_else(|_|panic!("real fixed page has no controlled retirement"));assert_eq!(owner.original().unwrap().as_slice().as_ptr(),pointer);
  let staging=RetainedCloneGrant{maximum_release_bytes:0,..phase};let mut staged=false;for _ in 0..OriginalFixtureCaller::maximum_turns(){if owner.next_release_byte_demand().unwrap()==block{staged=true;break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(staging,|same|{let demand=owner.selected_demand(staging.maximum_copy_bytes)?;close_original_retained_step(same,demand,|grant|owner.step(grant))}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(progress.released_bytes,0)}assert!(staged);
  for row in input["cases"].as_array().unwrap().iter().take(2){let loan=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_release_bytes:row["releaseBytes"].as_u64().unwrap()as usize,..phase};let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_retained_step(same,demand,|grant|owner.step(grant))}));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert!(!owner.terminal_is_empty());assert_eq!(cx.original_recipient_identity(),recipient)}
  let exact=&input["cases"][2];let loan=RetainedCloneGrant{maximum_items:exact["items"].as_u64().unwrap()as usize,maximum_release_bytes:exact["releaseBytes"].as_u64().unwrap()as usize,..phase};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_retained_step(same,demand,|grant|owner.step(grant))}));let progress=result.unwrap().progress();assert_eq!(progress.released_bytes,block);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,block));
  for _ in 0..OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_retained_step(same,demand,|grant|owner.step(grant))}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes))}assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.original_recipient_identity(),recipient);assert_eq!(cx.operation(),OperationId(97146));assert_eq!(cx.generation(),Generation(17));assert_eq!(cx.deadline_us(),1500);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();eprintln!("[DEBUG] originalFixedPagePhysicalBlock4096=true zeroShortLoans0=true exactRelease4096=true sameOriginalRecipientDeadline=true paidScaffoldRelease=true terminalDrop0=true");
 }
}
