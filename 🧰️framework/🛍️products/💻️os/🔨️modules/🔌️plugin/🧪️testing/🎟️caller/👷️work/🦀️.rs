//! 👷️ Pure Work laws retain one authentic physical source for every close turn and root retirement.
use semio_framework_job::{OriginalJobFixtureCaller,OriginalTurnFailure,StepContext,InteractiveJobCloseStep,OperationId,Generation};
use semio_framework_value::RetainedCloneGrant;
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
 pub fn close<A:semio_framework_plugin_decl_app::ArtifactApp,W:semio_framework_plugin_retained_command::ArtifactCommandWork<A>>(&mut self,work:&mut W,grant:RetainedCloneGrant)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.turn(grant,|cx|work.close_step(cx))}
 /// 🧳️ The real typed instance closes under this caller's existing source, preserving both borrowed and original failure owners.
 pub fn close_instance_owner_turn<O:semio_framework_plugin_decl_tool_contract::ArtifactInstanceOperationOwner+'static>(&mut self,handle:&semio_framework_plugin_decl_tool_contract::ArtifactInstanceOperationOwnerHandle,loan:RetainedCloneGrant)->Result<bool,OriginalTurnFailure>{
  handle.with_mut::<O,_>(|owner|Ok(self.turn(loan,|cx|{let demand=owner.retirement_demands(loan.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,grant|{let step=owner.close_step(same,grant)?;Ok((step,owner.terminal_is_empty()))})?;Ok(owner.terminal_is_empty())}))).map_err(OriginalTurnFailure::from)?
 }
 pub fn finish(mut self){for _ in 0..self.source_stock.maximum_items{if self.decoder.terminal_is_empty(){break}self.caller.turn(self.source_stock,|cx|cx.with_original_retirement(|cx|{let demand=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.decoder.close_step(grant))?;Ok(())})).expect("same original fixture decoder close");}assert!(self.decoder.terminal_is_empty(),"finite original fixture source must fund its decoder close");self.caller.retire(self.source_stock);}
}
