//! 🔗️ Foreign keys search only actual original native collections and keep each paid typed cursor through cancellation.
use crate::snapshot::{original::EnergyModelSnapshot,state::*,state::fields::RetainedEntityLookup};
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep}};
macro_rules! collections{($( $kind:ident:$row:ident => $($path:ident).+ ),*$(,)?)=>{
 #[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ReferenceCollection{$($kind),*}
 enum Search{$($kind(RetainedEntityLookup<$row>)),*}
 pub struct ReferenceLookup{search:Search}
 impl ReferenceLookup{
 pub fn new(collection:ReferenceCollection)->Self{Self{search:match collection{$(ReferenceCollection::$kind=>Search::$kind(RetainedEntityLookup::new())),*}}}
 pub fn result(&self)->Option<bool>{match &self.search{$(Search::$kind(s)=>s.result().map(|rank|rank.is_some())),*}}
 pub fn demand(&self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,key:RetainedCloneRef<'_,u32>)->Result<RetirementDemand,ValueError>{match &self.search{$(Search::$kind(s)=>s.demand(original.project(0,|p|&p.model.$($path).+),key)),*}}
 pub fn advance(&mut self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,key:RetainedCloneRef<'_,u32>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match &mut self.search{$(Search::$kind(s)=>s.advance(original.project(0,|p|&p.model.$($path).+),key,g)),*}}
 pub fn begin_close(&mut self){match &mut self.search{$(Search::$kind(s)=>s.begin_close()),*}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match &self.search{$(Search::$kind(s)=>s.close_demand(body)),*}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match &mut self.search{$(Search::$kind(s)=>s.close_step(g)),*}}
 pub fn terminal_is_empty(&self)->bool{match &self.search{$(Search::$kind(s)=>s.terminal_is_empty()),*}}
 }
};}
collections!(
 Zones:StateZone=>geometry.zones,
 Surfaces:StateSurface=>geometry.surfaces,
 Spaces:StateSpace=>geometry.spaces,
 Constructions:StateConstruction=>materials.constructions,
 Materials:StateMaterial=>materials.materials,
 IdealLoads:StateIdealLoadsSystem=>controls.ideal_loads,
 AirLoops:StateModelAirLoop=>systems.air_loops,
 PvSystems:StatePvSystemAssignment=>generation.pv_systems,
 Batteries:StateBatteryAssignment=>generation.battery_storage,
 DailySchedules:StateDailySchedule=>outputs.schedules.daily,
 ConstantSchedules:StateConstantSchedule=>outputs.schedules.constants,
 WeeklySchedules:StateWeeklySchedule=>outputs.schedules.weekly,
 AnnualSchedules:StateAnnualSchedule=>outputs.schedules.annual,
 TimeSeriesSchedules:StateTimeSeriesSchedule=>outputs.schedules.time_series,
);
