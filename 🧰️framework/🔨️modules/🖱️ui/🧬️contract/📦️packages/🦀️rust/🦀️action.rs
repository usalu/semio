//! @emoji 🎬️ Versioned \`ActionId\`, \`Trigger\`, \`ActionBinding\`, \`UiIntent\` and neutral \`UiValue\`.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every \`fn\`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::mem::size_of;
use std::sync::{LazyLock, Mutex};

pub const UI_TEXT_MAX_BYTES: usize = 512;
pub const UI_FIXED_LIST_ITEMS: usize = 32;
pub const UI_FIXED_BYTES: usize = 32 * 1_024;
pub const UI_VALUE_PAGE_ITEMS: usize = 1;
pub const UI_VALUE_MAX_ITEMS: usize = 256;
pub const UI_VALUE_ADMISSION_SLOTS: usize = 256;
pub const UI_VALUE_AGGREGATE_ITEMS: usize = 256;
const UI_VALUE_NONE: usize = usize::MAX;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiText {
    bytes: [u8; UI_TEXT_MAX_BYTES],
    len: u16,
}

impl Default for UiText {
    fn default() -> Self {
        Self { bytes: [0; UI_TEXT_MAX_BYTES], len: 0 }
    }
}

impl fmt::Debug for UiText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(formatter)
    }
}

impl UiText {
    pub fn try_from_string(value: String) -> Result<Self, String> {
        if value.len() > UI_TEXT_MAX_BYTES {
            return Err(value);
        }
        let mut text = Self::default();
        text.bytes[..value.len()].copy_from_slice(value.as_bytes());
        text.len = value.len() as u16;
        Ok(text)
    }

    pub fn try_from_str(value: &str) -> Option<Self> {
        if value.len() > UI_TEXT_MAX_BYTES {
            return None;
        }
        let mut text = Self::default();
        text.bytes[..value.len()].copy_from_slice(value.as_bytes());
        text.len = value.len() as u16;
        Some(text)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).unwrap_or("")
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub const fn capacity(&self) -> usize {
        UI_TEXT_MAX_BYTES
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn try_format(arguments: fmt::Arguments<'_>) -> Option<Self> {
        use fmt::Write;
        let mut text = Self::default();
        text.write_fmt(arguments).ok()?;
        Some(text)
    }
}

impl fmt::Write for UiText {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        let start = self.len();
        let end = start.checked_add(value.len()).filter(|end| *end <= UI_TEXT_MAX_BYTES).ok_or(fmt::Error)?;
        self.bytes[start..end].copy_from_slice(value.as_bytes());
        self.len = u16::try_from(end).map_err(|_| fmt::Error)?;
        Ok(())
    }
}

impl fmt::Display for UiText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for UiText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::ops::Deref for UiText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl TryFrom<String> for UiText {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}

impl<'a> TryFrom<&'a str> for UiText {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from_str(value).ok_or(value)
    }
}

impl Serialize for UiText {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for UiText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::try_from_string(value).map_err(|value| serde::de::Error::custom(format!("UiText exceeds {UI_TEXT_MAX_BYTES} bytes with {}", value.len())))
    }
}

#[derive(Debug, PartialEq)]
pub struct UiFixedList<T, const N: usize = UI_FIXED_LIST_ITEMS> {
    items: Option<Box<[Option<T>]>>,
    len: usize,
}

impl<T: Eq, const N: usize> Eq for UiFixedList<T, N> {}

impl<T, const N: usize> Default for UiFixedList<T, N> {
    fn default() -> Self {
        Self { items: None, len: 0 }
    }
}

impl<T: Clone, const N: usize> Clone for UiFixedList<T, N> {
    fn clone(&self) -> Self {
        if self.items.is_none() {
            return Self::default();
        }
        let mut items = Vec::with_capacity(N);
        items.resize_with(N, || None);
        for (index, value) in self.iter().enumerate() {
            items[index] = Some(value.clone());
        }
        Self { items: Some(items.into_boxed_slice()), len: self.len }
    }
}

impl<T, const N: usize> UiFixedList<T, N> {
    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        let index = self.len;
        if index == N {
            return Err(value);
        }
        if self.items.is_none() {
            let mut items = Vec::with_capacity(N);
            items.resize_with(N, || None);
            self.items = Some(items.into_boxed_slice());
        }
        let Some(len) = self.len.checked_add(1) else { return Err(value) };
        let Some(items) = self.items.as_mut() else { return Err(value) };
        items[index] = Some(value);
        self.len = len;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.len.checked_sub(1)?;
        self.len = index;
        self.items.as_mut()?.get_mut(index)?.take()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        (index < self.len).then(|| self.items.as_ref()?.get(index)?.as_ref()).flatten()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.len).then(|| self.items.as_mut()?.get_mut(index)?.as_mut()).flatten()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.len.checked_sub(1).and_then(|index| self.items.as_mut()?.get_mut(index)?.as_mut())
    }

    pub fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        let last = self.len.checked_sub(1)?;
        self.len = last;
        let items = self.items.as_mut()?;
        let removed = items[index].take();
        if index != last {
            items[index] = items[last].take();
        }
        removed
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let len = self.len;
        self.items.as_deref_mut().map_or([].as_mut_slice(), |items| &mut items[..len]).iter_mut().filter_map(Option::as_mut)
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.items.as_deref().map_or([].as_slice(), |items| &items[..self.len]).iter().filter_map(Option::as_ref)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.items.as_ref().map_or(0, |items| items.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T, const N: usize> std::ops::Index<usize> for UiFixedList<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("fixed-list index must be within admitted length")
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for UiFixedList<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("fixed-list index must be within admitted length")
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a UiFixedList<T, N> {
    type Item = &'a T;
    type IntoIter = std::iter::FilterMap<std::slice::Iter<'a, Option<T>>, fn(&Option<T>) -> Option<&T>>;

    fn into_iter(self) -> Self::IntoIter {
        fn present<T>(value: &Option<T>) -> Option<&T> {
            value.as_ref()
        }
        self.items.as_deref().map_or([].as_slice(), |items| &items[..self.len]).iter().filter_map(present::<T>)
    }
}

impl<T, const N: usize> IntoIterator for UiFixedList<T, N> {
    type Item = T;
    type IntoIter = std::iter::FilterMap<std::vec::IntoIter<Option<T>>, fn(Option<T>) -> Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        fn present<T>(value: Option<T>) -> Option<T> {
            value
        }
        self.items.map_or_else(Vec::new, |items| items.into_vec()).into_iter().filter_map(present::<T>)
    }
}

impl<T: Serialize, const N: usize> Serialize for UiFixedList<T, N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len))?;
        for value in self {
            sequence.serialize_element(value)?;
        }
        sequence.end()
    }
}

impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for UiFixedList<T, N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{SeqAccess, Visitor};
        struct FixedListVisitor<T, const N: usize>(std::marker::PhantomData<T>);
        impl<'de, T: Deserialize<'de>, const N: usize> Visitor<'de> for FixedListVisitor<T, N> {
            type Value = UiFixedList<T, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a bounded fixed UI list")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut values = UiFixedList::default();
                while let Some(value) = access.next_element::<T>()? {
                    if values.try_push(value).is_err() {
                        return Err(serde::de::Error::custom(format!("UiFixedList exceeds {N} items")));
                    }
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(FixedListVisitor::<T, N>(std::marker::PhantomData))
    }
}

#[derive(Debug, PartialEq)]
pub struct UiFixedMap<V> {
    entries: UiFixedList<(UiText, V)>,
}

impl<V> Default for UiFixedMap<V> {
    fn default() -> Self {
        Self { entries: UiFixedList::default() }
    }
}

impl<V: Clone> Clone for UiFixedMap<V> {
    fn clone(&self) -> Self {
        Self { entries: self.entries.clone() }
    }
}

impl<V> UiFixedMap<V> {
    pub fn try_push(&mut self, key: UiText, value: V) -> Result<(), (UiText, V)> {
        if self.entries.len().checked_sub(1).and_then(|index| self.entries.get(index)).is_some_and(|(last, _)| last >= &key) {
            return Err((key, value));
        }
        self.entries.try_push((key, value)).map_err(|(key, value)| (key, value))
    }

    pub fn pop(&mut self) -> Option<(UiText, V)> {
        self.entries.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&UiText, &V)> {
        self.entries.iter().map(|(key, value)| (key, value))
    }

    pub fn get(&self, index: usize) -> Option<(&UiText, &V)> {
        self.entries.get(index).map(|(key, value)| (key, value))
    }

    pub const fn capacity(&self) -> usize {
        UI_FIXED_LIST_ITEMS
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<V: Serialize> Serialize for UiFixedMap<V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (key, value) in self.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for UiFixedMap<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct FixedMapVisitor<V>(std::marker::PhantomData<V>);
        impl<'de, V: Deserialize<'de>> Visitor<'de> for FixedMapVisitor<V> {
            type Value = UiFixedMap<V>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an ascending bounded fixed UI map")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut values = UiFixedMap::default();
                while let Some((key, value)) = access.next_entry::<UiText, V>()? {
                    if values.try_push(key, value).is_err() {
                        return Err(serde::de::Error::custom(format!("UiFixedMap requires at most {UI_FIXED_LIST_ITEMS} ascending unique entries")));
                    }
                }
                Ok(values)
            }
        }
        deserializer.deserialize_map(FixedMapVisitor(std::marker::PhantomData))
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct UiFixedBytes {
    bytes: Box<[u8]>,
    len: u16,
}

impl Default for UiFixedBytes {
    fn default() -> Self {
        Self { bytes: vec![0; UI_FIXED_BYTES].into_boxed_slice(), len: 0 }
    }
}

impl fmt::Debug for UiFixedBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("UiFixedBytes").field(&self.as_slice().len()).finish()
    }
}

impl UiFixedBytes {
    pub fn try_from_vec(value: Vec<u8>) -> Result<Self, Vec<u8>> {
        if value.len() > UI_FIXED_BYTES {
            return Err(value);
        }
        let mut fixed = Self::default();
        fixed.bytes[..value.len()].copy_from_slice(&value);
        fixed.len = value.len() as u16;
        Ok(fixed)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn capacity(&self) -> usize {
        UI_FIXED_BYTES
    }
}

impl Serialize for UiFixedBytes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.as_slice())
    }
}

impl<'de> Deserialize<'de> for UiFixedBytes {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{SeqAccess, Visitor};
        struct FixedBytesVisitor;
        impl<'de> Visitor<'de> for FixedBytesVisitor {
            type Value = UiFixedBytes;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("empty bytes; populated bytes require incremental page transport")
            }

            fn visit_bytes<E: serde::de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
                UiFixedBytes::try_from_vec(value.to_vec()).map_err(|value| E::custom(format!("UiFixedBytes exceeds {UI_FIXED_BYTES} bytes with {}", value.len())))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut bytes = Vec::with_capacity(access.size_hint().unwrap_or(0).min(UI_FIXED_BYTES));
                while let Some(byte) = access.next_element::<u8>()? {
                    if bytes.len() == UI_FIXED_BYTES {
                        return Err(serde::de::Error::custom(format!("UiFixedBytes exceeds {UI_FIXED_BYTES} bytes")));
                    }
                    bytes.push(byte);
                }
                UiFixedBytes::try_from_vec(bytes).map_err(|_| serde::de::Error::custom(format!("UiFixedBytes exceeds {UI_FIXED_BYTES} bytes")))
            }
        }
        deserializer.deserialize_bytes(FixedBytesVisitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UiCollectionKind {
    List,
    Map,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UiCollectionHandle {
    slot: usize,
    epoch: u64,
    kind: UiCollectionKind,
}

#[derive(Debug)]
enum UiPageValue {
    List(UiValue),
    Map(UiText, UiValue),
}

#[derive(Debug)]
struct UiPageSlot {
    epoch: u64,
    next: usize,
    value: Option<UiPageValue>,
}

impl Default for UiPageSlot {
    fn default() -> Self {
        Self { epoch: 0, next: UI_VALUE_NONE, value: None }
    }
}

#[derive(Debug)]
struct UiCollectionSlot {
    epoch: u64,
    kind: UiCollectionKind,
    head: usize,
    tail: usize,
    aliases: u64,
    items: usize,
    bytes: usize,
    occupied: bool,
    retiring: bool,
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
        }
    }
}

struct UiValueArena {
    pages: Box<[UiPageSlot]>,
    collections: Box<[UiCollectionSlot]>,
    free_pages: Box<[usize]>,
    free_page_count: usize,
    free_collections: Box<[usize]>,
    free_collection_count: usize,
    retirement: Box<[usize]>,
    retirement_head: usize,
    retirement_len: usize,
    items: usize,
    bytes: usize,
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
            retirement: vec![UI_VALUE_NONE; UI_VALUE_ADMISSION_SLOTS].into_boxed_slice(),
            retirement_head: 0,
            retirement_len: 0,
            items: 0,
            bytes: 0,
        }
    }
}

static UI_VALUE_ARENA: LazyLock<Mutex<UiValueArena>> = LazyLock::new(|| Mutex::new(UiValueArena::default()));

fn with_ui_value_arena<T>(f: impl FnOnce(&mut UiValueArena) -> T) -> T {
    let mut arena = UI_VALUE_ARENA.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut arena)
}

impl UiValueArena {
    fn reserve_collection(&mut self, kind: UiCollectionKind) -> Option<UiCollectionHandle> {
        let bytes = size_of::<UiCollectionSlot>();
        let next_bytes = self.bytes.checked_add(bytes)?;
        if self.free_collection_count == 0 || next_bytes > UI_VALUE_AGGREGATE_BYTES {
            return None;
        }
        let next_epoch = self.collections[self.free_collections[self.free_collection_count - 1]].epoch.checked_add(1)?;
        let free_collection_count = self.free_collection_count.checked_sub(1)?;
        let slot = self.free_collections[free_collection_count];
        self.collections[slot] = UiCollectionSlot {
            epoch: next_epoch,
            kind,
            head: UI_VALUE_NONE,
            tail: UI_VALUE_NONE,
            aliases: 1,
            items: 0,
            bytes,
            occupied: true,
            retiring: false,
        };
        self.free_collection_count = free_collection_count;
        self.bytes = next_bytes;
        Some(UiCollectionHandle { slot, epoch: next_epoch, kind })
    }

    fn collection_mut(&mut self, handle: UiCollectionHandle) -> Option<&mut UiCollectionSlot> {
        let slot = self.collections.get_mut(handle.slot)?;
        (slot.occupied && slot.epoch == handle.epoch && slot.kind == handle.kind).then_some(slot)
    }

    fn collection(&self, handle: UiCollectionHandle) -> Option<&UiCollectionSlot> {
        let slot = self.collections.get(handle.slot)?;
        (slot.occupied && slot.epoch == handle.epoch && slot.kind == handle.kind).then_some(slot)
    }

    fn try_push_page(&mut self, handle: UiCollectionHandle, value: UiPageValue) -> Result<(), UiPageValue> {
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

    fn try_clone_handle(&mut self, handle: UiCollectionHandle) -> Option<UiCollectionHandle> {
        let collection = self.collection_mut(handle)?;
        if collection.retiring {
            return None;
        }
        collection.aliases = collection.aliases.checked_add(1)?;
        Some(handle)
    }

    fn release_handle(&mut self, handle: UiCollectionHandle) {
        let retirement_full = self.retirement_len == UI_VALUE_ADMISSION_SLOTS;
        let should_retire = {
            let Some(collection) = self.collection_mut(handle) else { return };
            if collection.aliases == 0 || collection.retiring {
                return;
            }
            if collection.aliases == 1 && retirement_full {
                return;
            }
            collection.aliases = match collection.aliases.checked_sub(1) {
                Some(aliases) => aliases,
                None => return,
            };
            collection.aliases == 0
        };
        if should_retire {
            if !self.enqueue_retirement(handle.slot) {
                if let Some(collection) = self.collection_mut(handle) {
                    collection.aliases = 1;
                }
            }
        }
    }

    fn enqueue_retirement(&mut self, slot: usize) -> bool {
        if self.retirement_len == UI_VALUE_ADMISSION_SLOTS {
            return false;
        }
        let Some(collection) = self.collections.get_mut(slot) else { return false };
        if collection.retiring || !collection.occupied {
            return false;
        }
        let Some(retirement_len) = self.retirement_len.checked_add(1) else { return false };
        collection.retiring = true;
        let tail = (self.retirement_head + self.retirement_len) % UI_VALUE_ADMISSION_SLOTS;
        self.retirement[tail] = slot;
        self.retirement_len = retirement_len;
        true
    }

    fn pop_retirement(&mut self) -> Option<usize> {
        if self.retirement_len == 0 {
            return None;
        }
        let slot = self.retirement[self.retirement_head];
        self.retirement[self.retirement_head] = UI_VALUE_NONE;
        self.retirement_head = (self.retirement_head + 1) % UI_VALUE_ADMISSION_SLOTS;
        self.retirement_len = self.retirement_len.checked_sub(1)?;
        Some(slot)
    }

    fn retire_one(&mut self) -> Option<UiPageValue> {
        let slot = self.pop_retirement()?;
        let head = self.collections.get(slot).filter(|collection| collection.occupied && collection.retiring)?.head;
        if head == UI_VALUE_NONE {
            if !self.release_collection(slot) {
                self.requeue_retirement(slot);
            }
            return None;
        }
        let page = &mut self.pages[head];
        let next = page.next;
        let Some(free_page_count) = self.free_page_count.checked_add(1).filter(|count| *count <= UI_VALUE_AGGREGATE_ITEMS) else {
            self.requeue_retirement(slot);
            return None;
        };
        let value = page.value.take();
        page.next = UI_VALUE_NONE;
        self.free_pages[self.free_page_count] = head;
        self.free_page_count = free_page_count;
        if let Some(collection) = self.collections.get_mut(slot) {
            collection.head = next;
            if next == UI_VALUE_NONE {
                collection.tail = UI_VALUE_NONE;
            }
        }
        if next == UI_VALUE_NONE {
            if !self.release_collection(slot) {
                self.requeue_retirement(slot);
            }
        } else {
            self.requeue_retirement(slot);
        }
        value
    }

    fn requeue_retirement(&mut self, slot: usize) {
        if self.retirement_len == UI_VALUE_ADMISSION_SLOTS {
            return;
        }
        let tail = (self.retirement_head + self.retirement_len) % UI_VALUE_ADMISSION_SLOTS;
        self.retirement[tail] = slot;
        if let Some(retirement_len) = self.retirement_len.checked_add(1) {
            self.retirement_len = retirement_len;
        }
    }

    fn release_collection(&mut self, slot: usize) -> bool {
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
        self.collections[slot] = UiCollectionSlot { epoch, ..UiCollectionSlot::default() };
        self.free_collections[self.free_collection_count] = slot;
        self.free_collection_count = free_collection_count;
        true
    }

    fn try_clone_value(&mut self, value: &UiValue) -> Option<UiValue> {
        Some(match value {
            UiValue::Null => UiValue::Null,
            UiValue::Bool(value) => UiValue::Bool(*value),
            UiValue::Number(value) => UiValue::Number(*value),
            UiValue::Text(value) => UiValue::Text(value.clone()),
            UiValue::List(value) => UiValue::List(value.try_clone_in(self)?),
            UiValue::Map(value) => UiValue::Map(value.try_clone_in(self)?),
        })
    }

    fn page_value(&mut self, handle: UiCollectionHandle, page: usize) -> Option<(usize, UiPageValue)> {
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
    let retired = with_ui_value_arena(UiValueArena::retire_one);
    drop(retired);
    with_ui_value_arena(|arena| arena.retirement_len == 0)
}

#[derive(Debug)]
pub struct UiList {
    handle: Option<UiCollectionHandle>,
    len: usize,
}

impl Default for UiList {
    fn default() -> Self {
        Self { handle: None, len: 0 }
    }
}

impl UiList {
    fn try_clone_in(&self, arena: &mut UiValueArena) -> Option<Self> {
        let handle = match self.handle {
            Some(handle) => Some(arena.try_clone_handle(handle)?),
            None => None,
        };
        Some(Self { handle, len: self.len })
    }

    pub fn credited_clone(&self) -> Option<Self> {
        with_ui_value_arena(|arena| self.try_clone_in(arena))
    }

    pub fn try_from_values(values: Vec<UiValue>) -> Result<Self, Vec<UiValue>> {
        if values.is_empty() { Ok(Self::default()) } else { Err(values) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn page_count(&self) -> usize {
        self.len
    }

    pub fn backing_bytes(&self) -> usize {
        self.handle
            .and_then(|handle| with_ui_value_arena(|arena| arena.collection(handle).map(|collection| collection.bytes)))
            .unwrap_or(0)
    }

    pub fn cursor(&self) -> UiListCursor {
        let (handle, next) = match self.handle {
            Some(handle) => with_ui_value_arena(|arena| {
                let handle = arena.try_clone_handle(handle)?;
                Some((Some(handle), arena.collection(handle)?.head))
            })
            .unwrap_or((None, UI_VALUE_NONE)),
            None => (None, UI_VALUE_NONE),
        };
        UiListCursor { handle, next }
    }

    pub fn iter(&self) -> UiListCursor {
        self.cursor()
    }
}

impl PartialEq for UiList {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl Drop for UiList {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            with_ui_value_arena(|arena| arena.release_handle(handle));
        }
    }
}

pub struct UiListCursor {
    handle: Option<UiCollectionHandle>,
    next: usize,
}

impl Iterator for UiListCursor {
    type Item = UiValue;

    fn next(&mut self) -> Option<Self::Item> {
        let handle = self.handle?;
        let (next, value) = with_ui_value_arena(|arena| arena.page_value(handle, self.next))?;
        self.next = next;
        match value {
            UiPageValue::List(value) => Some(value),
            UiPageValue::Map(_, _) => None,
        }
    }
}

impl Drop for UiListCursor {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            with_ui_value_arena(|arena| arena.release_handle(handle));
        }
    }
}

pub struct UiListBuilder {
    handle: Option<UiCollectionHandle>,
    len: usize,
}

impl UiListBuilder {
    pub fn try_new() -> Option<Self> {
        let handle = with_ui_value_arena(|arena| arena.reserve_collection(UiCollectionKind::List))?;
        Some(Self { handle: Some(handle), len: 0 })
    }

    pub fn push(&mut self, value: UiValue) -> Result<(), UiValue> {
        let Some(handle) = self.handle else { return Err(value) };
        let Some(next_len) = self.len.checked_add(1).filter(|len| *len <= UI_VALUE_MAX_ITEMS) else { return Err(value) };
        with_ui_value_arena(|arena| arena.try_push_page(handle, UiPageValue::List(value))).map_err(|value| match value {
            UiPageValue::List(value) => value,
            UiPageValue::Map(_, value) => value,
        })?;
        self.len = next_len;
        Ok(())
    }

    pub fn finish(mut self) -> UiList {
        let handle = self.handle.take();
        if self.len == 0 {
            if let Some(handle) = handle {
                with_ui_value_arena(|arena| arena.release_handle(handle));
            }
            UiList::default()
        } else {
            UiList { handle, len: self.len }
        }
    }
}

impl Drop for UiListBuilder {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            with_ui_value_arena(|arena| arena.release_handle(handle));
        }
    }
}

impl Serialize for UiList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len))?;
        for value in self.iter() {
            sequence.serialize_element(&value)?;
        }
        sequence.end()
    }
}

impl<'de> Deserialize<'de> for UiList {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{SeqAccess, Visitor};
        struct UiListVisitor;

        impl<'de> Visitor<'de> for UiListVisitor {
            type Value = UiList;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a fixed-page UI value list")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let Some(mut builder) = UiListBuilder::try_new() else { return Err(serde::de::Error::custom("UiList admission failed")) };
                while let Some(value) = access.next_element::<UiValue>()? {
                    if builder.push(value).is_err() {
                        return Err(serde::de::Error::custom(format!("UiList exceeds {UI_VALUE_MAX_ITEMS} items or the aggregate page budget")));
                    }
                }
                Ok(builder.finish())
            }
        }
        deserializer.deserialize_seq(UiListVisitor)
    }
}

#[derive(Debug)]
pub struct UiMap {
    handle: Option<UiCollectionHandle>,
    len: usize,
}

impl Default for UiMap {
    fn default() -> Self {
        Self { handle: None, len: 0 }
    }
}

impl UiMap {
    fn try_clone_in(&self, arena: &mut UiValueArena) -> Option<Self> {
        let handle = match self.handle {
            Some(handle) => Some(arena.try_clone_handle(handle)?),
            None => None,
        };
        Some(Self { handle, len: self.len })
    }

    pub fn credited_clone(&self) -> Option<Self> {
        with_ui_value_arena(|arena| self.try_clone_in(arena))
    }

    pub fn try_from_entries(entries: Vec<(String, UiValue)>) -> Result<Self, Vec<(String, UiValue)>> {
        if entries.is_empty() { Ok(Self::default()) } else { Err(entries) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn page_count(&self) -> usize {
        self.len
    }

    pub fn backing_bytes(&self) -> usize {
        self.handle
            .and_then(|handle| with_ui_value_arena(|arena| arena.collection(handle).map(|collection| collection.bytes)))
            .unwrap_or(0)
    }

    pub fn cursor(&self) -> UiMapCursor {
        let (handle, next) = match self.handle {
            Some(handle) => with_ui_value_arena(|arena| {
                let handle = arena.try_clone_handle(handle)?;
                Some((Some(handle), arena.collection(handle)?.head))
            })
            .unwrap_or((None, UI_VALUE_NONE)),
            None => (None, UI_VALUE_NONE),
        };
        UiMapCursor { handle, next, current: None }
    }

    pub fn iter(&self) -> UiMapCursor {
        self.cursor()
    }
}

impl PartialEq for UiMap {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl Drop for UiMap {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            with_ui_value_arena(|arena| arena.release_handle(handle));
        }
    }
}

pub struct UiMapCursor {
    handle: Option<UiCollectionHandle>,
    next: usize,
    current: Option<(UiText, UiValue)>,
}

impl UiMapCursor {
    pub fn advance(&mut self) -> Option<(&UiText, &UiValue)> {
        let handle = self.handle?;
        let (next, value) = with_ui_value_arena(|arena| arena.page_value(handle, self.next))?;
        self.next = next;
        self.current = match value {
            UiPageValue::Map(key, value) => Some((key, value)),
            UiPageValue::List(_) => None,
        };
        self.current.as_ref().map(|(key, value)| (key, value))
    }

    pub fn current(&self) -> Option<(&UiText, &UiValue)> {
        self.current.as_ref().map(|(key, value)| (key, value))
    }

    pub fn take_current(&mut self) -> Option<(UiText, UiValue)> {
        self.current.take()
    }
}

impl Iterator for UiMapCursor {
    type Item = (UiText, UiValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()?;
        self.current.take()
    }
}

impl Drop for UiMapCursor {
    fn drop(&mut self) {
        drop(self.current.take());
        if let Some(handle) = self.handle.take() {
            with_ui_value_arena(|arena| arena.release_handle(handle));
        }
    }
}

pub struct UiMapBuilder {
    handle: Option<UiCollectionHandle>,
    len: usize,
    last_key: Option<UiText>,
}

impl UiMapBuilder {
    pub fn try_new() -> Option<Self> {
        let handle = with_ui_value_arena(|arena| arena.reserve_collection(UiCollectionKind::Map))?;
        Some(Self { handle: Some(handle), len: 0, last_key: None })
    }

    pub fn push(&mut self, key: String, value: UiValue) -> Result<(), (String, UiValue)> {
        let Some(handle) = self.handle else { return Err((key, value)) };
        let Some(fixed_key) = UiText::try_from_str(&key) else { return Err((key, value)) };
        let Some(next_len) = self.len.checked_add(1).filter(|len| *len <= UI_VALUE_MAX_ITEMS) else { return Err((key, value)) };
        if self.last_key.as_ref().is_some_and(|last| last >= &fixed_key) {
            return Err((key, value));
        }
        let retained_key = fixed_key.clone();
        if let Err(page) = with_ui_value_arena(|arena| arena.try_push_page(handle, UiPageValue::Map(fixed_key, value))) {
            let value = match page {
                UiPageValue::Map(_, value) | UiPageValue::List(value) => value,
            };
            return Err((key, value));
        }
        self.last_key = Some(retained_key);
        self.len = next_len;
        Ok(())
    }

    pub fn finish(mut self) -> UiMap {
        let handle = self.handle.take();
        if self.len == 0 {
            if let Some(handle) = handle {
                with_ui_value_arena(|arena| arena.release_handle(handle));
            }
            UiMap::default()
        } else {
            UiMap { handle, len: self.len }
        }
    }
}

impl Drop for UiMapBuilder {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            with_ui_value_arena(|arena| arena.release_handle(handle));
        }
    }
}

impl Serialize for UiMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len))?;
        for (key, value) in self.iter() {
            map.serialize_entry(&key, &value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for UiMap {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct UiMapVisitor;

        impl<'de> Visitor<'de> for UiMapVisitor {
            type Value = UiMap;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a fixed-page UI value map")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let Some(mut builder) = UiMapBuilder::try_new() else { return Err(serde::de::Error::custom("UiMap admission failed")) };
                while let Some((key, value)) = access.next_entry::<String, UiValue>()? {
                    if builder.push(key, value).is_err() {
                        return Err(serde::de::Error::custom(format!("UiMap requires at most {UI_VALUE_MAX_ITEMS} ascending unique entries within the aggregate page budget")));
                    }
                }
                Ok(builder.finish())
            }
        }
        deserializer.deserialize_map(UiMapVisitor)
    }
}

//#region 🔖️Action

/// 🆔️ A versioned action address. `scope` names the controller/domain (the old `ActionDescriptor`'s
/// stringly `controller_id`, e.g. `"cad-play"`, grepped verbatim from the plugin fleet's
/// `ActionFactory::new(CONTROLLER_ID)` call sites), `name` the verb (the old `action`, e.g.
/// `"objectMove"`/`"setValue"`/`"addWidget"`), and `version` is new: it lets a renderer reject or
/// migrate a stale action instead of silently invoking the wrong one — the one axis the old stringly
/// pair never carried.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionId {
    pub scope: UiText,
    pub name: UiText,
    pub version: u16,
}

impl ActionId {
    /// 🏭️ `const fn`-friendly constructor — every field already owned, no allocation happens inside.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(scope: UiText, name: UiText, version: u16) -> Self {
        Self { scope, name, version }
    }

    /// 🏭️ Version-1 convenience constructor — the common case; the plugin fleet will write thousands
    /// of these from `&str`/`String` call sites the old `ActionFactory::action` already used.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn try_v1(scope: impl AsRef<str>, name: impl AsRef<str>) -> Option<Self> {
        Some(Self { scope: UiText::try_from_str(scope.as_ref())?, name: UiText::try_from_str(name.as_ref())?, version: 1 })
    }
}

impl fmt::Display for ActionId {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}@{}", self.scope, self.name, self.version)
    }
}

/// 🎯️ The lifecycle moment on a node that fires an [`ActionBinding`] — replaces the old single
/// implicit "the" action every node carried with a closed, named set, so one node can bind several
/// distinct moments (e.g. `Change` while typing, `Commit` on blur) without inventing parallel fields.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Trigger {
    #[default]
    Activate,
    Change,
    Commit,
    Delta,
    Drop,
    Submit,
    Abort,
    RepeatLast,
    HoverPreview,
}

/// 🔗️ One node-carried binding from a [`Trigger`] moment to a versioned [`ActionId`]. Replaces every
/// old `on_change`/`action`/`drop_action`/... field scattered across the wgpu target's per-component
/// node structs — a record's `bindings: Vec<ActionBinding>` is the one place any of them now live.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionBinding {
    pub trigger: Trigger,
    pub action: ActionId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
    /// 🔐️ An optional capability token a host must hold before this binding is even offered —
    /// orthogonal to `args`, which is data the action consumes rather than a permission gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<UiText>,
}

impl ActionBinding {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            trigger: self.trigger,
            action: self.action.clone(),
            args: match self.args.as_ref() {
                Some(args) => Some(args.credited_clone()?),
                None => None,
            },
            capability: self.capability.clone(),
        })
    }
}

/// 📋️ A reference to a resolved context menu — replaces the old `UiMenuRef`'s `DslValue` args with
/// the crate-neutral [`UiValue`].
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRef {
    pub id: UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
}

impl MenuRef {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            id: self.id.clone(),
            args: match self.args.as_ref() {
                Some(args) => Some(args.credited_clone()?),
                None => None,
            },
        })
    }
}

/// 🎬️ One user action against a specific node at a specific revision — what a renderer emits and the
/// headless runtime dispatches. `revision`/`node_key` let the runtime recognise and drop a `Stale`
/// intent (one whose `revision` trails the surface's current revision by more than one) instead of
/// applying it against geometry the user never actually saw.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiIntent {
    pub surface: crate::SurfaceId,
    pub revision: crate::UiRevision,
    pub node: crate::UiNodeId,
    /// 🔑️ The node's own [`crate::UiNodeRecord::key`], carried alongside the id so a replay or a log
    /// entry still identifies the intended element after id churn from an intervening reconciliation.
    pub node_key: UiText,
    pub trigger: Trigger,
    pub action: ActionId,
    /// 🔁️ Echoed verbatim from the firing [`ActionBinding::args`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
    /// ✍️ The trigger-specific payload: `Change`'s new value, `Delta`'s signed step count, `Drop`'s
    /// dropped payload — `None` for triggers that carry no data of their own (`Activate`, `Submit`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<UiValue>,
    /// 🔢️ Renderer-monotonic per surface — lets the runtime order and de-duplicate intents
    /// independently of transport delivery order.
    pub seq: u64,
}
//#endregion 🔖️Action

//#region 🔖️Value
/// 🧬️ A neutral, JSON-shaped value — the ONE recursive type in this crate. Every node in
/// `🦀️document.rs` avoids inline recursion by addressing children through [`crate::UiNodeId`] instead
/// of nesting a node inside another; `UiValue` is the deliberate exception because it does not
/// describe document shape at all, it describes an arbitrary opaque payload (action args, extension
/// props) that genuinely IS JSON-shaped, and owned fixed list/map pages give the schema an indirection
/// to resolve (heap-allocated, not an inline field) rather than the infinitely-sized-struct problem
/// direct node-in-node nesting would create.
///
/// ⚠️ The os-kernel's `DslValue` (`🧰️framework/🔨️modules/🌱️value/🦀️component.rs`) must NEVER appear in
/// this crate — this crate has no such dependency and stays `wasm32-wasip2`/`wasm32-unknown-unknown`
/// safe by construction. `From`/`Into` conversions between `UiValue` and `DslValue` belong in the
/// os-kernel crate, never here.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UiValue {
    #[default]
    Null,
    Bool(bool),
    Number(f64),
    Text(UiText),
    List(UiList),
    Map(UiMap),
}

impl UiValue {
    pub fn credited_clone(&self) -> Option<Self> {
        with_ui_value_arena(|arena| arena.try_clone_value(self))
    }
}

//#endregion 🔖️Value

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn ui_text(value: &str) -> UiText {
        UiText::try_from_str(value).expect("bounded fixture text")
    }

    fn ui_list(values: impl IntoIterator<Item = UiValue>) -> UiList {
        let mut builder = UiListBuilder::try_new().expect("fixed list builder");
        for value in values {
            builder.push(value).expect("fixed list page");
        }
        builder.finish()
    }

    fn ui_map(entries: impl IntoIterator<Item = (String, UiValue)>) -> UiMap {
        let mut entries: Vec<_> = entries.into_iter().collect();
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        let mut builder = UiMapBuilder::try_new().expect("fixed map builder");
        for (key, value) in entries {
            builder.push(key, value).expect("fixed map page");
        }
        builder.finish()
    }

    #[test]
    fn action_id_displays_scope_dot_name_at_version() {
        let id = ActionId::try_v1("cad-play", "objectMove").expect("bounded action id");
        assert_eq!(id.to_string(), "cad-play.objectMove@1");
        assert_eq!(ActionId::new(ui_text("app"), ui_text("submit"), 3).to_string(), "app.submit@3");
    }

    #[test]
    fn fixed_owners_keep_bounded_payloads_off_the_stack() {
        assert!(size_of::<UiFixedBytes>() <= size_of::<usize>() * 3);
        assert!(size_of::<UiFixedList<UiFixedBytes>>() <= size_of::<usize>() * 3);
        assert!(size_of::<UiValueArena>() <= size_of::<usize>() * 24);
    }

    #[test]
    fn fixed_bytes_admit_the_scene_packet_census_exactly() {
        let admitted = vec![7; UI_FIXED_BYTES];
        assert_eq!(UiFixedBytes::try_from_vec(admitted).expect("exact scene packet census").len(), UI_FIXED_BYTES);
        let rejected = vec![7; UI_FIXED_BYTES + 1];
        assert_eq!(UiFixedBytes::try_from_vec(rejected).expect_err("scene packet over census").len(), UI_FIXED_BYTES + 1);
    }

    #[allow(clippy::needless_pass_by_value)]
    fn value_round_trips(value: UiValue) {
        let first = serde_json::to_string(&value).expect("serialize");
        let deserialized: UiValue = serde_json::from_str(&first).expect("deserialize");
        let second = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(first, second);
        assert_eq!(value, deserialized);
    }

    #[test]
    fn every_ui_value_shape_round_trips() {
        value_round_trips(UiValue::Null);
        value_round_trips(UiValue::Bool(true));
        value_round_trips(UiValue::Number(-3.5));
        value_round_trips(UiValue::Text(ui_text("hi")));
        value_round_trips(UiValue::List(ui_list([UiValue::Number(1.0), UiValue::Text(ui_text("two"))])));
        value_round_trips(UiValue::Map(ui_map([
            ("id".to_string(), UiValue::Text(ui_text("widget"))),
            ("nested".to_string(), UiValue::List(ui_list([UiValue::Bool(false), UiValue::Null]))),
        ])));
    }

    #[test]
    fn ui_value_default_is_null() {
        assert_eq!(UiValue::default(), UiValue::Null);
    }

    #[test]
    fn action_binding_round_trips_with_and_without_args() {
        let full = ActionBinding { trigger: Trigger::Change, action: ActionId::try_v1("app", "setValue").expect("bounded action id"), args: Some(UiValue::Text(ui_text("scope"))), capability: Some(ui_text("edit")) };
        let first = serde_json::to_string(&full).expect("serialize");
        let back: ActionBinding = serde_json::from_str(&first).expect("deserialize");
        assert_eq!(full, back);

        let minimal = ActionBinding::default();
        let json = serde_json::to_value(&minimal).expect("serialize");
        assert!(json.get("args").is_none());
        assert!(json.get("capability").is_none());
    }

    #[test]
    fn menu_ref_round_trips() {
        let menu = MenuRef { id: ui_text("context.tree-item"), args: Some(UiValue::Number(2.0)) };
        let first = serde_json::to_string(&menu).expect("serialize");
        let back: MenuRef = serde_json::from_str(&first).expect("deserialize");
        assert_eq!(menu, back);
    }

    #[test]
    fn ui_intent_round_trips() {
        let intent = UiIntent {
            surface: crate::SurfaceId::try_from("note.play.navigator").expect("bounded surface id"),
            revision: crate::UiRevision(4),
            node: crate::UiNodeId(9),
            node_key: ui_text("row-9"),
            trigger: Trigger::Delta,
            action: ActionId::try_v1("cad-play", "objectMove").expect("bounded action id"),
            args: Some(UiValue::Number(1.0)),
            input: Some(UiValue::Number(-2.0)),
            seq: 42,
        };
        let first = serde_json::to_string(&intent).expect("serialize");
        let deserialized: UiIntent = serde_json::from_str(&first).expect("deserialize");
        let second = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(first, second);
        assert_eq!(intent, deserialized);
    }

    #[test]
    fn every_trigger_variant_round_trips() {
        for trigger in [Trigger::Activate, Trigger::Change, Trigger::Commit, Trigger::Delta, Trigger::Drop, Trigger::Submit, Trigger::Abort, Trigger::RepeatLast, Trigger::HoverPreview] {
            let json = serde_json::to_string(&trigger).expect("serialize");
            let back: Trigger = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(trigger, back);
        }
    }

    #[test]
    fn fixed_page_max_plus_one_returns_the_exact_untransferred_owner() {
        let mut arena = UiValueArena::default();
        let handle = arena.reserve_collection(UiCollectionKind::List).expect("fixed list authority");
        for index in 0..UI_VALUE_MAX_ITEMS {
            arena.try_push_page(handle, UiPageValue::List(UiValue::Number(index as f64))).expect("fixed page admission");
        }
        let refused = UiPageValue::List(UiValue::Text(ui_text("exact-refusal-owner")));
        let refused = arena.try_push_page(handle, refused).expect_err("maximum plus one must refuse before transfer");
        assert!(matches!(refused, UiPageValue::List(UiValue::Text(text)) if text.as_str() == "exact-refusal-owner"));
    }

    #[test]
    fn ascending_map_duplicate_refusal_preserves_key_and_value() {
        let mut builder = UiMapBuilder::try_new().expect("fixed map builder");
        builder.push("a".to_string(), UiValue::Number(1.0)).expect("first key");
        let (key, value) = builder.push("a".to_string(), UiValue::Text(ui_text("owner"))).expect_err("duplicate key must be inert");
        assert_eq!(key, "a");
        assert_eq!(value, UiValue::Text(ui_text("owner")));
    }

    #[test]
    fn credited_alias_keeps_pages_live_after_original_handle_is_lost() {
        let original = ui_list([UiValue::Text(ui_text("retained"))]);
        let alias = original.credited_clone().expect("credited alias slot");
        drop(original);
        assert_eq!(alias.cursor().next(), Some(UiValue::Text(ui_text("retained"))));
        drop(alias);
        assert!(close_ui_value_page_one());
    }

    #[test]
    fn poisoned_arena_lock_recovers_without_losing_fixed_authority() {
        let _ = std::panic::catch_unwind(|| {
            let _guard = UI_VALUE_ARENA.lock().expect("unpoisoned fixture entry");
            panic!("poison fixed arena");
        });
        let list = ui_list([UiValue::Bool(true)]);
        assert_eq!(list.cursor().next(), Some(UiValue::Bool(true)));
    }

    #[test]
    fn arena_initialization_is_a_fixed_control_and_page_taxonomy() {
        let started = std::time::Instant::now();
        let arena = UiValueArena::default();
        assert_eq!(arena.free_page_count, UI_VALUE_AGGREGATE_ITEMS);
        assert_eq!(arena.free_collection_count, UI_VALUE_ADMISSION_SLOTS);
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }
}
//#endregion 🧪️Tests
