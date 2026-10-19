//! 📋️ Growing fold vectors retain the exact incoming member and original backing through separately granted growth and push.
use super::*;

struct FoldVectorPush<'a,T:RetireOwned> {
    control:HistoryFoldControl,
    target:&'a mut Vec<T>,
    member:&'a mut Option<T>,
    capacity:usize,
    demand:RetirementDemand,
}
impl<T:RetireOwned> Unpin for FoldVectorPush<'_,T>{}
impl<T:RetireOwned> FoldVectorPush<'_,T>{
    fn quote(&self)->Result<(usize,RetirementDemand),ValueError>{
        let width=size_of::<T>();
        if self.target.len()<self.target.capacity(){return Ok((self.target.capacity(),RetirementDemand {copy_bytes:size_of::<Vec<T>>()+size_of::<Option<T>>()+width,depth:1,..Default::default()}))}
        let capacity=self.target.capacity().checked_mul(2).and_then(|capacity|self.target.len().checked_add(1).map(|minimum|capacity.max(minimum))).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fold vector growth count overflow"))?;
        let bytes=capacity.checked_mul(width).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fold vector growth extent overflow"))?;
        let previous=self.target.capacity().checked_mul(width).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fold vector original extent overflow"))?;
        let copied=previous.checked_add(2*size_of::<Vec<T>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fold vector growth copy overflow"))?;
        Ok((capacity,RetirementDemand {copy_bytes:copied,capacity_bytes:bytes,release_bytes:previous,depth:1,..Default::default()}))
    }
    fn register(&mut self,capacity:usize,demand:RetirementDemand){
        self.control.0.pending_construction_capacity.fetch_sub(self.demand.capacity_bytes,Ordering::Relaxed);
        self.control.0.pending_original_copy.fetch_sub(self.demand.copy_bytes,Ordering::Relaxed);
        self.control.0.pending_original_release.fetch_sub(self.demand.release_bytes,Ordering::Relaxed);
        self.capacity=capacity;self.demand=demand;
        self.control.0.pending_construction_capacity.fetch_add(demand.capacity_bytes,Ordering::Relaxed);
        self.control.0.pending_original_copy.fetch_add(demand.copy_bytes,Ordering::Relaxed);
        self.control.0.pending_original_release.fetch_add(demand.release_bytes,Ordering::Relaxed);
    }
}
impl<T:RetireOwned> Future for FoldVectorPush<'_,T>{
    type Output=Result<(),semio_framework_replication_error::ProtocolError>;
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let this=self.get_mut();let grant=this.control.original_grant();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,this.demand){return Poll::Pending;}
        if this.target.len()<this.target.capacity(){
            this.target.push(this.member.take().expect("original vector member remains retained until its admitted push"));
            this.control.accept_original_progress(RetainedCloneProgress {copied_items:1,copied_bytes:this.demand.copy_bytes,..Default::default()});
            this.register(0,Default::default());return Poll::Ready(Ok(()));
        }
        let previous=this.target.capacity();let additional=this.capacity-this.target.len();
        if this.target.try_reserve_exact(additional).is_err(){return Poll::Ready(Err(value_cause(ValueError::literal(ValueRefusalKind::AllocationFailed,"original fold vector growth failed"))))}
        let progress=RetainedCloneProgress {copied_items:1,copied_bytes:this.demand.copy_bytes,retained_capacity_bytes:this.target.capacity()*size_of::<T>(),released_bytes:previous*size_of::<T>()};
        this.control.accept_original_progress(progress);
        match this.quote(){Ok((capacity,demand))=>this.register(capacity,demand),Err(error)=>return Poll::Ready(Err(value_cause(error)))}
        Poll::Pending
    }
}
impl<T:RetireOwned> Drop for FoldVectorPush<'_,T>{fn drop(&mut self){self.register(0,Default::default());}}

impl HistoryFoldControl{
    /// 🪴️ An existing vector grows geometrically under actual copy, allocation and old-backing release authority.
    pub async fn push_vec<T:RetireOwned>(&self,target:&mut Vec<T>,value:T)->Result<(),semio_framework_replication_error::ProtocolError>{
        let mut member=self.track(Some(value)).unwrap_or_else(|_|unreachable!("original vector member declares controlled retirement")).await;
        let mut push=FoldVectorPush {control:self.clone(),target,member:&mut member,capacity:0,demand:Default::default()};
        let(capacity,demand)=push.quote().map_err(value_cause)?;push.register(capacity,demand);push.await
    }
}
