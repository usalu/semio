//! 🪟️ Original admitted bytes and the real window lease drive one private auxiliary controller render.
use super::{ReceivingSource,staged::StagedRequestInput};
use semio_framework::action_staging::StagedFormRequest;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_plugin_runtime_ops::ops_surface_render::staged_context::{self,StagedWindowContextCapture,StagedWindowContextLease};
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_value::{RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::RetainedCloneClose;
use std::mem::{ManuallyDrop,size_of};

pub const STAGED_AUXILIARY_TRANSPORT_ADMITTED:bool=false;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StagedAuxiliaryProgress{Pending,Transferred}
pub struct StagedAuxiliaryInput{
    witness:ReceivingSource,input:ManuallyDrop<Option<StagedRequestInput>>,request:ManuallyDrop<Option<StagedFormRequest>>,retirement:RetainedCloneClose,
    capture:StagedWindowContextCapture,lease:Option<StagedWindowContextLease>,instance:u32,body:&'static str,phase:u8,controller_started:bool,controller_closed:bool,closing:bool,closed:bool,failed:bool,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.auxiliary-owner")}
fn demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<StagedAuxiliaryInput>()+2*size_of::<StagedFormRequest>(),depth:1,..Default::default()}}
fn metadata(cx:&mut StepContext<'_>,closing:bool)->Result<bool,OriginalTurnFailure>{
    let grant=cx.retained_grant();let demand=demand();if (if closing{cx.retirement_should_yield()}else{cx.should_yield()})||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(false);}
    if !closing&&cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"io.cancelled").into());}
    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}
impl StagedAuxiliaryInput{
    /// 🎫️ Only the unissued actual Pack receiver can donate an input; ordinary requests confer no admission.
    pub fn admit(donor:&mut Option<StagedRequestInput>,instance:u32,body:&'static str,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
        donor.as_ref().ok_or_else(invalid)?.validate_unissued(cx)?;if body.is_empty()||body.len()>semio_framework::MAX_SURFACE_BODY_KEY_BYTES{return Err(invalid().into());}if !metadata(cx,false)?{return Ok(None);}let witness=ReceivingSource::new(cx)?;
        Ok(Some(Self{witness,input:ManuallyDrop::new(donor.take()),request:ManuallyDrop::new(None),retirement:Default::default(),capture:Default::default(),lease:None,instance,body,phase:0,controller_started:false,controller_closed:false,closing:false,closed:false,failed:false}))
    }
    async fn advance<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,receive:&mut dyn FnMut(&mut Option<ComponentTree>,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>,cx:&mut StepContext<'_>)->Result<StagedAuxiliaryProgress,OriginalTurnFailure>{
        if !metadata(cx,false)?{return Ok(StagedAuxiliaryProgress::Pending);}
        match self.phase{
            0=>{if let Some(request)=self.input.as_mut().ok_or_else(invalid)?.step(cx)?{*self.request=Some(request);self.phase=1;}},
            1=>{let request=self.request.as_ref().ok_or_else(invalid)?;if let Some(lease)=staged_context::capture(runtime,self.instance,&request.token.window,request.locale,&mut self.capture,cx).await?{self.lease=Some(lease);self.phase=2;}},
            2=>{let lease=self.lease.as_mut().ok_or_else(invalid)?;let donor=&mut *self.request;let started=&mut self.controller_started;let body=self.body;let result=staged_context::with_context(runtime,lease,&mut |app,view,cx|{*started=true;app.receive_render_work_request(body,donor,view,cx)},cx).await?;if result.is_some(){if donor.is_some(){return Err(invalid().into());}self.phase=3;}},
            3=>{let lease=self.lease.as_mut().ok_or_else(invalid)?;let body=self.body;let result=staged_context::with_context(runtime,lease,&mut |app,view,cx|{let Some(())=semio_framework_async::poll::resolve_ready(app.render(body,None,view,cx))?else{return Ok(None)};let source=app.original_render_tree_source(cx)?;if !receive(source,cx)?{return Ok(None);}if source.is_some(){return Err(invalid().into());}Ok(Some(()))},cx).await?;if result.is_some(){self.phase=4;}},
            4=>return Ok(StagedAuxiliaryProgress::Transferred),
            _=>return Err(invalid().into()),
        }Ok(if self.phase==4{StagedAuxiliaryProgress::Transferred}else{StagedAuxiliaryProgress::Pending})
    }
    /// 🧷️ The installed source recipient receives the exact native tree donor; transfer is never an ACK or readiness claim.
    pub async fn step<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,receive:&mut dyn FnMut(&mut Option<ComponentTree>,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>,cx:&mut StepContext<'_>)->Result<StagedAuxiliaryProgress,OriginalTurnFailure>{
        self.witness.validate(cx)?;if self.failed||self.closing{return Err(invalid().into());}let result=self.advance(runtime,receive,cx).await;if result.is_err(){self.failed=true;}result
    }
    /// ♻️ Revoke the controller first; the caller keeps this owner through its real publication/ACK lifecycle.
    pub async fn close_step<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.witness.validate(cx)?;self.closing=true;
        if self.controller_started&&!self.controller_closed{if staged_context::close_work(runtime,self.lease.as_mut().ok_or_else(invalid)?,cx).await?==Some(true){if metadata(cx,true)?{self.controller_closed=true;}}return Ok(false);}
        if !self.retirement.is_empty()||self.request.is_some(){let demand=self.retirement.next_owner_demands::<StagedFormRequest>(self.request.is_some(),&None,4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|if self.retirement.is_empty(){self.retirement.begin_granted(&mut self.request,grant)?.ok_or_else(invalid)}else{self.retirement.step_granted(grant)})?;return Ok(false);}
        if let Some(input)=self.input.as_mut(){if !input.terminal_is_empty(){input.close_step(cx)?;return Ok(false);}if metadata(cx,true)?{self.input.take();}return Ok(false);}
        if !metadata(cx,true)?{return Ok(false);}self.lease=None;self.capture=Default::default();self.closed=true;Ok(true)
    }
    pub fn terminal_is_empty(&self)->bool{self.closed&&self.input.is_none()&&self.request.is_none()&&self.retirement.is_empty()&&self.lease.is_none()&&(!self.controller_started||self.controller_closed)}
}
impl Drop for StagedAuxiliaryInput{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"auxiliary request requires original controller and input retirement");}}
