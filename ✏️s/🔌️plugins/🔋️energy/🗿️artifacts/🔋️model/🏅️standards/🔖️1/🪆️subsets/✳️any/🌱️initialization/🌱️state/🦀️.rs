//! 🌱️ Original Job admission retains every authored native Energy genesis field and decoder identity.
use semio_framework_plugin_ownership::original_store_genesis::OriginalStoreInitialSource;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_s_artifact_energy_model_snapshot::{EnergyModelSnapshot,original::genesis::EnergyModelInitialBirth};
use std::mem::size_of;
/// 🌱️ The mounted caller retains every original Energy genesis field until paid transfer or cancellation.
pub struct EnergyNativeInitialSource{binding:OriginalSourceBinding,decoder:(u64,u64),cursor:EnergyModelInitialBirth,body:Option<usize>,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native Energy initial source changed its original caller or destination")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
impl OriginalStoreInitialSource<EnergyModelSnapshot> for EnergyNativeInitialSource{
 fn admit(cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{let d=RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed(2*size_of::<Self>())};let binding=OriginalSourceBinding::borrow(cx);admit_original_construction(cx,output,construction(d),|control|{let decoder=(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(invalid)?);Ok((Box::new(Self{binding,decoder,cursor:EnergyModelInitialBirth::new(),body:None,closing:false}),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))})?;Ok(output.is_some())}
 fn advance(&mut self,cx:&mut StepContext<'_>,output:&mut Option<EnergyModelSnapshot>)->Result<bool,OriginalTurnFailure>{
  if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder)||self.closing||output.is_some(){return Err(invalid().into())}
  let grant=cx.retained_grant();
  if self.cursor.ready(){let mut d=self.cursor.take_demand()?;d.copy_bytes+=size_of::<Option<EnergyModelSnapshot>>();d.depth+=1;admit_original_construction(cx,output,construction(d),|_|{let(value,mut p)=self.cursor.take(grant)?.ok_or_else(invalid)?;p.copied_bytes+=size_of::<Option<EnergyModelSnapshot>>();Ok((value,p))})?;return Ok(output.is_some())}
  let body=self.body.unwrap_or(grant.maximum_copy_bytes);let mut d=self.cursor.demand(body)?;let metadata=if self.body.is_none(){size_of::<Option<usize>>()}else{0};d.copy_bytes+=metadata;d.depth+=1;
  let mut performed=None;admit_original_construction(cx,&mut performed,construction(d),|_|{let step=self.cursor.advance(body,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-metadata,maximum_depth:grant.maximum_depth-1,..grant})?;let mut p=step.progress();if p.copied_items>0{self.body=Some(body);p.copied_bytes+=metadata;}Ok(((),p))})?;Ok(false)
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(fixed(size_of::<bool>()*2))}self.cursor.close_demand(self.body.unwrap_or(body))}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid().into())}let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|grant|{if !self.closing{self.closing=true;self.cursor.begin_close();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}self.cursor.close_step(grant)})?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
 fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.terminal_is_empty()}
 fn boxed_release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),..fixed(size_of::<Option<Box<Self>>>())}}
}


#[cfg(test)]#[path="../🧪️tests/🦀️.rs"]pub(super) mod tests;
