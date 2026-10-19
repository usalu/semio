//! 🌳️ Original text insertion retains its incoming and displaced rows through every granted AVL transition.
use super::*;

struct FoldIndexInsert<'a,K:Ord+RetireOwned,V:RetireOwned> {
    control: HistoryFoldControl,
    index: &'a mut HistoryFoldIndex<K,V>,
    cursor: &'a mut HistoryFoldIndexInsert<K,V>,
    displaced: &'a mut Option<(K,V)>,
    compare: fn(&K,&K,usize,RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>,
    comparison_demand: fn(&K,&K,usize,usize)->Result<RetirementDemand,ValueError>,
    ready: bool,
    replace: bool,
    demand: RetirementDemand,
}

impl<K:Ord+RetireOwned,V:RetireOwned> Unpin for FoldIndexInsert<'_,K,V> {}

impl<K:Ord+RetireOwned,V:RetireOwned> FoldIndexInsert<'_,K,V> {
    fn quote(&self) -> Result<RetirementDemand,ValueError> {
        if self.ready {
            let mut demand=self.cursor.next_demands(self.index,0,|_,_,_,_|Ok(Default::default()))?;
            demand.copy_bytes=self.cursor.commit_copy_byte_demand(self.index)?;
            return Ok(demand);
        }
        self.cursor.next_controlled_demands(self.index,self.control.page_bytes(),self.comparison_demand)
    }

    fn register(&mut self,demand:RetirementDemand) {
        self.control.0.pending_construction_capacity.fetch_sub(self.demand.capacity_bytes,Ordering::Relaxed);
        self.control.0.pending_original_copy.fetch_sub(self.demand.copy_bytes,Ordering::Relaxed);
        self.control.0.pending_original_depth.fetch_sub(self.demand.depth,Ordering::Relaxed);
        self.demand=demand;
        self.control.0.pending_construction_capacity.fetch_add(demand.capacity_bytes,Ordering::Relaxed);
        self.control.0.pending_original_copy.fetch_add(demand.copy_bytes,Ordering::Relaxed);
        self.control.0.pending_original_depth.fetch_add(demand.depth,Ordering::Relaxed);
    }
}

impl<K:Ord+RetireOwned,V:RetireOwned> Future for FoldIndexInsert<'_,K,V> {
    type Output=Result<bool,semio_framework_replication_error::ProtocolError>;
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output> {
        let this=self.get_mut();
        if this.ready&&!this.replace&&this.cursor.matching_slot().is_some(){this.register(Default::default());return Poll::Ready(Ok(false));}
        let grant=this.control.original_grant();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,this.demand){return Poll::Pending;}
        let result=if this.ready {
            this.cursor.commit_or_replace_controlled(this.index,grant,|value|value).map(|(slot,original,progress)|{*this.displaced=original;(slot.is_some(),progress)})
        }else{
            this.cursor.step_controlled(this.index,grant,this.compare).map(|(ready,progress)|{this.ready=ready;(false,progress)})
        };
        match result {
            Ok((done,progress))=>{
                this.control.accept_original_progress(progress);
                if done {let inserted=this.displaced.is_none();this.register(Default::default());return Poll::Ready(Ok(inserted));}
                match this.quote(){Ok(demand)=>this.register(demand),Err(error)=>return Poll::Ready(Err(value_cause(error)))}
                Poll::Pending
            }
            Err(error)=>{this.control.accept_original_progress(error.retained_progress());Poll::Ready(Err(value_cause(error)))}
        }
    }
}

impl<K:Ord+RetireOwned,V:RetireOwned> Drop for FoldIndexInsert<'_,K,V> {fn drop(&mut self){self.register(Default::default());}}

fn text_demand(left:&String,right:&String,position:usize,body:usize)->Result<RetirementDemand,ValueError>{
    text_slice_demand(left,right,position,body)
}

pub(super) fn text_slice_demand(left:&str,right:&str,position:usize,body:usize)->Result<RetirementDemand,ValueError>{
    let remaining=left.len().min(right.len()).checked_sub(position).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original text insertion cursor exceeds its keys"))?;
    Ok(RetirementDemand {copy_bytes:size_of::<usize>()+size_of::<Option<std::cmp::Ordering>>()+2*remaining.min(body/2),depth:1,..Default::default()})
}

fn ordered_text_demand<K>(left:&(K,String),right:&(K,String),position:usize,body:usize)->Result<RetirementDemand,ValueError>{
    let mut demand=text_demand(&left.1,&right.1,position,body)?;
    demand.copy_bytes=demand.copy_bytes.checked_add(copy_demand(&left.0,&right.0,0,0)?.copy_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"ordered original key quote overflow"))?;
    Ok(demand)
}

fn compare_ordered_text<K:Ord>(left:&(K,String),right:&(K,String),position:usize,grant:RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>{
    let fixed=copy_demand(&left.0,&right.0,0,0)?;
    if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,fixed){return Ok((None,position,Default::default()))}
    let order=left.0.cmp(&right.0);
    if order!=std::cmp::Ordering::Equal{return Ok((Some(order),0,RetainedCloneProgress {copied_items:1,copied_bytes:fixed.copy_bytes,..Default::default()}))}
    let(order,position,mut progress)=HistoryFoldIndexInsert::<String,()>::compare_original_text(&left.1,&right.1,position,RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes-fixed.copy_bytes,..grant})?;
    if progress.copied_items!=0{progress.copied_bytes+=fixed.copy_bytes;}
    Ok((order,position,progress))
}

pub(super) fn actor_demand(left:&Option<semio_framework_value::SharedUtf8>,right:&Option<semio_framework_value::SharedUtf8>,position:usize,body:usize)->Result<RetirementDemand,ValueError>{
    match(left,right){(Some(left),Some(right))=>text_slice_demand(left,right,position,body),_=>Ok(RetirementDemand {copy_bytes:2*size_of::<Option<semio_framework_value::SharedUtf8>>(),depth:1,..Default::default()})}
}

pub(super) fn compare_actor(left:&Option<semio_framework_value::SharedUtf8>,right:&Option<semio_framework_value::SharedUtf8>,position:usize,grant:RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>{
    match(left,right){(Some(left),Some(right))=>HistoryFoldIndexInsert::<String,()>::compare_original_text(left,right,position,grant),_=>{let demand=actor_demand(left,right,position,0)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok((None,position,Default::default()))}Ok((Some(left.cmp(right)),0,RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}}
}

pub(super) fn copy_demand<K>(_:&K,_:&K,position:usize,_:usize)->Result<RetirementDemand,ValueError>{
    if position!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixed original key has a text offset"))}
    let copy_bytes=size_of::<K>().checked_mul(2).and_then(|bytes|bytes.checked_add(size_of::<Option<std::cmp::Ordering>>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixed key quote overflow"))?;
    Ok(RetirementDemand {copy_bytes,depth:1,..Default::default()})
}

pub(super) fn compare_copy<K:Ord>(left:&K,right:&K,position:usize,grant:RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>{
    let demand=copy_demand(left,right,position,0)?;
    if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok((None,position,Default::default()))}
    Ok((Some(left.cmp(right)),0,RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
}

impl HistoryFoldControl {
    /// 🪴️ Every comparison, page birth and commit uses the coroutine's original grant; cancellation retains both rows.
    pub async fn insert_text<V:RetireOwned>(&self,index:&mut HistoryFoldIndex<String,V>,key:String,value:V)->Result<bool,semio_framework_replication_error::ProtocolError> {
        self.insert_original_index(index,key,value,true,|left,right,position,grant|HistoryFoldIndexInsert::<String,()>::compare_original_text(left,right,position,grant),text_demand).await
    }

    /// 🪹️ Occupied rows preserve the original first value and retire the supplied unused row under the same source.
    pub async fn get_or_insert_text<V:RetireOwned>(&self,index:&mut HistoryFoldIndex<String,V>,key:String,value:V)->Result<bool,semio_framework_replication_error::ProtocolError> {
        self.insert_original_index(index,key,value,false,|left,right,position,grant|HistoryFoldIndexInsert::<String,()>::compare_original_text(left,right,position,grant),text_demand).await
    }

    /// 🔢️ Fixed Copy keys admit each bounded comparison, native page and structural commit separately.
    pub async fn insert_copy<K:Copy+Ord+RetireOwned,V:RetireOwned>(&self,index:&mut HistoryFoldIndex<K,V>,key:K,value:V)->Result<bool,semio_framework_replication_error::ProtocolError>{self.insert_original_index(index,key,value,true,compare_copy::<K>,copy_demand::<K>).await}

    /// 🪹️ Fixed-key membership retains the first original value and closes an unused incoming row explicitly.
    pub async fn get_or_insert_copy<K:Copy+Ord+RetireOwned,V:RetireOwned>(&self,index:&mut HistoryFoldIndex<K,V>,key:K,value:V)->Result<bool,semio_framework_replication_error::ProtocolError>{self.insert_original_index(index,key,value,false,compare_copy::<K>,copy_demand::<K>).await}

    /// 🧭️ Ordered event keys compare their fixed clock prefix and bounded original UTF8 identity separately.
    pub async fn insert_ordered_text<K:Copy+Ord+RetireOwned,V:RetireOwned>(&self,index:&mut HistoryFoldIndex<(K,String),V>,key:(K,String),value:V)->Result<bool,semio_framework_replication_error::ProtocolError>{self.insert_original_index(index,key,value,true,compare_ordered_text::<K>,ordered_text_demand::<K>).await}

    /// 👤️ The original optional shared actor is preserved when its native history row is already occupied.
    pub async fn get_or_insert_actor<V:RetireOwned>(&self,index:&mut HistoryFoldIndex<Option<semio_framework_value::SharedUtf8>,V>,key:Option<semio_framework_value::SharedUtf8>,value:V)->Result<bool,semio_framework_replication_error::ProtocolError>{self.insert_original_index(index,key,value,false,compare_actor,actor_demand).await}

    /// 🪪️ Supersession keys reuse the actual bounded text comparator while preserving their mutation identity type.
    pub async fn insert_mutation<V:RetireOwned>(&self,index:&mut HistoryFoldIndex<MutationId,V>,key:MutationId,value:V)->Result<bool,semio_framework_replication_error::ProtocolError>{self.insert_original_index(index,key,value,true,|left,right,position,grant|HistoryFoldIndexInsert::<String,()>::compare_original_text(&left.0,&right.0,position,grant),|left,right,position,body|text_demand(&left.0,&right.0,position,body)).await}

    async fn insert_original_index<K:Ord+RetireOwned,V:RetireOwned>(&self,index:&mut HistoryFoldIndex<K,V>,key:K,value:V,replace:bool,compare:fn(&K,&K,usize,RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>,comparison_demand:fn(&K,&K,usize,usize)->Result<RetirementDemand,ValueError>)->Result<bool,semio_framework_replication_error::ProtocolError> {
        let mut cursor=self.track(HistoryFoldIndexInsert::new(key,value)).unwrap_or_else(|_|unreachable!("original text row declares controlled retirement")).await;
        let mut displaced=self.track(None::<(K,V)>).unwrap_or_else(|_|unreachable!("displaced original row declares controlled retirement")).await;
        let mut insert=FoldIndexInsert {control:self.clone(),index,cursor:&mut cursor,displaced:&mut displaced,compare,comparison_demand,ready:false,replace,demand:Default::default()};
        let demand=insert.quote().map_err(value_cause)?;
        insert.register(demand);
        insert.await
    }
}
