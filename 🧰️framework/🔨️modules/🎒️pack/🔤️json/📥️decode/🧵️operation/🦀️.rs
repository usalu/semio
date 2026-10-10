//! 🧵️ Original source and control remain bound while independent cumulative wallets fund parsing and physical closure.
use super::{JsonError,JsonReadLimits,JsonReadSource,JsonParsedValue,JsonSourceCursor,JsonMemberPolicy,normal_remaining};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::RetireOwned,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::ManuallyDrop;

/// 🛂️ Caller-authored total parse and cleanup authority accompanies the complete source limits.
#[derive(Clone,Copy,Debug)]
pub struct JsonReadPolicy{pub limits:JsonReadLimits,pub normal:RetainedCloneGrant,pub retirement:RetainedCloneGrant}

/// 🪪️ One retained operation conserves its original source, control, receipts and partial candidate.
pub struct JsonReadOperation<'operation,'callback,S:JsonReadSource+Copy,V:JsonParsedValue>{
    source:S,cursor:ManuallyDrop<Option<JsonSourceCursor<S,V>>>,closing:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    value:ManuallyDrop<Option<V>>,fault:ManuallyDrop<Option<JsonError>>,
    control:&'operation mut NativeDecodeControl<'callback>,policy:JsonReadPolicy,normal:RetainedCloneProgress,retirement:RetainedCloneProgress,
    normal_step:RetainedCloneProgress,retirement_step:RetainedCloneProgress,position:usize,close_started:bool,
}
static CLOSED_REFUSAL:JsonError=JsonError::Native(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON operation cannot parse after cleanup admission"));
fn intersect(total:RetainedCloneGrant,turn:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:total.maximum_items.min(turn.maximum_items),maximum_copy_bytes:total.maximum_copy_bytes.min(turn.maximum_copy_bytes),maximum_capacity_bytes:total.maximum_capacity_bytes.min(turn.maximum_capacity_bytes),maximum_release_bytes:total.maximum_release_bytes.min(turn.maximum_release_bytes),maximum_depth:total.maximum_depth.min(turn.maximum_depth)}}
fn debit(total:RetainedCloneGrant,used:&mut RetainedCloneProgress,receipt:RetainedCloneProgress)->Result<(),ValueError>{
    *used=used.checked_add(receipt).map_err(|error|error.with_retained_progress(receipt))?;
    if !used.fits(total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON operation exceeded original cumulative authority").with_retained_progress(receipt));}Ok(())
}
fn admit<T:RetireOwned>(original:&mut Option<T>,closing:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
    match semio_framework_value::admit_owned_retirement(original.take().expect("original JSON outcome retained before admission"),grant){
        Ok((owner,receipt))=>{*closing=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},
        Err((error,value))=>{*original=Some(value);Err(error)},
    }
}
impl<'operation,'callback,S:JsonReadSource+Copy,V:JsonParsedValue> JsonReadOperation<'operation,'callback,S,V>{
    /// 🎟️ Binds the complete caller policy and original control without scanning or allocating.
    pub fn new(source:S,members:JsonMemberPolicy,policy:JsonReadPolicy,control:&'operation mut NativeDecodeControl<'callback>)->Result<Self,ValueError>{
        let cursor=JsonSourceCursor::new(source,members,policy.limits)?;
        Ok(Self{source,cursor:ManuallyDrop::new(Some(cursor)),closing:ManuallyDrop::new(None),value:ManuallyDrop::new(None),fault:ManuallyDrop::new(None),control,policy,normal:Default::default(),retirement:Default::default(),normal_step:Default::default(),retirement_step:Default::default(),position:0,close_started:false})
    }
    pub fn source(&self)->S{self.source}
    pub fn position(&self)->usize{self.position}
    pub fn normal_receipt(&self)->RetainedCloneProgress{self.normal}
    pub fn retirement_receipt(&self)->RetainedCloneProgress{self.retirement}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_step}
    pub fn retirement_step_progress(&self)->RetainedCloneProgress{self.retirement_step}
    /// 🪟️ Borrows the original admitted result without moving its storage or cleanup authority.
    pub fn value(&self)->Option<&V>{self.value.as_ref()}
    /// 🚨️ Borrows the original diagnostic and duplicate-name backing until physical cleanup admission.
    pub fn error(&self)->Option<&JsonError>{self.fault.as_ref()}
    /// ⏱️ Intersects one caller turn with the unchanged remaining total parse allowance.
    pub fn step(&mut self,turn:RetainedCloneGrant)->Result<Option<&V>,&JsonError>{
        self.normal_step=Default::default();
        if self.close_started{return Err(&CLOSED_REFUSAL);}
        if self.value.is_none()&&self.fault.is_none(){
            match normal_remaining(self.policy.normal,self.normal){
                Err(error)=>{*self.fault=Some(error.into());},
                Ok(remaining)=>{
                    let admitted=intersect(remaining,turn);let cursor=self.cursor.as_mut().expect("open JSON operation retains original grammar");
                    let result=cursor.step(admitted.maximum_items,self.control,admitted);self.normal_step=cursor.normal_step_progress();self.position=cursor.position();
                    match result{Ok(value)=>{*self.value=value;},Err(error)=>{*self.fault=Some(error);}}
                    if let Err(error)=debit(self.policy.normal,&mut self.normal,self.normal_step){if self.fault.is_none(){*self.fault=Some(error.into());}}
                },
            }
        }
        match self.fault.as_ref(){Some(error)=>Err(error),None=>Ok(self.value.as_ref())}
    }
    /// ♻️ Debits actual cleanup receipts while retaining original ownership on every refusal.
    pub fn close_step(&mut self,turn:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.retirement_step=Default::default();
        let admitted=intersect(normal_remaining(self.policy.retirement,self.retirement)?,turn);
        let result=if self.closing.is_some(){semio_framework_value::close_factory_ticket(&mut self.closing,admitted)}else if let Some(cursor)=self.cursor.take(){
            let result=cursor.into_retirement(admitted);
            match result{
                Ok((owner,receipt))=>{*self.closing=Some(owner);self.close_started=true;Ok(RetainedCloneStep::Progress(receipt))},
                Err((error,cursor))=>{*self.cursor=Some(cursor);Err(error)},
            }
        }else if self.fault.is_some(){admit(&mut self.fault,&mut self.closing,admitted)}else if self.value.is_some(){admit(&mut self.value,&mut self.closing,admitted)}else{Ok(RetainedCloneStep::Complete(Default::default()))};
        self.retirement_step=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};debit(self.policy.retirement,&mut self.retirement,self.retirement_step)?;
        match result{Ok(RetainedCloneStep::Complete(receipt))if !self.terminal_is_empty()=>Ok(RetainedCloneStep::Progress(receipt)),result=>result}
    }
    pub fn terminal_is_empty(&self)->bool{self.close_started&&self.cursor.is_none()&&self.closing.is_none()&&self.value.is_none()&&self.fault.is_none()}
}
impl<S:JsonReadSource+Copy,V:JsonParsedValue> Drop for JsonReadOperation<'_,'_,S,V>{
    fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"JSON operation abandoned its original grammar, outcome or physical cleanup owner");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.closing);ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.fault);}}}
}
#[cfg(test)]
use super::test_allocation;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
