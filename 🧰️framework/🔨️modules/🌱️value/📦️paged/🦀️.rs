//! 📦️ Schema-preserving paged text, sequence, and insertion-ordered map owners.

use crate::{DslValue, FromValue, ToValue, ValueEdit, ValueError, ValueShape, list::PagedList};
use std::fmt::{Debug, Display, Formatter, Write};

#[path = "🔣️serde/🦀️.rs"]
mod serde_owners;

pub const PAGED_BYTES_CHUNK_BYTES: usize = 4096;
pub const PAGED_UTF8_CHUNK_BYTES: usize = 1024;

impl From<crate::list::PagedListError> for ValueError {
    fn from(error: crate::list::PagedListError) -> Self {
        let kind = match error.kind {
            crate::list::PagedListRefusalKind::OwnershipLimit => crate::ValueRefusalKind::OwnershipLimit,
            crate::list::PagedListRefusalKind::AllocationFailed => crate::ValueRefusalKind::AllocationFailed,
            crate::list::PagedListRefusalKind::InvariantViolated => crate::ValueRefusalKind::InvariantViolated,
        };
        ValueError::literal(kind, error.reason)
    }
}

fn append_cold<T, const N: usize>(values: &mut PagedList<T, N>, value: T) -> Result<(), ValueError> {
    while !values.has_reserved_slot() {
        let required = values.next_allocation_bytes().map_err(ValueError::from)?;
        let progress = values.reserve_one(required).map_err(|error| ValueError::from(error.refusal()))?;
        if !progress.progressed {
            return Err(ValueError::new(crate::ValueRefusalKind::InvariantViolated, "paged owner exact cold allocation did not progress"));
        }
    }
    values.push_reserved(value).map_err(|_| ValueError::new(crate::ValueRefusalKind::InvariantViolated, "paged owner rejected its reserved slot"))
}

fn paged_index(segment: &str, length: usize) -> Result<usize, ValueError> {
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("invalid canonical array index `{segment}`")));
    }
    let index = segment.parse::<usize>().map_err(|_| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("array index `{segment}` is out of range")))?;
    (index < length).then_some(index).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("array index `{segment}` is out of range for length {length}")))
}

/// 🧩️ Arbitrary octets retained in separately admitted fixed-size backing pages.
pub struct PagedBytes<const N: usize> {
    bytes: PagedList<u8, N>,
}

impl<const N: usize> Default for PagedBytes<N> {
    fn default() -> Self {
        Self { bytes: PagedList::default() }
    }
}

impl<const N: usize> PagedBytes<N> {
    pub fn try_from_slice(value: &[u8]) -> Result<Self, ValueError> {
        if value.len() > N {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("octet value has {} bytes, exceeding paged capacity {N}", value.len())));
        }
        Ok(Self { bytes: PagedList::try_from_iter(value.iter().copied()).map_err(ValueError::from)? })
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + '_ {
        self.bytes.iter().copied()
    }

    pub fn to_vec_owner(&self) -> Vec<u8> {
        self.iter().collect()
    }

    pub fn to_vec_owner_controlled(&self, control: &mut crate::NativeEncodeControl<'_>) -> Result<Vec<u8>, ValueError> {
        control.scoped_stage(|control| {
            control.begin_stage(self.len())?;
            let mut output = control.allocate_vec(self.len())?;
            for byte in self.iter() {
                output.push(byte);
                control.step()?;
            }
            Ok(output)
        })
    }

    #[doc(hidden)]
    pub fn retained_bytes(&self) -> &PagedList<u8, N> {
        &self.bytes
    }

    #[doc(hidden)]
    pub fn from_retained_bytes(bytes: PagedList<u8, N>) -> Self {
        Self { bytes }
    }

    #[doc(hidden)]
    pub fn into_retained_bytes(mut self) -> PagedList<u8, N> {
        std::mem::take(&mut self.bytes)
    }
}

impl<const N: usize> Clone for PagedBytes<N> {
    fn clone(&self) -> Self {
        Self { bytes: self.bytes.clone() }
    }
}

impl<const N: usize> PartialEq for PagedBytes<N> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<const N: usize> Eq for PagedBytes<N> {}

impl<const N: usize> Debug for PagedBytes<N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("PagedBytes").field("len", &self.len()).finish()
    }
}

impl<const N: usize> ToValue for PagedBytes<N> {
    fn to_value(&self) -> DslValue {
        DslValue::Bytes(self.to_vec_owner())
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        if path.is_empty() { Ok(self.to_value()) } else { Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("octets have no child `{}`", path[0]))) }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        if path.is_empty() { Ok(ValueShape::Bytes { len: self.len() }) } else { Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("octets have no child `{}`", path[0]))) }
    }
}

impl<const N: usize> FromValue for PagedBytes<N> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Bytes(value) => Self::try_from_slice(&value),
            value @ DslValue::Array(_) => crate::bytes::from_value(value).and_then(|value| Self::try_from_slice(&value)),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected intrinsic octets or their JSON array projection, found {other:?}"))),
        }
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if !path.is_empty() {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("octets have no child `{}`", path[0])));
        }
        match edit {
            ValueEdit::Set(value) => {
                *self = Self::from_value(value)?;
                Ok(())
            }
            ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged octet structural edits require the retained editor cursor")),
        }
    }
}

/// 📝️ Arbitrary valid UTF-8 retained as fixed-size, character-boundary-preserving chunks.
pub trait Utf8Text {
    fn text_bytes(&self) -> usize;
    fn text_chunk_count(&self) -> usize;
    fn text_chunk(&self, index: usize) -> Option<&str>;
}

#[path = "🎮️append/🦀️.rs"]
mod append;
pub use append::PagedUtf8AppendCursor;

impl Utf8Text for str {
    fn text_bytes(&self) -> usize { self.len() }
    fn text_chunk_count(&self) -> usize { usize::from(!self.is_empty()) }
    fn text_chunk(&self, index: usize) -> Option<&str> { (index == 0 && !self.is_empty()).then_some(self) }
}

impl Utf8Text for String {
    fn text_bytes(&self) -> usize { self.len() }
    fn text_chunk_count(&self) -> usize { self.as_str().text_chunk_count() }
    fn text_chunk(&self, index: usize) -> Option<&str> { self.as_str().text_chunk(index) }
}

impl<T: Utf8Text + ?Sized> Utf8Text for &T {
    fn text_bytes(&self) -> usize { (**self).text_bytes() }
    fn text_chunk_count(&self) -> usize { (**self).text_chunk_count() }
    fn text_chunk(&self, index: usize) -> Option<&str> { (**self).text_chunk(index) }
}

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
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("UTF-8 value has {} bytes, exceeding paged capacity {N}", value.len())));
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

    pub fn try_from_str_controlled(value: &str, control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        if value.len() > N {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("UTF-8 value has {} bytes, exceeding paged capacity {N}", value.len())));
        }
        control.scoped_stage(|control| {
            control.begin_stage(value.len())?;
            let mut output = Self::default();
            let mut start = 0usize;
            while start < value.len() {
                let mut end = (start + PAGED_UTF8_CHUNK_BYTES).min(value.len());
                while !value.is_char_boundary(end) {
                    end -= 1;
                }
                while !output.chunks.has_reserved_slot() {
                    let required = output.chunks.next_allocation_bytes()?;
                    control.charge(required)?;
                    let progress = output.chunks.reserve_one(required).map_err(|error| ValueError::from(error.refusal()))?;
                    if !progress.progressed {
                        return Err(ValueError::new(crate::ValueRefusalKind::InvariantViolated, "paged UTF-8 controlled allocation did not progress"));
                    }
                }
                let copied = end - start;
                let chunk = control.copy_text(&value[start..end])?;
                output.chunks.push_reserved(chunk).map_err(|_| ValueError::new(crate::ValueRefusalKind::InvariantViolated, "paged UTF-8 controlled append rejected its admitted slot"))?;
                start = end;
                control.advance(copied)?;
            }
            output.byte_len = value.len();
            Ok(output)
        })
    }

    pub fn len(&self) -> usize {
        self.byte_len
    }

    pub fn is_empty(&self) -> bool {
        self.byte_len == 0
    }

    pub fn try_push_str(&mut self, value: &str) -> Result<(), ValueError> {
        let total = self.byte_len.checked_add(value.len()).filter(|total| *total <= N).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "paged UTF-8 append exceeds its declared capacity"))?;
        let mut start = 0;
        while start < value.len() {
            let mut end = (start + PAGED_UTF8_CHUNK_BYTES).min(value.len());
            while !value.is_char_boundary(end) { end -= 1; }
            append_cold(&mut self.chunks, value[start..end].to_owned())?;
            self.byte_len += end - start;
            start = end;
        }
        assert_eq!(self.byte_len, total);
        Ok(())
    }

    /// 🧮️ Appends caller-bounded text while charging original chunk and metadata backing before allocation.
    pub fn try_push_str_controlled(&mut self,value:&str,control:&mut crate::NativeDecodeControl<'_>)->Result<(),ValueError>{
        if value.is_empty(){return control.checkpoint();}
        self.byte_len.checked_add(value.len()).filter(|bytes|*bytes<=N).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"paged UTF-8 append exceeds its declared capacity"))?;
        if value.len()>PAGED_UTF8_CHUNK_BYTES{return Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"controlled text append requires one source chunk"));}
        control.checkpoint()?;
        while !self.chunks.has_reserved_slot(){let bytes=self.chunks.next_allocation_bytes()?;control.charge(bytes)?;self.chunks.reserve_one(bytes).map_err(|error|ValueError::from(error.refusal()))?;}
        let owned=control.copy_text(value)?;let bytes=owned.len();
        self.chunks.push_reserved(owned).map_err(|_|ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"controlled native text lost its admitted slot"))?;self.byte_len+=bytes;Ok(())
    }

    pub fn push_str(&mut self, value: &str) { self.try_push_str(value).expect("cold UTF-8 append must fit its declared capacity"); }
    pub fn clear(&mut self) { self.chunks.clear(); self.byte_len = 0; }
    pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ { self.chunks().flat_map(str::bytes) }
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ { self.chunks().flat_map(str::chars) }

    pub fn chunks(&self) -> impl DoubleEndedIterator<Item = &str> + ExactSizeIterator {
        self.chunks.iter().map(String::as_str)
    }

    pub fn eq_str(&self, other: &str) -> bool {
        self.byte_len == other.len() && self.chunks().flat_map(str::bytes).eq(other.bytes())
    }

    pub fn eq_text(&self, other: &(impl Utf8Text + ?Sized)) -> bool {
        self.byte_len == other.text_bytes() && self.chunks().flat_map(str::bytes).eq((0..other.text_chunk_count()).flat_map(|index| other.text_chunk(index).expect("native UTF-8 chunk index").bytes()))
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

    pub fn to_string_owner_controlled(&self, control: &mut crate::NativeEncodeControl<'_>) -> Result<String, ValueError> {
        control.scoped_stage(|control| {
            control.begin_stage(self.byte_len)?;
            control.charge(self.byte_len)?;
            let mut output = String::new();
            output.try_reserve_exact(self.byte_len).map_err(|_| ValueError::new(crate::ValueRefusalKind::AllocationFailed, "paged UTF-8 materialization allocation failed"))?;
            for chunk in self.chunks() {
                output.push_str(chunk);
                control.advance(chunk.len())?;
            }
            Ok(output)
        })
    }

    #[doc(hidden)]
    pub fn retained_chunks(&self) -> &PagedList<String, N> {
        &self.chunks
    }

    #[doc(hidden)]
    pub fn from_retained_chunks(chunks: PagedList<String, N>, byte_len: usize) -> Result<Self, ValueError> {
        let actual = chunks.iter().try_fold(0usize, |total, chunk| total.checked_add(chunk.len()).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "paged UTF-8 length overflow")))?;
        if actual != byte_len || actual > N || chunks.iter().any(|chunk| chunk.len() > PAGED_UTF8_CHUNK_BYTES) {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged UTF-8 chunks violate their byte or chunk envelope"));
        }
        Ok(Self { chunks, byte_len })
    }

    #[doc(hidden)]
    pub(crate) fn from_retained_chunks_cloned(chunks: PagedList<String, N>, byte_len: usize) -> Self {
        Self { chunks, byte_len }
    }

    #[doc(hidden)]
    pub fn into_retained_chunks(mut self) -> PagedList<String, N> {
        std::mem::take(&mut self.chunks)
    }
}

impl<const N: usize> Utf8Text for PagedUtf8<N> {
    fn text_bytes(&self) -> usize { self.byte_len }
    fn text_chunk_count(&self) -> usize { self.chunks.len() }
    fn text_chunk(&self, index: usize) -> Option<&str> { self.chunks.get(index).map(String::as_str) }
}

impl<const N: usize> Clone for PagedUtf8<N> {
    fn clone(&self) -> Self {
        Self { chunks: self.chunks.clone(), byte_len: self.byte_len }
    }
}

impl<const N: usize> PartialEq for PagedUtf8<N> {
    fn eq(&self, other: &Self) -> bool {
        self.byte_len == other.byte_len && self.chunks().flat_map(str::bytes).eq(other.chunks().flat_map(str::bytes))
    }
}

impl<const N: usize> Eq for PagedUtf8<N> {}

impl<const N: usize> From<&str> for PagedUtf8<N> {
    fn from(value: &str) -> Self { Self::try_from_str(value).expect("cold text construction must fit its declared capacity") }
}

impl<const N: usize> From<String> for PagedUtf8<N> {
    fn from(value: String) -> Self { Self::from(value.as_str()) }
}

impl<const N: usize> PagedUtf8<N> {
    /// 🧊️ Constructs an empty native text owner; retained writes use separately admitted chunks.
    pub fn new() -> Self { Self::default() }
}

impl<const N: usize> std::hash::Hash for PagedUtf8<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for byte in self.chunks().flat_map(str::bytes) { state.write_u8(byte); }
        state.write_u8(0xff);
    }
}

impl<const N: usize> PartialOrd for PagedUtf8<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

impl<const N: usize> Ord for PagedUtf8<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.chunks().flat_map(str::bytes).cmp(other.chunks().flat_map(str::bytes)) }
}

impl<const N: usize> PartialEq<str> for PagedUtf8<N> {
    fn eq(&self, other: &str) -> bool { self.eq_str(other) }
}

impl<const N: usize> PartialEq<&str> for PagedUtf8<N> {
    fn eq(&self, other: &&str) -> bool { self.eq_str(other) }
}

impl<const N: usize> PartialEq<String> for PagedUtf8<N> {
    fn eq(&self, other: &String) -> bool { self.eq_str(other) }
}

impl<const N: usize> PartialEq<PagedUtf8<N>> for String {
    fn eq(&self, other: &PagedUtf8<N>) -> bool { other.eq_str(self) }
}

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
        if path.is_empty() { Ok(self.to_value()) } else { Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("string has no child `{}`", path[0]))) }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        if path.is_empty() { Ok(ValueShape::String) } else { Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("string has no child `{}`", path[0]))) }
    }
}

impl<const N: usize> FromValue for PagedUtf8<N> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(value) => Self::try_from_str(&value),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if !path.is_empty() {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("string has no child `{}`", path[0])));
        }
        match edit {
            ValueEdit::Set(value) => {
                *self = Self::from_value(value)?;
                Ok(())
            }
            ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged UTF-8 structural edits require the retained editor cursor")),
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
        let DslValue::Array(items) = value else { return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an array, found {value:?}"))) };
        if items.len() > N {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("array length {} exceeds paged capacity {N}", items.len())));
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
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged list structural edits require the retained editor cursor")),
            };
        };
        let index = paged_index(segment, self.len())?;
        if rest.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *self.get_mut(index).expect("validated paged index") = T::from_value(value).map_err(|error| error.under(segment))?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged list structural edits require the retained editor cursor")),
            };
        }
        self.get_mut(index).expect("validated paged index").edit_value_at_path(rest, edit).map_err(|error| error.under(segment))
    }
}

/// 🗺️ Insertion-ordered object entries whose directory and payload pages grow independently.
pub struct PagedMap<V, const N: usize> {
    entries: PagedList<(PagedUtf8<{usize::MAX}>, V), N>,
}

impl<V, const N: usize> Default for PagedMap<V, N> {
    fn default() -> Self {
        Self { entries: PagedList::default() }
    }
}

impl<V, const N: usize> PagedMap<V, N> {
    pub fn try_from_entries(entries: impl IntoIterator<Item = (PagedUtf8<{usize::MAX}>, V)>) -> Result<Self, ValueError> {
        let mut output = Self::default();
        for (key, value) in entries {
            if output.entries.iter().any(|entry| entry.0 == key) {
                return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("duplicate paged map key `{key}`")));
            }
            append_cold(&mut output.entries, (key, value))?;
        }
        Ok(output)
    }

    pub fn try_from_fallible_entries<E: From<ValueError>>(entries: impl IntoIterator<Item = Result<(PagedUtf8<{usize::MAX}>, V), E>>) -> Result<Self, E> {
        let mut output = Self::default();
        for entry in entries {
            let (key, value) = entry?;
            if output.entries.iter().any(|entry| entry.0 == key) {
                return Err(E::from(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("duplicate paged map key `{key}`"))));
            }
            append_cold(&mut output.entries, (key, value)).map_err(E::from)?;
        }
        Ok(output)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&PagedUtf8<{usize::MAX}>, &V)> + ExactSizeIterator {
        self.entries.iter().map(|entry| (&entry.0, &entry.1))
    }

    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &PagedUtf8<{usize::MAX}>> + ExactSizeIterator {
        self.iter().map(|(key, _)| key)
    }

    pub fn values(&self) -> impl DoubleEndedIterator<Item = &V> + ExactSizeIterator {
        self.iter().map(|(_, value)| value)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> + ExactSizeIterator {
        self.entries.iter_mut().map(|entry| &mut entry.1)
    }

    pub fn get(&self, key: &(impl Utf8Text + ?Sized)) -> Option<&V> {
        self.iter().find(|(candidate, _)| candidate.eq_text(key)).map(|(_, value)| value)
    }

    pub fn get_mut(&mut self, key: &(impl Utf8Text + ?Sized)) -> Option<&mut V> {
        self.entries.iter_mut().find(|entry| entry.0.eq_text(key)).map(|entry| &mut entry.1)
    }

    /// 🗂️ Inserts a native entry through the cold collection constructor.
    pub fn try_insert(&mut self, key: PagedUtf8<{usize::MAX}>, value: V) -> Result<Option<V>, ValueError> {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.0 == key) {
            return Ok(Some(std::mem::replace(&mut entry.1, value)));
        }
        append_cold(&mut self.entries, (key, value))?;
        Ok(None)
    }

    pub fn insert(&mut self, key: impl Into<PagedUtf8<{usize::MAX}>>, value: V) -> Option<V> {
        self.try_insert(key.into(), value).expect("native paged map insertion")
    }

    pub fn remove(&mut self, key: &(impl Utf8Text + ?Sized)) -> Option<V> {
        let position = self.entries.iter().position(|entry| entry.0.eq_text(key))?;
        Some(self.entries.remove(position).1)
    }

    pub fn last_key_value(&self) -> Option<(&PagedUtf8<{usize::MAX}>, &V)> {
        self.iter().max_by(|left, right| left.0.cmp(right.0))
    }

    pub fn pop_last(&mut self) -> Option<(PagedUtf8<{usize::MAX}>, V)> {
        let position = self.entries.iter().enumerate().max_by(|left, right| left.1.0.cmp(&right.1.0))?.0;
        Some(self.entries.remove(position))
    }

    pub fn entry_at(&self, index: usize) -> Option<(&PagedUtf8<{usize::MAX}>, &V)> {
        self.entries.get(index).map(|entry| (&entry.0, &entry.1))
    }

    #[doc(hidden)]
    pub fn retained_entries(&self) -> &PagedList<(PagedUtf8<{usize::MAX}>, V), N> {
        &self.entries
    }

    #[doc(hidden)]
    pub fn from_retained_entries(entries: PagedList<(PagedUtf8<{usize::MAX}>, V), N>) -> Result<Self, ValueError> {
        let mut keys = std::collections::HashSet::with_capacity(entries.len());
        for (key, _) in entries.iter() {
            if !keys.insert(key) {
                return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("duplicate paged map key `{key}`")));
            }
        }
        Ok(Self { entries })
    }

    /// 🗂️ Adopts sorted unique admitted entries without allocating duplicate-key scratch.
    pub fn from_sorted_retained_entries_controlled(entries: PagedList<(PagedUtf8<{usize::MAX}>, V), N>, control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| {
            let total = entries.iter().try_fold(entries.len(), |total, (key, _)| total.checked_add(key.len()).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::WorkLimit, "paged map ordered validation workload overflow")))?;
            control.begin_stage(total)?;
            let mut previous: Option<&PagedUtf8<{usize::MAX}>> = None;
            for (key, _) in entries.iter() {
                if let Some(prior) = previous {
                    let mut ordering = std::cmp::Ordering::Equal;
                    for (left, right) in prior.chunks().flat_map(str::bytes).zip(key.chunks().flat_map(str::bytes)) {
                        control.step()?;
                        ordering = left.cmp(&right);
                        if !ordering.is_eq() { break; }
                    }
                    if ordering.is_eq() { ordering = prior.len().cmp(&key.len()); }
                    if !ordering.is_lt() { return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged map entries must have unique ascending keys")); }
                }
                previous = Some(key);
                control.step()?;
            }
            Ok(Self { entries })
        })
    }

    #[doc(hidden)]
    pub(crate) fn from_retained_entries_cloned(entries: PagedList<(PagedUtf8<{usize::MAX}>, V), N>) -> Self {
        Self { entries }
    }

    #[doc(hidden)]
    pub fn into_retained_entries(mut self) -> PagedList<(PagedUtf8<{usize::MAX}>, V), N> {
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
        DslValue::Object(self.iter().map(|(key, value)| (key.to_string_owner(), value.to_value())).collect())
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        self.get(*segment).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?.value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Object { len: self.len() }) };
        self.get(*segment).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?.value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        if path.is_empty() {
            return self.entry_at(index).map(|(key, _)| key.to_string_owner()).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {}", self.len())));
        }
        let (segment, rest) = path.split_first().expect("non-empty path checked above");
        self.get(*segment).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?.value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}

impl<V: FromValue, const N: usize> FromValue for PagedMap<V, N> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an object, found {value:?}"))) };
        if entries.len() > N {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object length {} exceeds paged capacity {N}", entries.len())));
        }
        let mut keys = std::collections::HashSet::with_capacity(entries.len());
        let mut output = PagedList::default();
        for (key, value) in entries {
            if !keys.insert(key.clone()) {
                return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("duplicate object key `{key}`")));
            }
            append_cold(&mut output, (PagedUtf8::try_from_str(&key)?, V::from_value(value).map_err(|error| error.under(key))?))?;
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
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged map structural edits require the retained editor cursor")),
            };
        };
        let target = self.get_mut(*segment).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
        if rest.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *target = V::from_value(value).map_err(|error| error.under(segment))?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged map structural edits require the retained editor cursor")),
            };
        }
        target.edit_value_at_path(rest, edit).map_err(|error| error.under(segment))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
