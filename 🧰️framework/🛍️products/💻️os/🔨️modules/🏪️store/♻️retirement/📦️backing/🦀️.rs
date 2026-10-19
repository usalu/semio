//! 📦️ Whole physical retirement of empty resident Store metadata backings.
use semio_framework_value::{ErasedSnapshotRetirement, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::collections::VecDeque;

fn extent<T>(capacity: usize) -> usize { capacity.checked_mul(size_of::<T>()).expect("resident metadata backing layout") }

pub fn denied() -> RetainedCloneStep { RetainedCloneStep::Progress(Default::default()) }

pub fn released(bytes: usize, terminal: bool) -> RetainedCloneStep {
    let progress = RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() };
    if terminal { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }
}

fn admit_release(grant: RetainedCloneGrant, demand: usize) -> Result<bool, ValueError> {
    if grant.maximum_items == 0 || grant.maximum_release_bytes < demand { return Ok(false); }
    if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "resident backing requires original frame depth")); }
    Ok(true)
}

pub fn vec_demand<T>(owner: &Vec<T>) -> usize { extent::<T>(owner.capacity()) }

pub fn deque_demand<T>(owner: &VecDeque<T>) -> usize { extent::<T>(owner.capacity()) }

pub fn close_vec<T>(owner: &mut Vec<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    if !owner.is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident vector backing still holds typed owners")); }
    let demand = vec_demand(owner);
    if demand == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
    if !admit_release(grant, demand)? { return Ok(denied()); }
    drop(std::mem::take(owner));
    Ok(released(demand, true))
}

pub fn close_deque<T>(owner: &mut VecDeque<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    if !owner.is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident deque backing still holds typed owners")); }
    let demand = deque_demand(owner);
    if demand == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
    if !admit_release(grant, demand)? { return Ok(denied()); }
    drop(std::mem::take(owner));
    Ok(released(demand, true))
}

pub fn close_history<T>(owner: &mut semio_framework_os_vcs::HistoryPageStack<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let demand = owner.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident history backing still holds typed owners"))?;
    if demand == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
    if !admit_release(grant, demand)? { return Ok(denied()); }
    if !owner.release_empty_page() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident history page changed after its exact demand")); }
    Ok(released(demand, owner.capacity() == 0))
}

/// 🧵️ Retains the original native vector allocation across ordered prefix publication.
pub struct SourceIterator<T: semio_framework_value::retirement::RetireOwned> {
    pub source: std::mem::ManuallyDrop<std::vec::IntoIter<T>>,
    source_bytes: usize,
    pub pending: std::mem::ManuallyDrop<Option<T>>,
    pub active: Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>,
    pub terminal: bool,
}

impl<T: semio_framework_value::retirement::RetireOwned> SourceIterator<T> {
    pub fn new(source: Vec<T>) -> Self {
        let source_bytes = extent::<T>(source.capacity());
        Self { source: std::mem::ManuallyDrop::new(source.into_iter()), source_bytes, pending: std::mem::ManuallyDrop::new(None), active: None, terminal: false }
    }

    pub fn as_slice(&self) -> &[T] { self.source.as_slice() }

    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.terminal { return Ok(Default::default()); }
        if let Some(owner) = self.active.as_ref() {
            let mut demand=owner.next_demand(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"source iterator child depth overflow"))?;return Ok(demand);
        }
        if self.pending.is_some() || self.source.len() != 0 {
            if !T::controlled_retirement_supported() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original source iterator element has no controlled retirement declaration")); }
            return Ok(RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { release_bytes: self.source_bytes, depth: 1, ..Default::default() })
    }
}

impl<T: semio_framework_value::retirement::RetireOwned> Iterator for SourceIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        assert!(!self.terminal && self.active.is_none() && self.pending.is_none(), "original source publication cannot race controlled retirement");
        self.source.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) { self.source.size_hint() }
}

impl<T: semio_framework_value::retirement::RetireOwned> ExactSizeIterator for SourceIterator<T> {}

impl<T: semio_framework_value::retirement::RetireOwned> ErasedSnapshotRetirement for SourceIterator<T> {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.demands(body) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demands(0).map(|demand| demand.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.demands(copy).map(|demand| demand.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.demands(0).map(|demand| demand.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.demands(0).map(|demand| demand.depth) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(denied()); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original source iterator depth refused")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(denied()); }
        if let Some(owner) = self.active.as_mut() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.step(child)?;
            let terminal = owner.terminal_is_empty();
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, terminal, "original source iterator child")?;
            if terminal { drop(self.active.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending.is_none() { *self.pending = self.source.next(); }
        if let Some(value) = self.pending.take() {
            match semio_framework_value::retirement::controlled::ControlledRetirement::new(value) {
                Ok(owner) => { self.active = Some(owner); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
                Err((error, value)) => { *self.pending = Some(value); return Err(error); }
            }
        }
        drop(std::mem::replace(&mut *self.source, Vec::new().into_iter()));
        self.source_bytes = 0;
        self.terminal = true;
        Ok(released(demand.release_bytes, true))
    }
    fn terminal_is_empty(&self) -> bool { self.terminal && self.source_bytes == 0 && self.source.len() == 0 && self.pending.is_none() && self.active.is_none() }
}

impl<T: semio_framework_value::retirement::RetireOwned> Drop for SourceIterator<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original source iterator reached Drop before physical backing retirement");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.source); } }
    }
}

/// 📋️ Keeps original paged source backing through paid ordered publication and controlled close.
pub struct PagedSourceIterator<T:semio_framework_value::retirement::RetireOwned,const N:usize={usize::MAX}>{
    source:std::mem::ManuallyDrop<Option<semio_framework_value_persistent::list::PagedList<T,N>>>,
    position:usize,
    vacant:fn()->T,
    active:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value_persistent::list::PagedList<T,N>>>,
    terminal:bool,
}
impl<T:semio_framework_value::retirement::RetireOwned,const N:usize> PagedSourceIterator<T,N>{
    pub fn new(source:semio_framework_value_persistent::list::PagedList<T,N>,vacant:fn()->T)->Self{Self{source:std::mem::ManuallyDrop::new(Some(source)),position:0,vacant,active:None,terminal:false}}
    pub fn peek(&self)->Option<&T>{self.source.as_ref().and_then(|source|source.get(self.position))}
    pub fn publication_complete(&self)->bool{self.source.as_ref().is_none_or(|source|self.position==source.len())}
    pub fn backing_bytes(&self)->usize{self.source.as_ref().map_or(0,semio_framework_value_persistent::list::PagedList::allocated_bytes)}
    pub fn take_next_into(&mut self,receiving:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        if receiving.is_some()||self.active.is_some()||self.terminal{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original paged source has an occupied or closing receiver"))}
        let source=self.source.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original paged source lost its retained backing"))?;
        if self.position==source.len(){return Ok(None)}
        let depth=source.next_get_depth_demand(self.position).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original paged source path exceeds its extent"))?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original paged source depth overflow"))?;
        let copy=size_of::<Option<T>>().checked_add(size_of::<usize>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original paged source header extent overflow"))?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth<depth{return Ok(Some(Default::default()))}
        *receiving=Some(std::mem::replace(source.get_mut(self.position).unwrap(),(self.vacant)()));self.position+=1;
        Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }
    pub fn demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if self.terminal{return Ok(Default::default())}
        if let Some(active)=self.active.as_ref(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:active.next_copy_byte_demand()?,capacity_bytes:active.next_capacity_byte_demand(copy)?,release_bytes:active.next_release_byte_demand()?,depth:active.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original paged source child depth overflow"))?})}
        Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value_persistent::list::PagedList<T,N>>>(),depth:1,..Default::default()})
    }
}
impl<T:semio_framework_value::retirement::RetireOwned,const N:usize> ErasedSnapshotRetirement for PagedSourceIterator<T,N>{
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.demands(0).map(|demand|demand.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.demands(copy).map(|demand|demand.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.demands(0).map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.demands(0).map(|demand|demand.depth)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal{return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(denied())}
        let demand=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original paged source close depth refused"))}if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(denied())}
        if let Some(active)=self.active.as_mut(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=active.step(child)?;let terminal=active.terminal_is_empty();semio_framework_value::retained_clone::admit_retained_clone_close(child,step,terminal,"original paged source child")?;if terminal{drop(self.active.take());self.terminal=true;}return Ok(if self.terminal{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
        let source=self.source.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original paged source retirement lost its backing"))?;match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(active)=>{self.active=Some(active);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))},Err((error,source))=>{*self.source=Some(source);Err(error)}}
    }
    fn terminal_is_empty(&self)->bool{self.terminal&&self.source.is_none()&&self.active.is_none()}
}
impl<T:semio_framework_value::retirement::RetireOwned,const N:usize> Drop for PagedSourceIterator<T,N>{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original paged source dropped before controlled page retirement");if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.source)}}}
}


#[cfg(test)]
#[path="📦️paged/🧪️tests/🦀️.rs"]
mod original_paged_source_tests;
