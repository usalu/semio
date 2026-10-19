//! 👁️ Immutable original indexes expose borrowed payloads only after bounded source reads under the coroutine grant.
use super::*;
use super::index::{text_slice_demand,copy_demand,compare_copy,actor_demand,compare_actor};

struct FoldIndexRead<'a,'q,K:Ord,V,Q:?Sized>{
    control:HistoryFoldControl,index:&'a HistoryFoldIndex<K,V>,key:&'q Q,probe:HistoryFoldIndexRead,
    compare:fn(&Q,&K,usize,RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>,
    comparison_demand:fn(&Q,&K,usize,usize)->Result<RetirementDemand,ValueError>,demand:RetirementDemand,
}
impl<K:Ord,V,Q:?Sized> Unpin for FoldIndexRead<'_,'_,K,V,Q>{}
impl<K:Ord,V,Q:?Sized> FoldIndexRead<'_,'_,K,V,Q>{
    fn quote(&self)->Result<RetirementDemand,ValueError>{self.probe.next_demands(self.index,self.key,self.control.page_bytes(),self.comparison_demand)}
    fn register(&mut self,demand:RetirementDemand){
        self.control.0.pending_original_copy.fetch_sub(self.demand.copy_bytes,Ordering::Relaxed);self.control.0.pending_original_depth.fetch_sub(self.demand.depth,Ordering::Relaxed);self.demand=demand;
        self.control.0.pending_original_copy.fetch_add(demand.copy_bytes,Ordering::Relaxed);self.control.0.pending_original_depth.fetch_add(demand.depth,Ordering::Relaxed);
    }
}
impl<'a,K:Ord,V,Q:?Sized> Future for FoldIndexRead<'a,'_,K,V,Q>{
    type Output=Result<Option<HistoryFoldIndexMatch>,semio_framework_replication_error::ProtocolError>;
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let this=self.get_mut();let grant=this.control.original_grant();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,this.demand){return Poll::Pending;}
        match this.probe.step(this.index,this.key,grant,this.compare){
            Ok((ready,progress))=>{
                this.control.accept_original_progress(progress);
                if ready.is_some(){let matched=this.probe.matched(this.index).map_err(value_cause);this.register(Default::default());return Poll::Ready(matched);}
                match this.quote(){Ok(demand)=>this.register(demand),Err(error)=>return Poll::Ready(Err(value_cause(error)))}
                Poll::Pending
            }
            Err(error)=>{this.control.accept_original_progress(error.retained_progress());Poll::Ready(Err(value_cause(error)))}
        }
    }
}
impl<K:Ord,V,Q:?Sized> Drop for FoldIndexRead<'_,'_,K,V,Q>{fn drop(&mut self){self.register(Default::default());}}

impl HistoryFoldControl{
    /// 🔤️ Each original UTF8 key page, index node and final payload borrow is quoted before reading.
    pub async fn get_text<'a,V:Sync>(&self,index:&'a HistoryFoldIndex<String,V>,key:&str)->Result<Option<&'a V>,semio_framework_replication_error::ProtocolError>{
        match self.read_index(index,key,|left,right,position,grant|HistoryFoldIndexInsert::<String,()>::compare_original_text(left,right,position,grant),|left,right,position,body|text_slice_demand(left,right,position,body)).await?{Some(found)=>found.value(index).map(Some).map_err(value_cause),None=>Ok(None)}
    }

    /// 🔢️ Scalar original membership uses one granted fixed-key comparison per native node.
    pub async fn get_copy<'a,K:Copy+Ord+Sync,V:Sync>(&self,index:&'a HistoryFoldIndex<K,V>,key:&K)->Result<Option<&'a V>,semio_framework_replication_error::ProtocolError>{match self.read_index(index,key,compare_copy::<K>,copy_demand::<K>).await?{Some(found)=>found.value(index).map(Some).map_err(value_cause),None=>Ok(None)}}

    /// 👤️ A paid original actor probe returns its actual mutable child after source and epoch validation.
    pub async fn get_actor_mut<'a,V:RetireOwned+Sync>(&self,index:&'a mut HistoryFoldIndex<Option<semio_framework_value::SharedUtf8>,V>,key:&Option<semio_framework_value::SharedUtf8>)->Result<Option<&'a mut V>,semio_framework_replication_error::ProtocolError>{
        match self.read_index(&*index,key,compare_actor,actor_demand).await?{Some(found)=>found.value_mut(index).map(Some).map_err(value_cause),None=>Ok(None)}
    }

    pub(super) async fn find_text<V:Sync>(&self,index:&HistoryFoldIndex<String,V>,key:&str)->Result<Option<HistoryFoldIndexMatch>,semio_framework_replication_error::ProtocolError>{self.read_index(index,key,|left,right,position,grant|HistoryFoldIndexInsert::<String,()>::compare_original_text(left,right,position,grant),|left,right,position,body|text_slice_demand(left,right,position,body)).await}
    pub(super) async fn find_copy<K:Copy+Ord+Sync,V:Sync>(&self,index:&HistoryFoldIndex<K,V>,key:&K)->Result<Option<HistoryFoldIndexMatch>,semio_framework_replication_error::ProtocolError>{self.read_index(index,key,compare_copy::<K>,copy_demand::<K>).await}

    async fn read_index<'a,K:Ord+Sync,V:Sync,Q:Sync+?Sized>(&self,index:&'a HistoryFoldIndex<K,V>,key:&Q,compare:fn(&Q,&K,usize,RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>,comparison_demand:fn(&Q,&K,usize,usize)->Result<RetirementDemand,ValueError>)->Result<Option<HistoryFoldIndexMatch>,semio_framework_replication_error::ProtocolError>{
        let mut read=FoldIndexRead {control:self.clone(),index,key,probe:HistoryFoldIndexRead::new(),compare,comparison_demand,demand:Default::default()};
        let demand=read.quote().map_err(value_cause)?;read.register(demand);read.await
    }
}
