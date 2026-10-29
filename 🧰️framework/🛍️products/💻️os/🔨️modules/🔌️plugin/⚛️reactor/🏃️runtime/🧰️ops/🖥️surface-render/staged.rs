//! 🪟️ Auxiliary queries borrow one live real-window context under original funded metadata turns.
use super::*;
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_plugin_plugin_program::surface_context::{SurfaceWindowLease,SurfaceWindowLeaseCursor};
use semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey;
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};

#[derive(Clone,Copy,PartialEq,Eq)]
pub struct StagedWindowContextLease {key:NativeCloseKey,source:SurfaceWindowLease,original:OriginalContext}
semio_framework_value::artifact_retire_leaf!(StagedWindowContextLease);
#[derive(Default,semio_framework_value::RetireOwned)]
pub struct StagedWindowContextCapture {key:Option<NativeCloseKey>,cursor:SurfaceWindowLeaseCursor,original:Option<OriginalContext>}
#[derive(Clone,Copy,PartialEq,Eq)]
struct OriginalContext{binding:OriginalSourceBinding,parent:usize,recipient:usize,cancel:usize,epoch:Option<u64>,observed:u64,deadline:u64}
semio_framework_value::artifact_retire_leaf!(OriginalContext);
impl OriginalContext{
    fn read(cx:&StepContext<'_>)->Result<Self,OriginalTurnFailure>{Ok(Self{binding:OriginalSourceBinding::borrow(cx),parent:cx.original_source_parent()?.source_identity()?,recipient:cx.original_recipient_identity(),cancel:cx.original_cancel_token()as*const _ as usize,epoch:cx.original_source_epoch(),observed:cx.now_us().ok_or_else(invalid)?,deadline:cx.deadline_us()})}
    fn validate(self,cx:&StepContext<'_>)->Result<Self,OriginalTurnFailure>{
        let next=Self::read(cx)?;
        if self.binding!=next.binding||self.parent!=next.parent||self.recipient!=next.recipient||self.cancel!=next.cancel||self.deadline!=next.deadline||self.epoch.is_some()!=next.epoch.is_some()||matches!((self.epoch,next.epoch),(Some(before),Some(after))if after<before)||next.observed<self.observed{return Err(invalid().into());}Ok(next)
    }
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.context-unavailable")}
fn admitted(cx:&StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    cx.original_source_parent()?;
    if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"io.cancelled").into());}
    let grant=cx.retained_grant();Ok(!cx.should_yield()&&grant.maximum_items>0&&grant.maximum_copy_bytes>=metadata_bytes()&&grant.maximum_depth>0)
}
fn metadata_bytes()->usize{std::mem::size_of::<StagedWindowContextCapture>().max(std::mem::size_of::<StagedWindowContextLease>())}
fn post(cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:metadata_bytes(),..Default::default()})?;cx.consume_fuel(1);Ok(())
}
/// 🎫️ Captures the actual live actor and unique mounted source without copying its ViewModel.
pub async fn capture<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance:u32,window:&str,locale:semio_framework::Locale,capture:&mut StagedWindowContextCapture,cx:&mut StepContext<'_>)->Result<Option<StagedWindowContextLease>,OriginalTurnFailure>{
    let original=match capture.original{Some(original)=>original.validate(cx)?,None=>OriginalContext::read(cx)?};
    if !admitted(cx)?{return Ok(None);}
    let key=runtime.live_instance_key(instance)?;
    if capture.key.is_some_and(|before|before!=key){return Err(invalid().into());}
    let source=with_instances_turn_mut(runtime,|list|{
        let instance=list.get(instance).and_then(|cell|cell.host_instance()).ok_or_else(invalid)?;
        post(cx)?;capture.key=Some(key);capture.original=Some(original);capture.cursor.step(&instance.surface_contexts,window,locale).map_err(OriginalTurnFailure::from)
    }).await?;
    if runtime.live_instance_key(instance)?!=key{return Err(invalid().into());}
    Ok(source.map(|source|StagedWindowContextLease{key,source,original}))
}
/// 🔍️ Rechecks actor lifetime and exact source metadata before lending the current view within one turn.
pub async fn with_context<PA:PluginApp>(
    runtime:&PluginRuntime<PA>,lease:&mut StagedWindowContextLease,
    receive:&mut dyn FnMut(&mut PA,&ViewModel,&mut StepContext<'_>)->Result<Option<()>,OriginalTurnFailure>,cx:&mut StepContext<'_>,
)->Result<Option<()>,OriginalTurnFailure>{
    let original=lease.original.validate(cx)?;
    if !admitted(cx)?{return Ok(None);}
    if runtime.live_instance_key(lease.key.instance())?!=lease.key{return Err(invalid().into());}
    let result=with_instances_turn_mut(runtime,|list|{
        let mut instance=list.get(lease.key.instance()).and_then(|cell|cell.host_instance()).ok_or_else(invalid)?;let instance=&mut *instance;
        let context=instance.surface_contexts.borrow_window_lease(lease.source).ok_or_else(invalid)?;
        post(cx)?;lease.original=original;receive(&mut instance.app,context.view_state,cx)
    }).await?;
    if runtime.live_instance_key(lease.key.instance())?!=lease.key{return Err(invalid().into());}
    Ok(result)
}
/// ♻️ Source revocation permits original work retirement without lending a replaced or missing view.
pub async fn close_work<PA:PluginApp>(runtime:&PluginRuntime<PA>,lease:&mut StagedWindowContextLease,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{
    let original=lease.original.validate(cx)?;cx.original_source_parent()?;let grant=cx.retained_grant();
    if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<metadata_bytes()||grant.maximum_depth==0{return Ok(None);}
    if runtime.live_instance_key(lease.key.instance())?!=lease.key{return Err(invalid().into());}
    let result=with_instances_turn_mut(runtime,|list|{let mut instance=list.get(lease.key.instance()).and_then(|cell|cell.host_instance()).ok_or_else(invalid)?;post(cx)?;lease.original=original;instance.app.close_render_work_request(cx)}).await?;
    if runtime.live_instance_key(lease.key.instance())?!=lease.key{return Err(invalid().into());}Ok(result)
}
