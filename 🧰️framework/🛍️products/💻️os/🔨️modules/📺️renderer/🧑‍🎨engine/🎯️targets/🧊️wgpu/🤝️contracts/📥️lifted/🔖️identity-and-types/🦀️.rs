//! 🔎️ The shell find items: the bounded, thread-bound list of palette find rows with its owner binding.
//!
//! Lifted verbatim out of the Shell (RENDERER-WP4, plan 5.2): visibility widened to `pub`, bodies unchanged. `contracts` mounts it with one `mod` line and the Shell re-exports it from `contracts`; the thread-bound binding cell exists once, here.


#[derive(Clone, Debug)]
pub struct ShellFindItem {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub surface_id: String,
    pub node_id: String,
}

pub const SHELL_FIND_ITEM_CAPACITY: usize = 256;

pub const SHELL_FIND_PAYLOAD_BYTES: usize = 1024 * 1024;

/// 🧺️ Fixed FIFO ownership for one generation of scene find items.
pub struct ShellFindItems {
    pub slots: Box<[Option<ShellFindItem>; SHELL_FIND_ITEM_CAPACITY]>,
    pub head: usize,
    pub len: usize,
    payload_bytes: usize,
    pub generation: u64,
}

impl Default for ShellFindItems {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; SHELL_FIND_ITEM_CAPACITY]), head: 0, len: 0, payload_bytes: 0, generation: 1 }
    }
}

impl ShellFindItems {
    fn item_bytes(item: &ShellFindItem) -> Option<usize> {
        item.id
            .capacity()
            .checked_add(item.label.capacity())?
            .checked_add(item.description.as_ref().map_or(0, String::capacity))?
            .checked_add(item.category.as_ref().map_or(0, String::capacity))?
            .checked_add(item.surface_id.capacity())?
            .checked_add(item.node_id.capacity())
    }

    pub fn try_push_at(&mut self, generation: u64, item: ShellFindItem) -> Result<(), ShellFindItem> {
        let Some(bytes) = Self::item_bytes(&item) else { return Err(item) };
        let Some(next_bytes) = self.payload_bytes.checked_add(bytes) else { return Err(item) };
        if generation != self.generation || self.len == self.slots.len() || next_bytes > SHELL_FIND_PAYLOAD_BYTES {
            return Err(item);
        }
        let Some(index) = self.head.checked_add(self.len).map(|index| index % self.slots.len()) else { return Err(item) };
        let Some(next_len) = self.len.checked_add(1) else { return Err(item) };
        if self.slots[index].is_some() {
            return Err(item);
        }
        self.slots[index] = Some(item);
        self.len = next_len;
        self.payload_bytes = next_bytes;
        Ok(())
    }

    pub fn try_push(&mut self, item: ShellFindItem) -> Result<(), ShellFindItem> {
        self.try_push_at(self.generation, item)
    }

    pub fn pop_front(&mut self) -> Option<ShellFindItem> {
        if self.len == 0 {
            return None;
        }
        let item = self.slots[self.head].take()?;
        let bytes = Self::item_bytes(&item)?;
        self.head = self.head.checked_add(1).map(|index| index % self.slots.len())?;
        self.len = self.len.checked_sub(1)?;
        self.payload_bytes = self.payload_bytes.checked_sub(bytes)?;
        Some(item)
    }

    pub fn begin_next_generation(&mut self) -> bool {
        if self.len != 0 {
            return false;
        }
        let Some(generation) = self.generation.checked_add(1) else { return false };
        self.head = 0;
        self.generation = generation;
        true
    }

    pub fn iter(&self) -> impl Iterator<Item = &ShellFindItem> {
        (0..self.len).filter_map(move |offset| self.head.checked_add(offset).map(|index| index % self.slots.len()).and_then(|index| self.slots[index].as_ref()))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.payload_bytes == 0 && self.slots.iter().all(Option::is_none)
    }

    pub fn close_step(&mut self) -> bool {
        if self.pop_front().is_some() {
            return false;
        }
        self.terminal_is_empty()
    }
}

#[derive(Clone, Copy)]
pub struct ActiveShellFindItems {
    pub pointer: std::ptr::NonNull<ShellFindItems>,
    pub generation: u64,
}

thread_local! {
    static ACTIVE_FIND_ITEMS: std::cell::Cell<Option<ActiveShellFindItems>> = const { std::cell::Cell::new(None) };
}

/// 🪢️ Non-nesting single-worker binding for the exact retained collector.
pub struct ShellFindItemBinding {
    pub active: ActiveShellFindItems,
}

impl ShellFindItems {
    pub fn bind(&mut self) -> Result<ShellFindItemBinding, ()> {
        let active = ActiveShellFindItems { pointer: std::ptr::NonNull::from(&mut *self), generation: self.generation };
        ACTIVE_FIND_ITEMS.with(|slot| {
            if slot.get().is_some() {
                return Err(());
            }
            slot.set(Some(active));
            Ok(ShellFindItemBinding { active })
        })
    }
}

impl Drop for ShellFindItemBinding {
    fn drop(&mut self) {
        ACTIVE_FIND_ITEMS.with(|slot| {
            if slot.get().is_some_and(|active| active.pointer == self.active.pointer && active.generation == self.active.generation) {
                slot.set(None);
            }
        });
    }
}

/// 📥️ Transfers one exact callback item or returns it unchanged on stale/full/unbound refusal.
pub fn try_push_find_item(item: ShellFindItem) -> Result<(), ShellFindItem> {
    ACTIVE_FIND_ITEMS.with(|slot| {
        let Some(active) = slot.get() else { return Err(item) };
        let collector = unsafe { active.pointer.as_ptr().as_mut() };
        let Some(collector) = collector else { return Err(item) };
        collector.try_push_at(active.generation, item)
    })
}
