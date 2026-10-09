//! 🧵️ Original source and control remain bound while independent cumulative wallets fund parsing and physical closure.
use super::{JsonError,JsonReadLimits,JsonReadSource,JsonParsedValue,JsonSourceCursor,JsonMemberPolicy,normal_remaining};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::ManuallyDrop;

/// 🛂️ Caller-authored total parse and cleanup authority accompanies the complete source limits.
#[derive(Clone,Copy,Debug)]
pub struct JsonReadPolicy{pub limits:JsonReadLimits,pub normal:RetainedCloneGrant,pub retirement:RetainedCloneGrant}

/// 🪪️ One retained operation conserves its original source, control, receipts and partial candidate.
pub struct JsonReadOperation<'operation,'callback,S:JsonReadSource+Copy,V:JsonParsedValue>{
    source:S,cursor:ManuallyDrop<Option<JsonSourceCursor<S,V>>>,closing:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    control:&'operation mut NativeDecodeControl<'callback>,policy:JsonReadPolicy,normal:RetainedCloneProgress,retirement:RetainedCloneProgress,
    normal_step:RetainedCloneProgress,retirement_step:RetainedCloneProgress,position:usize,close_started:bool,
}
fn intersect(total:RetainedCloneGrant,turn:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:total.maximum_items.min(turn.maximum_items),maximum_copy_bytes:total.maximum_copy_bytes.min(turn.maximum_copy_bytes),maximum_capacity_bytes:total.maximum_capacity_bytes.min(turn.maximum_capacity_bytes),maximum_release_bytes:total.maximum_release_bytes.min(turn.maximum_release_bytes),maximum_depth:total.maximum_depth.min(turn.maximum_depth)}}
fn debit(total:RetainedCloneGrant,used:&mut RetainedCloneProgress,receipt:RetainedCloneProgress)->Result<(),ValueError>{
    *used=used.checked_add(receipt).map_err(|error|error.with_retained_progress(receipt))?;
    if !used.fits(total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON operation exceeded original cumulative authority").with_retained_progress(receipt));}Ok(())
}
impl<'operation,'callback,S:JsonReadSource+Copy,V:JsonParsedValue> JsonReadOperation<'operation,'callback,S,V>{
    /// 🎟️ Binds the complete caller policy and original control without scanning or allocating.
    pub fn new(source:S,members:JsonMemberPolicy,policy:JsonReadPolicy,control:&'operation mut NativeDecodeControl<'callback>)->Result<Self,ValueError>{
        let cursor=JsonSourceCursor::new(source,members,policy.limits)?;
        Ok(Self{source,cursor:ManuallyDrop::new(Some(cursor)),closing:ManuallyDrop::new(None),control,policy,normal:Default::default(),retirement:Default::default(),normal_step:Default::default(),retirement_step:Default::default(),position:0,close_started:false})
    }
    pub fn source(&self)->S{self.source}
    pub fn position(&self)->usize{self.position}
    pub fn normal_receipt(&self)->RetainedCloneProgress{self.normal}
    pub fn retirement_receipt(&self)->RetainedCloneProgress{self.retirement}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_step}
    pub fn retirement_step_progress(&self)->RetainedCloneProgress{self.retirement_step}
    /// ⏱️ Intersects one caller turn with the unchanged remaining total parse allowance.
    pub fn step(&mut self,turn:RetainedCloneGrant)->Result<Option<V>,JsonError>{
        self.normal_step=Default::default();
        if self.close_started{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON operation cannot parse after cleanup admission").into());}
        let admitted=intersect(normal_remaining(self.policy.normal,self.normal)?,turn);
        let cursor=self.cursor.as_mut().expect("open JSON operation retains original grammar");
        let result=cursor.step(admitted.maximum_items,self.control,admitted);self.normal_step=cursor.normal_step_progress();self.position=cursor.position();
        debit(self.policy.normal,&mut self.normal,self.normal_step)?;result
    }
    /// ♻️ Debits actual cleanup receipts while retaining original ownership on every refusal.
    pub fn close_step(&mut self,turn:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.retirement_step=Default::default();self.close_started=true;
        let admitted=intersect(normal_remaining(self.policy.retirement,self.retirement)?,turn);
        if let Some(cursor)=self.cursor.take(){
            let result=cursor.into_retirement(admitted);
            match result{
                Ok((owner,receipt))=>{*self.closing=Some(owner);self.retirement_step=receipt;debit(self.policy.retirement,&mut self.retirement,receipt)?;return Ok(RetainedCloneStep::Progress(receipt));},
                Err((error,cursor))=>{*self.cursor=Some(cursor);self.retirement_step=error.retained_progress();debit(self.policy.retirement,&mut self.retirement,self.retirement_step)?;return Err(error);},
            }
        }
        let result=semio_framework_value::close_factory_ticket(&mut self.closing,admitted);
        self.retirement_step=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};debit(self.policy.retirement,&mut self.retirement,self.retirement_step)?;result
    }
    pub fn terminal_is_empty(&self)->bool{self.close_started&&self.cursor.is_none()&&self.closing.is_none()}
}
impl<S:JsonReadSource+Copy,V:JsonParsedValue> Drop for JsonReadOperation<'_,'_,S,V>{
    fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"JSON operation abandoned its original grammar or physical cleanup owner");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.closing);}}}
}
#[cfg(test)]
use super::test_allocation;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
