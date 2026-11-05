//! 🔗️ Immutable original view roots make routing aliases bounded independently of the view body.
use crate::ActionMeta;
use semio_framework::ViewModel;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction};
use semio_framework_value::{RetainedCloneProgress,RetirementDemand};
use std::{sync::Arc,mem::size_of};
#[path="routing.rs"]
pub mod routing;
#[path="delivery.rs"]
pub mod delivery;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
impl ActionMeta{
 /// 🌱️ The actual decoded view moves into its original native Arc receiving turn without cloning its body.
 pub fn admit_view_root(original:&mut Option<ViewModel>,output:&mut Option<Arc<ViewModel>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Ok(true)}if original.is_none(){return Ok(true)}let grant=cx.retained_grant();let mut work=0;let result=cx.with_native_receiving::<_,semio_framework_value_native_owned::OriginalReceivingPrimary>(|native|{let turn=native.receive_arc(original,output);work=turn.work;turn.result});let progress=result.as_ref().copied().unwrap_or_else(|error|error.retained_progress());let receipt=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(work);match result{Ok(_)=>{receipt?;Ok(output.is_some())},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None})}
 }
 /// 📏️ An actor and immutable view alias own only their concrete typed metadata headers.
 pub fn routing_alias_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),depth:1,..Default::default()}}
 /// 🪪️ Routing retains the actual actor, instance and admitted view under this original construction grant.
 pub fn admit_routing_alias(&self,output:&mut Option<Self>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=Self::routing_alias_demands();admit_original_construction(cx,output,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:0,release_bytes:0,depth:d.depth},|_|{let mut meta=self.clone();meta.expected_document_revision=None;semio_framework_value::RetainedWorkOutcome{result:Ok((meta,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})}
}
