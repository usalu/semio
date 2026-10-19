//! 🧳️ Empty logical collections retain their actual backings until original controlled retirement.
use super::*;
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[derive(Default,semio_framework_value::RetireOwned)]
pub(super) struct EnergyAuthorityRetirement{
 model:Model,
 config:SimulationConfig,
 weather:FixedTable<usize,WeatherRecord>,
 validation:[FixedTable<model::EntityId,()>;4],
 previous_temperatures:Vec<f64>,
 previous_loads:Vec<f64>,
 warmup_convergence:Option<WarmupConvergenceWork>,
 time_series:TimeSeriesTable,
 meters:MeterTable,
 time_series_order:Vec<String>,
 meter_order:Vec<String>,
 zone_temperature_history:Vec<f64>,
 final_summaries:SummaryTables,
 finalization:FinalizationWork,
 result_build:ResultBuildWork,
 close_string_owner:String,
}
impl EnergyAuthorityRetirement{
 fn take(authority:&mut EnergyJobAuthority)->Self{
  Self{
   model:std::mem::take(&mut authority.model),
   config:std::mem::take(&mut authority.config),
   weather:std::mem::take(&mut authority.weather),
   validation:[std::mem::take(&mut authority.validation.zone_ids),std::mem::take(&mut authority.validation.material_ids),std::mem::take(&mut authority.validation.construction_ids),std::mem::take(&mut authority.validation.surface_ids)],
   previous_temperatures:std::mem::take(&mut authority.previous_temperatures),
   previous_loads:std::mem::take(&mut authority.previous_loads),
   warmup_convergence:authority.warmup_convergence.take(),
   time_series:std::mem::take(&mut authority.time_series),
   meters:std::mem::take(&mut authority.meters),
   time_series_order:std::mem::take(&mut authority.time_series_order),
   meter_order:std::mem::take(&mut authority.meter_order),
   zone_temperature_history:std::mem::take(&mut authority.zone_temperature_history),
   final_summaries:std::mem::take(&mut authority.final_summaries),
   finalization:std::mem::take(&mut authority.finalization),
   result_build:std::mem::take(&mut authority.result_build),
   close_string_owner:std::mem::take(&mut authority.close_string_owner),
  }
 }
}
impl EnergyJobAuthority{
 pub(super) fn retained_tail_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(close)=self.closing_tail.as_ref(){return if close.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ControlledRetirement<EnergyAuthorityRetirement>>>(),depth:1,..Default::default()})}else{close.selected_demand(body)}}
  if !self.tail_moved{return Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<ControlledRetirement<EnergyAuthorityRetirement>>()+std::mem::size_of::<EnergyAuthorityRetirement>(),depth:1,..Default::default()})}
  Ok(Default::default())
 }
 pub(super) fn retire_retained_tail(&mut self,grant:RetainedCloneGrant)->Result<InteractiveJobCloseStep,ValueError>{
  let demand=self.retained_tail_demands(grant.maximum_copy_bytes)?;
  if !energy_grant_covers(grant,demand){return Ok(energy_idle())}
  if let Some(close)=self.closing_tail.as_mut(){
   if close.terminal_is_empty(){self.closing_tail=None;return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}})}
   return close.step(grant).map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()})
  }
  if !self.tail_moved{
   if !<EnergyAuthorityRetirement as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"Energy retained terminal owners have no controlled retirement"))}
   let owner=EnergyAuthorityRetirement::take(self);
   self.closing_tail=Some(ControlledRetirement::new(owner).unwrap_or_else(|_|unreachable!("validated concrete Energy tail supports controlled retirement")));
   self.tail_moved=true;
   return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}})
  }
  Ok(energy_done())
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
