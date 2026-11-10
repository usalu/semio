//! 📇️ A bounded catalogue derives from one genuine immutable registry lease and generation.
use neural_engine::{SharedRegistry,OperatorInfo};
use protocol::causal::transition::{HistoryFoldIndex,HistoryFoldIndexInsert};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,ReplyStash,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,retained_clone_grant_admits};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_ordered::ordered::SharedOwner;
use semio_framework_value_source_hold::SourceHold;
use std::{mem::{ManuallyDrop,size_of},ops::Deref,cmp::Ordering};
type Infos=HistoryFoldIndex<String,OperatorInfo>;
type InfoCursor=<OperatorInfo as RetainedClone>::Cursor;
type KeyCursor=<String as RetainedClone>::Cursor;
type Failure=OriginalRetirementFailure;
#[derive(semio_framework_value::RetireOwned)]
#[must_use]
pub struct FlowOperatorInfoMap(SharedOwner<Infos>);
impl Deref for FlowOperatorInfoMap{type Target=Infos;fn deref(&self)->&Infos{self.0.get()}}
impl FlowOperatorInfoMap{
 pub fn into_storage(self)->SharedOwner<Infos>{self.0}
 pub fn storage(&self)->&SharedOwner<Infos>{&self.0}
 /// 📏️ The derived catalogue already exists; only one counted alias of its actual allocation is funded.
 pub fn alias_demands(&self)->RetirementDemand{SharedOwner::<Infos>::duplicate_demand()}
 /// 🪢️ A job retains the same preborn catalogue allocation without copying any operator record.
 pub fn retain_original(&self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{let d=self.alias_demands();if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units().map_or(true,|work|work<1){return RetainedWorkOutcome{result:Ok(None),work:0}}let mut alias=None;let turn=self.0.duplicate_into(&mut alias,effect);RetainedWorkOutcome{result:turn.result.and_then(|_|alias.map(|alias|Some((Self(alias),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))).ok_or_else(||invalid("original catalogue alias was not published"))),work:turn.work}}
}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
#[must_use]
pub struct FlowOperatorCatalogue{pub infos:FlowOperatorInfoMap,pub registry:SharedRegistry,pub generation:u64}
impl FlowOperatorCatalogue{pub fn into_parts(self)->(SharedOwner<Infos>,SharedRegistry,u64){(self.infos.into_storage(),self.registry,self.generation)}}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct CatalogueFields{hold:Option<SourceHold<SharedRegistry,SharedRegistry>>,infos:Option<Infos>,info:Option<OperatorInfo>,key:Option<String>,insert:Option<HistoryFoldIndexInsert<String,OperatorInfo>>,registry:Option<SharedRegistry>,storage:Option<SharedOwner<Infos>>,output:Option<FlowOperatorCatalogue>}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn done(step:RetainedCloneStep)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),work:0})}
fn info_of(fields:&Option<CatalogueFields>,slot:usize)->Result<RetainedCloneRef<'_,OperatorInfo>,ValueError>{Ok(fields.as_ref().and_then(|fields|fields.hold.as_ref()).ok_or_else(||invalid("original catalogue source hold is absent"))?.original()?.project(slot,move|registry|registry.operator_info_slot(slot).expect("original catalogue slot is live")))}
fn key_of(fields:&Option<CatalogueFields>,slot:usize)->Result<RetainedCloneRef<'_,String>,ValueError>{Ok(info_of(fields,slot)?.project(usize::MAX,|info|&info.id))}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
/// 🧾️ Derives one catalogue slot by slot; every phase quotes its actual cost, is charged through the caller's turn effect and keeps its state when the turn denies or leaves a producer pending.
pub struct FlowOperatorCatalogueWork{fields:ManuallyDrop<Option<CatalogueFields>>,info_clone:Option<InfoCursor>,key_clone:Option<KeyCursor>,retirement:Option<ControlledRetirement<CatalogueFields>>,hold_reply:ReplyStash<Failure>,clone_reply:ReplyStash<Failure>,take_reply:ReplyStash<ValueError>,generation:u64,identity:neural_engine::RegistryIdentity,slot:usize,phase:u8,closing:bool,progress:RetainedCloneProgress}
impl FlowOperatorCatalogueWork{
 pub fn new(registry:SharedRegistry,generation:u64)->Self{let identity=registry.owner_identity();Self{fields:ManuallyDrop::new(Some(CatalogueFields{hold:Some(SourceHold::new(registry)),infos:Some(Infos::new()),info:None,key:None,insert:None,registry:None,storage:None,output:None})),info_clone:None,key_clone:None,retirement:None,hold_reply:Default::default(),clone_reply:Default::default(),take_reply:Default::default(),generation,identity,slot:0,phase:0,closing:false,progress:Default::default()}}
 pub fn registry_identity(&self)->neural_engine::RegistryIdentity{self.identity}
 pub fn generation(&self)->u64{self.generation}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 fn fields(&self)->Result<&CatalogueFields,ValueError>{self.fields.as_ref().ok_or_else(||invalid("original catalogue fields are absent"))}
 fn registry(&self)->Result<&SharedRegistry,ValueError>{Ok(self.fields()?.hold.as_ref().ok_or_else(||invalid("original catalogue source hold is absent"))?.original()?.get())}
 fn info_ref(&self)->Result<RetainedCloneRef<'_,OperatorInfo>,ValueError>{info_of(&self.fields,self.slot)}
 fn key_ref(&self)->Result<RetainedCloneRef<'_,String>,ValueError>{key_of(&self.fields,self.slot)}
 fn compare_quote(left:&String,right:&String,position:usize,body:usize)->Result<RetirementDemand,ValueError>{let metadata=size_of::<usize>()+size_of::<Option<Ordering>>();let remaining=left.len().min(right.len()).checked_sub(position).ok_or_else(||invalid("original catalogue key position changed"))?;let count=remaining.min(((body.saturating_sub(metadata))/2).max(usize::from(remaining!=0)));Ok(RetirementDemand{copy_bytes:metadata+2*count,depth:1,..Default::default()})}
 fn insert_demands(fields:&CatalogueFields,copy:usize)->Result<RetirementDemand,ValueError>{let insert=fields.insert.as_ref().unwrap();let infos=fields.infos.as_ref().unwrap();let header=insert.next_controlled_demands(infos,0,|_,_,_,_|Ok(Default::default()))?.copy_bytes;insert.next_controlled_demands(infos,copy.saturating_sub(header),Self::compare_quote)}
 fn close_demand<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,copy:usize)->Result<RetirementDemand,ValueError>{if cursor.close_started(){cursor.close_demands(copy)}else{cursor.begin_close_demand()}}
 /// 📏️ Quotes the next phase from the actual state under the caller's remaining Work.
 pub fn next_demands(&self,copy:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}let fields=self.fields()?;
  match self.phase{
   0=>fields.hold.as_ref().ok_or_else(||invalid("original catalogue source hold is absent"))?.quote(copy),
   1=>Ok(fixed(size_of::<InfoCursor>())),
   2=>self.info_clone.as_ref().unwrap().advance_demands(self.info_ref()?,copy,maximum),
   3=>self.info_clone.as_ref().unwrap().take_demand(),
   4=>Self::close_demand(self.info_clone.as_ref().unwrap(),copy),
   5=>Ok(fixed(size_of::<KeyCursor>())),
   6=>self.key_clone.as_ref().unwrap().advance_demands(self.key_ref()?,copy,maximum),
   7=>self.key_clone.as_ref().unwrap().take_demand(),
   8=>Self::close_demand(self.key_clone.as_ref().unwrap(),copy),
   10=>Self::insert_demands(fields,copy),
   11=>{let insert=fields.insert.as_ref().unwrap();let infos=fields.infos.as_ref().unwrap();let mut demand=insert.next_demands(infos,copy,Self::compare_quote)?;demand.copy_bytes=insert.commit_copy_byte_demand(infos)?;Ok(demand)},
   12=>Ok(fixed(size_of::<SharedRegistry>())),
   13=>SharedOwner::<Infos>::value_admission_demand(),
   _=>Ok(fixed(size_of::<usize>()))
  }
 }
 fn answer(&mut self,answer:Option<Result<RetainedCloneStep,Failure>>)->Result<Option<RetainedCloneStep>,Failure>{match answer{None=>Ok(None),Some(Err(failure))=>Err(failure),Some(Ok(step))=>{self.progress=step.progress();Ok(Some(step))}}}
 /// ➡️ Runs one admitted phase; `Ok(None)` keeps every cursor, reply and phase for the next turn.
 pub fn advance(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<FlowOperatorCatalogue>,Failure>{
  self.progress=Default::default();if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original catalogue work is closing").into())}
  let demand=self.next_demands(body,effect.remaining_work_units()?)?;if !retained_clone_grant_admits(effect.grant(),demand){return Ok(None)}
  let item=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  match self.phase{
   0=>{let hold_ready=self.fields()?.hold.as_ref().is_some_and(SourceHold::ready);if hold_ready{self.phase=1;return Ok(None)}let hold=self.fields.as_mut().unwrap().hold.as_mut().unwrap();let answer=self.hold_reply.run(|reply,recover|hold.step_into(body,effect,reply,recover).map(drop))?;self.answer(answer)?;if self.fields()?.hold.as_ref().is_some_and(SourceHold::ready){self.phase=1}},
   1=>{let count=self.registry()?.operator_info_slot_count();if self.slot==count{self.phase=12;return Ok(None)}if !effect.performed(1)?{return Ok(None)}if self.registry()?.operator_info_slot(self.slot).is_none(){self.slot+=1;self.progress=item;return Ok(None)}self.info_clone=Some(OperatorInfo::retained_clone_cursor());self.progress=item;self.phase=2},
   2=>{let max=effect.remaining_work_units()?;let Self{fields,info_clone,clone_reply,slot,..}=&mut *self;let source=info_of(fields,*slot)?;let cursor=info_clone.as_mut().unwrap();let answer=clone_reply.run(|reply,recover|if recover{cursor.recover_advance_into(source,body,max,effect,reply)}else{cursor.advance_into(source,body,max,effect,reply)}.map(drop))?;self.answer(answer)?;if self.info_clone.as_ref().unwrap().output_ready(){self.phase=3}},
   3=>{let mut output=None;let cursor=self.info_clone.as_mut().unwrap();let answer=self.take_reply.run(|reply,_|cursor.take(&mut output,effect,reply).map(drop))?;if let Some(answer)=answer{let step=answer?;if let Some(info)=output{self.fields.as_mut().unwrap().info=Some(info);self.progress=step.progress();self.phase=4}}},
   4=>{if !self.close_cursor(true,body,effect)?{return Ok(None)}if self.info_clone.as_ref().unwrap().terminal_is_empty(){self.info_clone=None;self.phase=5}},
   5=>{if !effect.performed(1)?{return Ok(None)}self.key_clone=Some(String::retained_clone_cursor());self.progress=item;self.phase=6},
   6=>{let max=effect.remaining_work_units()?;let Self{fields,key_clone,clone_reply,slot,..}=&mut *self;let source=key_of(fields,*slot)?;let cursor=key_clone.as_mut().unwrap();let answer=clone_reply.run(|reply,recover|if recover{cursor.recover_advance_into(source,body,max,effect,reply)}else{cursor.advance_into(source,body,max,effect,reply)}.map(drop))?;self.answer(answer)?;if self.key_clone.as_ref().unwrap().output_ready(){self.phase=7}},
   7=>{let mut output=None;let cursor=self.key_clone.as_mut().unwrap();let answer=self.take_reply.run(|reply,_|cursor.take(&mut output,effect,reply).map(drop))?;if let Some(answer)=answer{let step=answer?;if let Some(key)=output{self.fields.as_mut().unwrap().key=Some(key);self.progress=step.progress();self.phase=8}}},
   8=>{if !self.close_cursor(false,body,effect)?{return Ok(None)}if self.key_clone.as_ref().unwrap().terminal_is_empty(){self.key_clone=None;self.phase=9}},
   9=>{if !effect.performed(1)?{return Ok(None)}let fields=self.fields.as_mut().unwrap();fields.insert=Some(HistoryFoldIndexInsert::new(fields.key.take().unwrap(),fields.info.take().unwrap()));self.progress=item;self.phase=10},
   10=>{if !effect.performed(1)?{return Ok(None)}let fields=self.fields.as_mut().unwrap();let(ready,progress)=fields.insert.as_mut().unwrap().step_controlled(fields.infos.as_mut().unwrap(),effect.grant(),|left,right,position,grant|HistoryFoldIndexInsert::<String,OperatorInfo>::compare_original_text(left,right,position,grant))?;self.progress=progress;if ready{if fields.insert.as_ref().unwrap().matching_slot().is_some(){return Err(invalid("original registry catalogue contains a duplicate operator identity").into())}self.phase=11}},
   11=>{if !effect.performed(1)?{return Ok(None)}let fields=self.fields.as_mut().unwrap();let(slot,progress)=fields.insert.as_mut().unwrap().commit_controlled(fields.infos.as_mut().unwrap(),effect.grant(),|info|info)?;self.progress=progress;if slot.is_some(){fields.insert=None;self.slot+=1;self.phase=1}},
   12=>{if !effect.performed(1)?{return Ok(None)}let registry=self.registry()?.clone();self.fields.as_mut().unwrap().registry=Some(registry);self.progress=item;self.phase=13},
   13=>{let fields=self.fields.as_mut().unwrap();let turn=SharedOwner::<Infos>::admit_from(&mut fields.infos,&mut fields.storage,effect);self.progress=turn.result?.progress();if fields.storage.is_some(){self.phase=14}},
   14=>{if !effect.performed(1)?{return Ok(None)}let fields=self.fields.as_mut().unwrap();fields.output=Some(FlowOperatorCatalogue{infos:FlowOperatorInfoMap(fields.storage.take().unwrap()),registry:fields.registry.take().unwrap(),generation:self.generation});self.progress=item;self.phase=15},
   15=>{if !effect.performed(1)?{return Ok(None)}self.phase=16;self.progress=item;return Ok(self.fields.as_mut().unwrap().output.take())},
   _=>return Err(invalid("original catalogue output was transferred").into()),
  }Ok(None)
 }
 fn close_cursor(&mut self,info:bool,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,Failure>{
  macro_rules! close{($cursor:expr)=>{{let cursor=$cursor.as_mut().unwrap();if !cursor.close_started(){let turn=cursor.begin_close_paid(effect);self.progress=turn.result?;return Ok(true)}match cursor.close_step(body,effect){RetainedWorkPoll::Pending{..}=>Ok(false),RetainedWorkPoll::Complete(turn)=>{self.progress=turn.result?.progress();Ok(true)}}}}}
  if info{close!(self.info_clone)}else{close!(self.key_clone)}
 }
 pub fn restore_output(&mut self,output:FlowOperatorCatalogue){assert_eq!(self.phase,16);self.fields.as_mut().unwrap().output=Some(output);self.phase=15}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(cursor)=self.info_clone.as_ref(){return Self::close_demand(cursor,copy)}
  if let Some(cursor)=self.key_clone.as_ref(){return Self::close_demand(cursor,copy)}
  if let Some(owner)=self.retirement.as_ref(){return owner.selected_demand(copy)}
  Ok(RetirementDemand{copy_bytes:size_of::<ControlledRetirement<CatalogueFields>>(),capacity_bytes:size_of::<ControlledRetirement<CatalogueFields>>(),depth:usize::from(self.fields.is_some()),..Default::default()})
 }
 /// ♻️ Closes the active cursors, the unpublished birth and finally every owned field including the source hold.
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  if !self.closing{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("original catalogue close was not begun").into()),work:0})}
  if self.terminal_is_empty(){return done(RetainedCloneStep::Complete(Default::default()))}
  macro_rules! cursor{($cursor:expr)=>{if let Some(cursor)=$cursor.as_mut(){if !cursor.close_started(){let turn=cursor.begin_close_paid(effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress).map_err(Failure::from),work:turn.work})}let poll=cursor.close_step(body,effect);if cursor.terminal_is_empty(){$cursor=None}return poll}}}
  cursor!(self.info_clone);cursor!(self.key_clone);
  if let Some(owner)=self.retirement.as_mut(){let poll=owner.step(body,effect);if owner.terminal_is_empty(){self.retirement=None}return poll}
  self.retirement=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!("catalogue fields declare controlled retirement")));done(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.info_clone.is_none()&&self.key_clone.is_none()&&self.fields.is_none()&&self.retirement.is_none()}
}
impl Drop for FlowOperatorCatalogueWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original catalogue work requires explicit terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
