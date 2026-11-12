//! 🔗️ Foreign keys search only actual original native collections and keep each paid typed cursor through cancellation.
use crate::{snapshot::{original::EnergyModelSnapshot,state::*,state::fields::RetainedEntityLookup},custody::Cursor,scalar::Quote};
use semio_framework_value::{RetirementDemand,RetainedAllocationEffect,OriginalRetirementCloseLoan,OriginalRetirementPoll,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use crate::retained::{Done,Reply};
macro_rules! collections{($( $kind:ident:$row:ident => $($path:ident).+ ),*$(,)?)=>{
 #[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ReferenceCollection{$($kind),*}
 enum Search{$($kind(RetainedEntityLookup<$row>)),*}
 pub struct ReferenceLookup{search:Search}
 impl ReferenceLookup{
 pub fn new(collection:ReferenceCollection)->Self{Self{search:match collection{$(ReferenceCollection::$kind=>Search::$kind(RetainedEntityLookup::new())),*}}}
 pub fn result(&self)->Option<bool>{match &self.search{$(Search::$kind(s)=>s.result().map(|rank|rank.is_some())),*}}
 pub fn quote(&self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,key:RetainedCloneRef<'_,u32>)->Result<Quote,ValueError>{match &self.search{$(Search::$kind(s)=>{let rows=original.project(0,|p|&p.model.$($path).+);Ok((s.demand(rows,key)?,s.advance_work_demand(rows,key)?))}),*}}
 pub fn advance(&mut self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,key:RetainedCloneRef<'_,u32>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{match &mut self.search{$(Search::$kind(s)=>s.advance(original.project(0,|p|&p.model.$($path).+),key,effect)),*}}
 pub fn begin_close(&mut self){match &mut self.search{$(Search::$kind(s)=>s.begin_close()),*}}
 }
 impl Cursor for ReferenceLookup{
  fn mark(&mut self){self.begin_close();}
  fn finished(&self)->bool{match &self.search{$(Search::$kind(s)=>Cursor::finished(s)),*}}
  fn demand(&self,b:usize)->Result<RetirementDemand,ValueError>{match &self.search{$(Search::$kind(s)=>Cursor::demand(s,b)),*}}
  fn work(&self,b:usize)->Result<u64,ValueError>{match &self.search{$(Search::$kind(s)=>Cursor::work(s,b)),*}}
  fn birth(&self)->bool{match &self.search{$(Search::$kind(s)=>Cursor::birth(s)),*}}
  fn control(&self)->bool{match &self.search{$(Search::$kind(s)=>Cursor::control(s)),*}}
  fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{match &mut self.search{$(Search::$kind(s)=>Cursor::birth_step(s,o,g)),*}}
  fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{match &mut self.search{$(Search::$kind(s)=>Cursor::close(s,o,b,e,r,recover)),*}}
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
