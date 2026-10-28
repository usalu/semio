//! 🧳 Genuine original fields become one serializable sparse IDML member change.
use crate::schema::diff::IdmlPartPatch;
use semio_framework_os_kernel::RetainedFieldPreparation;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{ManuallyDrop,size_of};
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items!=0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
/// 🪡 Only selected original path and payload fields enter this typed event fragment.
pub struct RetainedIdmlPartPatch {remove:bool,path:RetainedFieldPreparation<String>,bytes:RetainedFieldPreparation<Vec<u8>>,result:ManuallyDrop<Option<IdmlPartPatch>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl RetainedIdmlPartPatch {
    /// 🌱 Empty typed headers are born inside the enclosing funded semantic preparation.
    pub fn new(remove:bool)->Self{Self{remove,path:Default::default(),bytes:Default::default(),result:ManuallyDrop::new(Some(IdmlPartPatch{path:String::new(),bytes:None})),retirement:Default::default(),phase:0,closing:false}}
    /// 📏 Quotes one genuine field fragment or original fixed-header ownership movement.
    pub fn demand(&self,path:RetainedCloneRef<'_,String>,bytes:Option<RetainedCloneRef<'_,Vec<u8>>>,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing||self.phase>1||self.remove!=bytes.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML member preparation profile changed or settled"));}
        if self.phase==0{if self.path.value().is_some(){let mut d=self.path.exchange_demand()?;d.copy_bytes+=size_of::<String>()+size_of::<u8>();return Ok(d);}return self.path.demand(path,body);}
        if let Some(bytes)=bytes{if self.bytes.value().is_some(){let mut d=self.bytes.exchange_demand()?;d.copy_bytes+=size_of::<Option<Vec<u8>>>()+size_of::<u8>();return Ok(d);}return self.bytes.demand(bytes,body);}
        Ok(RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()})
    }
    /// ▶ Copies authored child fields without constructing or cloning a package snapshot.
    pub fn advance(&mut self,path:RetainedCloneRef<'_,String>,bytes:Option<RetainedCloneRef<'_,Vec<u8>>>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let d=self.demand(path,bytes,body)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.phase==0{
            if self.path.value().is_some(){let fixed=size_of::<String>()+size_of::<u8>();let(value,mut p)=self.path.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-fixed,..grant})?;self.result.as_mut().expect("untransferred sparse member").path=value.expect("prepared original path");self.phase=1;p.copied_bytes+=fixed;return Ok(RetainedCloneStep::Progress(p));}
            return self.path.advance(path,body,grant).map(|step|RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(bytes)=bytes{
            if self.bytes.value().is_some(){let fixed=size_of::<Option<Vec<u8>>>()+size_of::<u8>();let(value,mut p)=self.bytes.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-fixed,..grant})?;self.result.as_mut().expect("untransferred sparse member").bytes=value;self.phase=2;p.copied_bytes+=fixed;return Ok(RetainedCloneStep::Complete(p));}
            return self.bytes.advance(bytes,body,grant).map(|step|RetainedCloneStep::Progress(step.progress()));
        }
        self.phase=2;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
    }
    /// 👁 Both source fields must settle before this fragment can enter a real IdmlDiff.
    pub fn ready(&self)->bool{!self.closing&&self.phase==2&&self.result.is_some()}
    /// 📤 The typed fragment transfers under one fixed native header quote.
    pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML sparse member is not ready"));}Ok(RetirementDemand{copy_bytes:size_of::<IdmlPartPatch>()*2+size_of::<u8>(),depth:1,..Default::default()})}
    /// 🎁 Moves one actual serializable IdmlPartPatch, leaving all source aliases closeable.
    pub fn take(&mut self,grant:RetainedCloneGrant)->Result<(Option<IdmlPartPatch>,RetainedCloneProgress),ValueError>{let d=self.take_demand()?;if !fits(d,grant){return Ok((None,Default::default()));}self.phase=3;Ok((self.result.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    /// 🛑 Partial fields and original source authority survive every cancellation cut.
    pub fn begin_close(&mut self){self.closing=true;self.path.begin_close();self.bytes.begin_close();}
    /// ♻ Each original field owner closes before any untransferred typed fragment.
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IDML sparse member closure not started"));}
        if !self.path.terminal_is_empty(){return self.path.close_demand(body);}if !self.bytes.terminal_is_empty(){return self.bytes.close_demand(body);}self.retirement.next_owner_demands::<IdmlPartPatch>(self.result.is_some(),&None,body)
    }
    /// 🪜 Retires one selected partial child through its real intrinsic source owner.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let d=self.close_demand(grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.path.terminal_is_empty(){return self.path.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}if !self.bytes.terminal_is_empty(){return self.bytes.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}if let Some(step)=self.retirement.begin_granted(&mut self.result,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}Ok(RetainedCloneStep::Complete(Default::default()))
    }
    /// 🪹 Original source aliases and every untransferred field have returned.
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.path.terminal_is_empty()&&self.bytes.terminal_is_empty()&&self.result.is_none()&&self.retirement.is_empty()}
}
impl Drop for RetainedIdmlPartPatch {fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"IDML sparse member requires typed source closure");if empty{unsafe{ManuallyDrop::drop(&mut self.result);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
