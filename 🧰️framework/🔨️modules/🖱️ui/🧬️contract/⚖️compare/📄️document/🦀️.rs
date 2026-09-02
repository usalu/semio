//! 📄️ An admitted comparison owns one exact immutable document read and the incoming component.

use super::*;
use crate::{Component, UiComponentCompareProgress, UiComponentComparisonCursor, UiTypedRetirementCursor, UiValueRetirementStep};
use std::mem::{size_of, ManuallyDrop};

//#region 🎟️Admission
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentCompareError { Admission, Contended, Poisoned, Closing, NodeIdentity }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiDocumentCompareAdmission {
    pub owner_bytes: usize,
    pub moved_bytes: usize,
    pub allocated_bytes: usize,
}

fn record_for_comparison<'a>(arena: &'a UiDocumentArena, lease: &UiDocumentLease, ordinal: usize, id: UiNodeId) -> Result<&'a UiNodeRecord, UiDocumentCompareError> {
    if lease.released || lease.claimed { return Err(UiDocumentCompareError::Closing); }
    let slot = lease.handle.and_then(|handle| arena.slot(handle)).ok_or(UiDocumentCompareError::Closing)?;
    if slot.retiring || !slot.complete { return Err(UiDocumentCompareError::Closing); }
    let record = slot.nodes.get_index(ordinal).ok_or(UiDocumentCompareError::NodeIdentity)?;
    if record.id != id { return Err(UiDocumentCompareError::NodeIdentity); }
    Ok(record)
}

struct OwnedDocumentComparison { lease: Option<UiDocumentLease>, incoming: Option<Component> }

pub struct UiDocumentComponentCompare {
    owned: ManuallyDrop<OwnedDocumentComparison>,
    cursor: UiComponentComparisonCursor,
    retirement: UiTypedRetirementCursor,
    ordinal: usize,
    id: UiNodeId,
    closing: bool,
}

impl UiDocumentComponentCompare {
    /// 🧮️ Includes all fixed comparison frames, inline owner initialization, and exact root moves.
    pub const fn required_admission_bytes() -> usize { size_of::<Self>() + size_of::<UiDocumentLease>() + size_of::<Component>() }

    /// 🎟️ No cursor initialization, alias minting, component copy, or heap allocation precedes admission.
    pub fn try_new(lease: UiDocumentLease, ordinal: usize, id: UiNodeId, incoming: Component, admitted_bytes: usize) -> Result<(Self, UiDocumentCompareAdmission), (UiDocumentCompareError, UiDocumentLease, Component)> {
        if admitted_bytes < Self::required_admission_bytes() { return Err((UiDocumentCompareError::Admission, lease, incoming)); }
        let arena = match UI_DOCUMENT_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return Err((UiDocumentCompareError::Contended, lease, incoming)),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err((UiDocumentCompareError::Poisoned, lease, incoming)),
        };
        if let Err(error) = record_for_comparison(&arena, &lease, ordinal, id) { return Err((error, lease, incoming)); }
        let admission = UiDocumentCompareAdmission { owner_bytes: size_of::<Self>(), moved_bytes: size_of::<UiDocumentLease>() + size_of::<Component>(), allocated_bytes: 0 };
        Ok((Self { owned: ManuallyDrop::new(OwnedDocumentComparison { lease: Some(lease), incoming: Some(incoming) }), cursor: Default::default(), retirement: Default::default(), ordinal, id, closing: false }, admission))
    }
    pub fn result(&self) -> Option<bool> { if self.closing { None } else { self.cursor.result() } }
    pub fn incoming(&self) -> Option<&Component> { if self.closing { None } else { self.owned.incoming.as_ref() } }

    pub fn advance(&mut self, items: usize, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        if self.closing { return Err("document component comparison is closing"); }
        if items == 0 || bytes == 0 { return Ok(Default::default()); }
        let arena = match UI_DOCUMENT_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(Default::default()),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err("document comparison arena is poisoned"),
        };
        let owned = &*self.owned;
        let lease = owned.lease.as_ref().ok_or("document comparison read is missing")?;
        let record = record_for_comparison(&arena, lease, self.ordinal, self.id).map_err(|_| "document comparison exact root or ordinal is not readable")?;
        self.cursor.advance(&record.component, owned.incoming.as_ref().ok_or("document comparison incoming root is missing")?, bytes)
    }

    pub fn take_completed(&mut self) -> Option<(UiDocumentLease, Component)> {
        if self.closing || self.cursor.result().is_none() { return None; }
        self.cursor.release_reads();
        let owned = &mut *self.owned;
        Some((owned.lease.take()?, owned.incoming.take()?))
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.terminal_is_empty() { return Ok(UiValueRetirementStep { complete: true, ..Default::default() }); }
        if items == 0 || bytes == 0 { return Ok(Default::default()); }
        self.closing = true;
        self.cursor.release_reads();
        let owned = &mut *self.owned;
        if owned.incoming.is_some() {
            let mut step = self.retirement.advance(&mut owned.incoming, items, bytes)?;
            step.complete = false;
            return Ok(step);
        }
        if let Some(lease) = owned.lease.as_mut() {
            let step = typed_retirement::close_document_read_owner(&mut lease.handle, &mut lease.released, &mut lease.claimed, items, bytes)?;
            if step.complete { owned.lease = None; }
            return Ok(step);
        }
        Ok(UiValueRetirementStep { complete: self.terminal_is_empty(), ..Default::default() })
    }
    pub fn terminal_is_empty(&self) -> bool { self.owned.lease.is_none() && self.owned.incoming.is_none() && self.cursor.reads_empty() && (!self.closing || self.retirement.terminal_is_empty()) }
}

impl Drop for UiDocumentComponentCompare {
    fn drop(&mut self) { if !self.terminal_is_empty() && !std::thread::panicking() { panic!("document comparison requires exact read and incoming retirement"); } }
}
//#endregion 🎟️Admission
