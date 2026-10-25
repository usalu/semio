//! 🗺️ Fixed map and bytes: the `UiFixedMap` keyed by bounded text and the `UiFixedBytes` byte buffer with their serde and value conversions. See [`super`].

use super::*;

#[derive(Debug, PartialEq)]
pub struct UiFixedMap<V> {
    pub(super) entries: UiFixedList<(UiText, V)>,
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
    #[expect(clippy::result_large_err, reason = "Sorted fixed-map admission returns both original inputs when order or capacity rejects them.")]
    pub fn try_push(&mut self, key: UiText, value: V) -> Result<(), (UiText, V)> {
        if self.entries.len().checked_sub(1).and_then(|index| self.entries.get(index)).is_some_and(|(last, _)| last >= &key) {
            return Err((key, value));
        }
        self.entries.try_push((key, value))
    }

    /// 🗂️ Admits one entry at its SORTED position instead of requiring the caller to arrive in order.
    ///
    /// ⚖️ A `UiFixedMap`'s wire form is a JSON object, and object key order carries no meaning: the
    /// wgpu bridge hands the guest's document to `JSON.stringify`, and every ECMAScript engine emits
    /// array-index-like keys (`"10".."32"`) first in numeric order and the remaining string keys
    /// (`"01".."09"`) after. Decoding through [`Self::try_push`] therefore refused the whole
    /// app-static catalogue document on the wgpu renderer while React's reader — which sorts the
    /// object's keys itself — read the same bytes. This is that reader's Rust half. One displaced
    /// payload is carried through the tail rather than copied, so admission allocates nothing beyond
    /// the list's own page grant. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
    #[expect(clippy::result_large_err, reason = "Sorted fixed-map admission returns both original inputs when duplication or capacity rejects them.")]
    pub fn try_insert(&mut self, key: UiText, value: V) -> Result<(), (UiText, V)> {
        let Err(position) = self.search(&key) else { return Err((key, value)) };
        if self.entries.len() == UI_FIXED_LIST_ITEMS || self.entries.reserve_next_slot().is_err() {
            return Err((key, value));
        }
        let mut carry = (key, value);
        for index in position..self.entries.len() {
            let Some(slot) = self.entries.get_mut(index) else { return Err(carry) };
            std::mem::swap(slot, &mut carry);
        }
        self.entries.try_push_reserved(carry)
    }

    /// 🔎️ Whether a key is already admitted, so a decoder can tell duplication from capacity.
    pub fn contains(&self, key: &UiText) -> bool {
        self.search(key).is_ok()
    }

    /// 🩺️ Names WHY [`Self::try_insert`] refused, for a decoder that can only report a string.
    ///
    /// ⚖️ The two refusals are different defects with different owners and a bare "requires at most N
    /// entries" hid one behind the other: a map at capacity is a producer publishing too many keys,
    /// while a map refused below capacity is the aggregate `ui_value_headroom` page grant running out
    /// mid-document — a whole-document budget question. The wgpu bridge surfaces this text verbatim in
    /// `renderDocument result parse failed`, which was the only evidence available for two generation3d
    /// examples whose preview document the shell refused (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn refusal(&self) -> String {
        let length = self.entries.len();
        if length >= UI_FIXED_LIST_ITEMS {
            format!("UiFixedMap is full at its {UI_FIXED_LIST_ITEMS}-entry capacity")
        } else {
            format!("UiFixedMap admission ran out of page grant at entry {} of at most {UI_FIXED_LIST_ITEMS}", length + 1)
        }
    }

    fn search(&self, key: &UiText) -> Result<usize, usize> {
        let mut low = 0;
        let mut high = self.entries.len();
        while low < high {
            let middle = low + (high - low) / 2;
            let Some((candidate, _)) = self.entries.get(middle) else { return Err(low) };
            match candidate.cmp(key) {
                std::cmp::Ordering::Less => low = middle + 1,
                std::cmp::Ordering::Greater => high = middle,
                std::cmp::Ordering::Equal => return Ok(middle),
            }
        }
        Err(low)
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
                formatter.write_str("a bounded fixed UI map of unique keys, in any order")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut values = UiFixedMap::default();
                while let Some((key, value)) = access.next_entry::<UiText, V>()? {
                    if values.contains(&key) {
                        return Err(serde::de::Error::custom(format!("UiFixedMap keys must be unique; '{key}' arrived twice")));
                    }
                    if values.try_insert(key, value).is_err() {
                        return Err(serde::de::Error::custom(values.refusal()));
                    }
                }
                Ok(values)
            }
        }
        deserializer.deserialize_map(FixedMapVisitor(std::marker::PhantomData))
    }
}

/// 🌱️ Hand-written, not derived — generic over `V`, same reason as `UiFixedList` above. Mirrors the
/// `Serialize`/`Deserialize` impls exactly (a bare `{key: value}` JSON object keyed by `UiText`'s own
/// wire string). Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
impl<V: ::semio_framework_value::ToValue> ::semio_framework_value::ToValue for UiFixedMap<V> {
    fn to_value(&self) -> ::semio_framework_value::DslValue {
        ::semio_framework_value::DslValue::object(self.iter().map(|(key, value)| (key.as_str().to_string(), value.to_value())))
    }
}

impl<V: ::semio_framework_value::FromValue> ::semio_framework_value::FromValue for UiFixedMap<V> {
    fn from_value(value: ::semio_framework_value::DslValue) -> Result<Self, ::semio_framework_value::ValueError> {
        let entries = match value {
            ::semio_framework_value::DslValue::Object(entries) => entries,
            other => return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object, found {other:?}"))),
        };
        let mut values = UiFixedMap::default();
        for (key, value) in entries {
            let key = UiText::try_from_string(key).map_err(|value| ::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("UiFixedMap key exceeds {UI_TEXT_MAX_BYTES} bytes with {}", value.len())))?;
            let value = <V as ::semio_framework_value::FromValue>::from_value(value)?;
            if values.contains(&key) {
                return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("UiFixedMap keys must be unique; '{key}' arrived twice")));
            }
            if values.try_insert(key, value).is_err() {
                return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, values.refusal()));
            }
        }
        Ok(values)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct UiFixedBytes {
    pub(super) bytes: Box<[u8]>,
    pub(super) len: u16,
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

/// 🧬️ Hand-written, not derived. `serde_json`'s `serialize_bytes` (the format this `Serialize` impl
/// runs under on every JSON-speaking caller) has no native bytes type and falls back to a plain
/// `Number` array — mirrored here byte for byte so `to_value`/`from_value` round-trip the SAME wire
/// shape as `serde_json::to_value`/`from_value` do. Ticket
/// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
impl ::semio_framework_value::ToValue for UiFixedBytes {
    fn to_value(&self) -> ::semio_framework_value::DslValue {
        ::semio_framework_value::DslValue::Array(self.as_slice().iter().map(|byte| ::semio_framework_value::DslValue::uint(*byte as u64)).collect())
    }
}

impl ::semio_framework_value::FromValue for UiFixedBytes {
    fn from_value(value: ::semio_framework_value::DslValue) -> Result<Self, ::semio_framework_value::ValueError> {
        let items = match value {
            ::semio_framework_value::DslValue::Array(items) => items,
            other => return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for UiFixedBytes, found {other:?}"))),
        };
        let mut bytes = Vec::with_capacity(items.len().min(UI_FIXED_BYTES));
        for item in items {
            let byte = <u8 as ::semio_framework_value::FromValue>::from_value(item)?;
            if bytes.len() == UI_FIXED_BYTES {
                return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("UiFixedBytes exceeds {UI_FIXED_BYTES} bytes")));
            }
            bytes.push(byte);
        }
        UiFixedBytes::try_from_vec(bytes).map_err(|_| ::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("UiFixedBytes exceeds {UI_FIXED_BYTES} bytes")))
    }
}
