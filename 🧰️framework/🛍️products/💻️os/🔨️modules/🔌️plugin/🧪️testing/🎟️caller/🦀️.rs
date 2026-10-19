//! 🎟️ Independently authored fixture authority survives original settlement and terminal closure.
use semio_framework_job::{CancelToken,Generation,JobPayloadAuthority,OperationId,StepBudget,StepContext,OriginalCallerSourceParent};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;

#[derive(serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct CallerPolicy{source_kind:String,operation:u64,generation:u64,fuel:u64,clock_us:u64,deadline_us:u64,maximum_turns:usize,grant:RetainedCloneGrant,retirement_loan:RetainedCloneGrant}

fn fixture_remaining(stock:RetainedCloneGrant,spent:RetainedCloneProgress)->Result<RetainedCloneGrant,ValueError>{if !spent.fits(stock){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture source admissions exceed its initiating stock"))}Ok(RetainedCloneGrant{maximum_items:stock.maximum_items-spent.copied_items,maximum_copy_bytes:stock.maximum_copy_bytes-spent.copied_bytes,maximum_capacity_bytes:stock.maximum_capacity_bytes-spent.retained_capacity_bytes,maximum_release_bytes:stock.maximum_release_bytes-spent.released_bytes,..stock})}

fn fixture_now_us()->Option<u64>{Some(1000)}

/// 🪙️ One original ledger, cancellation root, cursor and cumulative recipient own the fixture's whole financed body.
pub struct OriginalFixtureCaller{policy:CallerPolicy,budget:Option<StepBudget>,source:ManuallyDrop<Option<OriginalCallerSourceParent>>,sequence:u64,receipt:RetainedCloneProgress,decoder:semio_framework_job::OriginalJobDecodeOwner}
impl OriginalFixtureCaller{
 /// 🚪️ Creates original authority from closed test input before semantic work begins.
 pub fn admit()->Result<Self,ValueError>{
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).expect("closed original fixture caller policy");assert_eq!(policy.source_kind,"original-parent");assert_eq!(policy.clock_us,1000);let (cancel,root)=CancelToken::admit_root(policy.grant)?.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture cancellation birth is not funded"))?;
  let remaining=fixture_remaining(policy.grant,root)?;let (authority,ledger)=JobPayloadAuthority::admit(OperationId(policy.operation),Generation(policy.generation),remaining)?.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture ledger birth is not funded"))?;
  let remaining=fixture_remaining(policy.grant,root.checked_add(ledger)?)?;let(source,parent)=OriginalCallerSourceParent::admit(authority,cancel,remaining).unwrap_or_else(|_|panic!("original fixture parent birth is funded by its closed policy"));let receipt=root.checked_add(ledger)?.checked_add(parent)?;assert!(receipt.fits(policy.grant));let fuel=policy.fuel.checked_sub(receipt.copied_items as u64).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original fixture source admissions exceed initiating fuel"))?;let budget=Some(StepBudget::new(fuel,policy.deadline_us,policy.grant));Ok(Self{policy,budget,source:ManuallyDrop::new(Some(source)),sequence:0,receipt,decoder:semio_framework_job::OriginalJobDecodeOwner::empty()})
 }
 /// 🪢️ Lends one original context; another call cannot recreate its original fuel or ceiling.
 pub fn context(&mut self)->StepContext<'_>{let budget=self.budget.take().expect("original fixture context can only be issued once");self.source.as_ref().expect("original fixture parent remains retained while its context is lent").borrow_context(budget,fixture_now_us,&mut self.sequence,&mut self.receipt).unwrap().with_original_decoder(&mut self.decoder)}
 /// 📏️ Bounds caller loops without deriving finance from an owner's descriptive demand.
 pub fn maximum_turns()->usize{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().maximum_turns}
 /// 🛬️ Retires the body decoder against the same already-issued caller stock before its three parent roots close.
 pub fn retire_decoder(&mut self,decoder:&mut semio_framework_job::OriginalJobDecodeOwner,fuel:u64)->Result<u64,ValueError>{assert!(self.budget.is_none(),"original decoder retirement follows its single body loan");retire_fixture_decoder(self.policy.grant,self.policy.retirement_loan,self.policy.maximum_turns,&mut self.receipt,decoder,fuel)}
 /// 🧹️ Returns the original caller ledger and root with its remaining authored authority after all child payloads close.
 pub fn retire(&mut self,mut fuel:u64)->Result<RetainedCloneProgress,ValueError>{
  fuel=retire_fixture_decoder(self.policy.grant,self.policy.retirement_loan,self.policy.maximum_turns,&mut self.receipt,&mut self.decoder,fuel)?;
  for _ in 0..self.policy.maximum_turns{
   if self.source.as_ref().is_none_or(OriginalCallerSourceParent::terminal_is_empty){self.source.take();return Ok(self.receipt)}
   if fuel==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original fixture caller retirement exhausted its original fuel"))}let supplied=self.policy.grant;let posted=self.receipt;let wanted=self.policy.retirement_loan;let grant=RetainedCloneGrant{maximum_items:wanted.maximum_items.min(supplied.maximum_items.saturating_sub(posted.copied_items)),maximum_copy_bytes:wanted.maximum_copy_bytes.min(supplied.maximum_copy_bytes.saturating_sub(posted.copied_bytes)),maximum_capacity_bytes:wanted.maximum_capacity_bytes.min(supplied.maximum_capacity_bytes.saturating_sub(posted.retained_capacity_bytes)),maximum_release_bytes:wanted.maximum_release_bytes.min(supplied.maximum_release_bytes.saturating_sub(posted.released_bytes)),maximum_depth:wanted.maximum_depth.min(supplied.maximum_depth)};
   let step=self.source.as_mut().expect("original fixture parent remains in custody until funded retirement").close_step(grant)?;let progress=step.progress();fuel=fuel.saturating_sub(progress.copied_items as u64);self.receipt=self.receipt.checked_add(progress)?;if !self.receipt.fits(self.policy.grant)||!progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture source close exceeds its initiating stock").with_retained_progress(progress))}if progress==Default::default(){std::thread::yield_now()}
  }
  Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original fixture caller retirement exceeded its supplied turn bound"))
 }
}
fn retire_fixture_decoder(supplied:RetainedCloneGrant,wanted:RetainedCloneGrant,maximum_turns:usize,receipt:&mut RetainedCloneProgress,decoder:&mut semio_framework_job::OriginalJobDecodeOwner,mut fuel:u64)->Result<u64,ValueError>{
 for _ in 0..maximum_turns{if decoder.terminal_is_empty(){return Ok(fuel)}if fuel==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original fixture decoder exhausted its original fuel"))}let posted=*receipt;let grant=RetainedCloneGrant{maximum_items:wanted.maximum_items.min(supplied.maximum_items.saturating_sub(posted.copied_items)).min(usize::try_from(fuel).unwrap_or(usize::MAX)),maximum_copy_bytes:wanted.maximum_copy_bytes.min(supplied.maximum_copy_bytes.saturating_sub(posted.copied_bytes)),maximum_capacity_bytes:wanted.maximum_capacity_bytes.min(supplied.maximum_capacity_bytes.saturating_sub(posted.retained_capacity_bytes)),maximum_release_bytes:wanted.maximum_release_bytes.min(supplied.maximum_release_bytes.saturating_sub(posted.released_bytes)),maximum_depth:wanted.maximum_depth.min(supplied.maximum_depth)};
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||decoder.close_step(grant));let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));fuel=fuel.saturating_sub(progress.copied_items as u64);*receipt=receipt.checked_add(progress)?;if !progress.fits(grant)||!receipt.fits(supplied){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original decoder close exceeds initiating caller stock").with_retained_progress(progress))}result?;
  }Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original fixture decoder exceeds its supplied close turn bound"))
}
impl Drop for OriginalFixtureCaller{fn drop(&mut self){assert!(std::thread::panicking()||self.source.is_none(),"original fixture caller requires paid terminal retirement")}}

/// 🪙️ One fixture-lifetime original caller: owns the caller and its single lent context so a fixture app threads the same `cx` through every call and pays the caller's terminal retirement once, at an explicit [`OriginalFixtureScope::retire`] or else when the scope drops (a scope dropped while panicking pays nothing).
pub struct OriginalFixtureScope{context:Option<StepContext<'static>>,caller:*mut OriginalFixtureCaller}
impl OriginalFixtureScope{
 /// 🚪️ Admits one original caller and lends its sole context for the whole life of the scope.
 pub fn admit()->Self{let caller=Box::into_raw(Box::new(OriginalFixtureCaller::admit().expect("closed original fixture scope caller")));let context=unsafe{(*caller).context()};Self{context:Some(context),caller}}
 /// 🪢️ The one original context every call of the fixture's financed body shares.
 pub fn cx(&mut self)->&mut StepContext<'static>{self.context.as_mut().expect("original fixture scope context is lent until its retirement")}
 /// 🧹️ Returns the context, then pays the caller's terminal retirement from the fuel the body left.
 pub fn retire(&mut self)->RetainedCloneProgress{
  let context=self.context.take().expect("original fixture scope retires once");let fuel=context.fuel_remaining();drop(context);
  let mut caller=unsafe{Box::from_raw(self.caller)};self.caller=std::ptr::null_mut();
  caller.retire(fuel).expect("paid original fixture scope retirement")
 }
}
impl Drop for OriginalFixtureScope{fn drop(&mut self){if self.caller.is_null(){return}if std::thread::panicking(){drop(self.context.take());drop(unsafe{Box::from_raw(self.caller)})}else{self.retire();}}}

#[cfg(test)]
mod original_physical_laws{
 use super::*;

 /// 🪙️ The sole body context starts after its three original root admissions consumed CPU and stock.
 #[test]
 fn original_plugin_fixture_caller_charges_root_finance_before_body_and_closes_same_stock(){
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
  let mut caller=OriginalFixtureCaller::admit().unwrap();let roots=caller.receipt;assert_eq!(roots.copied_items,3);assert!(roots.fits(policy.grant));
  let mut cx=caller.context();assert_eq!(cx.fuel_remaining(),policy.fuel-roots.copied_items as u64);assert_eq!(cx.retained_progress(),roots);assert_eq!(cx.retained_grant(),fixture_remaining(policy.grant,roots).unwrap());
  let original=cx.original_recipient_identity();let before=cx.retained_progress();let fuel=cx.fuel_remaining();
  let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(0,policy.deadline_us,|same|{assert!(same.should_yield());assert_eq!(same.original_recipient_identity(),original);Ok(())}));
  assert!(result.is_ok());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
  drop(cx);let receipt=caller.retire(fuel).unwrap();assert!(receipt.fits(policy.grant));assert!(caller.source.is_none());assert!(receipt.released_bytes>=roots.retained_capacity_bytes);
 }

 use semio_framework_value::retirement::controlled::ControlledRetirement;
 use semio_framework_job::close_original_retained_step;
 #[test]
 fn original_guest_failure_debit_fixed_page_original_context_denial_and_paid_physical_release(){
  let input:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/📄️retirement.json")).unwrap();let bytes:Vec<u8>=serde_json::from_value(input["sourceBytes"].clone()).unwrap();let block=input["blockBytes"].as_u64().unwrap()as usize;let phase=serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().retirement_loan;let mut caller=OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let recipient=cx.original_recipient_identity();let(page,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::dsl::command_ingress::FixedCommandPage::try_copy_from(&bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(block,0));let pointer=page.as_slice().as_ptr();cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:block,..Default::default()}).unwrap();let mut owner=ControlledRetirement::new(page).unwrap_or_else(|_|panic!("real fixed page has no controlled retirement"));assert_eq!(owner.original().unwrap().as_slice().as_ptr(),pointer);
  let staging=RetainedCloneGrant{maximum_release_bytes:0,..phase};let mut staged=false;for _ in 0..OriginalFixtureCaller::maximum_turns(){if owner.next_release_byte_demand().unwrap()==block{staged=true;break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(staging,|same|{let demand=owner.selected_demand(staging.maximum_copy_bytes)?;close_original_retained_step(same,demand,|grant|owner.step(grant))}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(progress.released_bytes,0)}assert!(staged);
  for row in input["cases"].as_array().unwrap().iter().take(2){let loan=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_release_bytes:row["releaseBytes"].as_u64().unwrap()as usize,..phase};let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_retained_step(same,demand,|grant|owner.step(grant))}));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert!(!owner.terminal_is_empty());assert_eq!(cx.original_recipient_identity(),recipient)}
  let exact=&input["cases"][2];let loan=RetainedCloneGrant{maximum_items:exact["items"].as_u64().unwrap()as usize,maximum_release_bytes:exact["releaseBytes"].as_u64().unwrap()as usize,..phase};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_retained_step(same,demand,|grant|owner.step(grant))}));let progress=result.unwrap().progress();assert_eq!(progress.released_bytes,block);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,block));
  for _ in 0..OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_retained_step(same,demand,|grant|owner.step(grant))}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes))}assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.original_recipient_identity(),recipient);assert_eq!(cx.operation(),OperationId(97146));assert_eq!(cx.generation(),Generation(17));assert_eq!(cx.deadline_us(),1500);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
 }
 /// 🏗️ The fixture lends one actual decoder to construction and retires its same physical scaffold after the candidate closes.
 #[test]
 fn original_plugin_fixture_decoder_constructs_and_closes_under_the_same_source(){
  use semio_framework::action_bus::{ToolJobAdmissionDemand,OriginalToolSourceBinding,admit_original_tool_construction};
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let input:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/📄️retirement.json")).unwrap();let bytes:Vec<u8>=serde_json::from_value(input["sourceBytes"].clone()).unwrap();let count=bytes.len();assert!(count>0);let mut caller=OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let binding=OriginalToolSourceBinding::borrow(&cx);let demand=ToolJobAdmissionDemand{items:1,copy_bytes:count,capacity_bytes:count,release_bytes:0,depth:1};let mut output:Option<Vec<u8>>=None;
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_capacity_bytes:count-1,..policy.retirement_loan},|cx|admit_original_tool_construction(cx,&mut output,demand,|_|panic!("short original capacity cannot call construction"))));assert!(!result.unwrap());assert!(output.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(binding.matches(&cx));
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(policy.retirement_loan,|cx|admit_original_tool_construction(cx,&mut output,demand,|identity|{assert_eq!(identity.source_binding(),binding);let mut value=Vec::with_capacity(count);value.extend_from_slice(&bytes);let progress=RetainedCloneProgress{copied_items:1,copied_bytes:count,retained_capacity_bytes:value.capacity(),..Default::default()};Ok((value,progress))})));assert!(result.unwrap());assert_eq!(heap.requested_bytes,count);assert_eq!(heap.released_bytes,0);assert_eq!(output.as_deref(),Some(bytes.as_slice()));assert!(binding.matches(&cx));let mut owner=ControlledRetirement::new(output.take().unwrap()).unwrap_or_else(|_|panic!("native vector owner must close"));
  for _ in 0..policy.maximum_turns{if owner.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(policy.retirement_loan,|cx|{let demand=owner.selected_demand(policy.retirement_loan.maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|owner.step(grant))}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert!(binding.matches(&cx));}assert!(owner.terminal_is_empty());let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();assert!(caller.decoder.terminal_is_empty());println!("[DEBUG] plugin fixture real original decoder construction short-capacity0heap same source candidate custody paid scaffold close");
 }

}
