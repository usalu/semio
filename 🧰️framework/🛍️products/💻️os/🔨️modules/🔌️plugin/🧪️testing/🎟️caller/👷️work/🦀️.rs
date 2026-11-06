//! 👷️ Pure Work laws retain one authentic physical source for every close turn and root retirement.
use semio_framework_job::{OriginalJobFixtureCaller,OriginalTurnFailure,StepContext,InteractiveJobCloseStep,OperationId,Generation};
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::{RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value_native_owned::{NativeReceive,OriginalReceivingPrimary};
use semio_framework_value_native_owned::owned::NativeReceivingOriginal;
/// 🎟️ One finite original fixture stock is held from initial work through terminal close.
pub struct OriginalWorkFixture{caller:OriginalJobFixtureCaller,decoder:semio_framework_job::OriginalJobDecodeOwner,source_stock:RetainedCloneGrant}
impl OriginalWorkFixture{
 pub const fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096}}
 pub fn new()->Self{Self::with_identity(OperationId(991),Generation(17))}
 pub fn with_identity(operation:OperationId,generation:Generation)->Self{Self::with_source(operation,generation,Self::stock(),||Some(1),u64::MAX,1<<20)}
 /// 🏛️ Admits actual finite caller roots from independently authored fixture policy before any job step.
 pub fn with_source(operation:OperationId,generation:Generation,source_stock:RetainedCloneGrant,clock:fn()->Option<u64>,deadline:u64,fuel:u64)->Self{Self{caller:OriginalJobFixtureCaller::admit(operation,generation,source_stock,clock,deadline,fuel),decoder:semio_framework_job::OriginalJobDecodeOwner::empty(),source_stock}}
 /// 🛑 Borrows the actual initiating cancellation root without creating an alias.
 pub fn cancel(&self)->&semio_framework_job::CancelToken{self.caller.cancel()}
 pub fn turn<T>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{self.caller.turn_with_decoder(grant,&mut self.decoder,run)}
 /// ⏱️ Narrows one authentic work loan without replacing its physical caller or spending future turn fuel.
 pub fn work_turn<T>(&mut self,grant:RetainedCloneGrant,fuel:u64,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{self.turn(grant,|cx|{let mut value=None;cx.with_original_work_loan(fuel,cx.deadline_us(),|cx|{value=Some(run(cx)?);Ok(())})?;Ok(value.unwrap())})}
 pub fn close<A:semio_plugin_decl_app::ArtifactApp,W:semio_framework_plugin_retained_command::ArtifactCommandWork<A>>(&mut self,work:&mut W,grant:RetainedCloneGrant)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.turn(grant,|cx|cx.with_original_retirement(|cx|{
  use semio_plugin_ownership::bounded_ownership::{original_lifecycle_step,PluginLifecycleStep};
  let demand=semio_framework_value::RetirementDemand{copy_bytes:work.next_close_copy_byte_demand()?,capacity_bytes:work.next_close_capacity_byte_demand(cx.retained_grant().maximum_copy_bytes)?,release_bytes:work.next_close_release_byte_demand()?,depth:work.next_close_depth_demand()?};
  let loan=cx.retained_grant();let step=original_lifecycle_step(cx,loan,demand,|same,_|{let step=match work.close_step(same)?{InteractiveJobCloseStep::Pending{progress}=>PluginLifecycleStep::Progress(progress),InteractiveJobCloseStep::Complete{progress}=>PluginLifecycleStep::Complete(progress),InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::Blocked{reason:"original Work fixture awaits retained child custody"}};Ok((step,work.terminal_is_empty()))})?;
  Ok(match step{PluginLifecycleStep::Progress(progress)=>InteractiveJobCloseStep::Pending{progress},PluginLifecycleStep::Complete(progress)=>InteractiveJobCloseStep::Complete{progress},PluginLifecycleStep::Blocked{..}|PluginLifecycleStep::AwaitingInput{..}=>InteractiveJobCloseStep::Blocked})
 }))}
 /// ♻️ Retires an original fixture root through this caller's funded physical source.
 pub fn retire_owned<T:semio_framework_value::retirement::RetireOwned>(&mut self,value:T,grant:RetainedCloneGrant){
  let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::retain(value);
  for _ in 0..self.source_stock.maximum_items {if owner.terminal_is_empty(){break}self.turn(grant,|cx|{let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|issued|{let result=owner.step(issued);RetainedWorkOutcome {result,work:owner.last_turn_work()}})?;Ok(())}).expect("original fixture root retirement");}
  assert!(owner.terminal_is_empty(),"finite original fixture source must fund root retirement");
 }
 /// 🛬️ Receives the caller's genuine original input with the same decoder, grants and funded cleanup.
 pub fn receive_native<I,T>(&mut self,input:I,grant:RetainedCloneGrant)->T where I:semio_framework_value::retirement::RetireOwned+Sync,T:NativeReceive<I>{
  self.try_receive_native(input,grant).expect("original native fixture receiving")
 }
 /// 🚦️ Returns the complete original receiving refusal after funding every partially constructed owner.
 pub fn try_receive_native<I,T>(&mut self,input:I,grant:RetainedCloneGrant)->Result<T,OriginalTurnFailure> where I:semio_framework_value::retirement::RetireOwned+Sync,T:NativeReceive<I>{
  let mut admission=semio_framework_value::retained_clone::RetainedCloneSource::<I>::admission_owned(input,());let mut source=None;
  for _ in 0..self.source_stock.maximum_items{if source.is_some(){break}self.turn(grant,|cx|{let turn=if admission.phase()=="ready"{admission.take_output(&mut source,cx.retained_grant())}else{admission.advance(cx.retained_grant())};semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).expect("original native fixture source admission");}
  assert!(admission.terminal_is_empty());let mut owner=NativeReceivingOriginal::<I,T>::new(source.expect("original fixture source is admitted"));
  let mut failure=None;for _ in 0..self.source_stock.maximum_items{if owner.output_ready(){break}if let Err(error)=self.turn(grant,|cx|{
   let issued=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control|{let turn=owner.advance(control,issued.maximum_copy_bytes,issued);work=turn.work;turn.result});
   let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let receipt=cx.consume_receiving_retained(issued,progress);cx.consume_fuel(work);
   match result{Ok(_)=>{receipt?;Ok(())},Err(primary)=>Err(OriginalTurnFailure {effects:primary.retained_progress(),primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}
  }){failure=Some(error);break}}
  let mut output=None;if failure.is_none(){assert!(owner.output_ready());if let Err(error)=self.turn(grant,|cx|{let turn=owner.take(cx.retained_grant());let result=turn.result.map(|value|{let(value,progress)=value.expect("ready fixture output has its quoted grant");output=Some(value);RetainedCloneStep::Progress(progress)});semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome {result,work:turn.work})?;Ok(())}){failure=Some(error);}}
  owner.begin_close();for _ in 0..self.source_stock.maximum_items{if owner.terminal_is_empty(){break}self.turn(grant,|cx|{semio_framework_job::close_original_retained_step(cx,owner.close_demands(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.close_step(grant))?;Ok(())}).expect("original native fixture cleanup");}assert!(owner.terminal_is_empty());
  if let Some(error)=failure{if let Some(value)=output{self.retire_owned(value,grant);}Err(error)}else{Ok(output.expect("original fixture output transferred"))}
 }
 /// 🧳️ The real typed instance closes under this caller's existing source, preserving both borrowed and original failure owners.
 pub fn close_instance_owner_turn<O:semio_plugin_decl_tool_contract::ArtifactInstanceOperationOwner+'static>(&mut self,handle:&semio_plugin_decl_tool_contract::ArtifactInstanceOperationOwnerHandle,loan:RetainedCloneGrant)->Result<bool,OriginalTurnFailure>{
  handle.with_mut::<O,_>(|owner|Ok(self.turn(loan,|cx|{let demand=owner.retirement_demands(loan.maximum_copy_bytes)?;semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,grant|{let step=owner.close_step(same,grant)?;Ok((step,owner.terminal_is_empty()))})?;Ok(owner.terminal_is_empty())}))).map_err(OriginalTurnFailure::from)?
 }
 pub fn finish(mut self){for _ in 0..self.source_stock.maximum_items{if self.decoder.terminal_is_empty(){break}self.caller.turn(self.source_stock,|cx|cx.with_original_retirement(|cx|{let demand=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.decoder.close_step(grant))?;Ok(())})).expect("same original fixture decoder close");}assert!(self.decoder.terminal_is_empty(),"finite original fixture source must fund its decoder close");self.caller.retire(self.source_stock);}
}

/// 📋️ Test fixtures supply finite original source policy before receiving their ordered persistent rows.
pub fn receive_persistent_fixture<T>(input:semio_framework_value::DslValue,source_stock:RetainedCloneGrant)->semio_framework_value::persistent::PersistentList<T> where T:NativeReceive<semio_framework_value::DslValue>+Sync{
 let mut fixture=OriginalWorkFixture::with_source(OperationId(992),Generation(17),source_stock,||Some(1),u64::MAX,source_stock.maximum_items as u64*16);
 let grant=RetainedCloneGrant {maximum_items:source_stock.maximum_items.min(64),maximum_copy_bytes:source_stock.maximum_copy_bytes.min(32768),maximum_capacity_bytes:source_stock.maximum_capacity_bytes.min(1<<20),maximum_release_bytes:source_stock.maximum_release_bytes.min(1<<20),maximum_depth:source_stock.maximum_depth};
 let output=fixture.receive_native(input,grant);fixture.finish();output
}

/// 🪟️ A fixture donates its actual view body through native Arc admission before sharing metadata.
pub fn receive_view_fixture(view:semio_framework::ViewModel,source_stock:RetainedCloneGrant)->std::sync::Arc<semio_framework::ViewModel>{
 let mut fixture=OriginalWorkFixture::with_source(OperationId(993),Generation(17),source_stock,||Some(1),u64::MAX,source_stock.maximum_items as u64*16);
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:source_stock.maximum_copy_bytes.min(32768),maximum_capacity_bytes:source_stock.maximum_capacity_bytes.min(1<<20),maximum_release_bytes:source_stock.maximum_release_bytes.min(1<<20),maximum_depth:source_stock.maximum_depth};
  let mut original=Some(view);let mut output=None;let mut frame=semio_framework_value::native_decoding::NativeSharedAdmission::default();
  for _ in 0..source_stock.maximum_items{if output.is_some(){break}fixture.turn(grant,|cx|semio_plugin_decl_emission::ActionMeta::admit_view_root(&mut original,&mut output,&mut frame,cx)).expect("original fixture view admission");}
 assert!(original.is_none(),"finite original fixture source must fund its view root");fixture.finish();output.expect("original fixture view transferred")
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod receiving_tests;
