//! 📝️ Original staged events retain their decoder, window lease and rendered publication until acknowledgement.
use super::{ReceivingSource,staged::StagedRequestInput,auxiliary::{StagedAuxiliaryInput,StagedAuxiliaryProgress}};
use semio_framework::kernel::Event;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::{instance_lifetime::NativeCloseKey,life_instance_registry::PluginRuntime};
use semio_plugin_reactor_patches::PATCHES;
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind,retained_clone::RetainedCloneClose};
use std::{cell::RefCell,mem::{ManuallyDrop,size_of}};

struct StagedEvent {
    witness:ReceivingSource,key:NativeCloseKey,instance:u32,event:ManuallyDrop<Option<Event>>,wire:ManuallyDrop<Option<Vec<u8>>>,
    input:ManuallyDrop<Option<StagedRequestInput>>,auxiliary:ManuallyDrop<Option<StagedAuxiliaryInput>>,
    retirement:RetainedCloneClose,failure:ManuallyDrop<Option<OriginalTurnFailure>>,phase:u8,closing:bool,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.event-owner")}
fn metadata(cx:&mut StepContext<'_>,closing:bool)->Result<bool,OriginalTurnFailure>{
    let bytes=size_of::<StagedEvent>()+size_of::<Event>();
    let grant=cx.retained_grant();
    if (if closing{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false);}
    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}
impl StagedEvent {
    fn source(&self)->Result<(&str,&str),ValueError>{match self.event.as_ref(){Some(Event::StagedFormRequested{surface,body_key,..})=>Ok((surface,body_key)),_=>Err(invalid())}}
    async fn advance<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        if !metadata(cx,false)?{return Ok(());}
        match self.phase {
            0=>{let Some(Event::StagedFormRequested{request,..})=self.event.as_mut()else{return Err(invalid().into());};*self.wire=Some(std::mem::take(request));self.phase=1;},
            1=>{if let Some(input)=StagedRequestInput::admit(&mut self.wire,semio_framework::action_staging::StagedFormLimits::FRAMEWORK,semio_framework::action_staging::StagedFormLimits::FRAMEWORK.wire_bytes()?,cx)?{*self.input=Some(input);self.phase=2;}},
            2=>{let body=match self.event.as_ref(){Some(Event::StagedFormRequested{body_key,..})=>body_key.as_str(),_=>return Err(invalid().into())};if let Some(owner)=StagedAuxiliaryInput::admit(&mut self.input,self.instance,body,cx)?{*self.auxiliary=Some(owner);self.phase=3;}},
            3=>{
                let surface=match self.event.as_ref(){Some(Event::StagedFormRequested{surface,..})=>surface.as_str(),_=>return Err(invalid().into())};let key=self.key;
                let mut receive=|tree:&mut Option<semio_ui_runtime::ComponentTree>,cx:&mut StepContext<'_>|{
                    let surface=semio_ui_contract::SurfaceId::try_from(surface).map_err(|_|invalid())?;
                    let Ok(mut grant)=PATCHES.with(|patches|patches.reserve_mounted(surface,key))else{return Ok(false)};
                    let result=grant.commit_source(tree,cx);
                    if !matches!(result,Ok(true)){grant.cancel();}
                    result
                };
                if self.auxiliary.as_mut().ok_or_else(invalid)?.step(runtime,&mut receive,cx).await?==StagedAuxiliaryProgress::Transferred{self.phase=4;}
            },
            4=>{let(surface,_)=self.source()?;if PATCHES.with(|patches|patches.can_begin(surface)){self.closing=true;}},
            _=>return Err(invalid().into()),
        }Ok(())
    }
    async fn close<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if let Some(owner)=self.auxiliary.as_mut(){if !owner.terminal_is_empty(){owner.close_step(runtime,cx).await?;return Ok(false);}if metadata(cx,true)?{self.auxiliary.take();}return Ok(false);}
        if let Some(owner)=self.input.as_mut(){if !owner.terminal_is_empty(){owner.close_step(cx)?;return Ok(false);}if metadata(cx,true)?{self.input.take();}return Ok(false);}
        if !self.retirement.is_empty()||self.wire.is_some(){let demand=self.retirement.next_owner_demands::<Vec<u8>>(self.wire.is_some(),&None,4096)?;if self.retirement.is_empty(){super::retired(cx,demand,|effect|self.retirement.begin_granted(&mut self.wire,effect).result?.ok_or_else(invalid))?;}else{let body=cx.retained_grant().maximum_copy_bytes;semio_framework_job::close_original_effect_step(cx,demand,|effect|self.retirement.step_granted(body,effect))?;}return Ok(false);}
        if self.event.is_some(){let demand=self.retirement.next_owner_demands::<Event>(true,&None,4096)?;super::retired(cx,demand,|effect|self.retirement.begin_granted(&mut self.event,effect).result?.ok_or_else(invalid))?;return Ok(false);}
        self.witness.close(cx)
    }
    async fn step<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.witness.validate(cx)?;
        if cx.is_cancelled()||runtime.live_instance_key(self.instance).ok()!=Some(self.key){self.closing=true;}
        if !self.closing{if let Err(failure)=self.advance(runtime,cx).await{*self.failure=Some(failure);self.closing=true;}}
        if !self.closing{return Ok(false);}
        let complete=self.close(runtime,cx).await?;
        if complete{if let Some(failure)=self.failure.take(){return Err(failure);}}Ok(complete)
    }
    fn terminal(&self)->bool{self.closing&&self.event.is_none()&&self.wire.is_none()&&self.input.is_none()&&self.auxiliary.is_none()&&self.retirement.is_empty()&&self.witness.source_is_returned()&&self.failure.is_none()}
}
impl Drop for StagedEvent{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal(),"staged event requires original controller, publication and input retirement");}}
thread_local!{static STAGED_EVENTS:RefCell<[Option<StagedEvent>;semio_plugin_actor_host::requests::REQUEST_SLOTS]>=const{RefCell::new([const{None};semio_plugin_actor_host::requests::REQUEST_SLOTS])};}
pub fn retain<PA:PluginApp>(runtime:&PluginRuntime<PA>,donor:&mut Option<Event>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    let Some(Event::StagedFormRequested{instance,surface,body_key,request})=donor.as_ref()else{return Ok(false)};
    let instance=*instance;let key=runtime.live_instance_key(instance)?;
    if !surface.split_once(':').is_some_and(|(owner,name)|owner.parse::<u32>()==Ok(instance)&&!name.is_empty())||body_key.is_empty()||body_key.len()>semio_framework::MAX_SURFACE_BODY_KEY_BYTES||request.is_empty(){return Err(invalid().into());}
    STAGED_EVENTS.with(|slots|{
        let mut slots=slots.borrow_mut();let Some(slot)=slots.iter_mut().find(|slot|slot.is_none())else{return Ok(false)};
        if !metadata(cx,false)?{return Ok(false);}let witness=ReceivingSource::new(cx)?;
        *slot=Some(StagedEvent{witness,key,instance,event:ManuallyDrop::new(donor.take()),wire:ManuallyDrop::new(None),input:ManuallyDrop::new(None),auxiliary:ManuallyDrop::new(None),retirement:Default::default(),failure:ManuallyDrop::new(None),phase:0,closing:false});Ok(true)
    })
}
pub async fn advance<PA:PluginApp>(runtime:&PluginRuntime<PA>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
    let selected=STAGED_EVENTS.with(|slots|{let mut slots=slots.borrow_mut();let index=slots.iter().position(|slot|slot.as_ref().is_some_and(|owner|owner.witness.identity==(cx.operation(),cx.generation())));index.map(|index|(index,slots[index].take().unwrap()))});
    let Some((index,mut owner))=selected else{return Ok(())};
    let result=owner.step(runtime,cx).await;
    if !owner.terminal(){STAGED_EVENTS.with(|slots|slots.borrow_mut()[index]=Some(owner));}
    result.map(|_|())
}
pub fn pending()->bool{STAGED_EVENTS.with(|slots|slots.borrow().iter().any(Option::is_some))}
pub fn key_empty(key:NativeCloseKey)->bool{STAGED_EVENTS.with(|slots|slots.borrow().iter().flatten().all(|owner|owner.key!=key))}

/// 🛑️ Hiding the exact issued auxiliary surface revokes only its retained request.
pub fn cancel_surface(instance:u32,surface:&str){STAGED_EVENTS.with(|slots|{for owner in slots.borrow_mut().iter_mut().flatten(){if owner.instance==instance&&owner.source().is_ok_and(|(owned,_)|owned==surface){owner.closing=true;}}});}
