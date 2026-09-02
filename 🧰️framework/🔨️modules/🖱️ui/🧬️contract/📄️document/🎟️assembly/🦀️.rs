//! 🎟️ Canonical document assembly admits one page or record while preserving exact rejected owners.

use super::*;
use std::mem::size_of;
use std::sync::{MutexGuard, TryLockError};

//#region 🎟️Admission
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiDocumentAssemblyProgress {
    pub progressed: bool,
    pub metadata_items: usize,
    pub allocated_bytes: usize,
    pub initialized_bytes: usize,
    pub compared_bytes: usize,
    pub moved_bytes: usize,
    pub complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentAssemblyErrorKind { Occupied, MissingSurface, InvalidGeneration, ArenaFull, Stale, Closing, Contended, Poisoned, DuplicateNode, Allocation, MissingRoot }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiDocumentAssemblyError { pub kind: UiDocumentAssemblyErrorKind, pub allocated_bytes: usize, pub initialized_bytes: usize, pub compared_bytes: usize }

fn error(kind: UiDocumentAssemblyErrorKind) -> UiDocumentAssemblyError { UiDocumentAssemblyError { kind, allocated_bytes: 0, initialized_bytes: 0, compared_bytes: 0 } }
fn resident_error(cause: UiResidentFault) -> UiDocumentAssemblyError {
    error(match cause { UiResidentFault::Contended => UiDocumentAssemblyErrorKind::Contended, UiResidentFault::Poisoned => UiDocumentAssemblyErrorKind::Poisoned, UiResidentFault::Owner => UiDocumentAssemblyErrorKind::Stale, _ => UiDocumentAssemblyErrorKind::Allocation })
}
fn arena() -> Result<MutexGuard<'static, UiDocumentArena>, UiDocumentAssemblyError> {
    UI_DOCUMENT_ARENA.try_lock().map_err(|cause| error(match cause { TryLockError::WouldBlock => UiDocumentAssemblyErrorKind::Contended, TryLockError::Poisoned(_) => UiDocumentAssemblyErrorKind::Poisoned }))
}

#[derive(Debug, Default)]
pub struct UiDocumentAssembly {
    builder: Option<UiDocumentBuilder>,
    checking: Option<UiNodeId>,
    compared: usize,
    root_ordinal: Option<usize>,
    closing: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiDocumentRootIdentity(UiDocumentHandle);

impl UiDocumentAssembly {
    pub fn root_identity(&self) -> Option<UiDocumentRootIdentity> { self.builder.as_ref().and_then(|builder| builder.handle).map(UiDocumentRootIdentity) }
    pub const fn required_open_bytes() -> usize { size_of::<UiDocumentSlot>() + size_of::<Self>() + size_of::<SurfaceId>() + size_of::<UiResidentPermit>() }

    /// 🧊️ Cold convenience reserves the fixed surface ceiling; retained callers transfer their admitted job permit.
    pub fn open_into(&mut self, surface: &mut Option<SurfaceId>, generation: u64, revision: UiRevision, root: Option<UiNodeId>, layout_epoch: u64, items: usize, bytes: usize) -> Result<UiDocumentAssemblyProgress, UiDocumentAssemblyError> {
        if items == 0 || bytes < Self::required_open_bytes() { return Ok(Default::default()); }
        let mut permit = None;
        UiResidentPermit::try_reserve(UiResidentLimits { items: UI_RESIDENT_SURFACE_ITEMS, bytes: UI_RESIDENT_SURFACE_BYTES }, &mut permit, bytes).map_err(|_| error(UiDocumentAssemblyErrorKind::ArenaFull))?;
        let result = self.open_with_permit(&mut permit, surface, generation, revision, root, layout_epoch, items, bytes);
        if let Some(permit) = permit.as_mut() { let _ = permit.close_step(1); }
        result
    }

    /// 🎟️ The exact job reservation becomes the root's sole credit before any payload allocation.
    pub fn open_with_permit(&mut self, permit: &mut Option<UiResidentPermit>, surface: &mut Option<SurfaceId>, generation: u64, revision: UiRevision, root: Option<UiNodeId>, layout_epoch: u64, items: usize, bytes: usize) -> Result<UiDocumentAssemblyProgress, UiDocumentAssemblyError> {
        if self.builder.is_some() || self.closing { return Err(error(UiDocumentAssemblyErrorKind::Occupied)); }
        if generation == 0 { return Err(error(UiDocumentAssemblyErrorKind::InvalidGeneration)); }
        if surface.is_none() { return Err(error(UiDocumentAssemblyErrorKind::MissingSurface)); }
        if items == 0 || bytes < Self::required_open_bytes() { return Ok(Default::default()); }
        let key = permit.as_ref().and_then(UiResidentPermit::root_key).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?;
        if permit.as_ref().unwrap().limits().bytes < Self::required_open_bytes() { return Err(error(UiDocumentAssemblyErrorKind::Allocation)); }
        let mut arena = arena()?;
        let slot = &mut arena.slots[key.slot];
        if slot.occupied || slot.resident.is_some() || !slot.nodes.entries.terminal_is_empty() || slot.surface.is_some() { return Err(error(UiDocumentAssemblyErrorKind::Stale)); }
        *slot = UiDocumentSlot { resident: permit.take(), epoch: key.epoch, generation, surface: surface.take(), revision, root, layout_epoch, aliases: 1, occupied: true, ..UiDocumentSlot::empty() };
        self.builder = Some(UiDocumentBuilder { handle: Some(UiDocumentHandle { slot: key.slot, epoch: key.epoch, generation }), released: false, claimed: false });
        self.checking = None; self.compared = 0; self.root_ordinal = None;
        Ok(UiDocumentAssemblyProgress { progressed: true, metadata_items: 1, initialized_bytes: size_of::<UiDocumentSlot>() + size_of::<Self>(), moved_bytes: size_of::<SurfaceId>() + size_of::<UiResidentPermit>(), ..Default::default() })
    }

    fn handle(&self) -> Result<UiDocumentHandle, UiDocumentAssemblyError> {
        if self.closing { return Err(error(UiDocumentAssemblyErrorKind::Closing)); }
        self.builder.as_ref().and_then(|builder| builder.handle).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))
    }
    pub fn allocated_bytes(&self) -> Result<usize, UiDocumentAssemblyError> {
        let handle = self.handle()?;
        Ok(arena()?.slot(handle).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?.nodes.entries.allocated_bytes())
    }
    pub fn len(&self) -> Result<usize, UiDocumentAssemblyError> {
        let handle = self.handle()?;
        Ok(arena()?.slot(handle).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?.nodes.len())
    }

    pub fn next_allocation_bytes(&self) -> Result<usize, UiDocumentAssemblyError> {
        let handle = self.handle()?;
        let arena = arena()?;
        let slot = arena.slot(handle).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?;
        if slot.nodes.entries.has_reserved_slot() { return Ok(0); }
        slot.nodes.entries.next_allocation_bytes().map_err(|_| error(UiDocumentAssemblyErrorKind::Allocation))
    }

    /// 📥️ Each opportunity compares one fixed ID, admits one page, or moves one record.
    pub fn place_one(&mut self, source: &mut Option<UiNodeRecord>, items: usize, bytes: usize) -> Result<UiDocumentAssemblyProgress, UiDocumentAssemblyError> {
        let handle = self.handle()?;
        if items == 0 || bytes == 0 { return Ok(Default::default()); }
        let Some(id) = source.as_ref().map(|record| record.id) else { return Ok(UiDocumentAssemblyProgress { complete: true, ..Default::default() }); };
        let mut arena = arena()?;
        let slot = arena.slot_mut(handle).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?;
        if slot.retiring || slot.complete { return Err(error(UiDocumentAssemblyErrorKind::Closing)); }
        if self.checking != Some(id) {
            if bytes < size_of::<UiNodeId>() { return Ok(Default::default()); }
            self.checking = Some(id); self.compared = 0;
            return Ok(UiDocumentAssemblyProgress { progressed: true, metadata_items: 1, moved_bytes: size_of::<UiNodeId>(), ..Default::default() });
        }
        if self.compared < slot.nodes.len() {
            if bytes < 2 * size_of::<UiNodeId>() { return Ok(Default::default()); }
            if slot.nodes.get_index(self.compared).is_some_and(|record| record.id == id) { return Err(UiDocumentAssemblyError { compared_bytes: 2 * size_of::<UiNodeId>(), ..error(UiDocumentAssemblyErrorKind::DuplicateNode) }); }
            self.compared += 1;
            return Ok(UiDocumentAssemblyProgress { progressed: true, metadata_items: 1, compared_bytes: 2 * size_of::<UiNodeId>(), ..Default::default() });
        }
        if !slot.nodes.entries.has_reserved_slot() {
            let requested = slot.nodes.entries.next_allocation_bytes().map_err(|_| error(UiDocumentAssemblyErrorKind::Allocation))?;
            let result = slot.nodes.entries.try_reserve_one(bytes);
            let initialized = if slot.nodes.entries.has_reserved_slot() { 0 } else { requested };
            return result.map(|step| UiDocumentAssemblyProgress { progressed: step.progressed, metadata_items: usize::from(step.progressed), allocated_bytes: step.allocated_bytes, initialized_bytes: if step.progressed { initialized } else { 0 }, ..Default::default() }).map_err(|cause| UiDocumentAssemblyError { allocated_bytes: cause.allocated_bytes, initialized_bytes: if cause.allocated_bytes == 0 { 0 } else { initialized }, ..error(UiDocumentAssemblyErrorKind::Allocation) });
        }
        let ordinal = slot.nodes.len();
        let step = slot.nodes.entries.try_place_reserved(source, bytes).map_err(|_| error(UiDocumentAssemblyErrorKind::Stale))?;
        if source.is_none() { if slot.root == Some(id) { self.root_ordinal = Some(ordinal); } self.checking = None; self.compared = 0; }
        Ok(UiDocumentAssemblyProgress { progressed: step.progressed, metadata_items: usize::from(step.progressed), moved_bytes: step.placed_bytes, complete: source.is_none(), ..Default::default() })
    }

    /// 📉️ A caller's complete resident census may return unused credit while the root remains owned here.
    pub fn shrink_resident(&mut self, limits: UiResidentLimits, items: usize, bytes: usize) -> Result<UiDocumentAssemblyProgress, UiDocumentAssemblyError> {
        let handle = self.handle()?;
        if items == 0 || bytes < size_of::<UiResidentPermit>() { return Ok(Default::default()); }
        let mut arena = arena()?;
        let slot = arena.slot_mut(handle).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?;
        if slot.retiring || slot.complete { return Err(error(UiDocumentAssemblyErrorKind::Closing)); }
        let minimum = size_of::<UiDocumentSlot>().checked_add(slot.nodes.entries.allocated_bytes()).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Allocation))?;
        if limits.bytes < minimum || limits.items < slot.nodes.len() { return Err(error(UiDocumentAssemblyErrorKind::Allocation)); }
        let permit = slot.resident.as_mut().ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?;
        let progressed = permit.try_shrink(limits).map_err(resident_error)?;
        Ok(UiDocumentAssemblyProgress { progressed, metadata_items: usize::from(progressed), complete: progressed, ..Default::default() })
    }

    /// 📨️ Only the output obligation moves; the canonical slot retains the original root permit.
    pub fn split_resident_output(&mut self, target: &mut Option<UiResidentPermit>, items: usize, bytes: usize) -> Result<UiDocumentAssemblyProgress, UiDocumentAssemblyError> {
        let handle = self.handle()?;
        if target.is_some() { return Err(error(UiDocumentAssemblyErrorKind::Occupied)); }
        if items == 0 || bytes < size_of::<UiResidentPermit>() { return Ok(Default::default()); }
        let mut arena = arena()?;
        let slot = arena.slot_mut(handle).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?;
        if slot.retiring || slot.complete { return Err(error(UiDocumentAssemblyErrorKind::Closing)); }
        let progressed = slot.resident.as_mut().ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?.split_output_into(target, bytes).map_err(resident_error)?;
        Ok(UiDocumentAssemblyProgress { progressed, metadata_items: usize::from(progressed), moved_bytes: if progressed { size_of::<UiResidentPermit>() } else { 0 }, complete: progressed, ..Default::default() })
    }

    /// 📬️ The recorded root ordinal avoids a whole-document validation scan at publication.
    pub fn finish_into(&mut self, target: &mut Option<UiDocumentLease>, revision: UiRevision, items: usize, bytes: usize) -> Result<UiDocumentAssemblyProgress, UiDocumentAssemblyError> {
        if target.is_some() { return Err(error(UiDocumentAssemblyErrorKind::Occupied)); }
        let handle = self.handle()?;
        if items == 0 || bytes < size_of::<UiDocumentLease>() + size_of::<Self>() { return Ok(Default::default()); }
        let mut arena = arena()?;
        let slot = arena.slot_mut(handle).ok_or_else(|| error(UiDocumentAssemblyErrorKind::Stale))?;
        if slot.retiring || slot.complete || self.checking.is_some() { return Err(error(UiDocumentAssemblyErrorKind::Closing)); }
        let root = self.root_ordinal.and_then(|index| slot.nodes.get_index(index)).map(|record| record.id);
        if root.is_none() || root != slot.root { return Err(error(UiDocumentAssemblyErrorKind::MissingRoot)); }
        slot.revision = revision; slot.complete = true;
        self.builder.as_mut().unwrap().handle = None;
        self.builder = None; self.root_ordinal = None;
        *target = Some(UiDocumentLease { handle: Some(handle), released: false, claimed: false });
        Ok(UiDocumentAssemblyProgress { progressed: true, metadata_items: 1, moved_bytes: size_of::<UiDocumentLease>(), complete: true, ..Default::default() })
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<crate::UiValueRetirementStep, &'static str> {
        if self.terminal_is_empty() { return Ok(crate::UiValueRetirementStep { complete: true, ..Default::default() }); }
        if items == 0 || bytes == 0 { return Ok(Default::default()); }
        self.closing = true; self.checking = None; self.compared = 0; self.root_ordinal = None;
        let step = self.builder.as_mut().unwrap().close_step_with_grant(items, bytes)?;
        if step.complete { self.builder = None; }
        Ok(step)
    }
    pub fn terminal_is_empty(&self) -> bool { self.builder.is_none() }
}
//#endregion 🎟️Admission

//#region 📖️ExactRead
pub struct UiDocumentRead<'a> { guard: MutexGuard<'a, UiDocumentArena>, handle: UiDocumentHandle }
impl UiDocumentRead<'_> {
    pub fn resident_limits(&self) -> UiResidentLimits { self.guard.slot(self.handle).unwrap().resident.as_ref().expect("readable canonical root retains its permit").limits() }
    pub fn len(&self) -> usize { self.guard.slot(self.handle).unwrap().nodes.len() }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn allocated_bytes(&self) -> usize { self.guard.slot(self.handle).unwrap().nodes.entries.allocated_bytes() }
    pub fn node_at(&self, ordinal: usize) -> Option<&UiNodeRecord> { self.guard.slot(self.handle)?.nodes.get_index(ordinal) }
    pub fn exact_node(&self, ordinal: usize, id: UiNodeId) -> Result<&UiNodeRecord, UiDocumentLeaseError> {
        self.node_at(ordinal).filter(|record| record.id == id).ok_or(UiDocumentLeaseError::NodeIdentity)
    }
}

impl UiDocumentLease {
    pub fn root_identity(&self) -> Option<UiDocumentRootIdentity> { if self.released || self.claimed { None } else { self.handle.map(UiDocumentRootIdentity) } }
    /// 🔒️ A read borrows the exact immutable slot without cloning or exposing a mutable root.
    pub fn try_read(&self) -> Result<UiDocumentRead<'_>, UiDocumentLeaseError> {
        if self.released || self.claimed { return Err(UiDocumentLeaseError::Closing); }
        let handle = self.handle.ok_or(UiDocumentLeaseError::StaleHandle)?;
        let guard = UI_DOCUMENT_ARENA.try_lock().map_err(|cause| match cause { TryLockError::WouldBlock => UiDocumentLeaseError::Contended, TryLockError::Poisoned(_) => UiDocumentLeaseError::Poisoned })?;
        let slot = guard.slot(handle).ok_or(UiDocumentLeaseError::StaleHandle)?;
        if slot.retiring || !slot.complete { return Err(UiDocumentLeaseError::Closing); }
        Ok(UiDocumentRead { guard, handle })
    }

    pub fn try_alias_into(&self, target: &mut Option<Self>, bytes: usize) -> Result<bool, UiDocumentLeaseError> {
        if target.is_some() || bytes < size_of::<Self>() { return Ok(false); }
        if self.released || self.claimed { return Err(UiDocumentLeaseError::Closing); }
        let handle = self.handle.ok_or(UiDocumentLeaseError::StaleHandle)?;
        let mut arena = UI_DOCUMENT_ARENA.try_lock().map_err(|cause| match cause { TryLockError::WouldBlock => UiDocumentLeaseError::Contended, TryLockError::Poisoned(_) => UiDocumentLeaseError::Poisoned })?;
        let handle = arena.alias(handle)?;
        *target = Some(Self { handle: Some(handle), released: false, claimed: false });
        Ok(true)
    }
    pub fn same_root(&self, other: &Self) -> bool { self.handle.is_some() && self.handle == other.handle && !self.released && !other.released && !self.claimed && !other.claimed }
    pub fn close_read_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<crate::UiValueRetirementStep, &'static str> {
        typed_retirement::close_document_read_owner(&mut self.handle, &mut self.released, &mut self.claimed, items, bytes)
    }
}
//#endregion 📖️ExactRead
