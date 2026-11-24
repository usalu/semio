//! 🎟️ Independently authored fixture authority survives original settlement and terminal closure.
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError};

#[derive(serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct CallerPolicy{source_kind:String,operation:u64,generation:u64,fuel:u64,turn_fuel:u64,clock_us:u64,deadline_us:u64,maximum_turns:usize,maximum_close_turns:usize,maximum_idle_turns:usize,grant:RetainedCloneGrant,retirement_loan:RetainedCloneGrant}

thread_local!{static FIXTURE_DEADLINE_AFTER_READS:std::cell::Cell<Option<(u64,u64)>>=const{std::cell::Cell::new(None)};}
fn fixture_now_us()->Option<u64>{FIXTURE_DEADLINE_AFTER_READS.with(|armed|match armed.get(){Some((0,deadline))=>Some(deadline),Some((reads,deadline))=>{armed.set(Some((reads-1,deadline)));Some(1000)},None=>Some(1000)})}

/// 🪙️ One original ledger, cancellation root, cursor and cumulative recipient own the fixture's whole financed body; its roots are born and closed through the production effect-threaded admissions of the framework fixture caller. Its Source return table and publisher are born up front, as the production host births them before a turn that requested them: a lent long-lived context has no host to serve them between turns, so a body that needs them would otherwise wait on its own request forever.
pub struct OriginalFixtureCaller{policy:CallerPolicy,caller:OriginalJobFixtureCaller,decoder:semio_framework_job::OriginalJobDecodeOwner,lent:bool}
impl OriginalFixtureCaller{
 /// 🚪️ Creates original authority from closed test input before semantic work begins.
 pub fn admit()->Result<Self,ValueError>{
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).expect("closed original fixture caller policy");assert_eq!(policy.source_kind,"original-parent");assert_eq!(policy.clock_us,fixture_now_us().unwrap());
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(policy.operation),Generation(policy.generation),policy.grant,fixture_now_us,policy.deadline_us,policy.fuel);caller.birth_returns(policy.grant);
  Ok(Self{policy,caller,decoder:semio_framework_job::OriginalJobDecodeOwner::empty(),lent:false})
 }
 /// 🪢️ Lends one original context; another call cannot recreate its original fuel or ceiling.
 pub fn context(&mut self)->StepContext<'_>{assert!(!self.lent,"original fixture context can only be issued once");self.lent=true;self.caller.lend_refreshed_context().with_original_decoder(&mut self.decoder)}
 /// ⛽️ The fuel one host Interactive turn is granted: every fixture turn starts from it, never from what an earlier call left on the long-lived context.
 pub fn turn_fuel()->u64{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().turn_fuel}
 /// 🎫️ The stock one initiating native event is issued: the host births one per event, so every fixture dispatch is issued this same authored grant afresh instead of drawing on one fixture-lifetime stock.
 pub fn event_stock()->RetainedCloneGrant{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().grant}
 /// 💤️ Consecutive turns that may settle nothing before a loop fails typed.
 pub fn maximum_idle_turns()->usize{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().maximum_idle_turns}
 /// 📏️ Bounds caller loops without deriving finance from an owner's descriptive demand.
 pub fn maximum_turns()->usize{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().maximum_turns}
 /// 🛟️ Bounds app retirement separately from action settlement under the same original stock.
 pub fn maximum_close_turns()->usize{serde_json::from_str::<CallerPolicy>(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap().maximum_close_turns}
 /// 🛬️ Retires the body decoder against the same already-issued caller stock before its three parent roots close.
 pub fn retire_decoder(&mut self,decoder:&mut semio_framework_job::OriginalJobDecodeOwner,fuel:u64)->Result<u64,OriginalTurnFailure>{
  self.caller.settle_lent_fuel(fuel);
  for _ in 0..self.policy.maximum_turns {
   if decoder.terminal_is_empty(){return Ok(self.caller.fuel_remaining())}
   let (quote,work)=semio_framework_job::original_decoder_teardown_quote(decoder)?;
   let funded=self.caller.within_stock(quote);
   if funded!=quote||self.caller.fuel_remaining()<work {
    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture stock cannot fund its quoted decoder retirement").into());
   }
   self.caller.turn(funded,|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())}))?;
  }
  Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture decoder exceeds its supplied close turn bound").into())
 }
 /// 🧹️ Returns the original caller ledger and root with its remaining authored authority after all child payloads close.
 pub fn retire(&mut self,fuel:u64)->Result<RetainedCloneProgress,OriginalTurnFailure>{
  self.caller.settle_lent_fuel(fuel);let mut decoder=std::mem::replace(&mut self.decoder,semio_framework_job::OriginalJobDecodeOwner::empty());let retired=self.retire_decoder(&mut decoder,fuel);self.decoder=decoder;retired?;
  let loan=self.policy.retirement_loan;
  for _ in 0..self.policy.maximum_turns{
   if self.caller.terminal_is_empty(){let receipt=self.caller.cumulative_receipt();assert!(receipt.fits(self.caller.stock()),"original fixture caller exceeds the stock of its last initiating event");return Ok(receipt)}
   let step=self.caller.retire_step(loan)?;if step.progress()==Default::default(){std::thread::yield_now()}
  }
  Err(OriginalTurnFailure::from(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture caller retirement exceeded its supplied turn bound")))
 }
}
thread_local!{static UI_ARENA:std::cell::RefCell<(usize,Option<semio_ui_contract_values::testkit::UiArenaFixture>)>=const{std::cell::RefCell::new((0,None))};}

/// 🏟️ Holds the process-wide UI value arena for this thread while a fixture renders: the first holder opens the real arena Source and serializes against every other test thread, nested holders on the same thread share it, and the last one returns it. See [`semio_ui_contract_values::testkit::UiArenaFixture`].
pub struct FixtureUiArena(());
impl FixtureUiArena{
 /// 🚪️ Opens the arena for this thread, or joins the one this thread already holds.
 pub fn admit()->Self{
  let held=UI_ARENA.with(|slot|{let mut slot=slot.borrow_mut();slot.0+=1;slot.0>1});
  if !held{let arena=semio_ui_contract_values::testkit::UiArenaFixture::new();UI_ARENA.with(|slot|slot.borrow_mut().1=Some(arena))}
  Self(())
 }
}
impl Drop for FixtureUiArena{fn drop(&mut self){let last=UI_ARENA.with(|slot|{let mut slot=slot.borrow_mut();slot.0-=1;if slot.0==0{slot.1.take()}else{None}});drop(last)}}

/// 🪙️ One fixture-lifetime original caller: owns the caller and its single lent context so a fixture app threads the same `cx` through every call and pays the caller's terminal retirement once, at an explicit [`OriginalFixtureScope::retire`] or else when the scope drops (a scope dropped while panicking pays nothing).
pub struct OriginalFixtureScope{context:Option<StepContext<'static>>,caller:*mut OriginalFixtureCaller}
impl OriginalFixtureScope{
 /// 🚪️ Admits one original caller and lends its sole context for the whole life of the scope.
 pub fn admit()->Self{let caller=Box::into_raw(Box::new(OriginalFixtureCaller::admit().expect("closed original fixture scope caller")));let context=unsafe{(*caller).context()};Self{context:Some(context),caller}}
 /// 🪢️ The one original context every call of the fixture's financed body shares; every call begins a new host turn, so the receiving scope the previous call left open is settled first.
 pub fn cx(&mut self)->&mut StepContext<'static>{let cx=self.context.as_mut().expect("original fixture scope context is lent until its retirement");cx.settle_receiving_scope().expect("the previous fixture call settles its original receiving scope");cx}
 /// 🧯️ Pays the funded retirement of one refusal a fixture call returned, so asserting on a refusal never drops an unretired [`semio_framework_job::OriginalTurnFailure`]; each turn runs under the scope's one original context, bounded by [`OriginalFixtureCaller::maximum_close_turns`].
 pub fn retire_failure(&mut self,failure:OriginalTurnFailure){
  use semio_framework_value::{retained_clone::RetainedCloneClose,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};
  let mut state=(Some(failure),RetainedCloneClose::default());
  let retired=within_close_turns(OriginalFixtureCaller::maximum_close_turns(),&mut state,|state:&(Option<OriginalTurnFailure>,RetainedCloneClose)|state.0.is_none()&&state.1.is_empty(),|state:&mut (Option<OriginalTurnFailure>,RetainedCloneClose)|{
   let (failure,retirement)=state;let cx=self.cx();let body=cx.retained_grant().maximum_copy_bytes;
   let demand=retirement.next_owner_demands::<OriginalTurnFailure>(failure.is_some(),&None,body).expect("original fixture failure retirement demand");
   semio_framework_job::close_original_effect_step(cx,demand,|effect|{if !retirement.is_empty(){return retirement.step_granted(body,effect).map_err(semio_framework_job::OriginalTurnPrimary::from)}let turn=retirement.begin_granted(&mut *failure,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(semio_framework_job::OriginalTurnPrimary::from).map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work})}).expect("paid original fixture failure retirement");
  });
  assert!(retired,"original fixture failure exceeds its supplied close turn bound")
 }
 /// ⏰️ Runs one fixture call whose turn deadline runs out after `reads` real clock reads, then restores the clock the context observed before the call, so only this call sees the deadline pass.
 pub fn with_deadline_after_clock_reads<T>(&mut self,reads:u64,run:impl FnOnce(&mut StepContext<'static>)->T)->T{
  let cx=self.cx();let parked=cx.park_state();FIXTURE_DEADLINE_AFTER_READS.with(|armed|armed.set(Some((reads,cx.deadline_us()))));
  let result=run(cx);
  FIXTURE_DEADLINE_AFTER_READS.with(|armed|armed.set(None));cx.park_restore(parked);result
 }
 /// 🧹️ Returns the context, then pays the caller's terminal retirement from the fuel the body left.
 pub fn retire(&mut self)->RetainedCloneProgress{
  let context=self.context.take().expect("original fixture scope retires once");let fuel=context.fuel_remaining();drop(context);
  let mut caller=unsafe{Box::from_raw(self.caller)};self.caller=std::ptr::null_mut();
  caller.retire(fuel).expect("paid original fixture scope retirement")
 }
}
/// 🔁️ Runs at most `turns` close steps over `state`, checking its terminal witness before the first step and after every step, so a close that finishes on its last allowed turn is terminal.
fn within_close_turns<S>(turns:usize,state:&mut S,terminal:impl Fn(&S)->bool,mut step:impl FnMut(&mut S))->bool{
 for _ in 0..turns{if terminal(state){return true}step(state)}
 terminal(state)
}
impl Drop for OriginalFixtureScope{fn drop(&mut self){if self.caller.is_null(){return}if std::thread::panicking(){drop(self.context.take());drop(unsafe{Box::from_raw(self.caller)})}else{self.retire();}}}

#[cfg(test)]
mod original_physical_laws{
 use super::*;
 #[test]
 fn a_close_finishing_on_its_last_allowed_turn_is_terminal(){
  let mut steps=0usize;assert!(within_close_turns(3,&mut steps,|steps|*steps==3,|steps|*steps+=1));assert_eq!(steps,3);
  let mut steps=0usize;assert!(!within_close_turns(3,&mut steps,|steps|*steps==4,|steps|*steps+=1));assert_eq!(steps,3);
  let mut steps=0usize;assert!(within_close_turns(3,&mut steps,|steps|*steps==0,|steps|*steps+=1));assert_eq!(steps,0);
  let mut steps=0usize;assert!(!within_close_turns(0,&mut steps,|steps|*steps==1,|steps|*steps+=1));assert_eq!(steps,0);
 }
 fn fixture_remaining(stock:RetainedCloneGrant,spent:RetainedCloneProgress)->RetainedCloneGrant{assert!(spent.fits(stock),"original fixture source admissions exceed its initiating stock");RetainedCloneGrant{maximum_items:stock.maximum_items-spent.copied_items,maximum_copy_bytes:stock.maximum_copy_bytes-spent.copied_bytes,maximum_capacity_bytes:stock.maximum_capacity_bytes-spent.retained_capacity_bytes,maximum_release_bytes:stock.maximum_release_bytes-spent.released_bytes,..stock}}

 /// 🪙️ The sole body context starts after its three original root admissions consumed CPU and stock.
 #[test]
 fn original_plugin_fixture_caller_charges_root_finance_before_body_and_closes_same_stock(){
  let policy:CallerPolicy=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
  let mut caller=OriginalFixtureCaller::admit().unwrap();let roots=caller.caller.cumulative_receipt();let birth_fuel=caller.caller.fuel_remaining();assert!(roots.copied_items>0&&birth_fuel<policy.fuel);assert!(roots.fits(policy.grant));
  let mut cx=caller.context();let refreshed=cx.retained_progress();assert!(cx.fuel_remaining()<=birth_fuel);assert!(refreshed.copied_items>=roots.copied_items&&refreshed.copied_bytes>=roots.copied_bytes&&refreshed.fits(policy.grant));assert_eq!(cx.retained_grant(),fixture_remaining(policy.grant,refreshed));
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
