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
struct InsertWorkspace<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{pub(super) map:RetainedOrderedMap<K,V>,pub(super) key:K,pub(super) value:V}

struct InsertState<K: BoundedOrd + RetireOwned, V: RetireOwned+Sync> {
    pub(super) map: Option<RetainedOrderedMap<K, V>>,
    pub(super) output: Option<RetainedOrderedMap<K, V>>,
    pub(super) key: Option<K>,
    pub(super) value: Option<V>,
    pub(super) lookup: RetainedOrderedMapLookupCursor<K>,
    lookup_lease: super::super::RetainedCloneBorrowAuthority<InsertWorkspace<K,V>>,
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
    pub fn constructor_capacity_bytes()->usize{super::super::RetainedCloneBorrowAuthority::<InsertWorkspace<K,V>>::constructor_capacity_bytes()}
    pub fn constructor_copy_bytes()->usize{super::super::RetainedCloneBorrowAuthority::<InsertWorkspace<K,V>>::constructor_copy_bytes()}
    pub fn admit(map:RetainedOrderedMap<K,V>,key:K,value:V,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(crate::ValueError,RetainedOrderedMap<K,V>,K,V)>{
        if grant.maximum_items==0{return Err((crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"insertion constructor requires an item"),map,key,value));}
        let(lookup_lease,receipt)=match super::super::RetainedCloneBorrowAuthority::admit(InsertWorkspace{map,key,value},grant){Ok(v)=>v,Err((error,InsertWorkspace{map,key,value}))=>return Err((error,map,key,value))};
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
    }

    pub fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{ let state=&*self.state;
        if state.phase!=0{return Ok(Default::default());}
        if !state.lookup.closing{return state.lookup.advance_retirement_demands(body);}
        if !state.lookup.terminal_is_empty(){return Ok(crate::RetirementDemand{copy_bytes:state.lookup.next_close_copy_byte_demand()?,capacity_bytes:state.lookup.next_close_capacity_byte_demand(body)?,release_bytes:state.lookup.next_close_release_byte_demand()?,depth:state.lookup.next_close_depth_demand()?});}
        Ok(crate::RetirementDemand{copy_bytes:state.lookup_lease.next_take_copy_byte_demand()?,capacity_bytes:state.lookup_lease.next_take_capacity_byte_demand(body)?,release_bytes:state.lookup_lease.next_take_release_byte_demand()?,depth:state.lookup_lease.next_take_depth_demand()?})
    }
    pub fn advance(&mut self, grant: RetainedOrderedMapInsertGrant) -> Result<RetainedOrderedMapInsertStep, crate::ValueError> { let state=&mut *self.state;
        if state.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion cursor is closing"));
        }
        if state.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion cursor is spent"));
        }
        match state.phase {
            0 => {
                if state.lookup.closing{
                    if !state.lookup.terminal_is_empty(){let step=state.lookup.close_step(grant.retirement)?;return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement:step.progress(),..Default::default()}));}
                    let demand=crate::RetirementDemand{copy_bytes:state.lookup_lease.next_take_copy_byte_demand()?,capacity_bytes:state.lookup_lease.next_take_capacity_byte_demand(grant.retirement.maximum_copy_bytes)?,release_bytes:state.lookup_lease.next_take_release_byte_demand()?,depth:state.lookup_lease.next_take_depth_demand()?};
                    if grant.retirement.maximum_items==0||demand.copy_bytes>grant.retirement.maximum_copy_bytes||demand.capacity_bytes>grant.retirement.maximum_capacity_bytes||demand.release_bytes>grant.retirement.maximum_release_bytes||demand.depth>grant.retirement.maximum_depth{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}
                    match state.lookup_lease.take_authority(grant.retirement)?{
                        super::super::RetainedCloneSourceTake::Pending(retirement)=>return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement,..Default::default()})),
                        super::super::RetainedCloneSourceTake::Ready(InsertWorkspace{map,key,value},retirement)=>{state.map=Some(map);state.key=Some(key);state.value=Some(value);match state.lookup.complete.unwrap(){RetainedOrderedMapLookup::Found(_)=>{state.phase=255;},RetainedOrderedMapLookup::Missing(ordinal)=>{state.target=Some(ordinal);state.phase=1;}}return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement,..Default::default()}));}
                    }
                }
                let map_ref=state.lookup_lease.borrow(1,|workspace|&workspace.map);
                let key_ref=state.lookup_lease.borrow(2,|workspace|&workspace.key);
                let(result,comparison,retirement)=state.lookup.advance(map_ref,key_ref,grant.comparison,grant.retirement)?;
                if result.is_some(){state.lookup.begin_close();}
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{comparison,retirement,..Default::default()}))
            }
            255=>Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"retained ordered-map insertion refused a duplicate key")),
            1 => {
                let map = state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                if map.len % RETAINED_ORDERED_MAP_PAGE_CAPACITY == 0 {
                    if map.pages.len() == map.pages.capacity() {
                        state.phase = 10;
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let capacity = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion page capacity overflow"))?;
                    let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                    if !progress.fits(grant) {
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let mut page = Vec::new();
                    page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map insertion page allocation failed"))?;
                    let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion page actual capacity overflow"))?;
                    if actual > grant.maximum_capacity_bytes {
                        return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion page allocator exceeded its admitted capacity"));
                    }
                    map.pages.push(page);
                    state.shift_page = map.pages.len().checked_sub(1);
                    state.phase = 2;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }));
                }
                if grant.maximum_moved_items == 0 {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.shift_page = map.pages.len().checked_sub(1);
                state.phase = 2;
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()))
            }
            2 => {
                let map = state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let ordinal = state.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?;
                let target_page = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let shift_page = state.shift_page.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion shift page is missing"))?;
                if shift_page == target_page {
                    state.phase = 3;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()));
                }
                let (left, right) = map.pages.split_at_mut(shift_page);
                let previous = left.get_mut(shift_page - 1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion previous page is missing"))?;
                let current = right.first_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion current page is missing"))?;
                if current.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion shift page has no admitted slot"));
                }
                let moved_items = current.len().checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion move count overflow"))?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion shift overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let entry = previous.pop().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion previous page is empty"))?;
                current.insert(0, entry);
                state.shift_page = Some(shift_page - 1);
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            3 => {
                let map = state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let ordinal = state.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?;
                let page_index = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let entry_index = ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let page = map.pages.get_mut(page_index).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target page is missing"))?;
                if page.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target page has no admitted slot"));
                }
                let moved_items = page.len().saturating_sub(entry_index).checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion move count overflow"))?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion target shift overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let new_len = map.len.checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion length overflow"))?;
                page.insert(
                    entry_index,
                    (
                        state.key.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion key owner is missing"))?,
                        state.value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion value owner is missing"))?,
                    ),
                );
                map.len = new_len;
                state.output = state.map.take();
                state.phase = 4;
                Ok(RetainedOrderedMapInsertStep::Complete { ordinal, progress })
            }
            4 => Ok(RetainedOrderedMapInsertStep::Complete {
                ordinal: state.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?,
                progress: RetainedOrderedMapInsertProgress::default(),
            }),
            10 => {
                let map = state.map.as_ref().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let planned = map.pages.len().checked_mul(2).and_then(|value| value.checked_add(1)).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory growth overflow"))?;
                let capacity = planned.checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory capacity overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let mut directory = Vec::new();
                directory.try_reserve_exact(planned).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map directory growth allocation failed"))?;
                let actual = directory.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map directory allocator exceeded its admitted capacity"));
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
                let map = state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                state.old_directory = Some(std::mem::take(&mut map.pages));
                state.phase = 12;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            12 => {
                let old = state.old_directory.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map old directory is missing"))?;
                if state.directory_index == old.len() {
                    state.phase = 13;
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<(K, V)>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.new_directory.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map new directory is missing"))?.push(std::mem::take(&mut old[state.directory_index]));
                state.directory_index += 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            13 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?.pages =
                    state.new_directory.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map relocated directory is missing"))?;
                state.directory_index = 0;
                state.phase = 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion state is invalid")),
        }
    }

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

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> { let state=&*self.state; if !state.lookup.terminal_is_empty(){state.lookup.next_close_copy_byte_demand()}else if !state.close.is_empty(){state.close.next_copy_byte_demand()}else if state.key.is_some()||state.value.is_some()||state.old_directory.is_some()||state.new_directory.is_some()||state.map.is_some()||state.output.is_some(){Ok(0)}else{state.lookup_lease.next_close_copy_byte_demand()} }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, crate::ValueError> { let state=&*self.state;
        if !state.lookup.terminal_is_empty(){return state.lookup.next_close_capacity_byte_demand(body);}
        if !state.close.is_empty() { return state.close.next_capacity_byte_demand(body); }
        if state.key.is_some() { return state.close.next_owner_capacity_byte_demand::<K>(true, body); }
        if state.value.is_some() { return state.close.next_owner_capacity_byte_demand::<V>(true, body); }
        if state.old_directory.is_some() || state.new_directory.is_some() { return state.close.next_owner_capacity_byte_demand::<Vec<Vec<(K,V)>>>(true, body); }
        if state.map.is_some()||state.output.is_some(){state.close.next_owner_capacity_byte_demand::<RetainedOrderedMap<K,V>>(true,body)}else{state.lookup_lease.next_close_capacity_byte_demand(body)}
    }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> { let state=&*self.state; if !state.lookup.terminal_is_empty(){state.lookup.next_close_release_byte_demand()}else if !state.close.is_empty(){state.close.next_release_byte_demand()}else if state.key.is_some()||state.value.is_some()||state.old_directory.is_some()||state.new_directory.is_some()||state.map.is_some()||state.output.is_some(){Ok(0)}else{state.lookup_lease.next_close_release_byte_demand()} }
    pub fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError> { let state=&*self.state; if !state.lookup.terminal_is_empty(){state.lookup.next_close_depth_demand()}else if !state.close.is_empty(){state.close.next_depth_demand()}else if state.key.is_some()||state.value.is_some()||state.old_directory.is_some()||state.new_directory.is_some()||state.map.is_some()||state.output.is_some(){Ok(1)}else{state.lookup_lease.next_close_depth_demand()} }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> { let state=&mut *self.state;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !state.closing { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion must begin close before retiring ownership")); }
        if !state.lookup.terminal_is_empty(){state.lookup.begin_close();return state.lookup.close_step(grant);}
        if !state.close.is_empty() { return state.close.step_granted(grant); }
        if let Some(step) = state.close.begin_granted(&mut state.key, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.value, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.old_directory, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.new_directory, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.map, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.output, grant)? { return Ok(step); }
        state.lookup_lease.close_step(grant)
    }

    pub fn terminal_is_empty(&self) -> bool { let state=&*self.state;
        state.lookup.terminal_is_empty()&&state.lookup_lease.terminal_is_empty()&&state.closing && state.key.is_none() && state.value.is_none() && state.old_directory.is_none() && state.new_directory.is_none() && state.map.is_none() && state.output.is_none() && state.close.is_empty()
    }
}

impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> Drop for RetainedOrderedMapInsertCursor<K,V>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("insertion original owners require caller-funded closure")}}}
