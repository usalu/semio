//! ♻️ Exact fixed-arena value ownership, byte retirement and descendant completion.

use super::*;

#[path = "🌳️typed/🧱️component.rs"]
mod typed;
pub(crate) use typed::{UiTypedRetire, UiTypedRetirementCursor};
#[path = "📮️handback/🦀️.rs"]
mod handback;
pub(crate) use handback::{UiArenaHandback, UiArenaHandbacks};

#[path = "🌲️built/🦀️.rs"]
mod built;
pub use built::BuiltTreeRetirement;

static UI_VALUE_HANDBACKS: UiArenaHandbacks<UI_VALUE_ADMISSION_SLOTS, 4> = UiArenaHandbacks::new();

pub(super) fn hand_back_value(handle: UiCollectionHandle) { UI_VALUE_HANDBACKS.record(handle.slot, UiArenaHandback::ReleaseAlias); }

//#region 🪪️ExactValueOwner
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiValueRetirementStep {
    pub complete: bool,
    pub progressed: bool,
    pub released_items: usize,
    pub released_bytes: usize,
}

impl UiValueRetirementStep {
    fn progress(released_items: usize, released_bytes: usize) -> Self {
        Self { progressed: true, released_items, released_bytes, ..Self::default() }
    }
}

/// 🧵️ Owns one exact value and its final-alias descendants without a growing traversal stack.
pub struct UiValueRetirement {
    value: std::mem::ManuallyDrop<UiValue>,
    root: Option<UiCollectionHandle>,
    started: bool,
}

impl UiValueRetirement {
    /// 📥️ Moves the admitted fixed-size value; construction does not allocate, traverse or release.
    pub fn new(value: UiValue) -> Self {
        Self { value: std::mem::ManuallyDrop::new(value), root: None, started: false }
    }

    /// 🪶️ Advances one exact metadata item or a byte-bounded key/text prefix.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.terminal_is_empty() { return Ok(UiValueRetirementStep { complete: true, ..UiValueRetirementStep::default() }); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(UiValueRetirementStep::default()); }
        if let Some(root) = self.root {
            let mut arena = match UI_VALUE_ARENA.try_lock() {
                Ok(arena) => arena,
                Err(std::sync::TryLockError::WouldBlock) => return Ok(UiValueRetirementStep::default()),
                Err(std::sync::TryLockError::Poisoned(_)) => return Err("UI value retirement arena is poisoned"),
            };
            let mut step = arena.advance_exact_root(root, maximum_bytes, &UI_VALUE_HANDBACKS)?;
            let active = arena.collection(root).is_some();
            if !active { self.root = None; }
            step.complete = self.terminal_is_empty();
            return Ok(step);
        }
        let handle = match &*self.value { UiValue::List(value) => value.handle, UiValue::Map(value) => value.handle, _ => None };
        if let Some(handle) = handle {
            self.root = {
                let mut arena = match UI_VALUE_ARENA.try_lock() {
                    Ok(arena) => arena,
                    Err(std::sync::TryLockError::WouldBlock) => return Ok(UiValueRetirementStep::default()),
                    Err(std::sync::TryLockError::Poisoned(_)) => return Err("UI value retirement arena is poisoned"),
                };
                if UI_VALUE_HANDBACKS.has_slot_pending(handle.slot) { return arena.consume_handback(handle.slot, &UI_VALUE_HANDBACKS); }
                let root = arena.release_exact_handle(handle)?;
                if let Some(root) = root {
                    arena.unlink_retiring_root(root.slot);
                    arena.collections[root.slot].retirement_claimed = true;
                }
                root
            };
            match &mut *self.value {
                UiValue::List(value) => { value.handle = None; value.len = 0; }
                UiValue::Map(value) => { value.handle = None; value.len = 0; }
                _ => unreachable!(),
            }
            *self.value = UiValue::Null;
            self.started = true;
            return Ok(UiValueRetirementStep { complete: self.root.is_none(), ..UiValueRetirementStep::progress(usize::from(self.root.is_none()), 0) });
        }
        let bytes = if let UiValue::Text(text) = &mut *self.value {
            let bytes = text.len().min(maximum_bytes);
            text.len -= bytes as u16;
            bytes
        } else { 0 };
        if matches!(&*self.value, UiValue::Text(text) if !text.is_empty()) { return Ok(UiValueRetirementStep::progress(0, bytes)); }
        *self.value = UiValue::Null;
        self.started = true;
        Ok(UiValueRetirementStep { complete: true, ..UiValueRetirementStep::progress(1, bytes) })
    }

    /// 🧾️ Requires the exact value and final-descendant root to be gone, not a global empty queue.
    pub fn terminal_is_empty(&self) -> bool {
        self.started && self.root.is_none() && matches!(&*self.value, UiValue::Null)
    }
}

impl Drop for UiValueRetirement {
    fn drop(&mut self) {
        if self.terminal_is_empty() { return; }
        if let Some(root) = self.root.take() {
            UI_VALUE_HANDBACKS.record(root.slot, UiArenaHandback::ReturnClaim);
        }
        let handle = match &*self.value { UiValue::List(value) => value.handle, UiValue::Map(value) => value.handle, _ => None };
        if let Some(handle) = handle {
            hand_back_value(handle);
            match &mut *self.value {
                UiValue::List(value) => { value.handle = None; value.len = 0; }
                UiValue::Map(value) => { value.handle = None; value.len = 0; }
                _ => unreachable!(),
            }
            *self.value = UiValue::Null;
        }
        panic!("UiValueRetirement requires exact terminal closure");
    }
}
//#endregion 🪪️ExactValueOwner

//#region 🧵️ArenaTraversal
impl UiValueArena {
    pub(super) fn release_exact_handle(&mut self, handle: UiCollectionHandle) -> Result<Option<UiCollectionHandle>, &'static str> {
        let collection = self.collection_mut(handle).ok_or("stale UI value owner")?;
        if collection.retiring || collection.aliases == 0 { return Err("UI value owner already released"); }
        if collection.aliases > 1 { collection.aliases -= 1; return Ok(None); }
        collection.aliases = 0;
        collection.retiring = true;
        collection.retirement_cursor = Some(handle);
        self.link_retiring_root(handle.slot);
        Ok(Some(handle))
    }

    fn link_retiring_root(&mut self, slot: usize) {
        assert!(!self.collections[slot].retirement_queued);
        let tail = self.retirement_tail;
        self.collections[slot].retirement_previous = tail;
        self.collections[slot].retirement_next = UI_VALUE_NONE;
        self.collections[slot].retirement_queued = true;
        if tail == UI_VALUE_NONE { self.retirement_head = slot; } else { self.collections[tail].retirement_next = slot; }
        self.retirement_tail = slot;
        self.retirement_len += 1;
    }

    pub(super) fn unlink_retiring_root(&mut self, slot: usize) {
        if !self.collections[slot].retirement_queued { return; }
        let previous = self.collections[slot].retirement_previous;
        let next = self.collections[slot].retirement_next;
        if previous == UI_VALUE_NONE { self.retirement_head = next; } else { self.collections[previous].retirement_next = next; }
        if next == UI_VALUE_NONE { self.retirement_tail = previous; } else { self.collections[next].retirement_previous = previous; }
        self.collections[slot].retirement_previous = UI_VALUE_NONE;
        self.collections[slot].retirement_next = UI_VALUE_NONE;
        self.collections[slot].retirement_queued = false;
        self.retirement_len -= 1;
    }

    fn advance_exact_root(&mut self, root: UiCollectionHandle, maximum_bytes: usize, handbacks: &UiArenaHandbacks<UI_VALUE_ADMISSION_SLOTS, 4>) -> Result<UiValueRetirementStep, &'static str> {
        let Some(collection) = self.collection(root) else { return Ok(UiValueRetirementStep { complete: true, ..UiValueRetirementStep::default() }) };
        if !collection.retiring || !(collection.retirement_queued || collection.retirement_claimed) { return Err("UI value root has no final-owner authority"); }
        let cursor = collection.retirement_cursor.ok_or("UI value root lost its retained cursor")?;
        let current = self.collection(cursor).ok_or("UI value descendant cursor is stale")?;
        let head = current.head;
        if head == UI_VALUE_NONE {
            let parent = current.retirement_parent;
            if !self.release_collection(cursor.slot) { return Err("UI value descendant release invariant failed"); }
            if let Some(parent) = parent { self.collection_mut(root).ok_or("UI value root retired before its descendant")?.retirement_cursor = Some(parent); }
            return Ok(UiValueRetirementStep::progress(1, 0));
        }
        let page = self.pages[head].value.as_mut().ok_or("UI value page is empty before release")?;
        let value = match page {
            UiPageValue::List(value) => value,
            UiPageValue::Map(key, value) => {
                if !key.is_empty() {
                    let bytes = key.len().min(maximum_bytes);
                    key.len -= bytes as u16;
                    return Ok(UiValueRetirementStep::progress(0, bytes));
                }
                value
            }
        };
        if let UiValue::Text(text) = value {
            if !text.is_empty() {
                let bytes = text.len().min(maximum_bytes);
                text.len -= bytes as u16;
                return Ok(UiValueRetirementStep::progress(0, bytes));
            }
        }
        let nested = match value { UiValue::List(value) => value.handle, UiValue::Map(value) => value.handle, _ => None };
        if let Some(nested) = nested {
            if handbacks.has_slot_pending(nested.slot) { return self.consume_handback(nested.slot, handbacks); }
            let child = self.collection_mut(nested).ok_or("nested UI value owner is stale")?;
            if child.retiring || child.aliases == 0 { return Err("nested UI value owner already released"); }
            child.aliases -= 1;
            let final_alias = child.aliases == 0;
            if final_alias {
                child.retiring = true;
                child.retirement_parent = Some(cursor);
                self.collections[root.slot].retirement_cursor = Some(nested);
            }
            let value = match self.pages[head].value.as_mut().unwrap() { UiPageValue::List(value) | UiPageValue::Map(_, value) => value };
            match value {
                UiValue::List(value) => { value.handle = None; value.len = 0; }
                UiValue::Map(value) => { value.handle = None; value.len = 0; }
                _ => unreachable!(),
            }
            return Ok(UiValueRetirementStep::progress(usize::from(!final_alias), 0));
        }
        let next = self.pages[head].next;
        let value = self.pages[head].value.take();
        self.pages[head].next = UI_VALUE_NONE;
        self.free_pages[self.free_page_count] = head;
        self.free_page_count += 1;
        self.collections[cursor.slot].head = next;
        if next == UI_VALUE_NONE { self.collections[cursor.slot].tail = UI_VALUE_NONE; }
        drop(value);
        Ok(UiValueRetirementStep::progress(1, 0))
    }

    fn consume_handback(&mut self, slot: usize, handbacks: &UiArenaHandbacks<UI_VALUE_ADMISSION_SLOTS, 4>) -> Result<UiValueRetirementStep, &'static str> {
        let Some(obligation) = handbacks.take_one(slot) else { return Ok(UiValueRetirementStep::progress(0, 0)) };
        let result = match obligation {
            UiArenaHandback::ReleaseAlias => {
                let collection = &self.collections[slot];
                let handle = UiCollectionHandle { slot, epoch: collection.epoch, kind: collection.kind };
                self.release_exact_handle(handle).map(|_| UiValueRetirementStep::progress(1, 0))
            }
            UiArenaHandback::ReturnClaim => {
                let collection = &mut self.collections[slot];
                if !collection.occupied || !collection.retirement_claimed || !collection.retiring { Err("returned value claim is not retained") } else {
                    collection.retirement_claimed = false;
                    self.link_retiring_root(slot);
                    Ok(UiValueRetirementStep::progress(0, 0))
                }
            }
        };
        if result.is_err() { handbacks.record(slot, obligation); }
        result
    }

    pub(super) fn advance_retirement_queue(&mut self, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if let Some(slot) = UI_VALUE_HANDBACKS.next_slot(self.handback_cursor) {
            self.handback_cursor = (slot + 1) % UI_VALUE_ADMISSION_SLOTS;
            return self.consume_handback(slot, &UI_VALUE_HANDBACKS);
        }
        let slot = self.retirement_head;
        if slot == UI_VALUE_NONE { return Ok(UiValueRetirementStep { complete: true, ..UiValueRetirementStep::default() }); }
        let collection = &self.collections[slot];
        let root = UiCollectionHandle { slot, epoch: collection.epoch, kind: collection.kind };
        let mut step = self.advance_exact_root(root, maximum_bytes, &UI_VALUE_HANDBACKS)?;
        if self.collection(root).is_some() { self.unlink_retiring_root(slot); self.link_retiring_root(slot); }
        step.complete = self.retirement_len == 0 && !UI_VALUE_HANDBACKS.has_pending();
        Ok(step)
    }
}
//#endregion 🧵️ArenaTraversal

/// 🪶️ Advances exact queued value ownership without waiting for the arena or consuming a zero grant.
pub fn close_ui_value_page_with_grant(maximum_items: usize, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
    if maximum_items == 0 || maximum_bytes == 0 { return Ok(UiValueRetirementStep::default()); }
    let mut arena = match UI_VALUE_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(UiValueRetirementStep::default()),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("queued UI value retirement arena is poisoned"),
    };
    arena.advance_retirement_queue(maximum_bytes)
}

//#region 🧪️ExactOwnerLaws
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "📋️list/🧪️tests/🦀️.rs"]
mod fixed_list_tests;
#[cfg(test)]
#[path = "🌳️typed/🧪️tests/🦀️.rs"]
mod typed_tests;
//#endregion 🧪️ExactOwnerLaws
