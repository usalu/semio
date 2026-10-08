use crate::os_vcs::ArtifactGroupVisibility;
use super::{ArtifactStoreOneItemGrant, CompositionGraph, OWNED_DOCUMENT_MAXIMUM_MEMBERS};
use semio_framework_value::{ValueError, RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::sync::Arc;

type Row = (String, (String, String));

#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone, PartialEq, Eq))]
pub(super) struct OwnsRoot(Vec<Row>);

impl OwnsRoot {
    pub(super) fn get(&self, key: &str) -> Option<&(String, String)> { self.0.binary_search_by(|row| row.0.as_str().cmp(key)).ok().map(|index| &self.0[index].1) }
    pub(super) fn contains_key(&self, key: &str) -> bool { self.get(key).is_some() }
    pub(super) fn len(&self) -> usize { self.0.len() }
    pub(super) fn capacity(&self) -> usize { self.0.capacity() }
    pub(super) fn is_empty(&self) -> bool { self.0.is_empty() }
    pub(super) fn try_reserve(&mut self, count: usize) -> Result<(), std::collections::TryReserveError> { self.0.try_reserve_exact(count) }
    pub(super) fn insert(&mut self, key: String, value: (String, String)) {
        match self.0.binary_search_by(|row| row.0.cmp(&key)) { Ok(index) => self.0[index].1 = value, Err(index) => self.0.insert(index, (key, value)) }
    }
    pub(super) fn remove_entry(&mut self, key: &str) -> Option<Row> { self.0.binary_search_by(|row| row.0.as_str().cmp(key)).ok().map(|index| self.0.remove(index)) }
    pub(super) fn remove(&mut self, key: &str) -> Option<(String, String)> { self.remove_entry(key).map(|row| row.1) }
    pub(super) fn keys(&self) -> impl Iterator<Item = &String> { self.0.iter().map(|row| &row.0) }
    pub(super) fn retain(&mut self, mut keep: impl FnMut(&String, &mut (String, String)) -> bool) { self.0.retain_mut(|row| keep(&row.0, &mut row.1)); }
    pub(super) fn extend(&mut self, rows: impl IntoIterator<Item = Row>) { for (key, value) in rows { self.insert(key, value); } }
}

/// 🚦️ Exact bounded forest admission outcome; an accepted edge is distinct from a sealed root.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupOwnsStep { Blocked, Progress, EdgePrepared, RootPrepared }

/// 🚫️ Allocation-free ownership-group refusal categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupOwnsError { Foreign, Stale, Visibility, Busy, Capacity, EdgeChanged, AlreadyOwned, Cycle, Incomplete, Allocation, Terminal }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase { Reserve, Edges, MergeReserve, Merge, Ready, Closing, Closed }

#[derive(Clone, Copy)]
enum Ancestor { Edge, Existing(usize), Pending(usize) }

/// 🌲️ Private complete-root preparation retaining every metadata and backing owner until funded retirement.
pub struct GroupOwnsPreparation {
    authority: Option<Arc<()>>,
    visibility: Option<Arc<ArtifactGroupVisibility>>,
    generation: u64,
    expected: usize,
    accepted: usize,
    phase: Phase,
    additions: Vec<Option<Row>>,
    active: Option<Row>,
    ancestor: Option<Ancestor>,
    hops: usize,
    insertion: Option<usize>,
    original_index: usize,
    addition_index: usize,
    replacement: OwnsRoot,
    retired: Option<Row>,
    refused: bool,
    committed: bool,
}

fn row_bytes(row: &Row) -> usize { row.0.len().saturating_add(row.1.0.len()).saturating_add(row.1.1.len()) }
fn row_release(row: &Row) -> usize { if row.0.capacity() != 0 { row.0.capacity() } else if row.1.0.capacity() != 0 { row.1.0.capacity() } else { row.1.1.capacity() } }
fn arc_release<T>(owner: &Option<Arc<T>>) -> usize {
    owner.as_ref().filter(|arc| Arc::strong_count(arc) == 1 && Arc::weak_count(arc) == 0).map_or(0, |_| std::alloc::Layout::new::<[std::sync::atomic::AtomicUsize; 2]>().extend(std::alloc::Layout::new::<T>()).expect("Arc layout").0.pad_to_align().size())
}

impl GroupOwnsPreparation {
    pub fn completed_edges(&self) -> usize { self.accepted }
    pub fn matches_visibility(&self, visibility: &Arc<ArtifactGroupVisibility>) -> bool { self.visibility.as_ref().is_some_and(|own| Arc::ptr_eq(own, visibility)) }
    pub fn terminal_is_empty(&self) -> bool { self.phase == Phase::Closed }
    pub fn next_preparation_demands(&self, graph: &CompositionGraph) -> RetirementDemand {
        let (copy_bytes, capacity_bytes) = match self.phase {
            Phase::Reserve if self.expected != 0 => (std::mem::size_of::<Vec<Option<Row>>>(), self.expected.saturating_mul(std::mem::size_of::<Option<Row>>())),
            Phase::Reserve | Phase::Edges | Phase::MergeReserve => (std::mem::size_of::<OwnsRoot>(), graph.owns.len().saturating_add(self.additions.len()).saturating_mul(std::mem::size_of::<Row>())),
            Phase::Merge => {
                if self.original_index < graph.owns.len() && (self.addition_index == self.additions.len() || graph.owns.0[self.original_index].0 < self.additions[self.addition_index].as_ref().expect("retained addition").0) { let bytes = row_bytes(&graph.owns.0[self.original_index]); (std::mem::size_of::<Row>().saturating_add(bytes), bytes) }
                else if self.addition_index < self.additions.len() { (std::mem::size_of::<Row>(), 0) } else { (0, 0) }
            }
            _ => (0, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, release_bytes: 0, depth: 1 }
    }
    /// 📏️ Borrows the active edge and distinguishes original word copies from new native backing.
    pub fn next_edge_demands(&self, graph: &CompositionGraph, parent: &str, slot: &str, child: &str) -> Result<RetirementDemand, GroupOwnsError> {
        graph.check_owns_group(self)?;
        if self.expected == 0 { return Err(GroupOwnsError::Incomplete); }
        if self.phase == Phase::Reserve { return Ok(self.next_preparation_demands(graph)); }
        if self.phase != Phase::Edges || self.refused { return Err(GroupOwnsError::Incomplete); }
        let empty = RetirementDemand { copy_bytes: 0, capacity_bytes: 0, release_bytes: 0, depth: 1 };
        if let Some(index) = self.insertion {
            let row = self.additions[index].as_ref().expect("moving edge");
            if row.0 != child || row.1.0 != parent || row.1.1 != slot { return Err(GroupOwnsError::EdgeChanged); }
            let swapping = index != 0 && self.additions[index - 1].as_ref().expect("sorted neighbor").0 > row.0;
            return Ok(RetirementDemand { copy_bytes: usize::from(swapping) * std::mem::size_of::<Option<Row>>() * 2, ..empty });
        }
        if let Some(row) = &self.active {
            if row.0 != child || row.1.0 != parent || row.1.1 != slot { return Err(GroupOwnsError::EdgeChanged); }
            return Ok(RetirementDemand { copy_bytes: if self.ancestor.is_none() { std::mem::size_of::<Row>() } else { 0 }, ..empty });
        }
        let prior = graph.owns.get(child).or_else(|| self.additions.binary_search_by(|row| row.as_ref().expect("retained edge").0.as_str().cmp(child)).ok().map(|index| &self.additions[index].as_ref().expect("retained edge").1));
        if let Some((owner, owner_slot)) = prior { return if owner == parent && owner_slot == slot { Ok(empty) } else { Err(GroupOwnsError::AlreadyOwned) }; }
        let bytes = Self::edge_birth_bytes(parent, slot, child).ok_or(GroupOwnsError::Capacity)?;
        Ok(RetirementDemand { copy_bytes: bytes.saturating_add(std::mem::size_of::<Row>()), capacity_bytes: bytes, ..empty })
    }
    pub fn next_close_copy_byte_demand(&self) -> usize {
        if self.retired.is_none() && (self.active.is_some() || !self.replacement.is_empty() || !self.additions.is_empty()) { std::mem::size_of::<Row>() } else { 0 }
    }
    pub fn next_close_capacity_byte_demand(&self, _: usize) -> usize { 0 }
    pub fn next_close_depth_demand(&self) -> usize { usize::from(self.phase != Phase::Closed) }
    pub fn next_close_release_byte_demand(&self) -> usize {
        if let Some(row) = &self.retired { return row_release(row); }
        if self.active.is_some() || !self.replacement.is_empty() || !self.additions.is_empty() { return 0; }
        if self.replacement.capacity() != 0 { return self.replacement.capacity().saturating_mul(std::mem::size_of::<Row>()); }
        if self.additions.capacity() != 0 { return self.additions.capacity().saturating_mul(std::mem::size_of::<Option<Row>>()); }
        if self.authority.is_some() { return arc_release(&self.authority); }
        arc_release(&self.visibility)
    }
    pub fn edge_birth_bytes(parent: &str, slot: &str, child: &str) -> Option<usize> { parent.len().checked_add(slot.len())?.checked_add(child.len()) }
}

impl Drop for GroupOwnsPreparation {
    fn drop(&mut self) { assert!(self.terminal_is_empty(), "ownership group retains its root and physical owners until bounded close"); }
}

impl CompositionGraph {
    fn check_owns_group(&self, preparation: &GroupOwnsPreparation) -> Result<(), GroupOwnsError> {
        if !preparation.authority.as_ref().is_some_and(|authority| self.owns_authority.as_ref().is_some_and(|own| Arc::ptr_eq(authority, own))) { return Err(GroupOwnsError::Foreign); }
        if self.owns_generation != preparation.generation { return Err(GroupOwnsError::Stale); }
        if self.owns_group != preparation.visibility.as_ref().map(|visibility| Arc::as_ptr(visibility) as usize) { return Err(GroupOwnsError::Foreign); }
        if !preparation.visibility.as_ref().is_some_and(|visibility| visibility.pending()) { return Err(GroupOwnsError::Visibility); }
        Ok(())
    }

    /// 🌱️ Begins a bare private cursor without cloning identifiers or allocating row storage.
    pub fn begin_owns_group(&mut self, visibility: &Arc<ArtifactGroupVisibility>, edge_count: usize) -> Result<GroupOwnsPreparation, GroupOwnsError> {
        if self.owns_group.is_some() { return Err(GroupOwnsError::Busy); }
        if !visibility.pending() { return Err(GroupOwnsError::Visibility); }
        if self.owns.len().checked_add(edge_count).is_none_or(|count| count > OWNED_DOCUMENT_MAXIMUM_MEMBERS) { return Err(GroupOwnsError::Capacity); }
        self.owns_generation.checked_add(1).ok_or(GroupOwnsError::Capacity)?;
        let authority = Arc::clone(self.owns_authority.as_ref().ok_or(GroupOwnsError::Terminal)?);
        self.owns_group = Some(Arc::as_ptr(visibility) as usize);
        Ok(GroupOwnsPreparation { authority: Some(authority), visibility: Some(Arc::clone(visibility)), generation: self.owns_generation, expected: edge_count, accepted: 0, phase: Phase::Reserve, additions: Vec::new(), active: None, ancestor: None, hops: 0, insertion: None, original_index: 0, addition_index: 0, replacement: OwnsRoot::default(), retired: None, refused: false, committed: false })
    }

    /// 🪜️ Funds one metadata birth, ancestor hop or sorted-row move while all live roots remain unchanged.
    pub fn prepare_owns_group_edge(&self, preparation: &mut GroupOwnsPreparation, parent: &str, slot: &str, child: &str, grant: ArtifactStoreOneItemGrant) -> Result<GroupOwnsStep, GroupOwnsError> {
        self.check_owns_group(preparation)?;
        if preparation.refused { return Err(GroupOwnsError::Terminal); }
        let demand = preparation.next_edge_demands(self, parent, slot, child)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(GroupOwnsStep::Blocked); }
        if preparation.phase == Phase::Reserve {
            preparation.additions.try_reserve_exact(preparation.expected).map_err(|_| GroupOwnsError::Allocation)?;
            preparation.phase = Phase::Edges;
            return Ok(GroupOwnsStep::Progress);
        }
        if preparation.phase != Phase::Edges || preparation.accepted >= preparation.expected { return Err(GroupOwnsError::Incomplete); }
        if let Some(position) = preparation.insertion {
            let row = preparation.additions[position].as_ref().expect("moving edge");
            if row.0 != child || row.1.0 != parent || row.1.1 != slot { return Err(GroupOwnsError::EdgeChanged); }
            if position != 0 && preparation.additions[position - 1].as_ref().expect("sorted neighbor").0 > row.0 {
                preparation.additions.swap(position, position - 1);
                preparation.insertion = Some(position - 1);
                return Ok(GroupOwnsStep::Progress);
            }
            preparation.insertion = None;
            preparation.accepted += 1;
            return Ok(GroupOwnsStep::EdgePrepared);
        }
        if let Some(row) = &preparation.active {
            if row.0 != child || row.1.0 != parent || row.1.1 != slot { return Err(GroupOwnsError::EdgeChanged); }
        } else {
            let existing = self.owns.get(child).or_else(|| preparation.additions.binary_search_by(|row| row.as_ref().expect("sorted retained edge").0.as_str().cmp(child)).ok().map(|index| &preparation.additions[index].as_ref().expect("retained edge").1));
            if let Some((owner, owner_slot)) = existing {
                if owner != parent || owner_slot != slot { preparation.refused = true; return Err(GroupOwnsError::AlreadyOwned); }
                preparation.accepted += 1;
                return Ok(GroupOwnsStep::EdgePrepared);
            }
            preparation.active = Some((child.into(), (parent.into(), slot.into())));
            preparation.ancestor = Some(Ancestor::Edge);
            preparation.hops = 0;
            return Ok(GroupOwnsStep::Progress);
        }
        if let Some(ancestor) = preparation.ancestor {
            let current = match ancestor { Ancestor::Edge => preparation.active.as_ref().expect("edge owner").1.0.as_str(), Ancestor::Existing(index) => self.owns.0[index].1.0.as_str(), Ancestor::Pending(index) => preparation.additions[index].as_ref().expect("pending ancestor").1.0.as_str() };
            if current == child || preparation.hops > self.owns.len() + preparation.additions.len() { preparation.refused = true; return Err(GroupOwnsError::Cycle); }
            preparation.ancestor = if let Ok(index) = preparation.additions.binary_search_by(|row| row.as_ref().expect("sorted edge").0.as_str().cmp(current)) { Some(Ancestor::Pending(index)) } else { self.owns.0.binary_search_by(|row| row.0.as_str().cmp(current)).ok().map(Ancestor::Existing) };
            preparation.hops += 1;
            return Ok(GroupOwnsStep::Progress);
        }
        preparation.additions.push(preparation.active.take());
        preparation.insertion = Some(preparation.additions.len() - 1);
        Ok(GroupOwnsStep::Progress)
    }

    /// 🧱️ Builds one sorted replacement row per funded turn; the source graph stays immutable.
    pub fn seal_owns_group(&self, preparation: &mut GroupOwnsPreparation, grant: ArtifactStoreOneItemGrant) -> Result<GroupOwnsStep, GroupOwnsError> {
        self.check_owns_group(preparation)?;
        if preparation.refused || preparation.active.is_some() || preparation.insertion.is_some() || preparation.accepted != preparation.expected { return Err(GroupOwnsError::Incomplete); }
        let demand = preparation.next_preparation_demands(self);
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(GroupOwnsStep::Blocked); }
        if preparation.phase == Phase::Reserve && preparation.expected == 0 { preparation.phase = Phase::Edges; }
        if preparation.phase == Phase::Edges { preparation.phase = Phase::MergeReserve; }
        if preparation.phase == Phase::MergeReserve {
            preparation.replacement.0.try_reserve_exact(self.owns.len() + preparation.additions.len()).map_err(|_| GroupOwnsError::Allocation)?;
            preparation.phase = Phase::Merge;
            return Ok(GroupOwnsStep::Progress);
        }
        if preparation.phase == Phase::Ready { return Ok(GroupOwnsStep::RootPrepared); }
        if preparation.phase != Phase::Merge { return Err(GroupOwnsError::Incomplete); }
        if preparation.original_index == self.owns.len() && preparation.addition_index == preparation.additions.len() { preparation.phase = Phase::Ready; return Ok(GroupOwnsStep::RootPrepared); }
        let row = if preparation.original_index < self.owns.len() && (preparation.addition_index == preparation.additions.len() || self.owns.0[preparation.original_index].0 < preparation.additions[preparation.addition_index].as_ref().expect("retained addition").0) {
            let row = self.owns.0[preparation.original_index].clone(); preparation.original_index += 1; row
        } else { let row = preparation.additions[preparation.addition_index].take().expect("owned pending row"); preparation.addition_index += 1; row };
        preparation.replacement.0.push(row);
        Ok(GroupOwnsStep::Progress)
    }

    /// 🛡️ Proves exact ready-root authority before the shared visibility owner can flip.
    pub fn owns_group_ready(&self, preparation: &GroupOwnsPreparation) -> bool { self.check_owns_group(preparation).is_ok() && preparation.phase == Phase::Ready && !preparation.refused }

    /// ⚡️ Moves the complete sorted root and advances its generation once, with no post-flip allocation.
    pub fn commit_owns_group(&mut self, preparation: &mut GroupOwnsPreparation) -> Result<(), GroupOwnsError> {
        if preparation.phase != Phase::Ready || preparation.refused { return Err(GroupOwnsError::Incomplete); }
        if !preparation.authority.as_ref().is_some_and(|authority| self.owns_authority.as_ref().is_some_and(|own| Arc::ptr_eq(authority, own))) { return Err(GroupOwnsError::Foreign); }
        if self.owns_generation != preparation.generation { return Err(GroupOwnsError::Stale); }
        if self.owns_group != preparation.visibility.as_ref().map(|visibility| Arc::as_ptr(visibility) as usize) { return Err(GroupOwnsError::Foreign); }
        if !preparation.visibility.as_ref().is_some_and(|visibility| visibility.committed()) { return Err(GroupOwnsError::Visibility); }
        std::mem::swap(&mut self.owns, &mut preparation.replacement);
        self.owns_generation += 1;
        self.owns_group = None;
        preparation.committed = true;
        preparation.phase = Phase::Closing;
        Ok(())
    }

    /// 🧹️ Cancels a private cursor or retires its displaced root under whole-allocation grants.
    pub fn close_owns_group(&mut self, preparation: &mut GroupOwnsPreparation, grant: RetainedCloneGrant) -> RetainedCloneStep {
        let empty = RetainedCloneProgress::default();
        if preparation.phase == Phase::Closed { return RetainedCloneStep::Complete(empty); }
        if grant.maximum_items == 0 || grant.maximum_depth < preparation.next_close_depth_demand() { return RetainedCloneStep::Progress(empty); }
        if !preparation.committed && preparation.visibility.as_ref().is_some_and(|visibility| visibility.committed()) { return RetainedCloneStep::Progress(empty); }
        let copied_bytes = preparation.next_close_copy_byte_demand();
        let released_bytes = preparation.next_close_release_byte_demand();
        if grant.maximum_copy_bytes < copied_bytes || grant.maximum_release_bytes < released_bytes { return RetainedCloneStep::Progress(empty); }
        if preparation.authority.as_ref().is_some_and(|authority| self.owns_authority.as_ref().is_some_and(|own| Arc::ptr_eq(authority, own))) && self.owns_group == preparation.visibility.as_ref().map(|visibility| Arc::as_ptr(visibility) as usize) { self.owns_group = None; }
        preparation.phase = Phase::Closing;
        if let Some(row) = &mut preparation.retired {
            if row.0.capacity() != 0 { drop(std::mem::take(&mut row.0)); }
            else if row.1.0.capacity() != 0 { drop(std::mem::take(&mut row.1.0)); }
            else if row.1.1.capacity() != 0 { drop(std::mem::take(&mut row.1.1)); }
            else { preparation.retired.take(); }
        } else if preparation.active.is_some() { preparation.retired = preparation.active.take(); }
        else if !preparation.replacement.is_empty() { preparation.retired = preparation.replacement.0.pop(); }
        else if !preparation.additions.is_empty() { preparation.retired = preparation.additions.pop().flatten(); }
        else if preparation.replacement.capacity() != 0 { drop(std::mem::take(&mut preparation.replacement)); }
        else if preparation.additions.capacity() != 0 { drop(std::mem::take(&mut preparation.additions)); }
        else if preparation.authority.is_some() { preparation.authority.take(); }
        else if preparation.visibility.is_some() { preparation.visibility.take(); }
        else { preparation.phase = Phase::Closed; return RetainedCloneStep::Complete(empty); }
        RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes, released_bytes, ..empty })
    }
}

impl CompositionGraph {
    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(if self.owns_group.is_none() && self.owns_retiring.is_none() && !self.owns.is_empty() { std::mem::size_of::<Row>() } else if self.owns_retiring.is_none() && self.owns.capacity() == 0 && self.owns_authority.is_none() { self.links.copy_demand() } else { 0 })
    }
    pub fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(!self.terminal_is_empty())) }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if self.owns_group.is_some() { return Ok(0); }
        if let Some(row) = self.owns_retiring.as_ref() { return Ok(row_release(row)); }
        if !self.owns.is_empty() { return Ok(0); }
        if self.owns.capacity() != 0 { return Ok(self.owns.capacity().saturating_mul(std::mem::size_of::<Row>())); }
        if self.owns_authority.is_some() { return Ok(arc_release(&self.owns_authority)); }
        Ok(self.links.release_demand())
    }
    pub(super) fn close_owned_root(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, ValueError> {
        let empty = RetainedCloneProgress::default();
        let copied_bytes = self.next_close_copy_byte_demand()?;
        let released_bytes = self.next_close_release_byte_demand()?;
        if grant.maximum_copy_bytes < copied_bytes || grant.maximum_release_bytes < released_bytes { return Ok(Some(RetainedCloneStep::Progress(empty))); }
        if let Some(row) = self.owns_retiring.as_mut() {
            let owner = if row.0.capacity() != 0 { &mut row.0 } else if row.1.0.capacity() != 0 { &mut row.1.0 } else { &mut row.1.1 };
            drop(std::mem::take(owner));
            if row_release(row) == 0 { self.owns_retiring = None; }
        } else if let Some(row) = self.owns.0.pop() {
            self.owns_retiring = Some(row);
            self.owns_generation = self.owns_generation.checked_add(1).expect("ownership generation exhausted");
        } else if self.owns.capacity() != 0 { drop(std::mem::take(&mut self.owns)); }
        else if self.owns_authority.is_some() { self.owns_authority.take(); }
        else { return Ok(None); }
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes, released_bytes, ..empty })))
    }
}
