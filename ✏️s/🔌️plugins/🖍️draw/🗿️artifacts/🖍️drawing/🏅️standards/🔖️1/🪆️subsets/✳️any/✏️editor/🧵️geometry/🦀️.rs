//! 🧵️ Retained document geometry drives the editor through the real reactor.
use crate::DrawingSnapshot;
use crate::schema::scene_identity::SceneIdentity;
use crate::schema::scene_identity::admission::{scene_admission,SceneAdmissionStatus};
use crate::schema::scene_preparation::{DocumentVectorJob,DocumentVectorRetirement,DocumentSceneLimits,DocumentAlgorithmLimits};
use crate::schema::scene_booleans::DocumentBooleanLimits;
use crate::schema::scene_trace::DocumentTraceLimits;
use crate::schema::scene_retirement::ScenePlanCloseJob;
use crate::schema::scene_paint::scene::{ScenePaintJob,ScenePaintRetirement,PreparedScene,PreparedSceneCloseJob,PaintedSceneLimits};
use semio_framework::kernel::{Effect,JobPlacement};
use semio_framework_plugin::{AppRenderOperationContext,ArtifactView,PluginCloseStep};
use semio_framework_plugin::reactor::jobs::{BoundedJob,BoundedJobFactory,JobBudget,JobStep};
use std::{cell::RefCell,rc::Rc};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
const KIND:&str="semio.draw.mounted-vector";
const INSTANCES:usize=64;
const SHELLS:usize=256;
const TAG:u64=0x000d_0000_0000_0000;
const COUNTER_MAX:u64=u32::MAX as u64;
pub(crate) fn identity(render:AppRenderOperationContext)->SceneIdentity{SceneIdentity{instance:render.app_instance_id,base:render.base_revision.0,generation:render.generation.0,revision:render.canonical_base_revision}}
pub(crate) fn limits()->DocumentSceneLimits{Default::default()}
pub(crate) fn algorithms()->DocumentAlgorithmLimits{Default::default()}
pub(crate) fn paint_limits()->PaintedSceneLimits{PaintedSceneLimits{max_nodes:1024,max_segments:65536,max_points:262144,max_contours:65536,max_work:1000000000}}
struct State{
 identity:SceneIdentity,job_id:u64,producer:Option<DocumentVectorJob<'static>>,cache:Option<ScenePaintJob>,paint_cleanup:Option<ScenePaintRetirement>,cleanup:Option<DocumentVectorRetirement<'static>>,returned:Option<ControlledRetirement<store::SnapshotReadReturn>>,
 visual:Option<PreparedScene>,discard:Option<PreparedSceneCloseJob>,raw_discard:Option<ScenePlanCloseJob>,cancelled:bool,work:u64,
}
impl State{
 fn new(identity:SceneIdentity,job_id:u64,read:store::SnapshotRead<DrawingSnapshot>)->Self{Self{identity,job_id,producer:Some(DocumentVectorJob::from_snapshot_read(read,limits(),algorithms())),cache:None,paint_cleanup:None,cleanup:None,returned:None,visual:None,discard:None,raw_discard:None,cancelled:false,work:0}}
 fn transfer(&mut self,adopt:bool){if let Some(mut cache)=self.cache.take(){if !adopt{cache.cancel();}let(cleanup,output)=cache.into_retirement();self.paint_cleanup=Some(cleanup);if adopt{self.visual=Some(output.expect("only complete cache output may publish"));}else if let Some(scene)=output{self.discard=Some(PreparedSceneCloseJob::new(scene));}}if let Some(job)=self.producer.take(){let(cleanup,output)=job.into_retirement();self.cleanup=Some(cleanup);if let Some(plan)=output{self.raw_discard=Some(ScenePlanCloseJob::new(plan));}}}
 fn cleanup_pending(&self)->bool{self.paint_cleanup.is_some()||self.discard.is_some()||self.raw_discard.is_some()||self.cleanup.is_some()||self.returned.is_some()}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress::default();
  if let Some(child)=&mut self.paint_cleanup{if !child.terminal_is_empty(){return child.close_step(grant);}self.paint_cleanup=None;progress.copied_items=1;}
  else if let Some(child)=&mut self.discard{if !child.terminal_is_empty(){return child.close_step(grant);}self.discard=None;progress.copied_items=1;}
  else if let Some(child)=&mut self.raw_discard{if !child.terminal_is_empty(){return child.close_step(grant);}self.raw_discard=None;progress.copied_items=1;}
  else if let Some(child)=&mut self.cleanup{if let Some(read)=child.take_snapshot_read(){let witness=read.return_to_registry_witness().expect("mounted source returned exactly once");self.returned=Some(ControlledRetirement::new(witness).unwrap_or_else(|(error,_)|panic!("snapshot return owner refused: {error}")));progress.copied_items=1;}else if !child.terminal_is_empty(){return child.close_step(grant);}else{self.cleanup=None;progress.copied_items=1;}}
  else if let Some(child)=&mut self.returned{if !child.terminal_is_empty(){return child.step(grant);}self.returned=None;progress.copied_items=1;}
  Ok(if self.cleanup_pending(){RetainedCloneStep::Progress(progress)}else{RetainedCloneStep::Complete(progress)})
 }
 fn demands(&self,body:usize)->Result<[usize;4],ValueError>{
  macro_rules! child{($child:expr)=>{if let Some(child)=$child{if child.terminal_is_empty(){return Ok([0,0,0,1]);}return Ok([child.next_copy_byte_demand()?,child.next_capacity_byte_demand(body)?,child.next_release_byte_demand()?,child.next_depth_demand()?]);}}}
  child!(&self.paint_cleanup);child!(&self.discard);child!(&self.raw_discard);child!(&self.cleanup);
  child!(&self.returned);
  Ok([0;4])
 }
 fn begin_close(&mut self){self.cancelled=true;self.transfer(false);if let Some(scene)=self.visual.take(){self.discard=Some(PreparedSceneCloseJob::new(scene));}}
 fn empty(&self)->bool{self.producer.is_none()&&self.cache.is_none()&&self.paint_cleanup.is_none()&&self.cleanup.is_none()&&self.returned.is_none()&&self.visual.is_none()&&self.discard.is_none()&&self.raw_discard.is_none()}
}
#[derive(Clone,Copy)]
struct Instance{identity:SceneIdentity,candidate:Option<usize>,visual:Option<usize>,pending:Option<usize>,failed:bool}
struct Registry{instances:[Option<Instance>;INSTANCES],shells:[Rc<RefCell<Option<State>>>;SHELLS],retiring:[bool;SHELLS],counter:u64,cursor:usize}
impl Registry{fn new()->Self{Self{instances:[None;INSTANCES],shells:std::array::from_fn(|_|Rc::new(RefCell::new(None))),retiring:[false;SHELLS],counter:0,cursor:0}}}
thread_local!{static MOUNTED:RefCell<Registry>=RefCell::new(Registry::new());}
struct GeometryJob{shell:Rc<RefCell<Option<State>>>,slot:usize,identity:SceneIdentity,job:u64}
fn fail_current(identity:SceneIdentity,slot:usize){MOUNTED.with(|r|{if let Some(i)=r.borrow_mut().instances[identity.instance as usize%INSTANCES].as_mut().filter(|i|i.identity.matches(identity)&&i.candidate==Some(slot)){i.failed=true;}});}
impl BoundedJob for GeometryJob{
 fn step(&mut self,budget:JobBudget)->JobStep{
  let current=MOUNTED.with(|r|r.borrow().instances[self.identity.instance as usize%INSTANCES].is_some_and(|i|i.identity.matches(self.identity)&&i.candidate==Some(self.slot)));
  let mut shell=match self.shell.try_borrow_mut(){Ok(shell)=>shell,Err(_)=>return JobStep::Running(None)};
  let Some(state)=shell.as_mut()else{return JobStep::Failed(b"drawing.geometry.owner-missing".to_vec())};
  if state.job_id!=self.job||!state.identity.matches(self.identity){return JobStep::Failed(b"drawing.geometry.factory-stale".to_vec());}
  if !current||state.cancelled{state.transfer(false);fail_current(self.identity,self.slot);return JobStep::Failed(b"drawing.geometry.cancelled".to_vec());}
  let previous_work=state.work;let started=semio_framework_job::default_now_us();let deadline=started.map(|t|t.saturating_add(u64::from(budget.deadline_ms.min(8))*1000));
  for _ in 0..budget.fuel.min(65536){if deadline.is_some_and(|d|semio_framework_job::default_now_us().is_none_or(|t|t>=d)){break;}
   if state.cache.is_some(){
    if !state.producer.as_ref().is_some_and(|producer|producer.snapshot_authority_matches(self.identity.generation,self.identity.revision)){state.transfer(false);state.cancelled=true;fail_current(self.identity,self.slot);return JobStep::Failed(b"drawing.geometry.stale-source".to_vec());}
    match state.cache.as_mut().unwrap().advance(1){
     Ok(progress)=>{state.work+=1;if progress.done{let authority=state.producer.as_ref().unwrap().snapshot_authority_matches(self.identity.generation,self.identity.revision);state.transfer(authority);if !authority{state.cancelled=true;fail_current(self.identity,self.slot);return JobStep::Failed(b"drawing.geometry.stale-source".to_vec());}drop(shell);
      MOUNTED.with(|r|{let mut r=r.borrow_mut();let index=self.identity.instance as usize%INSTANCES;if let Some(mut i)=r.instances[index]{if i.identity.matches(self.identity)&&i.candidate==Some(self.slot){if let Some(old)=i.visual.filter(|old|*old!=self.slot){r.retiring[old]=true;}i.visual=Some(self.slot);r.instances[index]=Some(i);}}});return JobStep::Done(Vec::new());}}
     Err(error)=>{state.transfer(false);state.cancelled=true;fail_current(self.identity,self.slot);return JobStep::Failed(error.into_bytes());}
    }
   }else{
    let Some(producer)=&mut state.producer else{return JobStep::Done(Vec::new())};
    match producer.advance(1){
     Ok(progress)=>{state.work+=1;if progress.done{if !producer.snapshot_authority_matches(self.identity.generation,self.identity.revision){state.transfer(false);state.cancelled=true;fail_current(self.identity,self.slot);return JobStep::Failed(b"drawing.geometry.stale-source".to_vec());}let plan=producer.result().expect("complete vector plan transfers once");state.cache=Some(ScenePaintJob::new(plan,0.001,paint_limits()));}}
     Err(error)=>{state.transfer(false);state.cancelled=true;fail_current(self.identity,self.slot);return JobStep::Failed(error.to_string().into_bytes());}
    }
   }
  }JobStep::Running((state.work!=previous_work).then(||state.work.to_le_bytes().to_vec()))
 }
 fn cancel(&mut self){if let Ok(mut shell)=self.shell.try_borrow_mut(){if let Some(state)=shell.as_mut().filter(|state|state.job_id==self.job&&state.identity.matches(self.identity)){state.cancelled=true;}}}
 fn checkpoint(&self)->Option<Vec<u8>>{Some(input(self.slot,self.identity,self.job))}
 fn terminal_drop_is_shallow(&self)->bool{true}
}
fn input(slot:usize,identity:SceneIdentity,job:u64)->Vec<u8>{let mut bytes=Vec::with_capacity(63);bytes.push(1);bytes.extend_from_slice(&(slot as u16).to_le_bytes());bytes.extend_from_slice(&identity.instance.to_le_bytes());bytes.extend_from_slice(&identity.base.to_le_bytes());bytes.extend_from_slice(&identity.generation.to_le_bytes());bytes.extend_from_slice(&identity.revision);bytes.extend_from_slice(&job.to_le_bytes());bytes}
fn factory(job:u64,bytes:&[u8],_restored:Option<&[u8]>)->Result<Box<dyn BoundedJob>,Vec<u8>>{
 if bytes.len()!=63||bytes[0]!=1||u64::from_le_bytes(bytes[55..63].try_into().unwrap())!=job||job&!COUNTER_MAX!=TAG{return Err(b"drawing.geometry.input".to_vec());}
 let slot=usize::from(u16::from_le_bytes(bytes[1..3].try_into().unwrap()));let identity=SceneIdentity{instance:u32::from_le_bytes(bytes[3..7].try_into().unwrap()),base:u64::from_le_bytes(bytes[7..15].try_into().unwrap()),generation:u64::from_le_bytes(bytes[15..23].try_into().unwrap()),revision:bytes[23..55].try_into().unwrap()};
 MOUNTED.with(|r|{let r=r.borrow();let shell=r.shells.get(slot).ok_or_else(||b"drawing.geometry.slot".to_vec())?.clone();if !shell.try_borrow().is_ok_and(|owner|owner.as_ref().is_some_and(|state|state.job_id==job&&state.identity.matches(identity))){return Err(b"drawing.geometry.stale-factory".to_vec());}Ok(Box::new(GeometryJob{shell,slot,identity,job})as Box<dyn BoundedJob>)})
}
/// 🏭️ Register the genuine host factory before any render can spawn work.
pub fn initialize(){MOUNTED.with(|r|{let _=r.borrow().counter;});if !semio_framework_plugin::reactor::jobs::job_kind_is_admitted(KIND){semio_framework_plugin::reactor::jobs::register_bounded_job_kind(KIND,factory as BoundedJobFactory);}}
/// 🔍️ Reserve a free retained shell before requesting the genuine source read.
pub fn prepare(render:AppRenderOperationContext,document:&DrawingSnapshot)->bool{
 initialize();
 if render.app_instance_id==0||document.layers.len()>1024||document.assets.len()>1024{return false;}
 MOUNTED.with(|r|{let mut r=r.borrow_mut();let index=render.app_instance_id as usize%INSTANCES;let id=identity(render);let mut i=match r.instances[index]{Some(i)if i.identity.instance!=id.instance=>return false,Some(i)if i.identity.matches(id)=>return i.pending.is_some(),Some(i)=>i,None=>Instance{identity:id,candidate:None,visual:None,pending:None,failed:false}};
  if let Some(old)=i.pending.take(){r.retiring[old]=false;}
  let Some(slot)=r.shells.iter().enumerate().find_map(|(slot,shell)|(!r.retiring[slot]&&shell.try_borrow().is_ok_and(|owner|owner.is_none())&&!r.instances.iter().flatten().any(|i|i.pending==Some(slot))).then_some(slot))else{return false};
  i.identity=id;i.pending=Some(slot);i.failed=false;r.instances[index]=Some(i);true
 })
}
/// 🖱️ Reserve the current pointer source before the host supplies its genuine read.
pub fn prepare_query(source:SceneIdentity,document:&DrawingSnapshot)->bool{prepare(AppRenderOperationContext{app_instance_id:source.instance,base_revision:semio_framework_job::RevisionId(source.base),generation:semio_framework_job::Generation(source.generation),canonical_base_revision:source.revision},document)}
/// 🔁️ Start one revision job and cancel only its superseded private candidate.
pub fn reconcile(doc:&ArtifactView<'_,DrawingSnapshot>)->Vec<Effect>{let Some(render)=doc.render_operation()else{return Vec::new()};reconcile_source(render,||doc.take_snapshot_read())}
fn reconcile_source(render:AppRenderOperationContext,take:impl FnOnce()->Result<store::SnapshotRead<DrawingSnapshot>,semio_framework_plugin::Fault>)->Vec<Effect>{MOUNTED.with(|r|{let mut r=r.borrow_mut();let index=render.app_instance_id as usize%INSTANCES;let Some(mut i)=r.instances[index].filter(|i|i.identity.matches(identity(render)))else{return Vec::new()};let Some(slot)=i.pending else{return Vec::new()};let Some(next)=r.counter.checked_add(1).filter(|next|*next<=COUNTER_MAX)else{return Vec::new()};let Ok(read)=take()else{return Vec::new()};r.counter=next;let job=TAG|next;let mut effects=Vec::with_capacity(2);
 if let Some(old)=i.candidate.filter(|old|Some(*old)!=i.visual){r.retiring[old]=true;if let Some(state)=r.shells[old].borrow_mut().as_mut(){state.cancelled=true;effects.push(Effect::CancelJob{job:state.job_id});}}
 *r.shells[slot].borrow_mut()=Some(State::new(i.identity,job,read));i.pending=None;i.candidate=Some(slot);r.instances[index]=Some(i);effects.push(Effect::SpawnJob{job,kind:KIND.to_string(),input:input(slot,i.identity,job),placement:JobPlacement::Isolated});effects
})}
/// 👁️ Borrow the last complete picture while the current revision prepares its replacement.
pub fn with_visual<R>(render:Option<AppRenderOperationContext>,build:impl FnOnce(Option<&PreparedScene>,u32,bool)->R)->R{let shell=render.and_then(|render|MOUNTED.with(|r|{let r=r.borrow();let i=r.instances[render.app_instance_id as usize%INSTANCES]?;if i.identity.instance!=render.app_instance_id{return None;}Some(r.shells[i.visual?].clone())}));let Some(shell)=shell else{return build(None,0,false)};let Ok(owner)=shell.try_borrow()else{return build(None,0,false)};let Some(state)=owner.as_ref()else{return build(None,0,false)};build(state.visual.as_ref(),(state.job_id&COUNTER_MAX)as u32,render.is_some_and(|render|state.identity.matches(identity(render))))}
/// 🎯️ Borrow geometry only while every captured source lane matches the mounted live revision.
pub struct MountedSceneQuery<'a>{pub scene:&'a PreparedScene,pub source:SceneIdentity,pub build:u64}
impl MountedSceneQuery<'_>{
 pub fn stamp(&self,entry:usize)->Option<crate::schema::scene_paint::PreparedRegionStamp>{(entry<self.scene.geometry.len()).then_some(crate::schema::scene_paint::PreparedRegionStamp{source:self.source,build:self.build,entry,flatness:self.scene.flatness})}
}
/// 🔐️ A short borrow supplies the actual complete cache and its exact source/build authority.
pub fn with_query<R>(captured:SceneIdentity,query:impl FnOnce(SceneAdmissionStatus,Option<MountedSceneQuery<'_>>)->R)->R{
 let observed=MOUNTED.with(|r|{let r=r.borrow();let i=r.instances[captured.instance as usize%INSTANCES].filter(|i|i.identity.instance==captured.instance)?;Some((i,i.visual.map(|slot|r.shells[slot].clone())))});
 let Some((i,shell))=observed else{return query(scene_admission(captured,None,None,false,false),None)};
 let preparing=i.pending.is_some()||i.candidate.is_some();let baseline=scene_admission(captured,Some(i.identity),None,preparing,i.failed);
 if matches!(baseline,SceneAdmissionStatus::Stale)||captured.instance==0{return query(baseline,None);}
 let Some(shell)=shell else{return query(baseline,None)};let Ok(owner)=shell.try_borrow()else{return query(if baseline==SceneAdmissionStatus::Failed{baseline}else{SceneAdmissionStatus::Pending},None)};
 let state=owner.as_ref();let plan=state.and_then(|state|state.visual.as_ref());let visual=state.filter(|_|plan.is_some()).map(|state|state.identity);let status=scene_admission(captured,Some(i.identity),visual,preparing,i.failed);query(status,if status==SceneAdmissionStatus::Ready{Some(MountedSceneQuery{scene:plan.unwrap(),source:state.unwrap().identity,build:state.unwrap().job_id})}else{None})
}
/// 🎟️ The exact mounted owner delegates each cleanup currency to its original child.
pub struct GeometryOwner{instance:u32}
impl GeometryOwner{
 pub fn new(instance:u32)->Self{Self{instance}}
 fn terminal(registry:&Registry,instance:u32)->bool{!registry.instances.iter().flatten().any(|entry|entry.identity.instance==instance)&&registry.shells.iter().all(|shell|shell.try_borrow().is_ok_and(|owner|owner.as_ref().is_none_or(|state|state.identity.instance!=instance)))}
 fn slot(registry:&Registry,instance:u32)->Result<Option<usize>,ValueError>{for offset in 0..SHELLS{let slot=(registry.cursor+offset)%SHELLS;let owner=registry.shells[slot].try_borrow().map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"Drawing geometry worker is checked out"))?;if owner.as_ref().is_some_and(|state|state.identity.instance==instance&&state.cleanup_pending()){return Ok(Some(slot));}}Ok(None)}
 fn demands(&self,body:usize)->Result<[usize;4],ValueError>{MOUNTED.with(|registry|{let registry=registry.borrow();let Some(slot)=Self::slot(&registry,self.instance)?else{return Ok([0;4])};let owner=registry.shells[slot].try_borrow().map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"Drawing geometry worker is checked out"))?;owner.as_ref().unwrap().demands(body)})}
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?[0])}
 pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?[1])}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?[2])}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?[3])}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{MOUNTED.with(|registry|{let mut registry=registry.borrow_mut();let Some(slot)=Self::slot(&registry,self.instance)?else{return Ok(if Self::terminal(&registry,self.instance){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())})};let mut owner=registry.shells[slot].try_borrow_mut().map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"Drawing geometry worker is checked out"))?;let state=owner.as_mut().unwrap();let step=state.close_step(grant)?;let empty=state.empty();if empty{*owner=None;}drop(owner);if empty{for instance in registry.instances.iter_mut().flatten(){if instance.candidate==Some(slot){instance.candidate=None;}if instance.visual==Some(slot){instance.visual=None;}}registry.retiring[slot]=false;}Ok(if Self::terminal(&registry,self.instance){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})})}
 /// 🚪️ Normal lifecycle handoff retains private children before caller-funded cleanup.
 pub fn begin_close(&mut self)->Result<(),ValueError>{MOUNTED.with(|registry|{let mut registry=registry.borrow_mut();let index=self.instance as usize%INSTANCES;if registry.instances[index].is_some_and(|instance|instance.identity.instance==self.instance){registry.instances[index]=None;}for slot in 0..SHELLS{let mut owner=registry.shells[slot].try_borrow_mut().map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"Drawing geometry worker is checked out"))?;if let Some(state)=owner.as_mut().filter(|state|state.identity.instance==self.instance){state.begin_close();drop(owner);registry.retiring[slot]=true;}}Ok(())})}
 pub fn terminal_is_empty(&self)->bool{terminal_is_empty(self.instance)}
}
/// 🧹️ The two-currency host cannot authorize physical cleanup of retained geometry.
pub fn maintenance(instance:u32,maximum_items:usize,maximum_bytes:usize)->PluginCloseStep{
 if maximum_items==0||maximum_bytes==0{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
 MOUNTED.with(|registry|{let registry=registry.borrow();if registry.shells.iter().enumerate().any(|(slot,shell)|shell.try_borrow().map_or(true,|owner|owner.as_ref().is_some_and(|state|state.identity.instance==instance&&(registry.retiring[slot]||state.cleanup_pending()||state.cancelled)))){PluginCloseStep::Blocked{reason:"Drawing geometry cleanup requires item, copy, capacity, release and depth grants"}}else{PluginCloseStep::Complete}})
}
/// 🚪️ Detach instance references before retaining each exact owned shell for cleanup.
pub fn close(instance:u32,maximum_items:usize,maximum_bytes:usize)->PluginCloseStep{MOUNTED.with(|r|{let mut r=r.borrow_mut();let index=instance as usize%INSTANCES;if let Some(i)=r.instances[index].filter(|i|i.identity.instance==instance){for slot in [i.candidate,i.visual].into_iter().flatten(){r.retiring[slot]=true;}r.instances[index]=None;}});maintenance(instance,maximum_items,maximum_bytes)}
/// 🧾️ Completion includes every retained shell and genuine source return witness.
pub fn terminal_is_empty(instance:u32)->bool{MOUNTED.with(|r|{let r=r.borrow();!r.instances.iter().flatten().any(|i|i.identity.instance==instance)&&r.shells.iter().all(|shell|shell.try_borrow().is_ok_and(|owner|owner.as_ref().is_none_or(|state|state.identity.instance!=instance)))})}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
