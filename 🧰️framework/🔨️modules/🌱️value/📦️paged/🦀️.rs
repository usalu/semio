//! 📦️ Schema-preserving paged text, sequence, and insertion-ordered map owners.

use crate::{DslValue, FromValue, ToValue, ValueEdit, ValueError, ValueShape, list::PagedList};
use std::fmt::{Debug, Display, Formatter, Write};

pub const PAGED_UTF8_CHUNK_BYTES: usize = 1024;

fn append_cold<T, const N: usize>(values: &mut PagedList<T, N>, value: T) -> Result<(), ValueError> {
    while !values.has_reserved_slot() {
        let required = values.next_allocation_bytes().map_err(ValueError::new)?;
        let progress = values.reserve_one(required).map_err(|error| ValueError::new(error.reason))?;
        if !progress.progressed {
            return Err(ValueError::new("paged owner exact cold allocation did not progress"));
        }
    }
    values.push_reserved(value).map_err(|_| ValueError::new("paged owner rejected its reserved slot"))
}

fn paged_index(segment: &str, length: usize) -> Result<usize, ValueError> {
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ValueError::new(format!("invalid canonical array index `{segment}`")));
    }
    let index = segment.parse::<usize>().map_err(|_| ValueError::new(format!("array index `{segment}` is out of range")))?;
    (index < length).then_some(index).ok_or_else(|| ValueError::new(format!("array index `{segment}` is out of range for length {length}")))
}

/// 📝️ Arbitrary valid UTF-8 retained as fixed-size, character-boundary-preserving chunks.
pub struct PagedUtf8<const N: usize> {
    chunks: PagedList<String, N>,
    byte_len: usize,
}

impl<const N: usize> Default for PagedUtf8<N> {
    fn default() -> Self {
        Self { chunks: PagedList::default(), byte_len: 0 }
    }
}

impl<const N: usize> PagedUtf8<N> {
    pub fn try_from_str(value: &str) -> Result<Self, ValueError> {
        if value.len() > N {
            return Err(ValueError::new(format!("UTF-8 value has {} bytes, exceeding paged capacity {N}", value.len())));
        }
        let mut output = Self::default();
        let mut start = 0usize;
        while start < value.len() {
            let mut end = (start + PAGED_UTF8_CHUNK_BYTES).min(value.len());
            while !value.is_char_boundary(end) {
                end -= 1;
            }
            append_cold(&mut output.chunks, value[start..end].to_owned())?;
            start = end;
        }
        output.byte_len = value.len();
        Ok(output)
    }

    pub fn len(&self) -> usize {
        self.byte_len
    }

    pub fn is_empty(&self) -> bool {
        self.byte_len == 0
    }

    pub fn chunks(&self) -> impl DoubleEndedIterator<Item = &str> + ExactSizeIterator {
        self.chunks.iter().map(String::as_str)
    }

    pub fn write_to(&self, output: &mut impl Write) -> std::fmt::Result {
        for chunk in self.chunks() {
            output.write_str(chunk)?;
        }
        Ok(())
    }

    pub fn to_string_owner(&self) -> String {
        let mut output = String::with_capacity(self.byte_len);
        self.write_to(&mut output).expect("String formatting is infallible");
        output
    }

    #[doc(hidden)]
    pub fn retained_chunks(&self) -> &PagedList<String, N> {
        &self.chunks
    }

    #[doc(hidden)]
    pub fn from_retained_chunks(chunks: PagedList<String, N>, byte_len: usize) -> Result<Self, ValueError> {
        let actual = chunks.iter().try_fold(0usize, |total, chunk| total.checked_add(chunk.len()).ok_or_else(|| ValueError::new("paged UTF-8 length overflow")))?;
        if actual != byte_len || actual > N || chunks.iter().any(|chunk| chunk.len() > PAGED_UTF8_CHUNK_BYTES) {
            return Err(ValueError::new("paged UTF-8 chunks violate their byte or chunk envelope"));
        }
        Ok(Self { chunks, byte_len })
    }

    #[doc(hidden)]
    pub fn into_retained_chunks(mut self) -> PagedList<String, N> {
        std::mem::take(&mut self.chunks)
    }
}

impl<const N: usize> Clone for PagedUtf8<N> {
    fn clone(&self) -> Self {
        Self { chunks: self.chunks.clone(), byte_len: self.byte_len }
    }
}

impl<const N: usize> PartialEq for PagedUtf8<N> {
    fn eq(&self, other: &Self) -> bool {
        self.byte_len == other.byte_len && self.chunks().eq(other.chunks())
    }
}

impl<const N: usize> Eq for PagedUtf8<N> {}

impl<const N: usize> Debug for PagedUtf8<N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple("PagedUtf8").field(&self.to_string_owner()).finish()
    }
}

impl<const N: usize> Display for PagedUtf8<N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_to(formatter)
    }
}

impl<const N: usize> ToValue for PagedUtf8<N> {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string_owner())
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        if path.is_empty() { Ok(self.to_value()) } else { Err(ValueError::new(format!("string has no child `{}`", path[0]))) }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        if path.is_empty() { Ok(ValueShape::String) } else { Err(ValueError::new(format!("string has no child `{}`", path[0]))) }
    }
}

impl<const N: usize> FromValue for PagedUtf8<N> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(value) => Self::try_from_str(&value),
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if !path.is_empty() {
            return Err(ValueError::new(format!("string has no child `{}`", path[0])));
        }
        match edit {
            ValueEdit::Set(value) => {
                *self = Self::from_value(value)?;
                Ok(())
            }
            ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new("paged UTF-8 structural edits require the retained editor cursor")),
        }
    }
}

impl<T: ToValue, const N: usize> ToValue for PagedList<T, N> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let index = paged_index(segment, self.len())?;
        self.get(index).expect("validated paged index").value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: self.len() }) };
        let index = paged_index(segment, self.len())?;
        self.get(index).expect("validated paged index").value_shape_at_path(rest).map_err(|error| error.under(segment))
    }
}

impl<T: FromValue, const N: usize> FromValue for PagedList<T, N> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Array(items) = value else { return Err(ValueError::new(format!("expected an array, found {value:?}"))) };
        if items.len() > N {
            return Err(ValueError::new(format!("array length {} exceeds paged capacity {N}", items.len())));
        }
        let mut output = Self::default();
        for (index, item) in items.into_iter().enumerate() {
            append_cold(&mut output, T::from_value(item).map_err(|error| error.under(index))?)?;
        }
        Ok(output)
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = Self::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new("paged list structural edits require the retained editor cursor")),
            };
        };
        let index = paged_index(segment, self.len())?;
        if rest.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *self.get_mut(index).expect("validated paged index") = T::from_value(value).map_err(|error| error.under(segment))?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new("paged list structural edits require the retained editor cursor")),
            };
        }
        self.get_mut(index).expect("validated paged index").edit_value_at_path(rest, edit).map_err(|error| error.under(segment))
    }
}

/// 🗺️ Insertion-ordered object entries whose directory and payload pages grow independently.
pub struct PagedMap<V, const N: usize> {
    entries: PagedList<(String, V), N>,
}

impl<V, const N: usize> Default for PagedMap<V, N> {
    fn default() -> Self {
        Self { entries: PagedList::default() }
    }
}

impl<V, const N: usize> PagedMap<V, N> {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&String, &V)> + ExactSizeIterator {
        self.entries.iter().map(|entry| (&entry.0, &entry.1))
    }

    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &String> + ExactSizeIterator {
        self.iter().map(|(key, _)| key)
    }

    pub fn values(&self) -> impl DoubleEndedIterator<Item = &V> + ExactSizeIterator {
        self.iter().map(|(_, value)| value)
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.iter().find(|(candidate, _)| candidate.as_str() == key).map(|(_, value)| value)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        self.entries.iter_mut().find(|entry| entry.0 == key).map(|entry| &mut entry.1)
    }

    pub fn entry_at(&self, index: usize) -> Option<(&String, &V)> {
        self.entries.get(index).map(|entry| (&entry.0, &entry.1))
    }

    #[doc(hidden)]
    pub fn retained_entries(&self) -> &PagedList<(String, V), N> {
        &self.entries
    }

    #[doc(hidden)]
    pub fn from_retained_entries(entries: PagedList<(String, V), N>) -> Result<Self, ValueError> {
        let mut keys = std::collections::HashSet::with_capacity(entries.len());
        for (key, _) in entries.iter() {
            if !keys.insert(key.as_str()) {
                return Err(ValueError::new(format!("duplicate paged map key `{key}`")));
            }
        }
        Ok(Self { entries })
    }

    #[doc(hidden)]
    pub fn into_retained_entries(mut self) -> PagedList<(String, V), N> {
        std::mem::take(&mut self.entries)
    }
}

impl<V: Clone, const N: usize> Clone for PagedMap<V, N> {
    fn clone(&self) -> Self {
        Self { entries: self.entries.clone() }
    }
}

impl<V: PartialEq, const N: usize> PartialEq for PagedMap<V, N> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<V: Eq, const N: usize> Eq for PagedMap<V, N> {}

impl<V: Debug, const N: usize> Debug for PagedMap<V, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<V: ToValue, const N: usize> ToValue for PagedMap<V, N> {
    fn to_value(&self) -> DslValue {
        DslValue::Object(self.iter().map(|(key, value)| (key.clone(), value.to_value())).collect())
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        self.get(segment).ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?.value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Object { len: self.len() }) };
        self.get(segment).ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?.value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        if path.is_empty() {
            return self.entry_at(index).map(|(key, _)| key.clone()).ok_or_else(|| ValueError::new(format!("object key index {index} is out of range for length {}", self.len())));
        }
        let (segment, rest) = path.split_first().expect("non-empty path checked above");
        self.get(segment).ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?.value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}

impl<V: FromValue, const N: usize> FromValue for PagedMap<V, N> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { return Err(ValueError::new(format!("expected an object, found {value:?}"))) };
        if entries.len() > N {
            return Err(ValueError::new(format!("object length {} exceeds paged capacity {N}", entries.len())));
        }
        let mut keys = std::collections::HashSet::with_capacity(entries.len());
        let mut output = PagedList::default();
        for (key, value) in entries {
            if !keys.insert(key.clone()) {
                return Err(ValueError::new(format!("duplicate object key `{key}`")));
            }
            append_cold(&mut output, (key.clone(), V::from_value(value).map_err(|error| error.under(key))?))?;
        }
        Self::from_retained_entries(output)
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = Self::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new("paged map structural edits require the retained editor cursor")),
            };
        };
        let target = self.get_mut(segment).ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
        if rest.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *target = V::from_value(value).map_err(|error| error.under(segment))?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new("paged map structural edits require the retained editor cursor")),
            };
        }
        target.edit_value_at_path(rest, edit).map_err(|error| error.under(segment))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
