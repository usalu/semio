//! 🗝️ Ui map: the arena-backed `UiMap` with its cursor, builder and serde. See [`super`].

use super::*;

#[derive(Debug, Default)]
pub struct UiMap {
    pub handle: Option<UiCollectionHandle>,
    pub(super) len: usize,
}

impl UiMap {
    pub(super) fn try_clone_in(&self, arena: &mut UiValueArena) -> Option<Self> {
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
        if entries.is_empty() {
            Ok(Self::default())
        } else {
            Err(entries)
        }
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
        self.handle.and_then(|handle| with_ui_value_arena(|arena| arena.collection(handle).map(|collection| collection.bytes))).unwrap_or(0)
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
            retirement::hand_back_value(handle);
        }
    }
}

pub struct UiMapCursor {
    pub(super) handle: Option<UiCollectionHandle>,
    pub(super) next: usize,
    pub(super) current: Option<(UiText, UiValue)>,
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
            retirement::hand_back_value(handle);
        }
    }
}

pub struct UiMapBuilder {
    pub(super) handle: Option<UiCollectionHandle>,
    pub(super) len: usize,
}

impl UiMapBuilder {
    /// 🧷️ Transfers even an empty source collection to its explicit original retirement owner.
    pub fn finish_initialized(mut self)->UiMap{UiMap{handle:self.handle.take(),len:self.len}}
    /// 🎟️ Borrows existing collection authority without initializing global backing or blocking its owner.
    pub fn try_new_initialized()->Result<Option<Self>,&'static str>{
        let Some(mut arena)=collection_arena::initialized_arena()?else{return Ok(None)};
        let handle=arena.reserve_collection(UiCollectionKind::Map).ok_or("UI map source capacity refused")?;Ok(Some(Self{handle:Some(handle),len:0}))
    }
    /// 🗝️ Moves one already bounded key and value without temporary String allocation or lost refusal custody.
    pub fn insert_initialized(&mut self,donor:&mut Option<(UiText,UiValue)>)->Result<bool,&'static str>{
        let handle=self.handle.ok_or("UI map source is closed")?;let next_len=self.len.checked_add(1).filter(|len|*len<=UI_VALUE_MAX_ITEMS).ok_or("UI map source capacity exceeded")?;
        let Some(mut arena)=collection_arena::initialized_arena()?else{return Ok(false)};
        let(key,_)=donor.as_ref().ok_or("UI map source donor is empty")?;
        let position=match arena.search_map_key(handle,key){Ok(_)=>return Err("UI map source key is duplicated"),Err(position)=>position};
        let prev=arena.map_prev_at_position(handle,position).ok_or("UI map source is unavailable")?;let(key,value)=donor.take().unwrap();
        if let Err(pair)=arena.try_insert_map_page(handle,prev,key,value){*donor=Some(pair);return Err("UI map source capacity refused");}
        self.len=next_len;Ok(true)
    }
    pub fn try_new() -> Option<Self> {
        let handle = with_ui_value_arena(|arena| arena.reserve_collection(UiCollectionKind::Map))?;
        Some(Self { handle: Some(handle), len: 0 })
    }

    /// 🔎️ Whether a key is already admitted, so a decoder can tell duplication from capacity.
    pub fn contains(&self, key: &UiText) -> bool {
        let Some(handle) = self.handle else { return false };
        with_ui_value_arena(|arena| arena.search_map_key(handle, key).is_ok())
    }

    /// 🗂️ Admits one entry at its SORTED position instead of requiring the caller to arrive in order.
    ///
    /// ⚖️ A `UiMap`'s wire form is a JSON object, and object key order carries no meaning: the wgpu
    /// bridge hands the guest's document to `JSON.stringify`, and every ECMAScript engine emits
    /// array-index-like keys first in numeric order and the remaining string keys after. Decoding
    /// through [`Self::push`] therefore refused whole documents on the wgpu renderer while React's
    /// reader — which sorts the object's keys itself — read the same bytes. One displaced page is
    /// spliced into the arena-backed chain rather than buffered into a vector.
    #[expect(clippy::result_large_err, reason = "Map admission preserves the original key allocation and value owner for retry when duplication or credits reject them.")]
    pub fn try_insert(&mut self, key: String, value: UiValue) -> Result<(), (String, UiValue)> {
        let Some(handle) = self.handle else { return Err((key, value)) };
        let Some(fixed_key) = UiText::try_from_str(&key) else { return Err((key, value)) };
        let Some(next_len) = self.len.checked_add(1).filter(|len| *len <= UI_VALUE_MAX_ITEMS) else { return Err((key, value)) };
        let insert = with_ui_value_arena(|arena| match arena.search_map_key(handle, &fixed_key) {
            Ok(_) => Err((key, value)),
            Err(position) => {
                let Some(prev) = arena.map_prev_at_position(handle, position) else { return Err((key, value)) };
                arena.try_insert_map_page(handle, prev, fixed_key, value).map_err(|(key, value)| (key.as_str().to_string(), value))
            }
        });
        if let Err(pair) = insert {
            return Err(pair);
        }
        self.len = next_len;
        Ok(())
    }

    /// 🩺️ Names WHY [`Self::try_insert`] refused, for a decoder that can only report a string.
    pub fn refusal(&self) -> String {
        if self.len >= UI_VALUE_MAX_ITEMS {
            format!("UiMap is full at its {UI_VALUE_MAX_ITEMS}-entry capacity")
        } else {
            format!("UiMap admission ran out of page grant at entry {} of at most {UI_VALUE_MAX_ITEMS}", self.len + 1)
        }
    }

    #[expect(clippy::result_large_err, reason = "Map admission preserves the original key allocation and value owner for retry when ordering or credits reject them.")]
    pub fn push(&mut self, key: String, value: UiValue) -> Result<(), (String, UiValue)> {
        self.try_insert(key, value)
    }

    pub fn finish(mut self) -> UiMap {
        let handle = self.handle.take();
        if self.len == 0 {
            if let Some(handle) = handle {
                retirement::hand_back_value(handle);
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
            retirement::hand_back_value(handle);
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
                formatter.write_str("a fixed-page UI value map of unique keys, in any order")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let Some(mut builder) = UiMapBuilder::try_new() else { return Err(serde::de::Error::custom("UiMap admission failed")) };
                while let Some((key, value)) = access.next_entry::<String, UiValue>()? {
                    let fixed_key = UiText::try_from_str(&key);
                    if fixed_key.as_ref().is_some_and(|fixed_key| builder.contains(fixed_key)) {
                        return Err(serde::de::Error::custom(format!("UiMap keys must be unique; '{key}' arrived twice")));
                    }
                    if builder.try_insert(key, value).is_err() {
                        return Err(serde::de::Error::custom(builder.refusal()));
                    }
                }
                Ok(builder.finish())
            }
        }
        deserializer.deserialize_map(UiMapVisitor)
    }
}
