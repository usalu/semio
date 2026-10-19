//! 🍂️ Original rank extraction retains the complete row in its preadmitted typed owner before publishing progress.
use super::*;

struct FoldIndexExtract<'a,K:Ord,V>{
    control:HistoryFoldControl,index:&'a mut HistoryFoldIndex<K,V>,output:&'a mut Option<(K,V)>,
    probe:HistoryFoldIndexRead,matched:Option<HistoryFoldIndexMatch>,searching:bool,demand:RetirementDemand,
}
impl<K:Ord,V> Unpin for FoldIndexExtract<'_,K,V>{}
impl<K:Ord,V> FoldIndexExtract<'_,K,V>{
    fn quote(&self)->Result<RetirementDemand,ValueError>{match self.matched{Some(found)=>found.remove_demand(self.index),None if self.searching=>self.probe.first_demand(self.index),None=>Ok(Default::default())}}
    fn register(&mut self,demand:RetirementDemand){
        self.control.0.pending_original_copy.fetch_sub(self.demand.copy_bytes,Ordering::Relaxed);self.control.0.pending_original_depth.fetch_sub(self.demand.depth,Ordering::Relaxed);self.demand=demand;
        self.control.0.pending_original_copy.fetch_add(demand.copy_bytes,Ordering::Relaxed);self.control.0.pending_original_depth.fetch_add(demand.depth,Ordering::Relaxed);
    }
}
impl<K:Ord,V> Future for FoldIndexExtract<'_,K,V>{
    type Output=Result<(),semio_framework_replication_error::ProtocolError>;
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let this=self.get_mut();let grant=this.control.original_grant();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,this.demand){return Poll::Pending;}
        if let Some(found)=this.matched{
            match found.remove_into(this.index,this.output,grant){
                Ok(progress)=>{this.control.accept_original_progress(progress);if this.output.is_some(){this.register(Default::default());return Poll::Ready(Ok(()));}}
                Err(error)=>{this.control.accept_original_progress(error.retained_progress());return Poll::Ready(Err(value_cause(error)))}
            }
        }else if this.searching{
            match this.probe.first_step(this.index,grant){
                Ok((ready,progress))=>{this.control.accept_original_progress(progress);if ready{this.searching=false;match this.probe.matched(this.index){Ok(found)=>this.matched=found,Err(error)=>return Poll::Ready(Err(value_cause(error)))}if this.matched.is_none(){this.register(Default::default());return Poll::Ready(Ok(()));}}}
                Err(error)=>{this.control.accept_original_progress(error.retained_progress());return Poll::Ready(Err(value_cause(error)))}
            }
        }else{this.register(Default::default());return Poll::Ready(Ok(()));}
        match this.quote(){Ok(demand)=>this.register(demand),Err(error)=>return Poll::Ready(Err(value_cause(error)))}
        Poll::Pending
    }
}
impl<K:Ord,V> Drop for FoldIndexExtract<'_,K,V>{fn drop(&mut self){self.register(Default::default());}}

impl HistoryFoldControl{
    /// 🌱️ The minimum probe and rank removal use separate grants; every extracted key and payload remains resident.
    pub async fn pop_first<K:Ord+RetireOwned,V:RetireOwned>(&self,index:&mut HistoryFoldIndex<K,V>)->Result<HistoryFoldOwned<Option<(K,V)>>,semio_framework_replication_error::ProtocolError>{self.extract_index(index,None,true).await}

    /// 🔤️ Text removal preserves the original key in paid retirement and returns only the Copy payload.
    pub async fn remove_text<V:Copy+RetireOwned+Sync>(&self,index:&mut HistoryFoldIndex<String,V>,key:&str)->Result<Option<V>,semio_framework_replication_error::ProtocolError>{
        let Some(found)=self.find_text(&*index,key).await?else{return Ok(None)};
        let row=self.extract_index(index,Some(found),false).await?;
        let value=row.as_ref().map(|(_,value)|*value);drop(row);Ok(value)
    }

    /// 🔢️ Scalar removal never grows a displaced-key buffer; the whole original row receives paid retirement.
    pub async fn remove_copy<K:Copy+Ord+RetireOwned+Sync,V:Copy+RetireOwned+Sync>(&self,index:&mut HistoryFoldIndex<K,V>,key:&K)->Result<Option<V>,semio_framework_replication_error::ProtocolError>{
        let Some(found)=self.find_copy(&*index,key).await?else{return Ok(None)};
        let row=self.extract_index(index,Some(found),false).await?;
        let value=row.as_ref().map(|(_,value)|*value);drop(row);Ok(value)
    }

    async fn extract_index<K:Ord+RetireOwned,V:RetireOwned>(&self,index:&mut HistoryFoldIndex<K,V>,matched:Option<HistoryFoldIndexMatch>,searching:bool)->Result<HistoryFoldOwned<Option<(K,V)>>,semio_framework_replication_error::ProtocolError>{
        let mut row=self.track(None::<(K,V)>).unwrap_or_else(|_|unreachable!("original extracted row declares controlled retirement")).await;
        let mut extract=FoldIndexExtract {control:self.clone(),index,output:&mut row,probe:HistoryFoldIndexRead::new(),matched,searching,demand:Default::default()};
        let demand=extract.quote().map_err(value_cause)?;extract.register(demand);extract.await?;
        Ok(row)
    }
}
