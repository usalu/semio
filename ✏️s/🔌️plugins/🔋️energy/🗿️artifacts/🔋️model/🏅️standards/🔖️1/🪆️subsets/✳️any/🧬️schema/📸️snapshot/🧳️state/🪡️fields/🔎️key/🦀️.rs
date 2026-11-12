//! 🔎️ Entity lookups inspect each original immutable row once and retain the genuine matched row projection.
use super::{invalid,nested,add,receipt,tagged};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_persistent::list::persistent::{PersistentList,PersistentListRead};
use protocol::{retained_mutation_demand_fits,retained::{Done,Flight,Reply,conclude,native_child}};
use std::mem::size_of;
/// 🪪️ The collection's declared row type supplies its fixed native identifier.
pub trait EntityRow:RetireOwned+Send+Sync{fn entity_id(&self)->u32;}
macro_rules! entity_rows{($($row:ident),* $(,)?)=>{$(impl EntityRow for crate::state::$row{fn entity_id(&self)->u32{self.id.0}})*};}
entity_rows!(StateZone,StateSpace,StateSurface,StateFenestration,StateMaterial,StateGlazingMaterial,StateGasMaterial,StateConstruction,StatePeopleGain,StateLightingGain,StateEquipmentGain,StateThermostat,StateIdealLoadsSystem,StateHumidistat,StateSetpointManager,StateZoneEquipmentAssignment,StateModelAirLoop,StatePlantLoopConfig,StateOutdoorAirSystem,StateShadingSurface,StateSpaceList,StateThermalEnclosure,StateMechanicalVentilation,StateElectricalLoadCenter,StatePvSystemAssignment,StateBatteryAssignment,StateShwSystemConfig,StateSolarThermalConfig,StateRefrigerationConfig,StateWaterSystemConfig,StateFaultDefinition,StateSizingObject,StateDaylightZoneConfig,StateInfiltration);

impl EntityRow for crate::state::StateRoomAirModelAssignment{fn entity_id(&self)->u32{self.zone_id.0}}

macro_rules! schedule_rows{($($row:ident),* $(,)?)=>{$(impl EntityRow for crate::state::$row{fn entity_id(&self)->u32{self.id.0}})*};}
schedule_rows!(StateConstantSchedule,StateDailySchedule,StateWeeklySchedule,StateAnnualSchedule,StateTimeSeriesSchedule);

/// 🧳️ Page cursor and selected original projection keep the actual source alive through every resumed turn.
pub struct RetainedEntityLookup<T:EntityRow>{key:Option<RetainedCloneBinding>,reader:PersistentListRead<T>,admission:RetainedOwnedProjectionAdmission<T>,found:Option<RetainedOwnedProjection<T>>,rank:Option<usize>,matched:bool,done:bool,closing:bool,flight:Flight}
impl<T:EntityRow> RetainedEntityLookup<T>{
 pub fn new()->Self{Self{key:None,reader:PersistentListRead::new(),admission:Default::default(),found:None,rank:None,matched:false,done:false,closing:false,flight:Default::default()}}
 pub fn result(&self)->Option<Option<usize>>{self.done.then_some(self.rank)}
 pub fn row(&self)->Option<&RetainedOwnedProjection<T>>{self.found.as_ref()}
 fn fixed()->usize{2*size_of::<Option<RetainedOwnedProjection<T>>>()+2*size_of::<Option<usize>>()+size_of::<u32>()+2}
 fn admit_worst()->RetirementDemand{RetirementDemand{copy_bytes:RetainedCloneBinding::alias_copy_bytes()+size_of::<Option<std::ptr::NonNull<T>>>(),depth:1,..Default::default()}}
 pub fn demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>)->Result<RetirementDemand,ValueError>{
  if self.closing||self.done||!self.flight.idle(){return Err(invalid());}key.check_binding(&self.key)?;
  if self.key.is_none(){return nested(key.binding_demand(&self.key)?.ok_or_else(invalid)?,1);}
  if self.matched{return nested(self.admission.take_demand()?,Self::fixed());}
  let mut d=self.reader.demand(original)?;let a=Self::admit_worst();d.copy_bytes=d.copy_bytes.checked_add(a.copy_bytes).ok_or_else(invalid)?;d.depth=d.depth.max(a.depth);nested(d,Self::fixed())
 }
 pub fn advance_work_demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>)->Result<u64,ValueError>{
  self.demand(original,key)?;
  if self.key.is_none(){return key.binding_work_demand(&self.key)}
  if self.matched{return self.admission.take_work_demand()}
  self.reader.advance_work_demand(original)?.checked_add(RetainedCloneBinding::alias_work_demand()).ok_or_else(invalid)
 }
 pub fn advance(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let d=self.demand(original,key)?;let units=self.advance_work_demand(original,key)?;
  if units==0||effect.remaining_work_units()?<units||!retained_mutation_demand_fits(d,effect.grant()){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.key.is_none(){let turn=key.bind(&mut self.key,effect);let p=turn.result?.ok_or_else(invalid)?;return Ok(RetainedCloneStep::Progress(add(p,1)?));}
  let extra=Self::fixed();let mut work:u128=0;
  if self.matched{let admission=&mut self.admission;let found=&mut self.found;let step=native_child(effect,&mut work,|e,r|admission.take(found,e,r))??;self.done=true;return Ok(RetainedCloneStep::Complete(add(step.progress(),extra)?));}
  let mut row=None;let reader=&mut self.reader;
  let step=native_child(effect,&mut work,|e,r|reader.advance_into(original,&mut row,e,r))??;
  let mut p=add(step.progress(),extra)?;
  if let Some((rank,projected))=row{
   if projected.get().entity_id()==*key.get(){let admission=&mut self.admission;let admitted=native_child(effect,&mut work,|e,r|admission.admit(projected,e,r))??;p=add(p,admitted.progress().copied_bytes)?;self.rank=Some(rank);self.matched=true;}
  }else if matches!(step,RetainedCloneStep::Complete(_)){self.done=true;}
  Ok(if self.done{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||!self.done||self.found.is_none(){return Err(invalid());}Ok(RetirementDemand{copy_bytes:2*size_of::<Option<RetainedOwnedProjection<T>>>(),depth:1,..Default::default()})}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(RetainedOwnedProjection<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original entity lookup take exceeded its action"));}Ok(Some((self.found.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if let Some(row)=self.found.as_ref(){return if row.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:2*size_of::<Option<RetainedOwnedProjection<T>>>(),depth:1,..Default::default()})}else{row.close_demand(body)};}
  if !self.admission.terminal_is_empty(){return self.admission.close_demand(body);}
  if !self.reader.is_closing(){return Ok(self.reader.begin_close_demand());}
  if !self.reader.terminal_is_empty(){return self.reader.close_demand(body);}
  RetainedCloneBinding::close_demand(&self.key,body)
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demand(body)?;
  if let Some(row)=self.found.as_ref(){return if row.terminal_is_empty(){Ok(1)}else{row.close_work_demand(body)};}
  if !self.admission.terminal_is_empty(){return self.admission.close_work_demand(body);}
  if !self.reader.is_closing(){return Ok(self.reader.begin_close_work_demand());}
  if !self.reader.terminal_is_empty(){return self.reader.close_work_demand(body);}
  RetainedCloneBinding::close_work_demand(&self.key,body)
 }
 pub fn requires_original_birth(&self)->bool{
  if !self.closing||!self.flight.idle(){return false}
  if let Some(row)=self.found.as_ref(){return !row.terminal_is_empty()&&row.requires_original_birth()}
  if !self.admission.terminal_is_empty(){return self.admission.requires_original_birth()}
  if !self.reader.terminal_is_empty(){return self.reader.is_closing()&&self.reader.requires_original_birth()}
  RetainedCloneBinding::requires_original_birth(&self.key)
 }
 pub fn requires_original_control(&self)->bool{
  if let Some(loaned)=self.flight.loaned(){return loaned}
  if let Some(row)=self.found.as_ref(){return !row.terminal_is_empty()&&row.requires_original_control()}
  if !self.admission.terminal_is_empty(){return self.admission.requires_original_control()}
  if !self.reader.terminal_is_empty(){return self.reader.is_closing()&&self.reader.requires_original_control()}
  RetainedCloneBinding::requires_original_control(&self.key)
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if let Some(row)=self.found.as_mut(){return row.close_original_birth_step(original,grant)}
  if !self.admission.terminal_is_empty(){return self.admission.close_original_birth_step(original,grant)}
  if !self.reader.terminal_is_empty(){return self.reader.close_original_birth_step(original,grant)}
  RetainedCloneBinding::close_original_birth_step(&mut self.key,original,grant)
 }
 close_entries!();
 pub fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut route:u8=0;
   if recover{route=self.flight.route().ok_or_else(invalid)?;}else{
    if !self.flight.idle(){return Err(invalid().into())}
    let demand=self.close_demand(body)?;if !retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if self.requires_original_birth(){return Err(invalid().into())}
    if self.requires_original_control()&&original.is_none(){return Err(invalid().into())}
    if let Some(row)=self.found.as_ref(){if row.terminal_is_empty(){work=1;drop(self.found.take());return Ok(RetainedCloneStep::Progress(receipt(demand)))}route=if row.requires_original_control(){2}else{1};}
    else if !self.admission.terminal_is_empty(){route=if self.admission.requires_original_control(){4}else{3};}
    else if !self.reader.is_closing(){let turn=self.reader.begin_close_paid(effect);work+=turn.work;return Ok(turn.result?)}
    else if !self.reader.terminal_is_empty(){route=if self.reader.requires_original_control(){6}else{5};}
    else if self.key.is_some(){route=if RetainedCloneBinding::requires_original_control(&self.key){8}else{7};}
    else{return Ok(RetainedCloneStep::Complete(Default::default()))}
   }
   let loan=if matches!(route,2|4|6|8){original.as_deref_mut()}else{None};let before=work;
   let missing=||ValueError::invariant("original entity lookup close lacks its original controller");
   let step=protocol::retained_routed!(self.flight,effect,recover,route,body,0,loan,work,|r| match route{
    1=>self.found.as_mut().ok_or_else(invalid)?.close_into(body,effect,r),
    2=>self.found.as_mut().ok_or_else(invalid)?.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    3=>self.admission.close_into(body,effect,r),
    4=>self.admission.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    5=>self.reader.close_into(body,effect,r),
    6=>self.reader.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    7=>RetainedCloneBinding::close_into(&mut self.key,body,effect,r),
    8=>RetainedCloneBinding::close_original_into(&mut self.key,original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    _=>Err(invalid())},
    match route{
    1=>self.found.as_mut().ok_or_else(invalid)?.recover_close_into(body,effect,r),
    2=>self.found.as_mut().ok_or_else(invalid)?.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    3=>self.admission.recover_close_into(body,effect,r),
    4=>self.admission.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    5=>self.reader.recover_close_into(body,effect,r),
    6=>self.reader.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    7=>RetainedCloneBinding::recover_close_into(&mut self.key,body,effect,r),
    8=>RetainedCloneBinding::recover_close_original_into(&mut self.key,original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    _=>Err(invalid())});
   let step=step.map_err(|error|tagged(error,work!=before))?;
   Ok(RetainedCloneStep::Progress(step.progress()))
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.key.is_none()&&self.reader.terminal_is_empty()&&self.admission.terminal_is_empty()&&self.found.is_none()&&self.flight.idle()}
}
impl<T:EntityRow> Default for RetainedEntityLookup<T>{fn default()->Self{Self::new()}}
impl<T:EntityRow> Drop for RetainedEntityLookup<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original entity lookup retains its genuine source and selected row");}}
