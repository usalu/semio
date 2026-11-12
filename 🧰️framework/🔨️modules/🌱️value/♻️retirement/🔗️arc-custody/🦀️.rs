//! 🔗️ Shared immutable owners retain Arc backing custody until aliases and weak leases close.

semio_framework_value_retirement_contract::shared_controlled_owner!();


use super::{admit_retirement_cursor,record_retirement_work,retirement_cursor_birth_demand};

/// 🔗️ A std Arc read lease retires through [`ArcLeaseRetirement`]: it releases its Arc backing, then retires the last payload as a separately funded child.
#[derive(Clone,Debug,PartialEq)]
pub struct ArcLease<T:RetireOwned+Sync>(pub Option<Arc<T>>);
impl<T:RetireOwned+Sync> ArcLease<T>{
    pub fn new(source:Arc<T>)->Self{Self(Some(source))}
    /// 🔗️ Borrows the leased allocation while the lease still holds it.
    pub fn get(&self)->Result<&T,ValueError>{self.0.as_deref().ok_or_else(||ValueError::invariant("Arc lease is empty"))}
    /// 📍️ The leased allocation's address, or zero once the lease is released.
    pub fn addr(&self)->usize{self.0.as_ref().map_or(0,|source|Arc::as_ptr(source) as usize)}
}
impl<T:RetireOwned+Sync> RetireOwned for ArcLease<T>{
    type RetirementCursor=ArcLeaseRetirement<T>;
    fn retirement_seed(self)->Self::RetirementCursor{ArcLeaseRetirement::from_source(self.0)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ArcLeaseRetirement<T>>())}
    fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
/// 🪪️ The one original Arc lease cursor: Arc backing release, then the elected payload as a child cursor.
#[doc(hidden)]
pub struct ArcLeaseRetirement<T:RetireOwned+Sync>{source:Option<Arc<T>>,payload:Option<T>}
impl<T:RetireOwned+Sync> ArcLeaseRetirement<T>{
    pub fn lease(source:Arc<T>)->Self{Self::from_source(Some(source))}
    pub fn from_source(source:Option<Arc<T>>)->Self{Self{source,payload:None}}
    pub fn close_step(&mut self,_body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{
        if self.terminal_is_empty(){return RetirementStep::Complete(0)}
        if self.payload.is_some(){let turn=admit_retirement_cursor(&mut self.payload,effect);return match turn.result{Ok(Some((cursor,progress)))=>RetirementStep::ChildProgress(cursor,progress,turn.work),Ok(None)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error.into(),turn.work)}}
        let demand=self.next_demand(0).expect("Arc lease demand");
        if effect.maximum_work_units()==0||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetirementStep::BudgetExhausted}
        let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};
        let source=self.source.take().expect("Arc lease source");
        let released_bytes=match Arc::into_inner(source){Some(payload)=>{self.payload=Some(payload);demand.release_bytes},None=>0};
        RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()},work)
    }
}
impl<T:RetireOwned+Sync> RetirementCursor for ArcLeaseRetirement<T>{
    crate::native_retirement_reply!();
    fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(!self.terminal_is_empty()))}
    fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,ValueError>{
        if let Some(payload)=self.payload.as_ref(){return retirement_cursor_birth_demand(payload)}
        Ok(self.source.as_ref().map_or_else(Default::default,|source|crate::RetirementDemand{copy_bytes:size_of::<Option<Arc<T>>>()+size_of::<Option<T>>(),release_bytes:if Arc::weak_count(source)==0{shared_retirement_allocation_bytes::<T>()}else{0},depth:1,..Default::default()}))
    }
    fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.payload.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
    fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}

