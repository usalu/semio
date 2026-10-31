//! 👁️ Borrowed native model fields preserve the sole immutable page and row authorities.
use super::*;
/// 🪟️ A fixed set of references gives consumers flat model fields without building a numerical mirror.
pub struct EnergyModelView<'a>{
 pub name:&'a String,
 pub version:&'a String,
 pub site:&'a StateSite,
 pub ground_temperature:&'a StateGroundTemperatureConfig,
 pub run_period:&'a crate::calendar::RunPeriod,
 pub zones:&'a PersistentList<StateZone>,
 pub spaces:&'a PersistentList<StateSpace>,
 pub surfaces:&'a PersistentList<StateSurface>,
 pub fenestrations:&'a PersistentList<StateFenestration>,
 pub shading_surfaces:&'a PersistentList<StateShadingSurface>,
 pub space_lists:&'a PersistentList<StateSpaceList>,
 pub thermal_enclosures:&'a PersistentList<StateThermalEnclosure>,
 pub adjacency_pairs:&'a PersistentList<StateAdjacencyPair>,
 pub materials:&'a PersistentList<StateMaterial>,
 pub glazing_materials:&'a PersistentList<StateGlazingMaterial>,
 pub gas_materials:&'a PersistentList<StateGasMaterial>,
 pub constructions:&'a PersistentList<StateConstruction>,
 pub people:&'a PersistentList<StatePeopleGain>,
 pub lighting:&'a PersistentList<StateLightingGain>,
 pub equipment:&'a PersistentList<StateEquipmentGain>,
 pub infiltrations:&'a PersistentList<StateInfiltration>,
 pub mechanical_ventilations:&'a PersistentList<StateMechanicalVentilation>,
 pub daylight_zones:&'a PersistentList<StateDaylightZoneConfig>,
 pub room_air_models:&'a PersistentList<StateRoomAirModelAssignment>,
 pub thermostats:&'a PersistentList<StateThermostat>,
 pub humidistats:&'a PersistentList<StateHumidistat>,
 pub setpoint_managers:&'a PersistentList<StateSetpointManager>,
 pub ideal_loads:&'a PersistentList<StateIdealLoadsSystem>,
 pub zone_equipment:&'a PersistentList<StateZoneEquipmentAssignment>,
 pub air_loops:&'a PersistentList<StateModelAirLoop>,
 pub plant_loops:&'a PersistentList<StatePlantLoopConfig>,
 pub outdoor_air_systems:&'a PersistentList<StateOutdoorAirSystem>,
 pub airflow_network:Option<&'a StateAirflowNetworkDefinition>,
 pub electrical_load_centers:&'a PersistentList<StateElectricalLoadCenter>,
 pub pv_systems:&'a PersistentList<StatePvSystemAssignment>,
 pub battery_storage:&'a PersistentList<StateBatteryAssignment>,
 pub shw_systems:&'a PersistentList<StateShwSystemConfig>,
 pub solar_thermal_systems:&'a PersistentList<StateSolarThermalConfig>,
 pub refrigeration_systems:&'a PersistentList<StateRefrigerationConfig>,
 pub water_systems:&'a PersistentList<StateWaterSystemConfig>,
 pub faults:&'a PersistentList<StateFaultDefinition>,
 pub output_variables:&'a PersistentList<StateOutputVariableSpec>,
 pub sizing_objects:&'a PersistentList<StateSizingObject>,
 pub schedules:&'a StateScheduleSet,
}
impl EnergyModelState{
 /// 🔎️ Borrows the original branch fields without allocating or duplicating their shared sources.
 pub fn read(&self)->EnergyModelView<'_>{let state=self;EnergyModelView{
 name:state.identity.name.get(),
 version:state.identity.version.get(),
 site:state.identity.site.get(),
 ground_temperature:state.identity.ground_temperature.get(),
 run_period:state.identity.run_period.get(),
 zones:&state.geometry.zones,
 spaces:&state.geometry.spaces,
 surfaces:&state.geometry.surfaces,
 fenestrations:&state.geometry.fenestrations,
 shading_surfaces:&state.geometry.shading_surfaces,
 space_lists:&state.geometry.space_lists,
 thermal_enclosures:&state.geometry.thermal_enclosures,
 adjacency_pairs:&state.geometry.adjacency_pairs,
 materials:&state.materials.materials,
 glazing_materials:&state.materials.glazing_materials,
 gas_materials:&state.materials.gas_materials,
 constructions:&state.materials.constructions,
 people:&state.loads.people,
 lighting:&state.loads.lighting,
 equipment:&state.loads.equipment,
 infiltrations:&state.loads.infiltrations,
 mechanical_ventilations:&state.loads.mechanical_ventilations,
 daylight_zones:&state.loads.daylight_zones,
 room_air_models:&state.loads.room_air_models,
 thermostats:&state.controls.thermostats,
 humidistats:&state.controls.humidistats,
 setpoint_managers:&state.controls.setpoint_managers,
 ideal_loads:&state.controls.ideal_loads,
 zone_equipment:&state.controls.zone_equipment,
 air_loops:&state.systems.air_loops,
 plant_loops:&state.systems.plant_loops,
 outdoor_air_systems:&state.systems.outdoor_air_systems,
 airflow_network:state.systems.airflow_network.as_ref().map(|source|source.get()),
 electrical_load_centers:&state.systems.electrical_load_centers,
 pv_systems:&state.generation.pv_systems,
 battery_storage:&state.generation.battery_storage,
 shw_systems:&state.generation.shw_systems,
 solar_thermal_systems:&state.generation.solar_thermal_systems,
 refrigeration_systems:&state.generation.refrigeration_systems,
 water_systems:&state.generation.water_systems,
 faults:&state.generation.faults,
 output_variables:&state.outputs.output_variables,
 sizing_objects:&state.outputs.sizing_objects,
 schedules:state.outputs.schedules.get(),
 }}
}


impl semio_s_plugin_energy_engine_scene::EnergySceneModel for EnergyModelView<'_>{
 fn name(&self)->&str{self.name}
 fn surfaces(&self)->impl Iterator<Item=semio_s_plugin_energy_engine_scene::EnergySceneSurface<'_>>{self.surfaces.iter().map(|row|semio_s_plugin_energy_engine_scene::EnergySceneSurface{id:row.id,zone_id:row.zone_id,name:&row.name,class:*row.class.get(),vertices_m:&row.vertices_m})}
 fn fenestrations(&self)->impl Iterator<Item=semio_s_plugin_energy_engine_scene::EnergySceneWindow<'_>>{self.fenestrations.iter().map(|row|semio_s_plugin_energy_engine_scene::EnergySceneWindow{id:row.id,surface_id:row.surface_id,name:&row.name,area_m2:row.area_m2,height_m:row.height_m,sill_height_m:row.sill_height_m,vertices_m:&row.vertices_m})}
 fn shading_surfaces(&self)->impl Iterator<Item=semio_s_plugin_energy_engine_scene::EnergySceneShading<'_>>{self.shading_surfaces.iter().map(|row|semio_s_plugin_energy_engine_scene::EnergySceneShading{id:row.id,name:&row.name,vertices_m:&row.vertices_m})}
 fn zones(&self)->impl Iterator<Item=semio_s_plugin_energy_engine_scene::EnergySceneZone<'_>>{self.zones.iter().map(|row|semio_s_plugin_energy_engine_scene::EnergySceneZone{id:row.id,name:&row.name})}
}

/// 🚨️ Latches the first row that failed to project, so a consumer surfaces it as one typed fault instead of treating the row as absent.
#[derive(Default)]
pub struct ProjectionFault(std::cell::RefCell<Option<semio_framework_value::ValueError>>);
impl ProjectionFault{
 fn record(&self,error:semio_framework_value::ValueError){let mut slot=self.0.borrow_mut();if slot.is_none(){*slot=Some(error);}}
 /// 🧾️ The first latched projection failure, if any row failed.
 pub fn finish(&self)->Result<(),semio_framework_value::ValueError>{match self.0.borrow_mut().take(){Some(error)=>Err(error),None=>Ok(())}}
}

/// 📋️ One native list read as engine rows: every access projects exactly the rows it touches and never the whole list.
pub struct Rows<'a,S:semio_framework_value::retirement::RetireOwned+semio_framework_value::ToValue+Send+Sync+'static,E>{list:&'a PersistentList<S>,fault:&'a ProjectionFault,engine:std::marker::PhantomData<fn()->E>}
impl<'a,S:semio_framework_value::retirement::RetireOwned+semio_framework_value::ToValue+Send+Sync+'static,E:semio_framework_value::FromValue+'a> Rows<'a,S,E>{
 fn over(list:&'a PersistentList<S>,fault:&'a ProjectionFault)->Self{Self{list,fault,engine:std::marker::PhantomData}}
 fn project(fault:&ProjectionFault,row:&S)->Option<E>{match E::from_value(row.to_value()){Ok(value)=>Some(value),Err(error)=>{fault.record(error);None}}}
 /// 🔁️ Lazily projects the rows in list order.
 pub fn iter(&self)->impl Iterator<Item=E>+'a{let fault=self.fault;self.list.iter().filter_map(move |row|Self::project(fault,row))}
 /// 🗂️ Projects exactly the row at `index`.
 pub fn get(&self,index:usize)->Option<E>{self.list.get(index).and_then(|row|Self::project(self.fault,row))}
 /// 📍️ The positions of the rows a native predicate keeps, without projecting any row.
 pub fn positions(&self,keep:impl Fn(&S)->bool)->Vec<usize>{self.list.iter().enumerate().filter(|(_,row)|keep(row)).map(|(index,_)|index).collect()}
 /// ➖️ Whether any native row satisfies the predicate, without projecting any row.
 pub fn holds(&self,keep:impl Fn(&S)->bool)->bool{self.list.iter().any(keep)}
 /// 🔭️ The first row a native predicate keeps, projecting only that row.
 pub fn find(&self,keep:impl Fn(&S)->bool)->Option<E>{self.list.iter().find(|row|keep(row)).and_then(|row|Self::project(self.fault,row))}
 /// 📖️ Iterates the native rows themselves, for consumers that read only their cheap fields.
 pub fn native(&self)->impl Iterator<Item=&'a S>+'a{self.list.iter()}
 /// 🔢️ The row count, without projecting any row.
 pub fn len(&self)->usize{self.list.len()}
 /// ⭕️ Whether the list holds no row.
 pub fn is_empty(&self)->bool{self.list.is_empty()}
}

/// 🔷️ The engine-shaped bounded read port over the native state: scalar leaves are borrowed, row collections project lazily.
pub struct EnergyRowsView<'a>{
 pub name:&'a str,
 pub version:&'a str,
 pub site:&'a StateSite,
 pub run_period:crate::calendar::RunPeriod,
 pub zones:Rows<'a,StateZone,Zone>,
 pub spaces:Rows<'a,StateSpace,Space>,
 pub surfaces:Rows<'a,StateSurface,Surface>,
 pub fenestrations:Rows<'a,StateFenestration,Fenestration>,
 pub shading_surfaces:Rows<'a,StateShadingSurface,ShadingSurface>,
 pub materials:Rows<'a,StateMaterial,Material>,
 pub glazing_materials:Rows<'a,StateGlazingMaterial,GlazingMaterial>,
 pub gas_materials:Rows<'a,StateGasMaterial,GasMaterial>,
 pub constructions:Rows<'a,StateConstruction,Construction>,
 pub people:Rows<'a,StatePeopleGain,PeopleGain>,
 pub lighting:Rows<'a,StateLightingGain,LightingGain>,
 pub equipment:Rows<'a,StateEquipmentGain,EquipmentGain>,
 pub infiltrations:Rows<'a,StateInfiltration,Infiltration>,
 pub thermostats:Rows<'a,StateThermostat,Thermostat>,
 pub humidistats:Rows<'a,StateHumidistat,Humidistat>,
 pub ideal_loads:Rows<'a,StateIdealLoadsSystem,IdealLoadsSystem>,
 pub air_loops:Rows<'a,StateModelAirLoop,ModelAirLoop>,
 pub plant_loops:Rows<'a,StatePlantLoopConfig,PlantLoopConfig>,
 pub schedules:&'a StateScheduleSet,
}
impl EnergyModelView<'_>{
 /// 🤲️ Reads the borrowed native fields as engine-shaped rows without copying the document.
 pub fn rows<'b>(&'b self,fault:&'b ProjectionFault)->EnergyRowsView<'b>{EnergyRowsView{
  name:self.name,version:self.version,site:self.site,run_period:*self.run_period,
  zones:Rows::over(self.zones,fault),
  spaces:Rows::over(self.spaces,fault),
  surfaces:Rows::over(self.surfaces,fault),
  fenestrations:Rows::over(self.fenestrations,fault),
  shading_surfaces:Rows::over(self.shading_surfaces,fault),
  materials:Rows::over(self.materials,fault),
  glazing_materials:Rows::over(self.glazing_materials,fault),
  gas_materials:Rows::over(self.gas_materials,fault),
  constructions:Rows::over(self.constructions,fault),
  people:Rows::over(self.people,fault),
  lighting:Rows::over(self.lighting,fault),
  equipment:Rows::over(self.equipment,fault),
  infiltrations:Rows::over(self.infiltrations,fault),
  thermostats:Rows::over(self.thermostats,fault),
  humidistats:Rows::over(self.humidistats,fault),
  ideal_loads:Rows::over(self.ideal_loads,fault),
  air_loops:Rows::over(self.air_loops,fault),
  plant_loops:Rows::over(self.plant_loops,fault),
 schedules:self.schedules,
 }}
}
