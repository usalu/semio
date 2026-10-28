//! 🔎️ One original metadata row contributes to the numerical census per admitted turn.
use super::*;
#[derive(Default)]
pub struct EnergyNumericalCensusCursor{phase:u8,index:usize,identifier_bytes:usize,model_bytes:usize,plant_equipment:usize,output:Option<EnergyNumericalCensus>,overflow:bool}
impl EnergyNumericalCensusCursor{
 pub fn demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Self>(),depth:1,..Default::default()}}
 pub fn complete(&self)->bool{self.output.is_some()||self.overflow}
 pub fn census(&self)->Option<EnergyNumericalCensus>{self.output}
 pub fn advance(&mut self,model:&Model,config:&SimulationConfig,grant:RetainedCloneGrant)->RetainedCloneProgress{if self.complete()||!energy_grant_covers(grant,self.demand()){return Default::default()}if self.visit(model,config).is_none(){self.overflow=true;}RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Self>(),..Default::default()}}
 fn visit(&mut self,model:&Model,config:&SimulationConfig)->Option<()>{match self.phase{
  0=>{self.identifier_bytes=model.name.capacity().checked_add(model.version.capacity())?.checked_add(model.name.len())?.checked_add(model.version.len())?;if let Some(weather)=&config.weather{self.identifier_bytes=self.identifier_bytes.checked_add(weather.location.capacity())?.checked_add(weather.location.len())?;self.model_bytes=observed_vector_bytes(&weather.records)?;}self.identifier_bytes=self.identifier_bytes.checked_add(("Facility Heating".len()+"Facility PV".len()).checked_mul(3)?)?;for(key,unit)in[("Electricity","kWh"),("Natural Gas","kWh"),("Energy Use Intensity","kWh/m²"),("Annual Energy Cost","USD"),("LCCA Present Value","USD")]{self.identifier_bytes=self.identifier_bytes.checked_add(key.len())?.checked_add(unit.len())?;}
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.zones)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.spaces)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.surfaces)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.fenestrations)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.materials)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.glazing_materials)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.gas_materials)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.constructions)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.people)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.lighting)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.equipment)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.thermostats)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.humidistats)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.setpoint_managers)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.ideal_loads)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.zone_equipment)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.air_loops)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.plant_loops)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.outdoor_air_systems)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.infiltrations)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.mechanical_ventilations)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.shading_surfaces)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.space_lists)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.thermal_enclosures)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.adjacency_pairs)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.electrical_load_centers)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.pv_systems)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.battery_storage)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.shw_systems)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.solar_thermal_systems)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.refrigeration_systems)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.water_systems)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.faults)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.output_variables)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.sizing_objects)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.daylight_zones)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&model.room_air_models)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&config.schedules.constants)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&config.schedules.daily)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&config.schedules.weekly)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&config.schedules.annual)?)?;
   self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&config.schedules.time_series)?)?;
   if let Some(network)=&model.airflow_network{self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&network.zone_node_ids)?)?.checked_add(observed_vector_bytes(&network.link_ids)?)?;}self.phase=1;},
  1=>{if let Some(owner)=model.zones.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;let n=owner.name.len();let series=("Zone Air Temperature [".len()+"Zone Heating Power [".len()+"Zone Cooling Power [".len()).checked_add(n.checked_mul(3)?)?.checked_add(3)?;let meters=(" Heating".len()+" Cooling".len()+" Fans".len()).checked_add(n.checked_mul(3)?)?;self.identifier_bytes=self.identifier_bytes.checked_add(series.checked_add(meters)?.checked_mul(3)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  2=>{if let Some(owner)=model.spaces.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  3=>{if let Some(owner)=model.surfaces.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.vertices_m)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  4=>{if let Some(owner)=model.fenestrations.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  5=>{if let Some(owner)=model.materials.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  6=>{if let Some(owner)=model.glazing_materials.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  7=>{if let Some(owner)=model.gas_materials.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  8=>{if let Some(owner)=model.constructions.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.layer_material_ids)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  9=>{if let Some(owner)=model.setpoint_managers.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  10=>{if let Some(owner)=model.air_loops.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.terminal_zone_ids)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  11=>{if let Some(owner)=model.plant_loops.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.equipment_ids)?)?;self.plant_equipment=self.plant_equipment.checked_add(owner.equipment_ids.capacity())?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  12=>{if let Some(owner)=model.shading_surfaces.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.vertices_m)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  13=>{if let Some(owner)=model.space_lists.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.space_ids)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  14=>{if let Some(owner)=model.thermal_enclosures.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.zone_ids)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  15=>{if let Some(owner)=model.electrical_load_centers.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.generator_ids)?)?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.pv_ids)?)?;self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.battery_ids)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  16=>{if let Some(owner)=model.output_variables.get(self.index){self.identifier_bytes=self.identifier_bytes.checked_add(owner.name.capacity())?;self.identifier_bytes=self.identifier_bytes.checked_add(owner.key.capacity())?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  17=>{if let Some(owner)=config.schedules.annual.get(self.index){self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.rules)?)?.checked_add(observed_vector_bytes(&owner.holiday_dates)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  18=>{if let Some(owner)=config.schedules.time_series.get(self.index){self.model_bytes=self.model_bytes.checked_add(observed_vector_bytes(&owner.values)?)?;self.index+=1;}else{self.phase+=1;self.index=0;}},
  _=>{self.output=Some(EnergyNumericalCensus::from_observations(model,config,self.plant_equipment,self.identifier_bytes,self.model_bytes)?);},
 }Some(())}
}
