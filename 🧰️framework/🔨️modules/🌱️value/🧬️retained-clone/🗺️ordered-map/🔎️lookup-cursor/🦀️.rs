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
    pub fn advance<V>(&mut self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,target:RetainedCloneRef<'_,K>,grant:BoundedOrdGrant,retirement:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Option<RetainedOrderedMapLookup>,BoundedOrdProgress,RetainedCloneProgress),crate::ValueError>{let mut work=0;let result=(||{
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"lookup is closing"));}
        let first=!self.initialized;
        if first&&grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
        if let Some(progress)={let turn=source.bind(&mut self.source,retirement);work+=turn.work;turn.result}?{return Ok((None,Default::default(),progress));}
        if let Some(progress)={let turn=target.bind(&mut self.target,retirement);work+=turn.work;turn.result}?{return Ok((None,Default::default(),progress));}
        if let Some(result)=self.complete{return Ok((Some(result),Default::default(),Default::default()));}
        let map=source.get();
        if first{work+=1;self.initialized=true;self.high=map.len();return Ok((None,BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));}
        if let Some(ordering)=self.pending{
            if let Some(comparison)=self.comparison.as_mut(){
                comparison.begin_close();let step={let turn=comparison.close_step(retirement);work+=turn.work;turn.result}?;
                if comparison.terminal_is_empty(){self.comparison=None;}
                return Ok((None,Default::default(),step.progress()));
            }
            if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
            work+=1;self.pending=None;
            match ordering{Ordering::Less=>self.low=self.middle+1,Ordering::Greater=>self.high=self.middle,Ordering::Equal=>unreachable!()}
            return Ok((None,BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));
        }
        if self.low==self.high{
            if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
            work+=1;let result=RetainedOrderedMapLookup::Missing(self.low);self.complete=Some(result);
            return Ok((Some(result),BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));
        }
        if self.comparison.is_none(){if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}let turn=K::admit_bounded_ord_cursor(retirement);work+=turn.work;let Some((comparison,p))=turn.result?else{return Ok((None,Default::default(),Default::default()));};self.middle=self.low+(self.high-self.low)/2;self.comparison=Some(comparison);return Ok((None,Default::default(),p));}
        let comparison=self.comparison.as_mut().ok_or_else(||ValueError::invariant("comparison absent"))?;
        match {let turn=comparison.compare(source.project(self.middle+1,|map|map.get_index(self.middle).unwrap().0),target,grant,retirement);work+=turn.work;turn.result}?{
            BoundedOrdStep::Authority(progress)=>Ok((None,Default::default(),progress)),
            BoundedOrdStep::Progress(p)=>Ok((None,p,Default::default())),
            BoundedOrdStep::Complete{ordering:Ordering::Equal,progress}=>{let result=RetainedOrderedMapLookup::Found(self.middle);self.complete=Some(result);Ok((Some(result),progress,Default::default()))},
            BoundedOrdStep::Complete{ordering,progress}=>{self.pending=Some(ordering);Ok((None,progress,Default::default()))}
        }
    })();crate::RetainedWorkOutcome{result,work}}
    pub fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if self.complete.is_some(){return Ok(Default::default());}
        if self.source.is_none()||self.target.is_none(){return Ok(crate::RetirementDemand{copy_bytes:RetainedCloneBinding::alias_copy_bytes(),depth:1,..Default::default()});}
        if self.pending.is_some(){return self.comparison.as_ref().map_or(Ok(Default::default()),|cursor|cursor.close_demands(body));}
        if !self.initialized||self.low==self.high{return Ok(Default::default());}
        self.comparison.as_ref().map_or_else(||Ok(K::bounded_ord_birth_demand()),|cursor|cursor.advance_retirement_demands(body))
    }
    pub fn begin_close(&mut self)->bool{let started=!self.closing;self.closing=true;if let Some(c)=self.comparison.as_mut(){c.begin_close();}started}
    /// 🎟️ Retains the original comparator or projected binding through one complete next-action quote.
    pub(super) fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if let Some(cursor)=self.comparison.as_ref(){if cursor.terminal_is_empty(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<K::Cursor>(),depth:1,..Default::default()});}let mut d=cursor.close_demands(body)?;d.depth=d.depth.checked_add(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"original lookup comparator close depth overflow"))?;return Ok(d);}
        RetainedCloneBinding::close_demand(if self.source.is_some(){&self.source}else{&self.target},body)
    }    pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){if c.terminal_is_empty(){Ok(size_of::<K::Cursor>())}else{c.next_close_copy_byte_demand()}}else{RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){if c.terminal_is_empty(){Ok(0)}else{c.next_close_capacity_byte_demand(body)}}else{RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.target},body)}}
    pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){if c.terminal_is_empty(){Ok(0)}else{c.next_close_release_byte_demand()}}else{RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){if c.terminal_is_empty(){Ok(1)}else{c.next_close_depth_demand()?.checked_add(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"original lookup comparator close depth overflow"))}}else{RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::ValueError>{let mut work=0;let result=(||{
        if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"lookup must begin close"));}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if !super::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(c)=self.comparison.as_mut(){
            if c.terminal_is_empty(){work+=1;self.comparison=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            let step={let turn=c.close_step(child);work+=turn.work;turn.result}?;
            let step=super::admit_retained_clone_close(child,step,c.terminal_is_empty(),"original lookup comparator close")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step={let turn=close_retained_binding(if self.source.is_some(){&mut self.source}else{&mut self.target},grant);work+=turn.work;turn.result}?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    })();crate::RetainedWorkOutcome{result,work}}
    pub fn terminal_is_empty(&self)->bool{self.comparison.is_none()&&self.source.is_none()&&self.target.is_none()}
}
