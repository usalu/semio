//! 📄️ Exact document-slot retirement; payload fields remain rooted until descendant completion.

use super::*;
use crate::{UiArenaHandback, UiArenaHandbacks, UiValueRetirementStep, UiTypedRetirementCursor};

static DOCUMENT_HANDBACKS: UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1> = UiArenaHandbacks::new();

//#region 🧵️SlotTraversal
impl UiDocumentArena {
    fn consume_handback(&mut self, index: usize) -> Result<UiValueRetirementStep, &'static str> {
        let Some(obligation) = DOCUMENT_HANDBACKS.take_one(index) else { return Ok(UiValueRetirementStep { progressed: true, ..Default::default() }) };
        let slot = &mut self.slots[index];
        let result = match obligation {
            UiArenaHandback::ReleaseAlias => {
                if !slot.occupied || slot.retiring || slot.aliases == 0 { Err("returned document alias is not retained") } else {
                    slot.aliases -= 1;
                    if slot.aliases == 0 { slot.retiring = true; slot.complete = false; }
                    Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() })
                }
            }
            UiArenaHandback::ReturnClaim => {
                if !slot.occupied || !slot.retiring || !slot.retirement_claimed { Err("returned document claim is not retained") } else {
                    slot.retirement_claimed = false;
                    Ok(UiValueRetirementStep { progressed: true, ..Default::default() })
                }
            }
        };
        if result.is_err() { DOCUMENT_HANDBACKS.record(index, obligation); }
        result
    }

    fn retire_exact(&mut self, handle: UiDocumentHandle, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let Some(slot) = self.slot_mut(handle) else { return Ok(UiValueRetirementStep { complete: true, ..Default::default() }) };
        if !slot.retiring || slot.aliases != 0 { return Ok(UiValueRetirementStep::default()); }
        let mut step = match slot.retire_scalar {
            0 => slot.retirement.advance(&mut slot.nodes.entries, 1, maximum_bytes)?,
            1 => slot.retirement.advance(&mut slot.surface, 1, maximum_bytes)?,
            2 => { slot.root = None; UiValueRetirementStep { complete: true, progressed: true, ..Default::default() } }
            3 => { slot.revision = UiRevision(0); UiValueRetirementStep { complete: true, progressed: true, ..Default::default() } }
            4 => { slot.layout_epoch = 0; UiValueRetirementStep { complete: true, progressed: true, ..Default::default() } }
            5 => {
                if !slot.nodes.entries.terminal_is_empty() || slot.surface.is_some() { return Err("document credit still retains typed roots"); }
                let resident = slot.resident.as_mut().ok_or("document root lost its resident permit")?;
                let result = resident.close_step(1).map_err(|error| error.reason())?;
                if result.complete { slot.resident = None; }
                UiValueRetirementStep { complete: result.complete, progressed: result.progressed, released_items: result.released_permits, released_bytes: 0 }
            }
            6 => {
                if !slot.nodes.entries.terminal_is_empty() || slot.surface.is_some() || slot.resident.is_some() { return Err("document terminal retains typed roots or credit"); }
                let epoch = slot.epoch;
                *slot = UiDocumentSlot { epoch, ..Default::default() };
                return Ok(UiValueRetirementStep { complete: true, progressed: true, released_items: 1, released_bytes: 0 });
            }
            _ => return Err("document retirement phase is invalid"),
        };
        if step.complete {
            slot.retire_scalar += 1;
            slot.retirement = UiTypedRetirementCursor::default();
        }
        step.complete = false;
        Ok(step)
    }
}

pub(super) fn close_document_owner(handle: &mut Option<UiDocumentHandle>, released: &mut bool, claimed: &mut bool, maximum_items: usize, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
    let Some(exact) = *handle else { return Ok(UiValueRetirementStep { complete: true, ..Default::default() }) };
    if maximum_items == 0 || maximum_bytes == 0 { return Ok(UiValueRetirementStep::default()); }
    let mut arena = match UI_DOCUMENT_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(UiValueRetirementStep::default()),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("document retirement arena is poisoned"),
    };
    if !arena.active(exact) { *handle = None; *claimed = false; return Ok(UiValueRetirementStep { complete: true, ..Default::default() }); }
    if DOCUMENT_HANDBACKS.has_slot_pending(exact.slot) { return arena.consume_handback(exact.slot); }
    if !*released {
        arena.release(exact);
        *released = true;
        let slot = arena.slot_mut(exact).unwrap();
        if slot.retiring && !slot.retirement_claimed { slot.retirement_claimed = true; *claimed = true; }
        return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() });
    }
    if !*claimed {
        let slot = arena.slot_mut(exact).unwrap();
        if !slot.retiring || slot.retirement_claimed { return Ok(UiValueRetirementStep::default()); }
        slot.retirement_claimed = true;
        *claimed = true;
        return Ok(UiValueRetirementStep { progressed: true, ..Default::default() });
    }
    if !arena.slot(exact).unwrap().retirement_claimed { return Err("document retirement lost its exact claim"); }
    let step = arena.retire_exact(exact, maximum_bytes)?;
    if step.complete { *handle = None; *claimed = false; }
    Ok(step)
}

pub(super) fn close_document_read_owner(handle: &mut Option<UiDocumentHandle>, released: &mut bool, claimed: &mut bool, maximum_items: usize, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
    let Some(exact) = *handle else { return Ok(UiValueRetirementStep { complete: true, ..Default::default() }); };
    if maximum_items == 0 || maximum_bytes == 0 { return Ok(Default::default()); }
    if *released { return close_document_owner(handle, released, claimed, maximum_items, maximum_bytes); }
    let mut arena = match UI_DOCUMENT_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(Default::default()),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("document read retirement arena is poisoned"),
    };
    if !arena.active(exact) { return Err("document read lost its retained root"); }
    if DOCUMENT_HANDBACKS.has_slot_pending(exact.slot) { return arena.consume_handback(exact.slot); }
    let slot = arena.slot(exact).unwrap();
    if slot.retiring || slot.aliases == 0 { return Err("document read is not an exact live alias"); }
    arena.release(exact);
    *released = true;
    let slot = arena.slot_mut(exact).unwrap();
    if !slot.retiring {
        *handle = None;
        return Ok(UiValueRetirementStep { complete: true, progressed: true, released_items: 1, ..Default::default() });
    }
    if slot.retirement_claimed { return Err("final document read encountered a foreign claim"); }
    slot.retirement_claimed = true;
    *claimed = true;
    Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() })
}

pub(super) fn hand_back_document_owner(handle: Option<UiDocumentHandle>, released: bool, claimed: bool) {
    let Some(handle) = handle else { return };
    if !released { DOCUMENT_HANDBACKS.record(handle.slot, UiArenaHandback::ReleaseAlias); }
    if claimed { DOCUMENT_HANDBACKS.record(handle.slot, UiArenaHandback::ReturnClaim); }
}

/// 🪶️ Advances one unclaimed fixed document slot; contention never waits or drops a typed owner.
pub fn close_ui_document_page_with_grant(maximum_items: usize, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
    if maximum_items == 0 || maximum_bytes == 0 { return Ok(UiValueRetirementStep::default()); }
    let mut arena = match UI_DOCUMENT_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(UiValueRetirementStep::default()),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("document retirement arena is poisoned"),
    };
    if let Some(index) = DOCUMENT_HANDBACKS.next_slot(arena.close_cursor) {
        arena.close_cursor = (index + 1) % UI_DOCUMENT_LEASE_SLOTS;
        return arena.consume_handback(index);
    }
    let index = arena.close_cursor;
    arena.close_cursor = (index + 1) % UI_DOCUMENT_LEASE_SLOTS;
    let slot = &arena.slots[index];
    let mut step = if slot.occupied && slot.retiring && !slot.retirement_claimed {
        let exact = UiDocumentHandle { slot: index, epoch: slot.epoch, generation: slot.generation };
        arena.retire_exact(exact, maximum_bytes)?
    } else { UiValueRetirementStep { progressed: true, ..Default::default() } };
    step.complete = !arena.has_retirement() && !DOCUMENT_HANDBACKS.has_pending();
    Ok(step)
}
//#endregion 🧵️SlotTraversal

#[cfg(test)]
#[path = "🧪️document.rs"]
mod tests;
