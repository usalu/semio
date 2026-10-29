//! 🔎️ Entity lookups inspect each original immutable row once and retain the genuine matched row projection.
use super::{invalid,nested,child,add,receipt,tag};
use semio_framework_value::{RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_persistent::list::persistent::{PersistentList,PersistentListRead,PersistentListReadStep};
use protocol::retained_mutation_demand_fits;
use std::mem::size_of;

/// 🪪️ The collection's declared row type supplies its fixed native identifier.
pub trait EntityRow:RetireOwned+Send+Sync{fn entity_id(&self)->u32;}
macro_rules! entity_rows{($($row:ident),* $(,)?)=>{$(impl EntityRow for crate::state::$row{fn entity_id(&self)->u32{self.id.0}})*};}
entity_rows!(StateZone,StateSpace,StateSurface,StateFenestration,StateMaterial,StateGlazingMaterial,StateGasMaterial,StateConstruction,StatePeopleGain,StateLightingGain,StateEquipmentGain,StateThermostat,StateIdealLoadsSystem,StateHumidistat,StateSetpointManager,StateZoneEquipmentAssignment,StateModelAirLoop,StatePlantLoopConfig,StateOutdoorAirSystem,StateShadingSurface,StateSpaceList,StateThermalEnclosure,StateMechanicalVentilation,StateElectricalLoadCenter,StatePvSystemAssignment,StateBatteryAssignment,StateShwSystemConfig,StateSolarThermalConfig,StateRefrigerationConfig,StateWaterSystemConfig,StateFaultDefinition,StateSizingObject,StateDaylightZoneConfig,StateInfiltration);

impl EntityRow for crate::state::StateRoomAirModelAssignment{fn entity_id(&self)->u32{self.zone_id.0}}

macro_rules! schedule_rows{($($row:ident),* $(,)?)=>{$(impl EntityRow for crate::state::$row{fn entity_id(&self)->u32{self.id.0}})*};}
schedule_rows!(StateConstantSchedule,StateDailySchedule,StateWeeklySchedule,StateAnnualSchedule,StateTimeSeriesSchedule);

/// 🧳️ Page cursor and selected original projection keep the actual source alive through every resumed turn.
pub struct RetainedEntityLookup<T:EntityRow>{key:Option<RetainedCloneBinding>,reader:PersistentListRead<T>,found:Option<RetainedOwnedProjection<T>>,rank:Option<usize>,done:bool,closing:bool}
impl<T:EntityRow> RetainedEntityLookup<T>{
 pub fn new()->Self{Self{key:None,reader:PersistentListRead::new(),found:None,rank:None,done:false,closing:false}}
 pub fn result(&self)->Option<Option<usize>>{self.done.then_some(self.rank)}
 pub fn row(&self)->Option<&RetainedOwnedProjection<T>>{self.found.as_ref()}
 fn fixed()->usize{2*size_of::<Option<RetainedOwnedProjection<T>>>()+2*size_of::<Option<usize>>()+size_of::<u32>()+2}
 pub fn demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>)->Result<RetirementDemand,ValueError>{if self.closing||self.done{return Err(invalid());}key.check_binding(&self.key)?;if self.key.is_none(){return nested(RetirementDemand{copy_bytes:key.binding_copy_bytes(),depth:1,..Default::default()},1);}nested(self.reader.demand(original)?,Self::fixed())}
 pub fn advance(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(original,key)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.key.is_none(){let p=key.bind(&mut self.key,child(g,1))?.ok_or_else(invalid)?;return Ok(RetainedCloneStep::Progress(add(p,1)?));}let extra=Self::fixed();let step=self.reader.advance(original,child(g,extra)).map_err(|e|tag(e,extra))?;let p=add(step.progress(),extra)?;match step{PersistentListReadStep::Row(_,rank,row)if row.get().entity_id()==*key.get()=>{let(projection,progress)=row.retain_owned(child(g,extra)).map_err(|e|tag(e,extra))?;if progress.retained_capacity_bytes!=0||progress.released_bytes!=0{return Err(invalid().with_retained_progress(p));}self.found=Some(projection);self.rank=Some(rank);self.done=true;},PersistentListReadStep::Complete(_)=>self.done=true,_=>{}}Ok(if self.done{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||!self.done||self.found.is_none(){return Err(invalid());}Ok(RetirementDemand{copy_bytes:2*size_of::<Option<RetainedOwnedProjection<T>>>(),depth:1,..Default::default()})}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(RetainedOwnedProjection<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}Ok(Some((self.found.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;self.reader.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if let Some(row)=self.found.as_ref(){return if row.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:2*size_of::<Option<RetainedOwnedProjection<T>>>(),depth:1,..Default::default()})}else{row.close_demand(body)};}if !self.reader.terminal_is_empty(){return self.reader.close_demand(body);}RetainedCloneBinding::close_demand(&self.key,body)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(row)=self.found.as_mut(){if row.terminal_is_empty(){drop(self.found.take());return Ok(RetainedCloneStep::Progress(receipt(d)));}return row.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.reader.terminal_is_empty(){return self.reader.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}let s=RetainedCloneBinding::close_one(&mut self.key,g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.key.is_none()&&self.reader.terminal_is_empty()&&self.found.is_none()}
}
impl<T:EntityRow> Default for RetainedEntityLookup<T>{fn default()->Self{Self::new()}}
impl<T:EntityRow> Drop for RetainedEntityLookup<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original entity lookup retains its genuine source and selected row");}}
