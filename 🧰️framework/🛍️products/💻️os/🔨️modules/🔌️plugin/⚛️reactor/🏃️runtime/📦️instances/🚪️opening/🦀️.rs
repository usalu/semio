//! 🚪️ Native instance opening and app creation with the close grant constants.

use semio_framework_plugin_plugin_program::{AppInstance, PluginProgram};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework::{Fault, FaultFrom};
use semio_framework_value::{FromValue, ToValue};
use crate::inst_descriptor::NEXT_INSTANCE_ID;
use std::sync::atomic::Ordering;
use semio_framework_plugin_reactor_lifecycle::life_cleanup_pumps::PendingNativeCreation;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::instance_lifetime::PluginInstanceCloseLease;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::{PluginRuntime, RuntimeActorAuthority, instance_lifetime};
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell;
use semio_framework_plugin_reactor_app::app_runtime_helpers::plugin_internal_fault;
use crate::inst_wire_frames::with_instances_mut;
/// 🪪️ Retains original request inputs before factory work; a denied issuer never erases the partial app.
#[doc(hidden)]
pub async fn plugin_open_actor_instance<PA:PluginApp>(runtime:&PluginRuntime<PA>,request:semio_framework::kernel::ActorInstanceOpenRequest,app_id:String,actor:String,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 let mut lifetimes=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|plugin_internal_fault("lifecycle authority busy"))?;let slot=lifetimes.get_mut(request.instance_id).filter(|slot|slot.cell.matches_open(request)).ok_or_else(||plugin_internal_fault("exact opening lifecycle missing"))?;
 if slot.native_created||slot.cell.owner().is_some(){return Err(plugin_internal_fault("native opening already captured").into())}
 if slot.pending_creation.is_some(){return Err(plugin_internal_fault("original opening inputs already remain owned").into())}
 *slot.pending_creation=Some(PendingNativeCreation{request,source:std::mem::ManuallyDrop::new(Some((app_id,actor))),app:std::mem::ManuallyDrop::new(None),construction:std::mem::ManuallyDrop::new(None),native_factory:None,identity_bound:false,loan:std::mem::ManuallyDrop::new(None),identity:(cx.operation(),cx.generation()),parent:cx.original_source_parent()?.source_identity()?,returning_loan:false,authority:None,issuer_ready:false,closing:false,failure:std::mem::ManuallyDrop::new(None),metadata_source:std::mem::ManuallyDrop::new(None),metadata_closing:std::mem::ManuallyDrop::new(None),source_closing:std::mem::ManuallyDrop::new(None)});drop(lifetimes);
 plugin_resume_actor_opening(runtime,request.instance_id,identity,cx).await
}
async fn plugin_resume_actor_opening<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 match plugin_advance_actor_opening(runtime,instance_id,identity,cx).await{Ok(ready)=>Ok(ready),Err(failure)=>{let mut slots=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|plugin_internal_fault("opening refusal authority busy"))?;let Some(opening)=slots.get_mut(instance_id).and_then(|slot|slot.pending_creation.as_mut())else{return Err(failure)};if opening.closing{return Err(failure)}opening.closing=true;*opening.failure=Some(failure);Ok(false)}}
}
async fn plugin_advance_actor_opening<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
 let mut lifetimes=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|plugin_internal_fault("lifecycle authority busy"))?;let slot=lifetimes.get_mut(instance_id).ok_or_else(||plugin_internal_fault("original opening lifecycle missing"))?;let Some(opening)=slot.pending_creation.as_mut()else{return Ok(slot.native_created)};opening.validate(cx)?;
 if cx.should_yield(){return Ok(false)}
 if cx.is_cancelled()&&!opening.closing{opening.closing=true;*opening.failure=Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original actor creation was canceled before publication").into());}
 if opening.loan.is_none()&&!opening.returning_loan{let parent=cx.original_source_parent()?;let Some(loan)=parent.admit_loan(cx)?else{return Ok(false)};*opening.loan=Some(loan);return Ok(false)}
 if opening.construction.is_some()&&(opening.closing||opening.app.is_some()){
  let construction=opening.construction.as_mut().unwrap();if !construction.terminal_is_empty(){cx.with_original_retirement(|cx|construction.close_step(cx))?;return Ok(false)}
  let demand=construction.boxed_release_demands()?;cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,demand,|_|{opening.construction.take();Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))}))?;return Ok(false)
 }
 if opening.closing{
  if let Some(app)=opening.app.as_mut(){
   if !app.close_terminal_is_empty(){let loan=cx.retained_grant();app.close_step(cx,loan)?;return Ok(false)}
   if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}opening.app.take();opening.authority=None;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
  }
 }else{
 let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority busy"))?;let mut quarantine=runtime.close_quarantine.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime quarantine authority busy"))?;let mut actors=runtime.instance_actors.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime actor authority busy"))?;let mut document_backbones=runtime.document_backbones.try_borrow_mut().map_err(|_|plugin_internal_fault("document backbone binding authority busy"))?;
 if !instances.can_insert(instance_id)||!quarantine.can_insert(instance_id)||!actors.can_insert(instance_id)||!document_backbones.can_insert(instance_id){return Err(plugin_internal_fault("native opening slot collided").into())}
 if opening.app.is_none(){
  let source=opening.source.as_ref().ok_or_else(||plugin_internal_fault("original opening source missing"))?;
  if let Some(fault)=runtime.plugin_assembly_error.try_borrow().map_err(|_|plugin_internal_fault("plugin assembly authority busy"))?.clone(){return Err(fault.into())}
  let program=runtime.plugin.try_borrow().map_err(|_|plugin_internal_fault("plugin factory authority busy"))?;let program=program.as_ref().ok_or_else(||plugin_internal_fault("plugin not initialized"))?;
  if opening.native_factory.is_none(){let native=program.has_original_app_factory(&source.0);let mut done=None;semio_framework_job::admit_original_construction(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:std::mem::size_of::<Option<bool>>(),capacity_bytes:0,release_bytes:0,depth:1},|_|{opening.native_factory=Some(native);Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<bool>>(),..Default::default()}))})?;return Ok(false)}
  if opening.native_factory==Some(true){
   if opening.authority.is_none(){let bytes=2*std::mem::size_of::<RuntimeActorAuthority>()+source.1.len();semio_framework_job::admit_original_construction(cx,&mut opening.authority,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|Ok((RuntimeActorAuthority::new(&source.1).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original opening actor identity is empty or exceeds its declared bound"))?,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})))?;return Ok(false)}
   if opening.construction.is_none(){program.admit_original_app(&source.0,cx,&mut opening.construction)?;return Ok(false)}
   let ready=program.advance_original_app(&source.0,&source.1,instance_id,runtime.mounted_policy,opening.construction.as_mut().unwrap().as_mut(),cx,&mut opening.app)?;if ready&&opening.app.is_none(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original instance constructor completed without its original app").into())}return Ok(false)
  }
  if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}let actor=RuntimeActorAuthority::new(&source.1)?;let mut app=program.create_app(&source.0,protocol::ActorId(actor.to_string().into()),runtime.mounted_policy,identity)?.ok_or_else(||plugin_internal_fault("unknown app"))?;::semio_framework_async::poll::resolve_ready(app.bind_instance_id(instance_id));::semio_framework_async::poll::resolve_ready(app.bind_actor(&source.1));opening.authority=Some(actor);*opening.app=Some(app);opening.identity_bound=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
 }
 if !opening.identity_bound{let source=opening.source.as_ref().ok_or_else(||plugin_internal_fault("original opening identity source missing"))?;if !opening.app.as_mut().unwrap().bind_original_opening(instance_id,&source.1,cx)?{return Ok(false)}let bytes=std::mem::size_of::<bool>();let mut done=None;semio_framework_job::admit_original_construction(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|{opening.identity_bound=true;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))})?;return Ok(false)}
 if !opening.issuer_ready{if !opening.app.as_mut().unwrap().admit_cancellation_issuer(cx)?{return Ok(false)}opening.issuer_ready=true;return Ok(false)}
 }
 if let Some(source)=opening.source_closing.as_mut(){
  if !source.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,source.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|source.step(grant))?;return Ok(false)}
  if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}opening.source_closing.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
 }
 if opening.source.is_some(){
  if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}let source=opening.source.take().unwrap();match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(source)=>*opening.source_closing=Some(source),Err((failure,source))=>{*opening.source=Some(source);return Err(failure.into())}}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
 }
 if opening.closing{
  if let Some(owner)=opening.metadata_closing.as_mut(){if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(false)}if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}opening.metadata_closing.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if opening.metadata_source.is_some(){
   if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
   let source=opening.metadata_source.take().unwrap();
   match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>*opening.metadata_closing=Some(owner),Err((failure,source))=>{*opening.metadata_source=Some(source);return Err(failure.into())}}
   cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
  }
  if crate::inst_metadata_owner::take_original_instance_metadata_app_id(instance_id,&mut opening.metadata_source,cx)?!=Some(true){return Ok(false)}
 }
 if let Some(loan)=opening.loan.as_mut(){opening.returning_loan=true;let parent=cx.original_source_parent()?;loan.close_step(parent,cx)?;if loan.terminal_is_empty(){opening.loan.take();}return Ok(false)}
 if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
 if opening.closing{let failure=opening.failure.take();slot.pending_creation.take();lifetimes.remove_uncreated(instance_id).map_err(plugin_internal_fault)?;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return match failure{Some(failure)=>Err(failure),None=>Ok(false)}}
 let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority busy"))?;let mut actors=runtime.instance_actors.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime actor authority busy"))?;
 let actor=opening.authority.take().ok_or_else(||plugin_internal_fault("original opening actor authority missing"))?;let app=opening.app.take().unwrap();let cell=std::sync::Arc::new(RuntimeAppCell::new(AppInstance{id:instance_id,app,surface_contexts:Default::default()},runtime.mounted_policy));let lease=PluginInstanceCloseLease::from_cell(instance_id,&cell);let owner=semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeLifetimeOwner::from_lease(slot.cell.lifetime(),lease).expect("preflighted exact native lifetime");slot.cell.install_owner(owner).unwrap_or_else(|_|panic!("preflighted opening owner slot"));instances.insert_admitted(instance_id,cell);actors.insert_admitted(instance_id,actor);slot.native_created=true;slot.pending_creation.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}
/// 🔁️ Resumes the original allocation-bound opening once per supplied caller turn without running its factory again.
#[doc(hidden)]
pub async fn plugin_resume_actor_openings<PA:PluginApp>(runtime:&PluginRuntime<PA>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 let instance={let lifetimes=runtime.guest_lifetimes.try_borrow().map_err(|_|plugin_internal_fault("lifecycle authority busy"))?;lifetimes.pending_opening()};if let Some(instance)=instance{plugin_resume_actor_opening(runtime,instance,identity,cx).await?;}Ok(())
}
pub async fn plugin_create_app<PA: PluginApp>(runtime: &PluginRuntime<PA>, app_id: &str, actor: protocol::ActorId, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<u32, Fault> {
    plugin_create_app_with_id(runtime, NEXT_INSTANCE_ID.fetch_add(1, Ordering::SeqCst), app_id, actor, identity).await
}
/// 🧬️ Constructs every store under the explicit instance id and opened actor before publishing the app.
pub async fn plugin_create_app_with_id<PA: PluginApp>(runtime: &PluginRuntime<PA>, id: u32, app_id: &str, actor: protocol::ActorId, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<u32, Fault> {
    if let Some(fault) = runtime.plugin_assembly_error.borrow().clone() {
        return Err(fault);
    }
    if !runtime.instances.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?.can_insert(id)
        || !runtime.close_quarantine.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime close quarantine is busy"))?.can_insert(id)
        || !runtime.instance_actors.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?.can_insert(id)
    {
        return Err(plugin_internal_fault(format!("fixed instance authority is saturated, collided, or quarantined: {id}")));
    }
    {
        let program = runtime.plugin.borrow();
        let program = program.as_ref().ok_or_else(|| plugin_internal_fault("plugin not initialized"))?;
        let authority = RuntimeActorAuthority::new(&actor.0)?;
        let mut app = program.create_app(app_id, actor, runtime.mounted_policy, identity)?.ok_or_else(|| plugin_internal_fault(format!("unknown app: {app_id}")))?;
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(id));
        runtime.instance_actors.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?.insert_admitted(id, authority);
        ::semio_framework_async::poll::resolve_ready(with_instances_mut(runtime, |list| {
            list.insert_admitted(id, std::sync::Arc::new(RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
            Ok(())
        }))?;
        Ok(id)
    }
}
pub(super) const RUNTIME_CLOSE_INNER_GRANT_US: u64 = 2_000;
#[doc(hidden)]
pub const RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT: u8 = 8;
/// ⏱️ Real time a close ladder must spend making no progress before the structural accountant is
/// allowed to call it stalled — the SECOND half of the zero-progress verdict, next to the step
/// count above.
///
/// 🐛️ The count alone was the whole rule, and a count is not a duration: eight consecutive steps
/// that each observe an empty ladder cost microseconds, so a close that had nothing left to retire
/// faulted with `plugin.internal.zero-progress` reporting `elapsed 0us` against an `8000us`
/// ceiling — a stall credit judged exhausted before any time had passed (six-pane demonstrator
/// boot, ticket 26/08/28, `📓️fix-2026-09-16-concurrent-boot-timeout-and-close-fault.md`). A ladder
/// that is genuinely wedged still burns this credit within a few reactor turns; one that is merely
/// finishing never does.
#[doc(hidden)]
pub const RUNTIME_CLOSE_STALL_CREDIT_US: u64 = semio_framework_trace::INTERACTIVE_STEP_CEILING_US;
