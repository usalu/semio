//! 🏟️ Collection arena: the collection kinds and handles, the page and collection slots, the process-wide value arena and its headroom. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiCollectionKind {
    List,
    Map,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiCollectionHandle {
    pub slot: usize,
    pub epoch: u64,
    pub(super) kind: UiCollectionKind,
}

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "Arena page slots include the bounded map key inline so moving a page needs no separate key allocation.")]
pub enum UiPageValue {
    List(UiValue),
    Map(UiText, UiValue),
}

#[derive(Debug)]
pub struct UiPageSlot {
    pub(super) epoch: u64,
    pub next: usize,
    pub value: Option<UiPageValue>,
}

impl Default for UiPageSlot {
    fn default() -> Self {
        Self { epoch: 0, next: UI_VALUE_NONE, value: None }
    }
}

#[derive(Debug)]
pub struct UiCollectionSlot {
    pub(super) epoch: u64,
    pub(super) kind: UiCollectionKind,
    pub head: usize,
    pub(super) tail: usize,
    pub aliases: u64,
    pub(super) items: usize,
    pub(super) bytes: usize,
    pub(super) occupied: bool,
    pub retiring: bool,
    pub(super) retirement_parent: Option<UiCollectionHandle>,
    pub(super) retirement_cursor: Option<UiCollectionHandle>,
    pub(super) retirement_previous: usize,
    pub(super) retirement_next: usize,
    pub retirement_queued: bool,
    pub retirement_claimed: bool,
}

pub const UI_VALUE_AGGREGATE_BYTES: usize = UI_VALUE_AGGREGATE_ITEMS * size_of::<UiPageSlot>() + UI_VALUE_ADMISSION_SLOTS * size_of::<UiCollectionSlot>();

impl Default for UiCollectionSlot {
    fn default() -> Self {
        Self {
            epoch: 0,
            kind: UiCollectionKind::List,
            head: UI_VALUE_NONE,
            tail: UI_VALUE_NONE,
            aliases: 0,
            items: 0,
            bytes: 0,
            occupied: false,
            retiring: false,
            retirement_parent: None,
            retirement_cursor: None,
            retirement_previous: UI_VALUE_NONE,
            retirement_next: UI_VALUE_NONE,
            retirement_queued: false,
            retirement_claimed: false,
        }
    }
}

pub struct UiValueArena {
    pub pages: Box<[UiPageSlot]>,
    pub(super) collections: Box<[UiCollectionSlot]>,
    pub(super) free_pages: Box<[usize]>,
    pub free_page_count: usize,
    free_collections: Box<[usize]>,
    pub free_collection_count: usize,
    pub(super) retirement_head: usize,
    pub(super) retirement_tail: usize,
    pub(super) retirement_len: usize,
    pub(super) handback_cursor: usize,
    pub(super) items: usize,
    pub(super) bytes: usize,
}

impl Default for UiValueArena {
    fn default() -> Self {
        let mut pages = Vec::with_capacity(UI_VALUE_AGGREGATE_ITEMS);
        pages.resize_with(UI_VALUE_AGGREGATE_ITEMS, UiPageSlot::default);
        let mut collections = Vec::with_capacity(UI_VALUE_ADMISSION_SLOTS);
        collections.resize_with(UI_VALUE_ADMISSION_SLOTS, UiCollectionSlot::default);
        Self {
            pages: pages.into_boxed_slice(),
            collections: collections.into_boxed_slice(),
            free_pages: (0..UI_VALUE_AGGREGATE_ITEMS).rev().collect::<Vec<_>>().into_boxed_slice(),
            free_page_count: UI_VALUE_AGGREGATE_ITEMS,
            free_collections: (0..UI_VALUE_ADMISSION_SLOTS).rev().collect::<Vec<_>>().into_boxed_slice(),
            free_collection_count: UI_VALUE_ADMISSION_SLOTS,
            retirement_head: UI_VALUE_NONE,
            retirement_tail: UI_VALUE_NONE,
            retirement_len: 0,
            handback_cursor: 0,
            items: 0,
            bytes: 0,
        }
    }
}

pub static UI_VALUE_ARENA: LazyLock<Mutex<UiValueArena>> = LazyLock::new(|| Mutex::new(UiValueArena::default()));
static UI_VALUE_ARENA_PUBLISHED:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
pub(super) fn initialized_arena()->Result<Option<std::sync::MutexGuard<'static,UiValueArena>>,&'static str>{
    if !UI_VALUE_ARENA_PUBLISHED.load(std::sync::atomic::Ordering::Acquire){return Err("UI value arena source is not initialized");}
    match UI_VALUE_ARENA.try_lock(){Ok(arena)=>Ok(Some(arena)),Err(std::sync::TryLockError::WouldBlock)=>Ok(None),Err(std::sync::TryLockError::Poisoned(_))=>Err("UI value arena source is poisoned")}
}

pub const fn resident_static_backing_bytes() -> usize {
    size_of::<LazyLock<Mutex<UiValueArena>>>()
        + UI_VALUE_AGGREGATE_ITEMS * (size_of::<UiPageSlot>() + size_of::<usize>())
        + UI_VALUE_ADMISSION_SLOTS * (size_of::<UiCollectionSlot>() + size_of::<usize>())
        + size_of::<UiArenaHandbacks<UI_VALUE_ADMISSION_SLOTS, UI_VALUE_HANDBACK_WORDS>>()
}

/// ☣️ Runs `f` on the process-global value arena. A poisoned lock is recovered AND cleared: the arena's fixed slots keep their
/// authority through a panic, so every later `try_lock` (retirement, copy, compare) admits the arena again instead of refusing it
/// as poisoned forever.
pub fn with_ui_value_arena<T>(f: impl FnOnce(&mut UiValueArena) -> T) -> T {
    let source=&*UI_VALUE_ARENA;
    UI_VALUE_ARENA_PUBLISHED.store(true,std::sync::atomic::Ordering::Release);
    let mut arena = source.lock().unwrap_or_else(|poisoned| {
        UI_VALUE_ARENA.clear_poison();
        poisoned.into_inner()
    });
    f(&mut arena)
}

impl UiValueArena {
    pub fn reserve_collection(&mut self, kind: UiCollectionKind) -> Option<UiCollectionHandle> {
        let bytes = size_of::<UiCollectionSlot>();
        let next_bytes = self.bytes.checked_add(bytes)?;
        if self.free_collection_count == 0 || next_bytes > UI_VALUE_AGGREGATE_BYTES {
            return None;
        }
        let next_epoch = self.collections[self.free_collections[self.free_collection_count - 1]].epoch.checked_add(1)?;
        let free_collection_count = self.free_collection_count.checked_sub(1)?;
        let slot = self.free_collections[free_collection_count];
        self.collections[slot] = UiCollectionSlot { epoch: next_epoch, kind, head: UI_VALUE_NONE, tail: UI_VALUE_NONE, aliases: 1, items: 0, bytes, occupied: true, retiring: false, ..UiCollectionSlot::default() };
        self.free_collection_count = free_collection_count;
        self.bytes = next_bytes;
        Some(UiCollectionHandle { slot, epoch: next_epoch, kind })
    }

    pub(super) fn collection_mut(&mut self, handle: UiCollectionHandle) -> Option<&mut UiCollectionSlot> {
        let slot = self.collections.get_mut(handle.slot)?;
        (slot.occupied && slot.epoch == handle.epoch && slot.kind == handle.kind).then_some(slot)
    }

    pub fn collection(&self, handle: UiCollectionHandle) -> Option<&UiCollectionSlot> {
        let slot = self.collections.get(handle.slot)?;
        (slot.occupied && slot.epoch == handle.epoch && slot.kind == handle.kind).then_some(slot)
    }

    #[expect(clippy::result_large_err, reason = "Rejected arena admission returns the exact page and its retained value credits without allocating.")]
    pub fn try_push_page(&mut self, handle: UiCollectionHandle, value: UiPageValue) -> Result<(), UiPageValue> {
        let bytes = size_of::<UiPageSlot>();
        let Some(free_page_count) = self.free_page_count.checked_sub(1) else { return Err(value) };
        let page = self.free_pages[free_page_count];
        let Some(epoch) = self.pages[page].epoch.checked_add(1) else { return Err(value) };
        let Some(next_items) = self.items.checked_add(1).filter(|items| *items <= UI_VALUE_AGGREGATE_ITEMS) else { return Err(value) };
        let Some(next_bytes) = self.bytes.checked_add(bytes).filter(|bytes| *bytes <= UI_VALUE_AGGREGATE_BYTES) else { return Err(value) };
        let Some(collection) = self.collection(handle) else { return Err(value) };
        if collection.retiring {
            return Err(value);
        }
        let Some(collection_items) = collection.items.checked_add(1) else { return Err(value) };
        let Some(collection_bytes) = collection.bytes.checked_add(bytes) else { return Err(value) };
        let tail = collection.tail;
        self.free_page_count = free_page_count;
        self.pages[page] = UiPageSlot { epoch, next: UI_VALUE_NONE, value: Some(value) };
        if tail == UI_VALUE_NONE {
            self.collections[handle.slot].head = page;
        } else {
            self.pages[tail].next = page;
        }
        self.collections[handle.slot].tail = page;
        self.collections[handle.slot].items = collection_items;
        self.collections[handle.slot].bytes = collection_bytes;
        self.items = next_items;
        self.bytes = next_bytes;
        Ok(())
    }

    pub(super) fn search_map_key(&self, handle: UiCollectionHandle, key: &UiText) -> Result<usize, usize> {
        let Some(collection) = self.collection(handle) else { return Err(0) };
        let mut page = collection.head;
        let mut position = 0;
        while page != UI_VALUE_NONE {
            let Some(slot) = self.pages.get(page) else { return Err(position) };
            let Some(UiPageValue::Map(candidate, _)) = slot.value.as_ref() else { return Err(position) };
            match candidate.cmp(key) {
                std::cmp::Ordering::Equal => return Ok(page),
                std::cmp::Ordering::Greater => return Err(position),
                std::cmp::Ordering::Less => {
                    position += 1;
                    page = slot.next;
                }
            }
        }
        Err(position)
    }

    pub(super) fn map_prev_at_position(&self, handle: UiCollectionHandle, position: usize) -> Option<usize> {
        let Some(collection) = self.collection(handle) else { return None };
        if position == 0 {
            return Some(UI_VALUE_NONE);
        }
        let mut page = collection.head;
        for _ in 0..position - 1 {
            if page == UI_VALUE_NONE {
                return None;
            }
            page = self.pages[page].next;
        }
        Some(page)
    }

    #[expect(clippy::result_large_err, reason = "Rejected arena admission returns the exact key and value credits without allocating.")]
    pub(super) fn try_insert_map_page(&mut self, handle: UiCollectionHandle, prev: usize, key: UiText, value: UiValue) -> Result<(), (UiText, UiValue)> {
        let bytes = size_of::<UiPageSlot>();
        let Some(free_page_count) = self.free_page_count.checked_sub(1) else { return Err((key, value)) };
        let page = self.free_pages[free_page_count];
        let Some(epoch) = self.pages[page].epoch.checked_add(1) else { return Err((key, value)) };
        let Some(next_items) = self.items.checked_add(1).filter(|items| *items <= UI_VALUE_AGGREGATE_ITEMS) else { return Err((key, value)) };
        let Some(next_bytes) = self.bytes.checked_add(bytes).filter(|bytes| *bytes <= UI_VALUE_AGGREGATE_BYTES) else { return Err((key, value)) };
        let Some(collection) = self.collection_mut(handle) else { return Err((key, value)) };
        if collection.retiring {
            return Err((key, value));
        }
        let Some(collection_items) = collection.items.checked_add(1) else { return Err((key, value)) };
        let Some(collection_bytes) = collection.bytes.checked_add(bytes) else { return Err((key, value)) };
        let collection_head = collection.head;
        self.free_page_count = free_page_count;
        let next = if prev == UI_VALUE_NONE {
            collection_head
        } else {
            self.pages.get(prev).map(|slot| slot.next).unwrap_or(UI_VALUE_NONE)
        };
        self.pages[page] = UiPageSlot { epoch, next, value: Some(UiPageValue::Map(key, value)) };
        if prev == UI_VALUE_NONE {
            self.collections[handle.slot].head = page;
            if self.collections[handle.slot].tail == UI_VALUE_NONE {
                self.collections[handle.slot].tail = page;
            }
        } else {
            self.pages[prev].next = page;
            if self.collections[handle.slot].tail == prev {
                self.collections[handle.slot].tail = page;
            }
        }
        self.collections[handle.slot].items = collection_items;
        self.collections[handle.slot].bytes = collection_bytes;
        self.items = next_items;
        self.bytes = next_bytes;
        Ok(())
    }

    pub(super) fn try_clone_handle(&mut self, handle: UiCollectionHandle) -> Option<UiCollectionHandle> {
        let collection = self.collection_mut(handle)?;
        if collection.retiring {
            return None;
        }
        collection.aliases = collection.aliases.checked_add(1)?;
        Some(handle)
    }

    pub(super) fn release_collection(&mut self, slot: usize) -> bool {
        let (collection_items, collection_bytes, epoch) = {
            let Some(collection) = self.collections.get(slot) else { return false };
            if !collection.occupied || collection.head != UI_VALUE_NONE || collection.aliases != 0 {
                return false;
            }
            (collection.items, collection.bytes, collection.epoch)
        };
        let Some(items) = self.items.checked_sub(collection_items) else { return false };
        let Some(bytes) = self.bytes.checked_sub(collection_bytes) else { return false };
        let Some(free_collection_count) = self.free_collection_count.checked_add(1) else { return false };
        if free_collection_count > UI_VALUE_ADMISSION_SLOTS {
            return false;
        }
        self.items = items;
        self.bytes = bytes;
        self.unlink_retiring_root(slot);
        self.collections[slot] = UiCollectionSlot { epoch, ..UiCollectionSlot::default() };
        self.free_collections[self.free_collection_count] = slot;
        self.free_collection_count = free_collection_count;
        true
    }

    pub(super) fn try_clone_value(&mut self, value: &UiValue) -> Option<UiValue> {
        Some(match value {
            UiValue::Null => UiValue::Null,
            UiValue::Bool(value) => UiValue::Bool(*value),
            UiValue::Number(value) => UiValue::Number(*value),
            UiValue::Text(value) => UiValue::Text(value.clone()),
            UiValue::List(value) => UiValue::List(value.try_clone_in(self)?),
            UiValue::Map(value) => UiValue::Map(value.try_clone_in(self)?),
        })
    }

    pub(super) fn page_value(&mut self, handle: UiCollectionHandle, page: usize) -> Option<(usize, UiPageValue)> {
        let collection = self.collection(handle)?;
        if collection.retiring || page == UI_VALUE_NONE {
            return None;
        }
        let next = self.pages.get(page)?.next;
        let value = self.pages.get_mut(page)?.value.take()?;
        let cloned = match &value {
            UiPageValue::List(value) => self.try_clone_value(value).map(UiPageValue::List),
            UiPageValue::Map(key, value) => self.try_clone_value(value).map(|value| UiPageValue::Map(key.clone(), value)),
        };
        self.pages[page].value = Some(value);
        Some((next, cloned?))
    }
}

pub fn close_ui_value_page_one() -> bool {
    close_ui_value_page_with_grant(1, 4096).expect("exact UI value retirement queue remains valid").complete
}

/// 🎟️ Free admission credit in the process-wide `UiValue` arena, read without admitting anything. The
/// arena is sized for [`UI_VALUE_LIVE_PAGES`] panel pages, so an author walking a document larger than
/// [`UI_VALUE_PAGE_ROWS`] reads this before materialising one more interactive row and stops with a
/// continuation row rather than failing an admission mid-build.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiValueHeadroom {
    pub collections: usize,
    pub items: usize,
}

impl UiValueHeadroom {
    /// 🧾️ Interactive rows this headroom still admits, each costing [`UI_VALUE_ROW_COLLECTIONS`]
    /// collections and [`UI_VALUE_ROW_ITEMS`] pages.
    pub fn rows(self) -> usize {
        (self.collections / UI_VALUE_ROW_COLLECTIONS).min(self.items / UI_VALUE_ROW_ITEMS)
    }
}

/// 📏️ Reads [`UiValueHeadroom`]; never admits, never retires.
pub fn ui_value_headroom() -> UiValueHeadroom {
    with_ui_value_arena(|arena| UiValueHeadroom { collections: arena.free_collection_count, items: arena.free_page_count })
}
