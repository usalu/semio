//! 🔎️ Lookup cursor of the retained ordered map: the lookup outcome and the resumable key lookup. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOrderedMapLookup {
    Found(usize),
    Missing(usize),
}

pub struct RetainedOrderedMapLookupCursor<K: BoundedOrd> {
    pub(super) initialized:bool,
    low: usize,
    high: usize,
    pub(super) middle: usize,
    pub(super) comparison: Option<K::Cursor>,
    pending: Option<Ordering>,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) target: Option<RetainedCloneBinding>,
    pub(super) complete: Option<RetainedOrderedMapLookup>,
    pub(super) closing:bool,
}

impl<K: BoundedOrd> Default for RetainedOrderedMapLookupCursor<K> {
    fn default() -> Self {
        Self { initialized:false,low: 0, high: 0, middle: 0, comparison: None, pending: None, source: None, target:None, complete: None, closing:false }
    }
}

impl<K:BoundedOrd> RetainedOrderedMapLookupCursor<K>{
    pub fn advance<V>(&mut self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,target:RetainedCloneRef<'_,K>,grant:BoundedOrdGrant,retirement:RetainedCloneGrant)->Result<(Option<RetainedOrderedMapLookup>,BoundedOrdProgress,RetainedCloneProgress),crate::ValueError>{
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"lookup is closing"));}
        let first=!self.initialized;
        if first&&grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
        if let Some(progress)=source.bind(&mut self.source,retirement)?{return Ok((None,Default::default(),progress));}
        if let Some(progress)=target.bind(&mut self.target,retirement)?{return Ok((None,Default::default(),progress));}
        if let Some(result)=self.complete{return Ok((Some(result),Default::default(),Default::default()));}
        let map=source.get();
        if first{self.initialized=true;self.high=map.len();return Ok((None,BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));}
        if let Some(ordering)=self.pending{
            if let Some(comparison)=self.comparison.as_mut(){
                comparison.begin_close();let step=comparison.close_step(retirement)?;
                if comparison.terminal_is_empty(){self.comparison=None;}
                return Ok((None,Default::default(),step.progress()));
            }
            if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
            self.pending=None;
            match ordering{Ordering::Less=>self.low=self.middle+1,Ordering::Greater=>self.high=self.middle,Ordering::Equal=>unreachable!()}
            return Ok((None,BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));
        }
        if self.low==self.high{
            if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
            let result=RetainedOrderedMapLookup::Missing(self.low);self.complete=Some(result);
            return Ok((Some(result),BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));
        }
        self.middle=self.low+(self.high-self.low)/2;
        let comparison=self.comparison.get_or_insert_with(K::bounded_ord_cursor);
        match comparison.compare(source.project(self.middle+1,|map|map.get_index(self.middle).unwrap().0),target,grant,retirement)?{
            BoundedOrdStep::Authority(progress)=>Ok((None,Default::default(),progress)),
            BoundedOrdStep::Progress(p)=>Ok((None,p,Default::default())),
            BoundedOrdStep::Complete{ordering:Ordering::Equal,progress}=>{let result=RetainedOrderedMapLookup::Found(self.middle);self.complete=Some(result);Ok((Some(result),progress,Default::default()))},
            BoundedOrdStep::Complete{ordering,progress}=>{self.pending=Some(ordering);Ok((None,progress,Default::default()))}
        }
    }
    pub fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if self.complete.is_some(){return Ok(Default::default());}
        if self.source.is_none()||self.target.is_none(){return Ok(crate::RetirementDemand{copy_bytes:super::super::RetainedCloneSource::<u64>::constructor_copy_bytes(),depth:1,..Default::default()});}
        if self.pending.is_some(){return self.comparison.as_ref().map_or(Ok(Default::default()),|cursor|Ok(crate::RetirementDemand{copy_bytes:cursor.next_close_copy_byte_demand()?,capacity_bytes:cursor.next_close_capacity_byte_demand(body)?,release_bytes:cursor.next_close_release_byte_demand()?,depth:cursor.next_close_depth_demand()?}));}
        if !self.initialized||self.low==self.high{return Ok(Default::default());}
        self.comparison.as_ref().map_or(Ok(crate::RetirementDemand{copy_bytes:super::super::RetainedCloneSource::<u64>::constructor_copy_bytes(),depth:1,..Default::default()}),|cursor|cursor.advance_retirement_demands(body))
    }
    pub fn begin_close(&mut self)->bool{let started=!self.closing;self.closing=true;if let Some(c)=self.comparison.as_mut(){c.begin_close();}started}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_copy_byte_demand()}else{RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_capacity_byte_demand(body)}else{RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.target},body)}}
    pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_release_byte_demand()}else{RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_depth_demand()}else{RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{
        if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"lookup must begin close"));}
        if let Some(c)=self.comparison.as_mut(){let step=c.close_step(grant)?;if c.terminal_is_empty(){self.comparison=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        close_retained_binding(if self.source.is_some(){&mut self.source}else{&mut self.target},grant)
    }
    pub fn terminal_is_empty(&self)->bool{self.comparison.is_none()&&self.source.is_none()&&self.target.is_none()}
}
