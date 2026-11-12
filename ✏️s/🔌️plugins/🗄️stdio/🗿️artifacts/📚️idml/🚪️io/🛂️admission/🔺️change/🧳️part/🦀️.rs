//! 🧳 Genuine original fields become one serializable sparse IDML member change.
use crate::schema::diff::IdmlPartPatch;
use semio_os_kernel::{RetainedFieldPreparation,RetainedFlight as Flight,retained_polled as polled,retained_native_child as native_child};
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{ManuallyDrop,size_of};
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items!=0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
fn child(g:RetainedCloneGrant,fixed:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,..g}}
/// 🪡 Only selected original path and payload fields enter this typed event fragment.
pub struct RetainedIdmlPartPatch {remove:bool,path:RetainedFieldPreparation<String>,bytes:RetainedFieldPreparation<Vec<u8>>,result:ManuallyDrop<Option<IdmlPartPatch>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl RetainedIdmlPartPatch {
    /// 🌱 Empty typed headers are born inside the enclosing funded semantic preparation.
    pub fn new(remove:bool)->Self{Self{remove,path:Default::default(),bytes:Default::default(),result:ManuallyDrop::new(Some(IdmlPartPatch{path:String::new(),bytes:None})),retirement:Default::default(),phase:0,closing:false}}
    /// 📏 Quotes one genuine field fragment or original fixed-header ownership movement.
    pub fn demand(&self,path:RetainedCloneRef<'_,String>,bytes:Option<RetainedCloneRef<'_,Vec<u8>>>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
        if self.closing||self.phase>1||self.remove!=bytes.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML member preparation profile changed or settled"));}
        if self.phase==0{if self.path.value().is_some(){let mut d=self.path.exchange_demand()?;d.copy_bytes+=size_of::<String>()+size_of::<u8>();return Ok(d);}return self.path.demand(path,body,maximum);}
        if let Some(bytes)=bytes{if self.bytes.value().is_some(){let mut d=self.bytes.exchange_demand()?;d.copy_bytes+=size_of::<Option<Vec<u8>>>()+size_of::<u8>();return Ok(d);}return self.bytes.demand(bytes,body,maximum);}
        Ok(RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()})
    }
    /// ⛽ Quotes the selected child advance, transfer or fixed-header unit in original work units.
    pub fn work_demand(&self,path:RetainedCloneRef<'_,String>,bytes:Option<RetainedCloneRef<'_,Vec<u8>>>,body:usize,maximum:u64)->Result<u64,ValueError>{
        self.demand(path,bytes,body,maximum)?;
        if self.phase==0{return if self.path.value().is_some(){Ok(2)}else{self.path.advance_work_demand(path,body,maximum)};}
        match bytes{Some(bytes)=>if self.bytes.value().is_some(){Ok(2)}else{self.bytes.advance_work_demand(bytes,body,maximum)},None=>Ok(1)}
    }
    /// ▶ Copies authored child fields without constructing or cloning a package snapshot.
    pub fn advance(&mut self,path:RetainedCloneRef<'_,String>,bytes:Option<RetainedCloneRef<'_,Vec<u8>>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,flight:&mut Flight,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
        let g=effect.grant();
        if self.phase==0{
            if self.path.value().is_some(){let fixed=size_of::<String>()+size_of::<u8>();let mut value=None;let step=native_child(effect,work,|e,r|self.path.take_into(&mut value,&mut e.nested(child(g,fixed)),r))??;let mut p=step.progress();self.result.as_mut().expect("untransferred sparse member").path=value.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML prepared original path was not transferred"))?;*work+=1;self.phase=1;p.copied_bytes+=fixed;return Ok(RetainedCloneStep::Progress(p));}
            let step=semio_os_kernel::retained_routed!(flight,effect,recover,1,body,maximum,None,*work,|r| self.path.advance_into(path,body,maximum,&mut effect.nested(g),r),self.path.recover_advance_into(path,body,maximum,&mut effect.nested(g),r)).map_err(RetirementPollExit::from)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(bytes)=bytes{
            if self.bytes.value().is_some(){let fixed=size_of::<Option<Vec<u8>>>()+size_of::<u8>();let mut value=None;let step=native_child(effect,work,|e,r|self.bytes.take_into(&mut value,&mut e.nested(child(g,fixed)),r))??;let mut p=step.progress();self.result.as_mut().expect("untransferred sparse member").bytes=Some(value.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML prepared original bytes were not transferred"))?);*work+=1;self.phase=2;p.copied_bytes+=fixed;return Ok(RetainedCloneStep::Complete(p));}
            let step=semio_os_kernel::retained_routed!(flight,effect,recover,2,body,maximum,None,*work,|r| self.bytes.advance_into(bytes,body,maximum,&mut effect.nested(g),r),self.bytes.recover_advance_into(bytes,body,maximum,&mut effect.nested(g),r)).map_err(RetirementPollExit::from)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        *work+=1;self.phase=2;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()}))
    }
    /// 👁 Both source fields must settle before this fragment can enter a real IdmlDiff.
    pub fn ready(&self)->bool{!self.closing&&self.phase==2&&self.result.is_some()}
    /// 📤 The typed fragment transfers under one fixed native header quote.
    pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML sparse member is not ready"));}Ok(RetirementDemand{copy_bytes:size_of::<IdmlPartPatch>()*2+size_of::<u8>(),depth:1,..Default::default()})}
    /// 🎁 Moves one actual serializable IdmlPartPatch, leaving all source aliases closeable.
    pub fn take(&mut self,grant:RetainedCloneGrant)->Result<(Option<IdmlPartPatch>,RetainedCloneProgress),ValueError>{let d=self.take_demand()?;if !fits(d,grant){return Ok((None,Default::default()));}self.phase=3;Ok((self.result.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    /// 🛑 Partial fields and original source authority survive every cancellation cut.
    pub fn begin_close(&mut self){self.closing=true;}
    /// ♻ Each original field owner closes before any untransferred typed fragment.
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML sparse member closure not started"));}
        if !self.path.terminal_is_empty(){return if self.path.close_started(){self.path.close_demand(body)}else{Ok(self.path.begin_close_demand())};}
        if !self.bytes.terminal_is_empty(){return if self.bytes.close_started(){self.bytes.close_demand(body)}else{Ok(self.bytes.begin_close_demand())};}
        self.retirement.next_owner_demands::<IdmlPartPatch>(self.result.is_some(),&None,body)
    }
    /// ⛽ Quotes the selected child close, close start or typed fragment retirement in original work units.
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML sparse member closure not started"));}
        if !self.path.terminal_is_empty(){return if self.path.close_started(){self.path.close_work_demand(body)}else{Ok(self.path.begin_close_work_demand())};}
        if !self.bytes.terminal_is_empty(){return if self.bytes.close_started(){self.bytes.close_work_demand(body)}else{Ok(self.bytes.begin_close_work_demand())};}
        self.retirement.next_owner_work::<IdmlPartPatch>(self.result.is_some(),&None,body)
    }
    /// 🪜 Retires one selected partial child through its real intrinsic source owner.
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,flight:&mut Flight,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML sparse member closure not started").into());}
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let g=effect.grant();
        if !self.path.terminal_is_empty(){
            if !self.path.close_started(){let turn=self.path.begin_close_paid(effect);*work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?));}
            let step=semio_os_kernel::retained_routed!(flight,effect,recover,3,body,0,None,*work,|r| self.path.close_into(body,&mut effect.nested(g),r),self.path.recover_close_into(body,&mut effect.nested(g),r)).map_err(RetirementPollExit::from)?;return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.bytes.terminal_is_empty(){
            if !self.bytes.close_started(){let turn=self.bytes.begin_close_paid(effect);*work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?));}
            let step=semio_os_kernel::retained_routed!(flight,effect,recover,4,body,0,None,*work,|r| self.bytes.close_into(body,&mut effect.nested(g),r),self.bytes.recover_close_into(body,&mut effect.nested(g),r)).map_err(RetirementPollExit::from)?;return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.retirement.is_empty(){let step=polled(flight,effect,recover,5,body,work,|e|self.retirement.step_granted(body,e))?.map_err(RetirementPollExit::from)?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.result.is_some(){let turn=self.retirement.begin_granted(&mut self.result,effect);*work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(step.progress()));}}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    /// 🪹 Original source aliases and every untransferred field have returned.
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.path.terminal_is_empty()&&self.bytes.terminal_is_empty()&&self.result.is_none()&&self.retirement.is_empty()}
}
impl Drop for RetainedIdmlPartPatch {fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"IDML sparse member requires typed source closure");if empty{unsafe{ManuallyDrop::drop(&mut self.result);}}}}
