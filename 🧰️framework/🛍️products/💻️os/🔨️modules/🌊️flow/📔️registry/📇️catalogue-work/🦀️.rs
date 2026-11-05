//! 📇️ A bounded catalogue derives from one genuine immutable registry lease and generation.
use neural_engine::{SharedRegistry,RegistryIdentity,OperatorInfo};
use protocol::causal::transition::{HistoryFoldIndex,HistoryFoldIndexInsert};
use semio_framework_value::{RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedOwnedProjection};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement,arc_custody::{SharedControlledRetirement,shared_retirement_birth_bytes,shared_retirement_allocation_bytes}};
use std::{mem::{ManuallyDrop,size_of},sync::Arc,ops::Deref,cmp::Ordering};
type Infos=HistoryFoldIndex<String,OperatorInfo>;
#[must_use]
pub struct FlowOperatorInfoMap(Arc<Infos>);
impl Deref for FlowOperatorInfoMap{type Target=Infos;fn deref(&self)->&Infos{&self.0}}
impl FlowOperatorInfoMap{
 pub fn into_arc(self)->Arc<Infos>{self.0}
 pub fn arc(&self)->&Arc<Infos>{&self.0}
 /// 📏️ The derived catalogue already exists; only its actual immutable header is received.
 pub fn alias_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),depth:1,..Default::default()}}
 /// 🪢️ A job retains the same preborn catalogue allocation without copying any operator record.
 pub fn retain_original(&self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{let d=self.alias_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(None),work:0}}RetainedWorkOutcome{result:Ok(Some((Self(Arc::clone(&self.0)),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))),work:1}}
}
impl RetireOwned for FlowOperatorInfoMap{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SharedControlledRetirement::lease(self.0))}fn retirement_birth_bytes(&self)->Option<usize>{Some(shared_retirement_birth_bytes::<Infos>())}fn controlled_retirement_supported()->bool{true}}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
#[must_use]
pub struct FlowOperatorCatalogue{pub infos:FlowOperatorInfoMap,pub registry:SharedRegistry,pub generation:u64}
impl FlowOperatorCatalogue{pub fn into_parts(self)->(Arc<Infos>,SharedRegistry,u64){(self.infos.into_arc(),self.registry,self.generation)}}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct CatalogueFields{registry:Option<SharedRegistry>,infos:Option<Infos>,info:Option<OperatorInfo>,key:Option<String>,insert:Option<HistoryFoldIndexInsert<String,OperatorInfo>>,output:Option<FlowOperatorCatalogue>}
pub struct FlowOperatorCatalogueWork{fields:ManuallyDrop<Option<CatalogueFields>>,admission:Option<RetainedCloneSourceAdmission<SharedRegistry,SharedRegistry,()>>,source:Option<RetainedCloneSource<SharedRegistry>>,projection:Option<RetainedOwnedProjection<OperatorInfo>>,info_clone:Option<<OperatorInfo as RetainedClone>::Cursor>,key_clone:Option<<String as RetainedClone>::Cursor>,retirement:Option<ControlledRetirement<CatalogueFields>>,identity:RegistryIdentity,generation:u64,slot:usize,phase:u8,closing:bool,progress:RetainedCloneProgress,work:u64}
impl FlowOperatorCatalogueWork{
 pub fn new(registry:SharedRegistry,generation:u64)->Self{let identity=registry.owner_identity();Self{fields:ManuallyDrop::new(Some(CatalogueFields{registry:Some(registry),infos:Some(Infos::new()),info:None,key:None,insert:None,output:None})),admission:None,source:None,projection:None,info_clone:None,key_clone:None,retirement:None,identity,generation,slot:0,phase:0,closing:false,progress:Default::default(),work:0}}
 pub fn registry_identity(&self)->RegistryIdentity{self.identity}
 pub fn generation(&self)->u64{self.generation}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 fn registry(&self)->Result<&SharedRegistry,ValueError>{self.source.as_ref().and_then(RetainedCloneSource::original_authority).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original catalogue registry authority is absent"))}
 fn compare_quote(left:&String,right:&String,position:usize,body:usize)->Result<RetirementDemand,ValueError>{let metadata=size_of::<usize>()+size_of::<Option<Ordering>>();let remaining=left.len().min(right.len()).checked_sub(position).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original catalogue key position changed"))?;let count=remaining.min(((body.saturating_sub(metadata))/2).max(usize::from(remaining!=0)));Ok(RetirementDemand{copy_bytes:metadata+2*count,depth:1,..Default::default()})}
 fn insert_demands(fields:&CatalogueFields,copy:usize)->Result<RetirementDemand,ValueError>{let insert=fields.insert.as_ref().unwrap();let infos=fields.infos.as_ref().unwrap();let header=insert.next_controlled_demands(infos,0,|_,_,_,_|Ok(Default::default()))?.copy_bytes;insert.next_controlled_demands(infos,copy.saturating_sub(header),Self::compare_quote)}
 pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}let fields=self.fields.as_ref().unwrap();
  match self.phase{
   1=>{let admission=self.admission.as_ref().unwrap();if admission.phase()=="ready"{Ok(RetirementDemand{depth:1,..Default::default()})}else{admission.next_demand()}},
   2=>{let registry=self.registry()?;if self.slot==registry.operator_info_slot_count(){return Ok(RetirementDemand{depth:1,..Default::default()})}let mut demand=RetainedOwnedProjection::<OperatorInfo>::constructor_demand();demand.depth=demand.depth.max(registry.operator_info_slot_depth_demand(self.slot)?);Ok(demand)},
   3=>self.info_clone.as_ref().unwrap().advance_demands(self.projection.as_ref().unwrap().borrow()?,copy),
   4=>self.info_clone.as_ref().unwrap().take_demand(),
   5=>self.info_clone.as_ref().unwrap().close_demands(copy),
   6=>self.key_clone.as_ref().unwrap().advance_demands(self.projection.as_ref().unwrap().borrow()?.project(usize::MAX,|info|&info.id),copy),
   7=>self.key_clone.as_ref().unwrap().take_demand(),
   8=>self.key_clone.as_ref().unwrap().close_demands(copy),
   9=>self.projection.as_ref().unwrap().next_close_demand(copy),
   11=>Self::insert_demands(fields,copy),
   12=>{let insert=fields.insert.as_ref().unwrap();let infos=fields.infos.as_ref().unwrap();let mut demand=insert.next_demands(infos,copy,Self::compare_quote)?;demand.copy_bytes=insert.commit_copy_byte_demand(infos)?;Ok(demand)},
   14=>Ok(RetirementDemand{copy_bytes:size_of::<SharedRegistry>(),depth:1,..Default::default()}),
   15=>Ok(RetirementDemand{capacity_bytes:shared_retirement_allocation_bytes::<Infos>(),depth:1,..Default::default()}),
   _=>Ok(RetirementDemand{depth:1,..Default::default()}),
  }
 }
 pub fn advance(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<FlowOperatorCatalogue>>{self.progress=Default::default();self.work=0;let result=self.advance_original(grant).map_err(|error|{let original=error.retained_progress();if original!=Default::default(){self.progress=original}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}}
 fn advance_original(&mut self,grant:RetainedCloneGrant)->Result<Option<FlowOperatorCatalogue>,ValueError>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original catalogue work is closing"))}let demand=self.next_demands(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
  self.work=1;self.progress.copied_items=1;
  match self.phase{
   0=>{self.admission=Some(RetainedCloneSource::<SharedRegistry>::admission_owned(self.fields.as_mut().unwrap().registry.take().unwrap(),()));self.phase=1},
   1=>{let admission=self.admission.as_mut().unwrap();if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if admission.terminal_is_empty(){self.admission=None;self.phase=2}}else{let turn=admission.advance(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}},
   2=>{let registry=self.registry()?;if self.slot==registry.operator_info_slot_count(){self.phase=14}else if registry.operator_info_slot(self.slot).is_none(){self.slot+=1}else{let slot=self.slot;let turn=self.source.as_ref().unwrap().project_owned(slot,move|registry|registry.operator_info_slot(slot).unwrap(),grant);self.work=turn.work;self.progress=Default::default();let(projection,progress)=turn.result?;self.projection=Some(projection);self.info_clone=Some(OperatorInfo::retained_clone_cursor());self.progress=progress;self.phase=3}},
   3=>{let cursor=self.info_clone.as_mut().unwrap();let turn=cursor.advance(self.projection.as_ref().unwrap().borrow()?,grant.maximum_copy_bytes,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if cursor.output_ready(){self.phase=4}},
   4=>{let cursor=self.info_clone.as_mut().unwrap();let turn=cursor.take(grant);self.work=turn.work;self.progress=Default::default();if let Some((info,progress))=turn.result?{self.fields.as_mut().unwrap().info=Some(info);self.progress=progress;cursor.begin_close();self.phase=5}},
   5=>{let cursor=self.info_clone.as_mut().unwrap();let turn=cursor.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if cursor.terminal_is_empty(){self.info_clone=None;self.key_clone=Some(String::retained_clone_cursor());self.phase=6}},
   6=>{let cursor=self.key_clone.as_mut().unwrap();let turn=cursor.advance(self.projection.as_ref().unwrap().borrow()?.project(usize::MAX,|info|&info.id),grant.maximum_copy_bytes,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if cursor.output_ready(){self.phase=7}},
   7=>{let cursor=self.key_clone.as_mut().unwrap();let turn=cursor.take(grant);self.work=turn.work;self.progress=Default::default();if let Some((key,progress))=turn.result?{self.fields.as_mut().unwrap().key=Some(key);self.progress=progress;cursor.begin_close();self.phase=8}},
   8=>{let cursor=self.key_clone.as_mut().unwrap();let turn=cursor.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if cursor.terminal_is_empty(){self.key_clone=None;self.phase=9}},
   9=>{let projection=self.projection.as_mut().unwrap();let turn=projection.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if projection.terminal_is_empty(){self.projection=None;self.phase=10}},
   10=>{let fields=self.fields.as_mut().unwrap();fields.insert=Some(HistoryFoldIndexInsert::new(fields.key.take().unwrap(),fields.info.take().unwrap()));self.phase=11},
   11=>{self.progress=Default::default();let fields=self.fields.as_mut().unwrap();let(ready,progress)=fields.insert.as_mut().unwrap().step_controlled(fields.infos.as_mut().unwrap(),grant,|left,right,position,grant|HistoryFoldIndexInsert::<String,OperatorInfo>::compare_original_text(left,right,position,grant))?;self.progress=progress;if ready{if fields.insert.as_ref().unwrap().matching_slot().is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original registry catalogue contains a duplicate operator identity"))}self.phase=12}},
   12=>{self.progress=Default::default();let fields=self.fields.as_mut().unwrap();let(slot,progress)=fields.insert.as_mut().unwrap().commit_controlled(fields.infos.as_mut().unwrap(),grant,|info|info)?;self.progress=progress;if slot.is_some(){fields.insert=None;self.slot+=1;self.phase=2}},
   14=>{let registry=self.registry()?.clone();self.fields.as_mut().unwrap().registry=Some(registry);self.progress.copied_bytes=size_of::<SharedRegistry>();self.phase=15},
   15=>{let fields=self.fields.as_mut().unwrap();let infos=FlowOperatorInfoMap(Arc::new(fields.infos.take().unwrap()));fields.output=Some(FlowOperatorCatalogue{infos,registry:fields.registry.take().unwrap(),generation:self.generation});self.progress.retained_capacity_bytes=demand.capacity_bytes;self.phase=16},
   16=>{self.phase=17;return Ok(self.fields.as_mut().unwrap().output.take())},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original catalogue output was transferred")),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:FlowOperatorCatalogue){assert_eq!(self.phase,17);self.fields.as_mut().unwrap().output=Some(output);self.phase=16}
 pub fn begin_close(&mut self){self.closing=true;if let Some(cursor)=self.info_clone.as_mut(){cursor.begin_close();}if let Some(cursor)=self.key_clone.as_mut(){cursor.begin_close();}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}
  if let Some(cursor)=self.info_clone.as_ref(){return cursor.close_demands(copy)}
  if let Some(cursor)=self.key_clone.as_ref(){return cursor.close_demands(copy)}
  if let Some(projection)=self.projection.as_ref(){return projection.next_close_demand(copy)}
  if let Some(source)=self.source.as_ref(){return source.next_close_demand(copy)}
  if let Some(owner)=self.retirement.as_ref(){return owner.selected_demand(copy)}
  Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let original=error.retained_progress();if original!=Default::default(){self.progress=original}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}}
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original catalogue close was not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.next_close_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.work=1;
  if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);self.work=turn.work;self.progress=Default::default();let step=turn.result?;self.progress=step.progress();if let Some((registry,()))=returned{self.fields.as_mut().unwrap().registry=Some(registry)}if admission.terminal_is_empty(){self.admission=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(cursor)=self.info_clone.as_mut(){let turn=cursor.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if cursor.terminal_is_empty(){self.info_clone=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(cursor)=self.key_clone.as_mut(){let turn=cursor.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if cursor.terminal_is_empty(){self.key_clone=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(projection)=self.projection.as_mut(){let turn=projection.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if projection.terminal_is_empty(){self.projection=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(source)=self.source.as_mut(){let turn=source.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if source.terminal_is_empty(){self.source=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(owner)=self.retirement.as_mut(){let result=owner.step(grant);self.work=owner.last_turn_work();self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retirement=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
  self.retirement=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!("catalogue fields declare controlled retirement")));self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.admission.is_none()&&self.info_clone.is_none()&&self.key_clone.is_none()&&self.projection.is_none()&&self.source.is_none()&&self.fields.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowOperatorCatalogueWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original catalogue work requires explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
