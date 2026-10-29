//! 🌱️ Authored empty Energy state births each native immutable owner on a separate admitted turn.
use super::EnergyModelSnapshot;
use crate::state::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose},shared_issuance::{SealedShared,SealedSharedBirth,SharedIssuer}};
use semio_framework_value_persistent::list::persistent::PersistentList;
use std::mem::{size_of,ManuallyDrop};
#[derive(semio_framework_value::RetireOwned)]
struct Fields{
 schema:Option<SealedShared<String>>,
 name:Option<SealedShared<String>>,
 version:Option<SealedShared<String>>,
 site:Option<SealedShared<StateSite>>,
 building:Option<SealedShared<[f64;12]>>,
 shallow:Option<SealedShared<[f64;12]>>,
 ground:Option<SealedShared<StateGroundTemperatureConfig>>,
 run_period:Option<SealedShared<crate::calendar::RunPeriod>>,
 schedules:Option<SealedShared<StateScheduleSet>>,
 identity:Option<SealedShared<ModelIdentityState>>,
 geometry:Option<SealedShared<ModelGeometryState>>,
 materials:Option<SealedShared<ModelMaterialState>>,
 loads:Option<SealedShared<ModelLoadState>>,
 controls:Option<SealedShared<ModelControlState>>,
 systems:Option<SealedShared<ModelSystemState>>,
 generation:Option<SealedShared<ModelGenerationState>>,
 outputs:Option<SealedShared<ModelOutputState>>,
 referenced_model:Option<SealedShared<Option<store::ArtifactLink>>>,
 weather_link:Option<SealedShared<Option<store::ArtifactLink>>>,
 output:Option<EnergyModelSnapshot>,
}
/// 🌱️ Private genesis headers retain every partial physical birth until transfer or funded cancellation.
pub struct EnergyModelInitialBirth{fields:ManuallyDrop<Option<Fields>>,retirement:RetainedCloneClose,body:Option<usize>,phase:u8,closing:bool}
impl EnergyModelInitialBirth{
 pub fn new()->Self{Self{fields:ManuallyDrop::new(Some(Fields{schema:None,name:None,version:None,site:None,building:None,shallow:None,ground:None,run_period:None,schedules:None,identity:None,geometry:None,materials:None,loads:None,controls:None,systems:None,generation:None,outputs:None,referenced_model:None,weather_link:None,output:None})),retirement:Default::default(),body:None,phase:0,closing:false}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==20&&self.fields.as_ref().is_some_and(|f|f.output.is_some())}
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=20{return Err(invalid());}if self.body.is_some_and(|b|b!=body){return Err(invalid());}let mut d=match self.phase{
  0=>seal_demand::<String>(12),
  1=>seal_demand::<String>(0),
  2=>seal_demand::<String>(0),
  3=>seal_demand::<StateSite>(0),
  4=>seal_demand::<[f64;12]>(0),
  5=>seal_demand::<[f64;12]>(0),
  6=>seal_demand::<StateGroundTemperatureConfig>(0),
  7=>seal_demand::<crate::calendar::RunPeriod>(0),
  8=>seal_demand::<StateScheduleSet>(0),
  9=>seal_demand::<ModelIdentityState>(0),
  10=>seal_demand::<ModelGeometryState>(0),
  11=>seal_demand::<ModelMaterialState>(0),
  12=>seal_demand::<ModelLoadState>(0),
  13=>seal_demand::<ModelControlState>(0),
  14=>seal_demand::<ModelSystemState>(0),
  15=>seal_demand::<ModelGenerationState>(0),
  16=>seal_demand::<ModelOutputState>(0),
  17=>seal_demand::<Option<store::ArtifactLink>>(0),
  18=>seal_demand::<Option<store::ArtifactLink>>(0),
  19=>fixed(4*size_of::<EnergyModelSnapshot>()+1),_=>return Err(invalid()),
 };if self.body.is_none(){d.copy_bytes=d.copy_bytes.checked_add(size_of::<usize>()).ok_or_else(invalid)?;}Ok(d)}
 pub fn advance(&mut self,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(body)?;if !funds(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut p=receipt(d);self.body=Some(body);let fields=self.fields.as_mut().ok_or_else(invalid)?;match self.phase{
  0=>{let birth=SealedShared::<String>::prepare_birth(g)?;let value=String::from("energy.model");let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.schema=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+12;},
  1=>{let birth=SealedShared::<String>::prepare_birth(g)?;let value=String::new();let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.name=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  2=>{let birth=SealedShared::<String>::prepare_birth(g)?;let value=String::new();let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.version=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  3=>{let birth=SealedShared::<StateSite>::prepare_birth(g)?;let value=StateSite{latitude_deg:0.0,longitude_deg:0.0,elevation_m:0.0,time_zone_hours:0.0,north_axis_deg:0.0};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.site=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  4=>{let birth=SealedShared::<[f64;12]>::prepare_birth(g)?;let value=[18.0;12];let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.building=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  5=>{let birth=SealedShared::<[f64;12]>::prepare_birth(g)?;let value=[18.0;12];let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.shallow=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  6=>{let birth=SealedShared::<StateGroundTemperatureConfig>::prepare_birth(g)?;let value=StateGroundTemperatureConfig{building_surface_c:fields.building.take().ok_or_else(invalid)?,shallow_c:fields.shallow.take().ok_or_else(invalid)?,deep_c:18.0};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.ground=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  7=>{let birth=SealedShared::<crate::calendar::RunPeriod>::prepare_birth(g)?;let value=crate::calendar::RunPeriod{start_month:1,start_day:1,end_month:12,end_day:31,year:2026};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.run_period=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  8=>{let birth=SealedShared::<StateScheduleSet>::prepare_birth(g)?;let value=StateScheduleSet{constants:PersistentList::default(),daily:PersistentList::default(),weekly:PersistentList::default(),annual:PersistentList::default(),time_series:PersistentList::default()};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.schedules=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  9=>{let birth=SealedShared::<ModelIdentityState>::prepare_birth(g)?;let value=ModelIdentityState{name:fields.name.take().ok_or_else(invalid)?,version:fields.version.take().ok_or_else(invalid)?,site:fields.site.take().ok_or_else(invalid)?,ground_temperature:fields.ground.take().ok_or_else(invalid)?,run_period:fields.run_period.take().ok_or_else(invalid)?};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.identity=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  10=>{let birth=SealedShared::<ModelGeometryState>::prepare_birth(g)?;let value=ModelGeometryState{zones:PersistentList::default(),spaces:PersistentList::default(),surfaces:PersistentList::default(),fenestrations:PersistentList::default(),shading_surfaces:PersistentList::default(),space_lists:PersistentList::default(),thermal_enclosures:PersistentList::default(),adjacency_pairs:PersistentList::default()};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.geometry=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  11=>{let birth=SealedShared::<ModelMaterialState>::prepare_birth(g)?;let value=ModelMaterialState{materials:PersistentList::default(),glazing_materials:PersistentList::default(),gas_materials:PersistentList::default(),constructions:PersistentList::default()};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.materials=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  12=>{let birth=SealedShared::<ModelLoadState>::prepare_birth(g)?;let value=ModelLoadState{people:PersistentList::default(),lighting:PersistentList::default(),equipment:PersistentList::default(),infiltrations:PersistentList::default(),mechanical_ventilations:PersistentList::default(),daylight_zones:PersistentList::default(),room_air_models:PersistentList::default()};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.loads=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  13=>{let birth=SealedShared::<ModelControlState>::prepare_birth(g)?;let value=ModelControlState{thermostats:PersistentList::default(),humidistats:PersistentList::default(),setpoint_managers:PersistentList::default(),ideal_loads:PersistentList::default(),zone_equipment:PersistentList::default()};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.controls=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  14=>{let birth=SealedShared::<ModelSystemState>::prepare_birth(g)?;let value=ModelSystemState{air_loops:PersistentList::default(),plant_loops:PersistentList::default(),outdoor_air_systems:PersistentList::default(),airflow_network:None,electrical_load_centers:PersistentList::default()};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.systems=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  15=>{let birth=SealedShared::<ModelGenerationState>::prepare_birth(g)?;let value=ModelGenerationState{pv_systems:PersistentList::default(),battery_storage:PersistentList::default(),shw_systems:PersistentList::default(),solar_thermal_systems:PersistentList::default(),refrigeration_systems:PersistentList::default(),water_systems:PersistentList::default(),faults:PersistentList::default()};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.generation=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  16=>{let birth=SealedShared::<ModelOutputState>::prepare_birth(g)?;let value=ModelOutputState{output_variables:PersistentList::default(),sizing_objects:PersistentList::default(),schedules:fields.schedules.take().ok_or_else(invalid)?};let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.outputs=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  17=>{let birth=SealedShared::<Option<store::ArtifactLink>>::prepare_birth(g)?;let value=None;let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.referenced_model=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  18=>{let birth=SealedShared::<Option<store::ArtifactLink>>::prepare_birth(g)?;let value=None;let(value,progress)=birth.materialize(value,SharedIssuer::owned());fields.weather_link=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes+0;},
  19=>{fields.output=Some(EnergyModelSnapshot{schema:fields.schema.take().ok_or_else(invalid)?,model:EnergyModelState{identity:fields.identity.take().ok_or_else(invalid)?,geometry:fields.geometry.take().ok_or_else(invalid)?,materials:fields.materials.take().ok_or_else(invalid)?,loads:fields.loads.take().ok_or_else(invalid)?,controls:fields.controls.take().ok_or_else(invalid)?,systems:fields.systems.take().ok_or_else(invalid)?,generation:fields.generation.take().ok_or_else(invalid)?,outputs:fields.outputs.take().ok_or_else(invalid)?},referenced_model:fields.referenced_model.take().ok_or_else(invalid)?,weather_link:fields.weather_link.take().ok_or_else(invalid)?});},_=>return Err(invalid()),
 }self.phase+=1;semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"actual empty Energy model native genesis")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(fixed(2*size_of::<Option<EnergyModelSnapshot>>()+1))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(EnergyModelSnapshot,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !funds(d,g){return Ok(None);}let output=self.fields.as_mut().and_then(|f|f.output.take()).ok_or_else(invalid)?;self.phase+=1;Ok(Some((output,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}self.retirement.next_owner_demands::<Fields>(self.fields.is_some(),&None,self.body.unwrap_or(body))}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(invalid());}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.close_demand(self.body.unwrap_or(g.maximum_copy_bytes))?;if !funds(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let g=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,..g};if !self.retirement.is_empty(){return self.retirement.select(self.body.unwrap_or(g.maximum_copy_bytes))?.step(g);}if let Some(step)=self.retirement.begin_granted(&mut self.fields,g)?{return Ok(step);}Ok(RetainedCloneStep::Complete(Default::default()))}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.is_none()&&self.retirement.is_empty()}
}
impl Drop for EnergyModelInitialBirth{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Energy model genesis requires funded closure");}}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn seal_demand<T:Send+Sync+'static>(text_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:4*size_of::<T>()+4*size_of::<Option<SealedShared<T>>>()+2*size_of::<SealedSharedBirth<T>>()+text_bytes+1,capacity_bytes:SealedShared::<T>::birth_bytes()+text_bytes,depth:1,..Default::default()}}
fn funds(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Energy genesis phase or source changed")}
