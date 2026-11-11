//! 🎟️ Independently authored fixture authority survives original settlement and terminal closure.
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError};

#[derive(serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct CallerPolicy{source_kind:String,operation:u64,generation:u64,fuel:u64,turn_fuel:u64,clock_us:u64,deadline_us:u64,maximum_turns:usize,maximum_idle_turns:usize,grant:RetainedCloneGrant,retirement_loan:RetainedCloneGrant}

fn fixture_now_us()->Option<u64>{Some(1000)}

/// 🪙️ One original ledger, cancellation root, cursor and cumulative recipient own the fixture's whole financed body; its roots are born and closed through the production effect-threaded admissions of the framework fixture caller.
pub struct OriginalFixtureCaller{policy:CallerPolicy,caller:OriginalJobFixtureCaller,decoder:semio_framework_job::OriginalJobDecodeOwner,lent:bool}
impl OriginalFixtureCaller{
 /// 🚪️ Creates original authority from closed test input before semantic work begins.
 pub fn admit()->Result<Self,ValueError>{
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).expect("closed original fixture caller policy");assert_eq!(policy.source_kind,"original-parent");assert_eq!(policy.clock_us,fixture_now_us().unwrap());
  let caller=OriginalJobFixtureCaller::admit(OperationId(policy.operation),Generation(policy.generation),policy.grant,fixture_now_us,policy.deadline_us,policy.fuel);
  Ok(Self{policy,caller,decoder:semio_framework_job::OriginalJobDecodeOwner::empty(),lent:false})
 }
 /// 🪢️ Lends one original context; another call cannot recreate its original fuel or ceiling.
 pub fn context(&mut self)->StepContext<'_>{assert!(!self.lent,"original fixture context can only be issued once");self.lent=true;self.caller.lend_context().with_original_decoder(&mut self.decoder)}
 /// ⛽️ The fuel one host Interactive turn is granted: every fixture turn starts from it, never from what an earlier call left on the long-lived context.
 pub fn turn_fuel()->u64{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().turn_fuel}
 /// 💤️ Consecutive turns that may settle nothing before a loop fails typed.
 pub fn maximum_idle_turns()->usize{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().maximum_idle_turns}
 /// 📏️ Bounds caller loops without deriving finance from an owner's descriptive demand.
 pub fn maximum_turns()->usize{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().maximum_turns}
 /// 🛬️ Retires the body decoder against the same already-issued caller stock before its three parent roots close.
 pub fn retire_decoder(&mut self,decoder:&mut semio_framework_job::OriginalJobDecodeOwner,fuel:u64)->Result<u64,OriginalTurnFailure>{self.caller.settle_lent_fuel(fuel);let loan=self.policy.retirement_loan;for _ in 0..self.policy.maximum_turns{if decoder.terminal_is_empty(){return Ok(self.caller.fuel_remaining())}self.caller.turn(loan,|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())}))?}Err(OriginalTurnFailure::from(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture decoder exceeds its supplied close turn bound")))}
 /// 🧹️ Returns the original caller ledger and root with its remaining authored authority after all child payloads close.
 pub fn retire(&mut self,fuel:u64)->Result<RetainedCloneProgress,OriginalTurnFailure>{
  self.caller.settle_lent_fuel(fuel);let mut decoder=std::mem::replace(&mut self.decoder,semio_framework_job::OriginalJobDecodeOwner::empty());let retired=self.retire_decoder(&mut decoder,fuel);self.decoder=decoder;retired?;
  let loan=self.policy.retirement_loan;
  for _ in 0..self.policy.maximum_turns{
   if self.caller.terminal_is_empty(){let receipt=self.caller.cumulative_receipt();assert!(receipt.fits(self.policy.grant));return Ok(receipt)}
   let step=self.caller.retire_step(loan)?;if step.progress()==Default::default(){std::thread::yield_now()}
  }
  Err(OriginalTurnFailure::from(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture caller retirement exceeded its supplied turn bound")))
 }
}
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
 fn fixture_remaining(stock:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{assert!(spent.fits(stock),"original fixture source admissions exceed its initiating stock");RetainedCloneGrant{maximum_items:stock.maximum_items-spent.copied_items,maximum_copy_bytes:stock.maximum_copy_bytes-spent.copied_bytes,maximum_capacity_bytes:stock.maximum_capacity_bytes-spent.retained_capacity_bytes,maximum_release_bytes:stock.maximum_release_bytes-spent.released_bytes,..stock}}

 /// 🪙️ The sole body context starts after its three original root admissions consumed CPU and stock.
 #[test]
 fn original_plugin_fixture_caller_charges_root_finance_before_body_and_closes_same_stock(){
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
  let mut caller=OriginalFixtureCaller::admit().unwrap();let roots=caller.caller.cumulative_receipt();let birth_fuel=caller.caller.fuel_remaining();assert!(roots.copied_items>0&&birth_fuel<policy.fuel);assert!(roots.fits(policy.grant));
  let mut cx=caller.context();assert_eq!(cx.fuel_remaining(),birth_fuel);assert_eq!(cx.retained_progress(),roots);assert_eq!(cx.retained_grant(),fixture_remaining(policy.grant,roots));
  let original=cx.original_recipient_identity();let before=cx.retained_progress();let fuel=cx.fuel_remaining();
  let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(0,policy.deadline_us,|same|{assert!(same.should_yield());assert_eq!(same.original_recipient_identity(),original);Ok(())}));
  assert!(result.is_ok());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
  drop(cx);let receipt=caller.retire(fuel).unwrap();assert!(receipt.fits(policy.grant));assert!(caller.caller.terminal_is_empty());assert!(receipt.released_bytes>=roots.retained_capacity_bytes);
 }

 use semio_framework_value::retirement::controlled::ControlledRetirement;
 use semio_framework_job::close_original_effect_step;
 #[test]
 fn original_guest_failure_debit_fixed_page_original_context_denial_and_paid_physical_release(){
  let input:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/📄️retirement.json")).unwrap();let bytes:Vec<u8>=serde_json::from_value(input["sourceBytes"].clone()).unwrap();let block=input["blockBytes"].as_u64().unwrap()as usize;let phase=serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().retirement_loan;let mut caller=OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let recipient=cx.original_recipient_identity();let(page,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework::kernel::FixedCommandPage::try_copy_from(&bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(block,0));let pointer=page.as_slice().as_ptr();cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:block,..Default::default()}).unwrap();let mut owner=ControlledRetirement::new(page).unwrap_or_else(|_|panic!("real fixed page has no controlled retirement"));assert_eq!(owner.original().unwrap().as_slice().as_ptr(),pointer);
  let staging=RetainedCloneGrant{maximum_release_bytes:0,..phase};let mut staged=false;for _ in 0..OriginalFixtureCaller::maximum_turns(){if owner.next_release_byte_demand().unwrap()==block{staged=true;break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(staging,|same|{let demand=owner.selected_demand(staging.maximum_copy_bytes)?;close_original_effect_step(same,demand,|effect|{let body=effect.grant().maximum_copy_bytes;owner.step(body,effect)})}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(progress.released_bytes,0)}assert!(staged);
  for row in input["cases"].as_array().unwrap().iter().take(2){let loan=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_release_bytes:row["releaseBytes"].as_u64().unwrap()as usize,..phase};let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_effect_step(same,demand,|effect|{let body=effect.grant().maximum_copy_bytes;owner.step(body,effect)})}));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert!(!owner.terminal_is_empty());assert_eq!(cx.original_recipient_identity(),recipient)}
  let exact=&input["cases"][2];let loan=RetainedCloneGrant{maximum_items:exact["items"].as_u64().unwrap()as usize,maximum_release_bytes:exact["releaseBytes"].as_u64().unwrap()as usize,..phase};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_effect_step(same,demand,|effect|{let body=effect.grant().maximum_copy_bytes;owner.step(body,effect)})}));let progress=result.unwrap().progress();assert_eq!(progress.released_bytes,block);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,block));
  for _ in 0..OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|{let demand=owner.selected_demand(phase.maximum_copy_bytes)?;close_original_effect_step(same,demand,|effect|{let body=effect.grant().maximum_copy_bytes;owner.step(body,effect)})}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes))}assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.original_recipient_identity(),recipient);assert_eq!(cx.operation(),OperationId(97146));assert_eq!(cx.generation(),Generation(17));assert_eq!(cx.deadline_us(),1500);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
 }
 /// 🏗️ The fixture lends one actual decoder to construction and retires its same physical scaffold after the candidate closes.
 #[test]
 fn original_plugin_fixture_decoder_constructs_and_closes_under_the_same_source(){
  use semio_framework::action_bus::{ToolJobAdmissionDemand,OriginalToolSourceBinding,admit_original_tool_construction_settled};
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let input:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/📄️retirement.json")).unwrap();let bytes:Vec<u8>=serde_json::from_value(input["sourceBytes"].clone()).unwrap();let count=bytes.len();assert!(count>0);let mut caller=OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let binding=OriginalToolSourceBinding::borrow(&cx);let demand=ToolJobAdmissionDemand{items:1,copy_bytes:count,capacity_bytes:count,release_bytes:0,depth:1};let mut output:Option<Vec<u8>>=None;let mut admission=semio_framework_job::OriginalConstructionAdmission::default();
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_capacity_bytes:count-1,..policy.retirement_loan},|cx|admit_original_tool_construction_settled(cx,&mut admission,&mut output,demand,|_|panic!("short original capacity cannot call construction"))));assert!(!result.unwrap());assert!(output.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(binding.matches(&cx));
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(policy.retirement_loan,|cx|admit_original_tool_construction_settled(cx,&mut admission,&mut output,demand,|scope|{assert_eq!(scope.identity().source_binding(),binding);let mut value=Vec::with_capacity(count);value.extend_from_slice(&bytes);let progress=RetainedCloneProgress{copied_items:1,copied_bytes:count,retained_capacity_bytes:value.capacity(),..Default::default()};Ok((value,progress))})));assert!(result.unwrap());assert_eq!(heap.requested_bytes,count);assert_eq!(heap.released_bytes,0);assert_eq!(output.as_deref(),Some(bytes.as_slice()));assert!(binding.matches(&cx));let mut owner=ControlledRetirement::new(output.take().unwrap()).unwrap_or_else(|_|panic!("native vector owner must close"));
  for _ in 0..policy.maximum_turns{if owner.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(policy.retirement_loan,|cx|{let demand=owner.selected_demand(policy.retirement_loan.maximum_copy_bytes)?;close_original_effect_step(cx,demand,|effect|{let body=effect.grant().maximum_copy_bytes;owner.step(body,effect)})}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert!(binding.matches(&cx));}assert!(owner.terminal_is_empty());let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));for _ in 0..policy.maximum_turns{if admission.terminal_is_empty(){break}cx.with_retained_loan(policy.retirement_loan,|cx|admission.close_step(cx)).unwrap();}assert!(admission.terminal_is_empty());let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();assert!(caller.decoder.terminal_is_empty());
 }

}

#[cfg(test)]
mod host_lane_authority_law {
    /// 🚦️ The fixture caller's per-turn fuel is the host's Interactive lane budget and its lifetime stock is exactly one such turn per allowed fixture turn, read from the same constant the kernel grants turns from, so the two can never drift apart.
    #[test]
    fn fixture_caller_fuel_equals_the_host_interactive_lane_budget() {
        let policy: super::CallerPolicy = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
        assert_eq!(policy.turn_fuel, semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive).fuel);
        assert_eq!(policy.fuel, policy.turn_fuel * policy.maximum_turns as u64);
    }

    /// 📌️ The pinned literals: 2,000,000 fuel per turn is the host Interactive grant, and the lifetime stock is that grant times the 32,768 allowed turns.
    #[test]
    fn fixture_caller_json_pins_the_literal_turn_and_lifetime_fuel() {
        let policy: super::CallerPolicy = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
        let host = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive).fuel;
        assert_eq!((policy.turn_fuel, host), (2_000_000, 2_000_000));
        assert_eq!((policy.maximum_turns, policy.fuel), (32_768, 65_536_000_000));
        assert_eq!(policy.fuel, host * policy.maximum_turns as u64);
        assert_eq!(super::OriginalFixtureCaller::turn_fuel(), host);
    }

    /// 🚧️ A step whose demand can never fit an untouched turn is refused typed at once, a fitting demand is admitted.
    #[test]
    fn work_step_exceeding_the_whole_turn_grant_is_refused_typed_not_idled() {
        let mut scope = super::OriginalFixtureScope::admit();
        let cx = scope.cx();
        let grant = cx.retained_grant();
        assert!(cx.admit_work_step(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }).unwrap());
        let oversized = semio_framework_value::RetirementDemand { copy_bytes: grant.maximum_copy_bytes + 1, depth: 1, ..Default::default() };
        let refused = cx.admit_work_step(oversized).unwrap_err();
        assert_eq!(refused.kind, semio_framework_value::ValueRefusalKind::WorkLimit);
    }

    /// 🚦️ Every fixture turn is granted exactly one host Interactive turn of fuel, independent of what an earlier call left on the long-lived context, and a zero-progress loop is bounded strictly below the whole turn stock.
    #[test]
    fn fixture_turn_fuel_is_the_host_interactive_budget_and_idle_turns_are_bounded() {
        assert_eq!(super::OriginalFixtureCaller::turn_fuel(), semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive).fuel);
        assert!(super::OriginalFixtureCaller::maximum_idle_turns() > 0 && super::OriginalFixtureCaller::maximum_idle_turns() < super::OriginalFixtureCaller::maximum_turns());
    }
}

#[path="👷️work/🦀️.rs"]
mod original_work_fixture;
pub use original_work_fixture::{OriginalReceiveFailure,OriginalWorkFixture,receive_persistent_fixture,receive_view_fixture};
#[path="⌨️typing-run/🦀️.rs"]
mod typing_run_model;
pub use typing_run_model::{typing_run,TypingRun};
