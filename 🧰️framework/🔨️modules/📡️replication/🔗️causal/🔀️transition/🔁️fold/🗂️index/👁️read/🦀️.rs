//! 👁️ Borrowed native index probes retain only scalar position and never clone an original key or payload.
use super::*;

#[derive(Clone,Copy,Default)]
pub struct HistoryFoldIndexRead {identity:usize,epoch:Option<u64>,next:Option<usize>,position:usize,rank:usize,finished:Option<Option<usize>>}
#[derive(Clone,Copy)]
pub struct HistoryFoldIndexMatch {identity:usize,epoch:u64,slot:usize,rank:usize}
impl HistoryFoldIndexRead {
    pub fn new()->Self{Self::default()}
    fn bound<K:Ord,V>(&self,index:&HistoryFoldIndex<K,V>)->bool{self.identity==index as *const _ as usize&&self.epoch==Some(index.mutation_epoch)}
    pub fn matched<K:Ord,V>(&self,index:&HistoryFoldIndex<K,V>)->Result<Option<HistoryFoldIndexMatch>,ValueError>{
        if !self.bound(index){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original index match changed its source"))}
        Ok(self.finished.flatten().map(|slot|HistoryFoldIndexMatch {identity:self.identity,epoch:self.epoch.unwrap(),slot,rank:self.rank}))
    }

    /// 📐️ The original cursor, node header, page path and next bounded comparison supply one complete quote.
    pub fn next_demands<K:Ord,V,Q:?Sized>(&self,index:&HistoryFoldIndex<K,V>,key:&Q,body:usize,compare:impl FnOnce(&Q,&K,usize,usize)->Result<semio_framework_value::RetirementDemand,ValueError>)->Result<semio_framework_value::RetirementDemand,ValueError>{
        use semio_framework_value::RetirementDemand;
        if self.finished.is_some()&&self.bound(index){return Ok(Default::default())}
        let metadata=2*size_of::<Self>();
        if !self.bound(index)||self.next.is_none(){return Ok(RetirementDemand {copy_bytes:metadata,depth:1,..Default::default()})}
        let slot=self.next.unwrap();let(candidate,_)=index.nodes[slot].entry.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original read quote lost its live node"))?;
        let mut demand=compare(key,candidate,self.position,body)?;
        demand.copy_bytes=demand.copy_bytes.checked_add(metadata).and_then(|bytes|bytes.checked_add(2*size_of::<FoldIndexNode<K,V>>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original read header quote overflow"))?;
        demand.depth=demand.depth.max(index.next_slot_depth_demand(slot)?);
        Ok(demand)
    }

    /// 🧭️ One actual grant advances one original node or key byte page; mutation epochs restart the borrowed probe.
    pub fn step<K:Ord,V,Q:?Sized>(&mut self,index:&HistoryFoldIndex<K,V>,key:&Q,grant:RetainedCloneGrant,compare:impl FnOnce(&Q,&K,usize,RetainedCloneGrant)->Result<(Option<Ordering>,usize,RetainedCloneProgress),ValueError>)->Result<(Option<Option<usize>>,RetainedCloneProgress),ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.finished.is_some()&&self.bound(index){return Ok((self.finished,empty))}
        let metadata=2*size_of::<Self>();
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<metadata{return Ok((None,empty))}
        if !self.bound(index){self.identity=index as *const _ as usize;self.epoch=Some(index.mutation_epoch);self.next=index.root;self.position=0;self.rank=0;self.finished=None;return Ok((None,RetainedCloneProgress {copied_items:1,copied_bytes:metadata,..empty}))}
        let Some(slot)=self.next else{self.finished=Some(None);return Ok((self.finished,RetainedCloneProgress {copied_items:1,copied_bytes:metadata,..empty}))};
        let header=metadata+2*size_of::<FoldIndexNode<K,V>>();
        if grant.maximum_copy_bytes<header||grant.maximum_depth<index.next_slot_depth_demand(slot)?{return Ok((None,empty))}
        let node=&index.nodes[slot];let(candidate,_)=node.entry.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original read lost its live node"))?;
        let child=RetainedCloneGrant {maximum_copy_bytes:grant.maximum_copy_bytes-header,..grant};
        let(order,position,mut progress)=match compare(key,candidate,self.position,child){Ok(result)=>result,Err(error)=>{let mut progress=error.retained_progress();progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.checked_add(header).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original failed read receipt overflow"))?;return Err(error.with_retained_progress(progress))}};
        if !progress.fits(child){progress.copied_bytes=progress.copied_bytes.checked_add(header).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original read receipt overflow"))?;return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original read comparison exceeded its grant").with_retained_progress(progress))}
        if progress.copied_items==0{return Ok((None,empty))}
        self.position=position;progress.copied_bytes+=header;
        if let Some(order)=order{match order{Ordering::Less=>self.next=node.left,Ordering::Greater=>{self.rank=self.rank.checked_add(index.count(node.left)+1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original read rank overflow").with_retained_progress(progress))?;self.next=node.right;},Ordering::Equal=>{self.rank=self.rank.checked_add(index.count(node.left)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original matching rank overflow").with_retained_progress(progress))?;self.finished=Some(Some(slot));}}self.position=0;}
        Ok((self.finished,progress))
    }

    /// 🌱️ The minimum probe follows one admitted left link without comparing or cloning any original key.
    pub fn first_demand<K:Ord,V>(&self,index:&HistoryFoldIndex<K,V>)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if self.finished.is_some()&&self.bound(index){return Ok(Default::default())}
        let mut demand=semio_framework_value::RetirementDemand {copy_bytes:2*size_of::<Self>(),depth:1,..Default::default()};
        if self.bound(index){if let Some(slot)=self.next{demand.copy_bytes=demand.copy_bytes.checked_add(2*size_of::<FoldIndexNode<K,V>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original minimum probe quote overflow"))?;demand.depth=demand.depth.max(index.next_slot_depth_demand(slot)?);}}
        Ok(demand)
    }

    pub fn first_step<K:Ord,V>(&mut self,index:&HistoryFoldIndex<K,V>,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
        let demand=self.first_demand(index)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok((false,Default::default()))}
        if self.finished.is_some()&&self.bound(index){return Ok((true,Default::default()))}
        if !self.bound(index){self.identity=index as *const _ as usize;self.epoch=Some(index.mutation_epoch);self.next=index.root;self.position=0;self.rank=0;self.finished=None;}
        else if let Some(slot)=self.next{let node=&index.nodes[slot];if let Some(left)=node.left{self.next=Some(left);}else{self.finished=Some(Some(slot));}}
        else{self.finished=Some(None);}
        Ok((self.finished.is_some(),RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }
}

impl HistoryFoldIndexMatch{
    fn validate<K:Ord,V>(&self,index:&HistoryFoldIndex<K,V>)->Result<(),ValueError>{if self.identity!=index as *const _ as usize||self.epoch!=index.mutation_epoch{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original matched row changed its index or mutation epoch"))}Ok(())}
    pub fn value<'a,K:Ord,V>(&self,index:&'a HistoryFoldIndex<K,V>)->Result<&'a V,ValueError>{self.validate(index)?;index.slot_entry(self.slot).map(|(_,value)|value).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original matched row is no longer present"))}
    pub fn value_mut<'a,K:Ord,V>(&self,index:&'a mut HistoryFoldIndex<K,V>)->Result<&'a mut V,ValueError>{self.validate(index)?;index.slot_entry_mut(self.slot).map(|(_,value)|value).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original matched mutable row is no longer present"))}

    /// 🍂️ Quotes the exact original rank path and its bounded AVL rebalancing headers before extraction.
    pub fn remove_demand<K:Ord,V>(&self,index:&HistoryFoldIndex<K,V>)->Result<semio_framework_value::RetirementDemand,ValueError>{
        self.validate(index)?;
        let height=index.height(index.root);
        let copy_bytes=height.checked_add(1).and_then(|nodes|nodes.checked_mul(3)).and_then(|nodes|nodes.checked_mul(size_of::<FoldIndexNode<K,V>>())).and_then(|bytes|bytes.checked_add(2*size_of::<Option<(K,V)>>()+size_of::<Self>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original rank removal quote overflow"))?;
        let depth=height.checked_add(2).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original rank removal depth overflow"))?.max(index.next_slot_depth_demand(self.slot)?);
        Ok(semio_framework_value::RetirementDemand {copy_bytes,depth,..Default::default()})
    }

    pub fn remove_into<K:Ord,V>(&self,index:&mut HistoryFoldIndex<K,V>,output:&mut Option<(K,V)>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let demand=self.remove_demand(index)?;
        if output.is_some()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Default::default())}
        let(root,original)=index.remove_rank_at(index.root,self.rank);index.root=root;*output=original;
        let progress=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        if output.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original rank removal lost its matched row").with_retained_progress(progress))}
        Ok(progress)
    }
}
