//! 🛑️ Original cancellation nodes, waiter capacity and ancestor aliases close in separate admitted turns.
use super::{CancelToken,CancelNode};
use std::{mem::{ManuallyDrop,size_of},sync::Arc,task::Waker};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind,shared_retirement_allocation_bytes};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CancelTokenRetirementBlocked{SharedAlias,WeakAlias,RegisteredWaiter,WaiterContention}
semio_framework_value::artifact_retire_leaf!(CancelTokenRetirementBlocked);
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum CancelTokenRetirementError{Blocked(CancelTokenRetirementBlocked),Refused(ValueError)}
/// 🪢️ Keeps original cancellation custody after refusal and never recursively drops an ancestor.
pub struct CancelTokenRetirement{current:ManuallyDrop<Option<CancelToken>>,node:ManuallyDrop<Option<CancelNode>>,waiter_capacity:usize}
impl CancelTokenRetirement{
    /// 🧳️ Transfers the original small token into inline custody, separately from retirement work.
    pub fn from_token(token:CancelToken)->Self{Self{current:ManuallyDrop::new(Some(token)),node:ManuallyDrop::new(None),waiter_capacity:0}}
    pub fn terminal_is_empty(&self)->bool{self.current.is_none()&&self.node.is_none()}
    /// 🔎️ Borrows the original root identity without changing cancellation custody.
    pub fn is_original_alias_witness(&self,witness:&CancelToken)->bool{self.node.is_none()&&self.current.as_ref().is_some_and(|original|Arc::ptr_eq(&original.0,&witness.0))}
    /// 📏️ Quotes only the selected original Arc, empty waiter backing or retained node metadata.
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.current.is_some(){return Ok(RetirementDemand{copy_bytes:0,release_bytes:shared_retirement_allocation_bytes::<CancelNode>(),depth:1,..Default::default()});}
        if self.node.is_none(){return Ok(Default::default());}
        if self.waiter_capacity>0{return Ok(RetirementDemand{copy_bytes:0,release_bytes:self.waiter_capacity.checked_mul(size_of::<(u64,Waker)>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"cancellation waiter backing overflow"))?,depth:1,..Default::default()});}
        Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})
    }
    /// ↩️ Returns this original token alias while its matching borrowed strong root stays live through the paid turn.
    fn alias_return_work(&self)->u64{u64::from(!self.terminal_is_empty())}
    pub fn return_alias_work_demand(&self)->Result<u64,ValueError>{Ok(self.alias_return_work())}
    pub fn return_alias_step(&mut self,witness:&CancelToken,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,CancelTokenRetirementError>{let grant=effect.grant();
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
        let work=self.alias_return_work();let demand=RetirementDemand{depth:1,..Default::default()};
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
        let remaining=match effect.remaining_work_units(){Ok(remaining)=>remaining,Err(error)=>return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(error)),work:0}};if remaining<work{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}match effect.performed(work){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(ValueError::literal(ValueRefusalKind::WorkLimit,"cancellation alias return exceeded its Work grant"))),work:u128::from(work)},Err(error)=>return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(error)),work:0}}
        let work=u128::from(work);let matches=self.current.as_ref().is_some_and(|original|Arc::ptr_eq(&original.0,&witness.0));
        if !matches{return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed cancellation root does not witness this original alias"))),work};}
        if self.node.is_some(){return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(ValueError::literal(ValueRefusalKind::InvariantViolated,"cancellation node custody cannot be returned as a root alias"))),work};}
        drop(self.current.take());
        RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work}
    }
    /// 🎟️ Releases one actual original frontier and retains every blocked original alias and ancestor.
    /// ⛽️ Quotes one original node inspection, buffer release or parent transition.
    pub fn retirement_work_demand(&self)->u64{u64::from(!self.terminal_is_empty())}
    pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,CancelTokenRetirementError>{let grant=effect.grant();
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
        let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(error)),work:0}};
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
        let remaining=match effect.remaining_work_units(){Ok(remaining)=>remaining,Err(error)=>return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(error)),work:0}};if remaining==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(ValueError::literal(ValueRefusalKind::WorkLimit,"cancellation close exceeded its Work grant"))),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(CancelTokenRetirementError::Refused(error)),work:0}}
        let mut work:u128=1;
        let result=(||{
            if let Some(current)=self.current.as_ref(){
                let observed=current.0.waiters.try_lock();work=1;
                {let waiters=observed.ok_or(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::WaiterContention))?;if !waiters.is_empty(){return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::RegisteredWaiter));}}
                if Arc::weak_count(&current.0)>0{return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::WeakAlias));}
                if Arc::strong_count(&current.0)>1{return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::SharedAlias));}
                let Some(CancelToken(original))=self.current.take()else{return Ok(RetainedCloneStep::Complete(Default::default()))};return match Arc::try_unwrap(original){Ok(mut node)=>{self.waiter_capacity=node.waiters.get_mut().capacity();*self.node=Some(node);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))},Err(original)=>{*self.current=Some(CancelToken(original));Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::SharedAlias))}};
            }
            let Some(node)=self.node.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};let waiters=node.waiters.get_mut();
            if !waiters.is_empty(){work=1;return Err(CancelTokenRetirementError::Blocked(CancelTokenRetirementBlocked::RegisteredWaiter));}
            if waiters.capacity()>0{drop(std::mem::take(waiters));self.waiter_capacity=0;work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}));}
            self.waiter_capacity=0;let Some(mut node)=self.node.take()else{return Ok(RetainedCloneStep::Complete(Default::default()))};*self.current=node.parent.take();drop(node);work=1;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
        })();
        RetainedWorkOutcome{result,work}
    }
}
#[cfg(feature="testkit")]
impl CancelTokenRetirement{
    /// 🔬️ Borrows the original token still held before its node is captured.
    #[doc(hidden)]
    pub fn current(&self)->Option<&CancelToken>{self.current.as_ref()}
    /// 🔬️ Borrows the captured original node.
    #[doc(hidden)]
    pub fn node(&self)->Option<&CancelNode>{self.node.as_ref()}
    /// 🔬️ Erases this custody behind the typed retirement ladder cursor.
    #[doc(hidden)]
    pub fn into_cursor(self)->CancelTokenRetirementCursor{CancelTokenRetirementCursor(self)}
}
impl Drop for CancelTokenRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"cancellation retirement abandoned its original node");}}
/// 🪜️ Erases original cancellation custody behind the typed retirement ladder of owning containers.
#[doc(hidden)]
pub struct CancelTokenRetirementCursor(CancelTokenRetirement);
impl RetireOwned for CancelToken{
    type RetirementCursor=CancelTokenRetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor{CancelTokenRetirementCursor(CancelTokenRetirement::from_token(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<CancelTokenRetirementCursor>())}
    fn controlled_retirement_supported()->bool{true}
    fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
impl RetirementCursor for CancelTokenRetirementCursor{
    fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(self.0.retirement_work_demand())}
    fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.0.retirement_demands()}

    fn close_into<'r>(&mut self,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>,semio_framework_value::ValueError>{Ok(reply.complete(Ok(self.close_sync(effect))))}
    fn recover_close_into<'r>(&mut self,_body:usize,_:&mut semio_framework_value::RetainedAllocationEffect<'_>,_:semio_framework_value::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::invariant("synchronous close has no unrecorded callback"))}
    fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands()?.depth)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands()?.copy_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.retirement_demands().ok().map(|demand|demand.release_bytes)}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl CancelTokenRetirementCursor{
 fn close_sync(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetirementStep{
        match effect.remaining_work_units(){Ok(work)if work>=self.0.retirement_work_demand()=>{},Ok(_)=>return RetirementStep::BudgetExhausted,Err(error)=>return RetirementStep::Failure(error.into(),0)}let turn=self.0.close_step(effect);
        match turn.result{
            Ok(step)=>{let progress=step.progress();if progress!=RetainedCloneProgress::default()||turn.work!=0{RetirementStep::Progress(progress,turn.work)}else if self.0.terminal_is_empty(){RetirementStep::Complete(turn.work)}else{RetirementStep::BudgetExhausted}}
            Err(CancelTokenRetirementError::Blocked(blocked))=>RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,match blocked{CancelTokenRetirementBlocked::SharedAlias=>"original cancellation node retains a shared alias",CancelTokenRetirementBlocked::WeakAlias=>"original cancellation node retains a weak alias",CancelTokenRetirementBlocked::RegisteredWaiter=>"original cancellation node retains a registered wake owner",CancelTokenRetirementBlocked::WaiterContention=>"original cancellation waiter guard is occupied"}).into(),turn.work),
            Err(CancelTokenRetirementError::Refused(error))=>RetirementStep::Failure(error.into(),turn.work),
        }
    }
}

#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod actual_work_tests;
