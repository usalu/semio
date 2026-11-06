//! ➕️ Insert cursor of the retained ordered map: the insertion step, grant and progress records, the workspace and state and the resumable insertion cursor with its drop guard. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOrderedMapInsertStep {
    Progress(RetainedOrderedMapInsertProgress),
    Complete { ordinal: usize, progress: RetainedOrderedMapInsertProgress },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedOrderedMapInsertGrant {
    pub retirement:RetainedCloneGrant,
    pub comparison: BoundedOrdGrant,
    pub maximum_moved_items: usize,
    pub maximum_moved_bytes: usize,
    pub maximum_capacity_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedOrderedMapInsertProgress {
    pub retirement:RetainedCloneProgress,
    pub comparison: BoundedOrdProgress,
    pub moved_items: usize,
    pub moved_bytes: usize,
    pub retained_capacity_bytes: usize,
}

impl RetainedOrderedMapInsertProgress {
    pub fn fits(self, grant: RetainedOrderedMapInsertGrant) -> bool {
        self.retirement.fits(grant.retirement) && self.comparison.fits(grant.comparison) && self.moved_items <= grant.maximum_moved_items && self.moved_bytes <= grant.maximum_moved_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes
    }
}

/// 🧯 Owns one exclusive unpublished workspace through completion or bounded cancellation.
#[derive(crate::RetireOwned)]
#[value(crate="crate")]
pub(super) struct InsertWorkspace<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{pub(super) map:RetainedOrderedMap<K,V>,pub(super) key:K,pub(super) value:V}

pub(super) struct InsertState<K: BoundedOrd + RetireOwned, V: RetireOwned+Sync> {
    pub(super) map: Option<RetainedOrderedMap<K, V>>,
    pub(super) output: Option<RetainedOrderedMap<K, V>>,
    pub(super) key: Option<K>,
    pub(super) value: Option<V>,
    pub(super) lookup: RetainedOrderedMapLookupCursor<K>,
    lookup_lease: staged_lease::StagedLease<InsertWorkspace<K,V>>,
    pub(super) target: Option<usize>,
    shift_page: Option<usize>,
    old_directory: Option<Vec<Vec<(K, V)>>>,
    new_directory: Option<Vec<Vec<(K, V)>>>,
    directory_index: usize,
    pub(super) phase: u8,
    pub(super) spent: bool,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
}

pub struct RetainedOrderedMapInsertCursor<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{pub(super) state:std::mem::ManuallyDrop<InsertState<K,V>>}

impl<K: BoundedOrd + RetireOwned, V: RetireOwned+Sync> RetainedOrderedMapInsertCursor<K, V> {
    pub fn constructor_capacity_bytes()->usize{0}
    pub fn constructor_copy_bytes()->usize{0}
    pub fn admit(map:RetainedOrderedMap<K,V>,key:K,value:V,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,RetainedOrderedMap<K,V>,K,V)>{let work=0;let result=(||{
        let(lookup_lease,receipt)=(staged_lease::StagedLease::new(InsertWorkspace{map,key,value}),RetainedCloneProgress::default());let _=grant;
        Ok((Self {state:std::mem::ManuallyDrop::new(InsertState {
            map: None,
            output: None,
            key: None,
            value: None,
            lookup: Default::default(),
            lookup_lease,
            target: None,
            shift_page: None,
            old_directory: None,
            new_directory: None,
            directory_index: 0,
            phase: 0,
            spent: false,
            closing: false,
            close: Default::default(),
        })},receipt))
    })();crate::RetainedWorkOutcome{result,work}}

    pub fn next_advance_demands(&self, maximum_copy_bytes: usize) -> Result<RetainedOrderedMapInsertGrant, ValueError> {
        let state = &*self.state;
        let overflow = || ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "ordered-map insertion next-action extent overflow");
        let mut grant = RetainedOrderedMapInsertGrant::default();
        if state.closing || state.spent {
            return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion cannot advance after close or transfer"));
        }
        match state.phase {
            0 => {
                let demand = self.advance_retirement_demands(maximum_copy_bytes)?;
                grant.retirement = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes,
                    maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
                grant.comparison = BoundedOrdGrant { maximum_items: 1, maximum_bytes: maximum_copy_bytes.min(64) };
            }
            1 => {
                let map = state.map.as_ref().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no map"))?;
                if map.len % RETAINED_ORDERED_MAP_PAGE_CAPACITY == 0 && map.pages.len() < map.pages.capacity() {
                    grant.maximum_capacity_bytes = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
                } else { grant.maximum_moved_items = 1; }
            }
            2 => {
                let ordinal = state.target.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no target"))?;
                let shift = state.shift_page.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no shift page"))?;
                if shift == ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY {grant.maximum_moved_items=1;} else {
                    let page = state.map.as_ref().and_then(|map| map.pages.get(shift)).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no shift owner"))?;
                    grant.maximum_moved_items = page.len().checked_add(1).ok_or_else(overflow)?;
                    grant.maximum_moved_bytes = grant.maximum_moved_items.checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
                }
            }
            3 => {
                let ordinal = state.target.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no target"))?;
                let page = state.map.as_ref().and_then(|map| map.pages.get(ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY)).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no target owner"))?;
                grant.maximum_moved_items = page.len().saturating_sub(ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY).checked_add(1).ok_or_else(overflow)?;
                grant.maximum_moved_bytes = grant.maximum_moved_items.checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
            }
            10 => {
                let map = state.map.as_ref().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map directory quote has no map"))?;
                grant.maximum_capacity_bytes = map.pages.len().checked_mul(2).and_then(|n| n.checked_add(1)).and_then(|n| n.checked_mul(size_of::<Vec<(K,V)>>())).ok_or_else(overflow)?;
            }
            11 | 13 => { grant.maximum_moved_items = 1; grant.maximum_moved_bytes = size_of::<Vec<Vec<(K,V)>>>(); }
            12 => {
                let old = state.old_directory.as_ref().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map directory quote has no original owner"))?;
                grant.maximum_moved_items=1;if state.directory_index != old.len() {grant.maximum_moved_bytes=size_of::<Vec<(K,V)>>();}
            }
            4 | 255 => {}
            _ => return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has an invalid phase")),
        }
        Ok(grant)
    }
    /// 🎟️ Reports whether the staged authority birth has finished its funded turns.
    pub fn birth_complete(&self)->bool{self.state.lookup_lease.ready()}
    pub fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{ let state=&*self.state;
        if state.phase!=0{return Ok(Default::default());}
        if !state.lookup_lease.ready(){return state.lookup_lease.birth_demand();}
        if !state.lookup.closing{return state.lookup.advance_retirement_demands(body);}
        if !state.lookup.terminal_is_empty(){return state.lookup.close_demands(body);}
        state.lookup_lease.next_take_demands(body)
    }
    pub fn advance(&mut self, grant: RetainedOrderedMapInsertGrant) -> crate::RetainedWorkOutcome<RetainedOrderedMapInsertStep, ValueError> {let mut work=0;let result=(||{ let state=&mut *self.state;
        if state.closing {
            return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion cursor is closing"));
        }
        if state.spent {
            return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion cursor is spent"));
        }
        match state.phase {
            0 => {
                if !state.lookup_lease.ready(){let turn=state.lookup_lease.advance_birth(grant.retirement);work+=turn.work;let retirement=turn.result?.progress();return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement,..Default::default()}));}
                if state.lookup.closing{
                    if !state.lookup.terminal_is_empty(){let step={let turn=state.lookup.close_step(grant.retirement);work+=turn.work;turn.result}?;return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement:step.progress(),..Default::default()}));}
                    let demand=state.lookup_lease.next_take_demands(grant.retirement.maximum_copy_bytes)?;
                    if grant.retirement.maximum_items==0||demand.copy_bytes>grant.retirement.maximum_copy_bytes||demand.capacity_bytes>grant.retirement.maximum_capacity_bytes||demand.release_bytes>grant.retirement.maximum_release_bytes||demand.depth>grant.retirement.maximum_depth{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}
                    match {let turn=state.lookup_lease.take_authority(grant.retirement);work+=turn.work;turn.result}?{
                        super::super::RetainedCloneSourceTake::Pending(retirement)=>return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement,..Default::default()})),
                        super::super::RetainedCloneSourceTake::Ready(InsertWorkspace{map,key,value},retirement)=>{state.map=Some(map);state.key=Some(key);state.value=Some(value);match state.lookup.complete.unwrap(){RetainedOrderedMapLookup::Found(_)=>{state.phase=255;},RetainedOrderedMapLookup::Missing(ordinal)=>{state.target=Some(ordinal);state.phase=1;}}return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement,..Default::default()}));}
                    }
                }
                let map_ref=state.lookup_lease.borrow(1,|workspace|&workspace.map);
                let key_ref=state.lookup_lease.borrow(2,|workspace|&workspace.key);
                let(result,comparison,retirement)={let turn=state.lookup.advance(map_ref,key_ref,grant.comparison,grant.retirement);work+=turn.work;turn.result}?;
                if result.is_some(){state.lookup.begin_close();}
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{comparison,retirement,..Default::default()}))
            }
            255=>Err(ValueError::literal(crate::ValueRefusalKind::InvalidValue,"retained ordered-map insertion refused a duplicate key")),
            1 => {
                let map = state.map.as_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                if map.len % RETAINED_ORDERED_MAP_PAGE_CAPACITY == 0 {
                    if map.pages.len() == map.pages.capacity() {
                        if grant.maximum_moved_items==0{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}work+=1;state.phase = 10;
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let capacity = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion page capacity overflow"))?;
                    let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                    if !progress.fits(grant) {
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let mut page = Vec::new();
                    work+=1;page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| ValueError::literal(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map insertion page allocation failed"))?;
                    let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion page actual capacity overflow"))?;
                    if actual > grant.maximum_capacity_bytes {
                        return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion page allocator exceeded its admitted capacity"));
                    }
                    map.pages.push(page);
                    state.shift_page = map.pages.len().checked_sub(1);
                    state.phase = 2;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }));
                }
                if grant.maximum_moved_items == 0 {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                work+=1;state.shift_page = map.pages.len().checked_sub(1);
                state.phase = 2;
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()))
            }
            2 => {
                let map = state.map.as_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let ordinal = state.target.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?;
                let target_page = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let shift_page = state.shift_page.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion shift page is missing"))?;
                if shift_page == target_page {
                    if grant.maximum_moved_items==0{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}work+=1;state.phase = 3;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()));
                }
                let (left, right) = map.pages.split_at_mut(shift_page);
                let previous = left.get_mut(shift_page - 1).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion previous page is missing"))?;
                let current = right.first_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion current page is missing"))?;
                if current.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion shift page has no admitted slot"));
                }
                let moved_items = current.len().checked_add(1).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion move count overflow"))?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion shift overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                work+=1;let entry = previous.pop().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion previous page is empty"))?;
                current.insert(0, entry);
                state.shift_page = Some(shift_page - 1);
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            3 => {
                let map = state.map.as_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let ordinal = state.target.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?;
                let page_index = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let entry_index = ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let page = map.pages.get_mut(page_index).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target page is missing"))?;
                if page.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target page has no admitted slot"));
                }
                let moved_items = page.len().saturating_sub(entry_index).checked_add(1).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion move count overflow"))?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion target shift overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let new_len = map.len.checked_add(1).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion length overflow"))?;
                work+=1;page.insert(
                    entry_index,
                    (
                        state.key.take().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion key owner is missing"))?,
                        state.value.take().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion value owner is missing"))?,
                    ),
                );
                map.len = new_len;
                state.output = state.map.take();
                state.phase = 4;
                Ok(RetainedOrderedMapInsertStep::Complete { ordinal, progress })
            }
            4 => Ok(RetainedOrderedMapInsertStep::Complete {
                ordinal: state.target.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?,
                progress: RetainedOrderedMapInsertProgress::default(),
            }),
            10 => {
                let map = state.map.as_ref().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let planned = map.pages.len().checked_mul(2).and_then(|value| value.checked_add(1)).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory growth overflow"))?;
                let capacity = planned.checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory capacity overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let mut directory = Vec::new();
                work+=1;directory.try_reserve_exact(planned).map_err(|_| ValueError::literal(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map directory growth allocation failed"))?;
                let actual = directory.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map directory allocator exceeded its admitted capacity"));
                }
                state.new_directory = Some(directory);
                state.phase = 11;
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }))
            }
            11 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let map = state.map.as_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                work+=1;state.old_directory = Some(std::mem::take(&mut map.pages));
                state.phase = 12;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            12 => {
                let old = state.old_directory.as_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map old directory is missing"))?;
                if state.directory_index == old.len() {
                    if grant.maximum_moved_items==0{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}work+=1;state.phase = 13;
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<(K, V)>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                work+=1;state.new_directory.as_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map new directory is missing"))?.push(std::mem::take(&mut old[state.directory_index]));
                state.directory_index += 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            13 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                work+=1;state.map.as_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?.pages =
                    state.new_directory.take().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map relocated directory is missing"))?;
                state.directory_index = 0;
                state.phase = 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            _ => Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion state is invalid")),
        }
    })();crate::RetainedWorkOutcome{result,work}}

    pub fn output_ready(&self)->bool{let state=&*self.state;state.output.is_some()&&!state.closing&&!state.spent}
    pub fn refused_workspace_ready(&self)->bool{let state=&*self.state;state.phase==255&&state.map.is_some()&&!state.closing&&!state.spent}
    pub fn take(&mut self) -> Option<RetainedOrderedMap<K, V>> { let state=&mut *self.state;
        let output = state.output.take();
        if output.is_some() {
            state.spent = true;
        }
        output
    }

    pub fn take_refused_workspace(&mut self) -> Option<RetainedOrderedMap<K, V>> { let state=&mut *self.state;
        if state.phase != 255 {
            return None;
        }
        let map = state.map.take();
        if map.is_some() {
            state.spent = true;
        }
        map
    }

    pub fn begin_close(&mut self) -> bool { let state=&mut *self.state;
        if state.closing {
            return false;
        }
        state.closing = true;
        true
    }

    pub(super) fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError> {
        let state=&*self.state;
        if !state.lookup.terminal_is_empty(){return state.lookup.close_demands(body);}
        if !state.close.is_empty(){return state.close.next_owner_demands::<K>(false,&None,body);}
        if state.key.is_some(){return state.close.next_owner_demands::<K>(true,&None,body);}
        if state.value.is_some(){return state.close.next_owner_demands::<V>(true,&None,body);}
        if state.old_directory.is_some()||state.new_directory.is_some(){return state.close.next_owner_demands::<Vec<Vec<(K,V)>>>(true,&None,body);}
        if state.map.is_some()||state.output.is_some(){return state.close.next_owner_demands::<RetainedOrderedMap<K,V>>(true,&None,body);}
        state.lookup_lease.next_close_demand(body)
    }
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep, ValueError> {let grant=effect.grant();let mut work=0;let result=(||{
        if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?) {return Ok(RetainedCloneStep::Progress(Default::default()));}
        let state=&mut *self.state;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !state.closing { return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion must begin close before retiring ownership")); }
        if !state.lookup.terminal_is_empty(){state.lookup.begin_close();return {let turn=state.lookup.close_step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"));work+=turn.work;turn.result};}
        if !state.close.is_empty() { return {let turn=state.close.step_granted(effect);work+=turn.work;turn.result}; }
        if let Some(step) = {let turn=state.close.begin_granted(&mut state.key,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=state.close.begin_granted(&mut state.value,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=state.close.begin_granted(&mut state.old_directory,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=state.close.begin_granted(&mut state.new_directory,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=state.close.begin_granted(&mut state.map,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=state.close.begin_granted(&mut state.output,effect);work+=turn.work;turn.result}? { return Ok(step); }
        {let turn=state.lookup_lease.close_step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"));work+=turn.work;turn.result}
    })();crate::RetainedWorkOutcome{result,work}}

    pub fn terminal_is_empty(&self) -> bool { let state=&*self.state;
        state.lookup.terminal_is_empty()&&state.lookup_lease.terminal_is_empty()&&state.closing && state.key.is_none() && state.value.is_none() && state.old_directory.is_none() && state.new_directory.is_none() && state.map.is_none() && state.output.is_none() && state.close.is_empty()
    }
}

impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> Drop for RetainedOrderedMapInsertCursor<K,V>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("insertion original owners require caller-funded closure")}}}
