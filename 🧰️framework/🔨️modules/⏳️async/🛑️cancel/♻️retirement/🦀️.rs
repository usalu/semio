//! 🛑️ Original cancellation nodes, waiter capacity and ancestor aliases close in separate admitted turns.
use super::{CancelToken,CancelNode};
use std::{mem::{ManuallyDrop,size_of},sync::Arc,task::Waker};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetirementTurnError,ValueError,ValueRefusalKind,advance_retirement_turn,shared_retirement_allocation_bytes};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CancelTokenRetirementBlocked{SharedAlias,WeakAlias,RegisteredWaiter,WaiterContention}
#[derive(Debug)]
pub enum CancelTokenRetirementError{Blocked(CancelTokenRetirementBlocked),Refused(ValueError)}
/// 🪢️ Keeps original cancellation custody after refusal and never recursively drops an ancestor.
pub struct CancelTokenRetirement{current:ManuallyDrop<Option<CancelToken>>,node:ManuallyDrop<Option<CancelNode>>}
impl CancelTokenRetirement{
    /// 🧳️ Transfers the original small token into inline custody, separately from retirement work.
    pub fn from_token(token:CancelToken)->Self{Self{current:ManuallyDrop::new(Some(token)),node:ManuallyDrop::new(None)}}
    pub fn terminal_is_empty(&self)->bool{self.current.is_none()&&self.node.is_none()}
    /// 🔎️ Borrows the original root identity without changing cancellation custody.
    pub fn is_original_alias_witness(&self,witness:&CancelToken)->bool{self.node.is_none()&&self.current.as_ref().is_some_and(|original|Arc::ptr_eq(&original.0,&witness.0))}
    /// 📏️ Quotes only the selected original Arc, empty waiter backing or retained node metadata.
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.current.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<CancelNode>()+size_of::<CancelToken>(),release_bytes:shared_retirement_allocation_bytes::<CancelNode>(),depth:1,..Default::default()});}
        let Some(node)=self.node.as_ref()else{return Ok(Default::default());};let waiters=node.waiters.try_lock().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original cancellation waiter guard is occupied"))?;
        if waiters.capacity()>0{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<(u64,Waker)>>(),release_bytes:waiters.capacity().checked_mul(size_of::<(u64,Waker)>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"cancellation waiter backing overflow"))?,depth:1,..Default::default()});}
        Ok(RetirementDemand{copy_bytes:size_of::<CancelNode>()+size_of::<CancelToken>(),depth:1,..Default::default()})
    }
    /// 🪢️ Returns this original token alias while its matching borrowed strong root stays live through the paid turn.
    pub fn return_alias_step(&mut self,witness:&CancelToken,grant:RetainedCloneGrant)->Result<RetainedCloneStep,CancelTokenRetirementError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let Some(original)=self.current.as_ref().filter(|original|Arc::ptr_eq(&original.0,&witness.0))else{return Err(CancelTokenRetirementError::Refused(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed cancellation root does not witness this original alias")));};
        if self.node.is_some(){return Err(CancelTokenRetirementError::Refused(ValueError::literal(ValueRefusalKind::InvariantViolated,"cancellation node custody cannot be returned as a root alias")));}
        let demand=RetirementDemand{copy_bytes:size_of::<CancelToken>(),depth:1,..Default::default()};
        let _=original;
        advance_retirement_turn(demand,grant,|_|{drop(self.current.take());let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};Ok::<_,CancelTokenRetirementError>((RetainedCloneStep::Complete(progress),true))}).map_err(|error|match error{RetirementTurnError::Owner(error)=>error,RetirementTurnError::Receipt(error)=>CancelTokenRetirementError::Refused(error)})
    }
    /// 🎟️ Releases one actual original frontier and retains every blocked original alias and ancestor.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,CancelTokenRetirementError>{
        let demand=self.retirement_demands().map_err(CancelTokenRetirementError::Refused)?;
        advance_retirement_turn(demand,grant,|_|{
            if let Some(current)=self.current.as_ref(){
                {let waiters=current.0.waiters.try_lock().ok_or(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::WaiterContention))?;if !waiters.is_empty(){return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::RegisteredWaiter));}}
                if Arc::weak_count(&current.0)>0{return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::WeakAlias));}
                if Arc::strong_count(&current.0)>1{return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::SharedAlias));}
                let CancelToken(original)=self.current.take().unwrap();match Arc::try_unwrap(original){Ok(node)=>{*self.node=Some(node);return Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),false));},Err(original)=>{*self.current=Some(CancelToken(original));return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::SharedAlias));}}
            }
            let Some(node)=self.node.as_mut()else{return Ok((RetainedCloneStep::Complete(Default::default()),true));};
            let waiters=node.waiters.get_mut();
            if !waiters.is_empty(){return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::RegisteredWaiter));}
            if waiters.capacity()>0{drop(std::mem::take(waiters));return Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),false));}
            let mut node=self.node.take().unwrap();*self.current=node.parent.take();drop(node);let terminal=self.terminal_is_empty();let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};Ok((if terminal{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)},terminal))
        }).map_err(|error|match error{RetirementTurnError::Owner(error)=>error,RetirementTurnError::Receipt(error)=>CancelTokenRetirementError::Refused(error)})
    }
}
impl Drop for CancelTokenRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"cancellation retirement abandoned its original node");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
