//! 📃️ Ui list: the arena-backed `UiList` with its cursor, builder and serde. See [`super`].

use super::*;

#[derive(Debug, Default)]
pub struct UiList {
    pub(super) handle: Option<UiCollectionHandle>,
    pub(super) len: usize,
}

impl UiList {
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

    pub fn try_from_values(values: Vec<UiValue>) -> Result<Self, Vec<UiValue>> {
        if values.is_empty() {
            Ok(Self::default())
        } else {
            Err(values)
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
            retirement::hand_back_value(handle);
        }
    }
}

pub struct UiListCursor {
    pub(super) handle: Option<UiCollectionHandle>,
    pub(super) next: usize,
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
            retirement::hand_back_value(handle);
        }
    }
}

pub struct UiListBuilder {
    pub(super) handle: Option<UiCollectionHandle>,
    pub(super) len: usize,
}

impl UiListBuilder {
    pub fn try_new() -> Option<Self> {
        let handle = with_ui_value_arena(|arena| arena.reserve_collection(UiCollectionKind::List))?;
        Some(Self { handle: Some(handle), len: 0 })
    }

    #[expect(clippy::result_large_err, reason = "A full or stale list builder hands back the original bounded value without allocating on refusal.")]
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
                retirement::hand_back_value(handle);
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
            retirement::hand_back_value(handle);
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
