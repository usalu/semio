//! 🔁️ `ToValue`/`FromValue` — the first-party analog of `serde::Serialize`/
//! `serde::de::DeserializeOwned`, over [`super::DslValue`] instead of a generic visitor.
//!
//! Exists to break the forced `serde` dependency `MutationDiff`/`Mutation` (`crate::mutation`)
//! used to bake into every implementor: those traits now bound on `ToValue + FromValue`, so a
//! plugin implementing them never needs to depend on `serde`. See
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
//! 🔍️research/📓️serde-replacement-surface.md` for the design rationale and the older
//! `super::dsl_value_serde` bridge this supersedes for plugin-facing code (that bridge itself
//! requires `serde::Serialize`/`serde::de::DeserializeOwned` on its input, so it cannot be the
//! plugin-facing seam — it stays only for framework-internal callers that still speak serde).
//!
//! `#[derive(ToValue, FromValue)]` (`semio-framework-value-derive`, `🌱️value/✨️derive`) implements
//! both traits for a `#[value(...)]`-annotated struct/enum; the scalar/container leaves below are
//! the hand-written base cases every derived impl bottoms out on.

use super::{DslValue, Number};

//#region 🔖️Traits
/// @emoji 🔁️ Converts `self` into a [`DslValue`] tree. First-party analog of `serde::Serialize`.
pub trait ToValue {
    fn to_value(&self) -> DslValue;

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        self.to_value().value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        Ok(ValueShape::of(&self.value_at_path(path)?))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        match self.value_at_path(path)? {
            DslValue::Object(entries) => entries.get(index).map(|(key, _)| key.clone()).ok_or_else(|| ValueError::new(format!("object key index {index} is out of range for length {}", entries.len()))),
            other => Err(ValueError::new(format!("expected an object, found {other:?}"))),
        }
    }
}

/// @emoji 🔁️ Hydrates `Self` from a [`DslValue`] tree. First-party analog of
/// `serde::de::DeserializeOwned`.
pub trait FromValue: Sized {
    fn from_value(value: DslValue) -> Result<Self, ValueError>;

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if !path.is_empty() {
            return Err(ValueError::new(format!("value has no child `{}`", path[0])));
        }
        match edit {
            ValueEdit::Set(value) => {
                *self = <Self as FromValue>::from_value(value)?;
                Ok(())
            }
            ValueEdit::Insert(_) => Err(ValueError::new("cannot insert at the value root")),
            ValueEdit::Remove => Err(ValueError::new("cannot remove the value root")),
        }
    }

}

/// @emoji ✏️ One structural edit over already-decoded value path segments.
#[derive(Clone, Debug, PartialEq)]
pub enum ValueEdit {
    Set(DslValue),
    Insert(DslValue),
    Remove,
}

/// @emoji 📐 Describes a value at a typed path without materializing its children.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueShape {
    Null,
    Bool,
    Number,
    String,
    Array { len: usize },
    Object { len: usize },
}

impl ValueShape {
    pub fn of(value: &DslValue) -> Self {
        match value {
            DslValue::Null => Self::Null,
            DslValue::Bool(_) => Self::Bool,
            DslValue::Number(_) => Self::Number,
            DslValue::String(_) => Self::String,
            DslValue::Array(items) => Self::Array { len: items.len() },
            DslValue::Object(entries) => Self::Object { len: entries.len() },
        }
    }
}

/// @emoji 🚨️ A decode failure, with a dotted field/index/variant path prefixed as the caller
/// unwinds (see [`ValueError::under`]) so a nested failure reads as `"steps.3.title: ..."`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueError(pub String);

impl std::fmt::Display for ValueError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ValueError {}

impl ValueError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    /// 🪆️ Prefixes one outer path segment (a field name, array index, or enum variant) onto an
    /// inner decode failure — called bottom-up as `from_value` unwinds a nested struct/array.
    pub fn under(self, segment: impl std::fmt::Display) -> Self {
        Self(format!("{segment}.{}", self.0))
    }
}

fn path_index(segment: &str, length: usize, insert: bool) -> Result<usize, ValueError> {
    if insert && segment == "-" {
        return Ok(length);
    }
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ValueError::new(format!("invalid canonical array index `{segment}`")));
    }
    let index = segment.parse::<usize>().map_err(|_| ValueError::new(format!("array index `{segment}` is out of range")))?;
    let admitted = if insert { index <= length } else { index < length };
    admitted.then_some(index).ok_or_else(|| ValueError::new(format!("array index `{segment}` is out of range for length {length}")))
}

fn object_entry_index(entries: &[(String, DslValue)], key: &str) -> Result<Option<usize>, ValueError> {
    let mut matches = entries.iter().enumerate().filter(|(_, (candidate, _))| candidate == key).map(|(index, _)| index);
    let first = matches.next();
    if matches.next().is_some() {
        return Err(ValueError::new(format!("duplicate object key `{key}`")));
    }
    Ok(first)
}
//#endregion 🔖️Traits

//#region 🔖️Scalars
/// @emoji 🔢️ Unsigned integer scalars — round-trip through [`Number::UInt`] so a wire-visible
/// `u64` field (e.g. `ttl_secs`) encodes as bare `3600`, never `3600.0`. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️dslvalue-integer-fidelity.md`.
macro_rules! impl_uint_codec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ToValue for $ty {
                fn to_value(&self) -> DslValue {
                    DslValue::Number(Number::UInt(*self as u64))
                }
            }
            impl FromValue for $ty {
                fn from_value(value: DslValue) -> Result<Self, ValueError> {
                    match value {
                        DslValue::Number(number) => number.as_u64().and_then(|n| <$ty>::try_from(n).ok())
                            .ok_or_else(|| ValueError::new(format!("expected an exact {} integer, found {number:?}", stringify!($ty)))),
                        other => Err(ValueError::new(format!("expected a number, found {other:?}"))),
                    }
                }
            }
        )+
    };
}
impl_uint_codec!(u8, u16, u32, u64, usize);

/// 🔢️ Signed integer scalars — round-trip through [`Number::Int`], same fidelity contract as the
/// unsigned family above.
macro_rules! impl_int_codec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ToValue for $ty {
                fn to_value(&self) -> DslValue {
                    DslValue::Number(Number::Int(*self as i64))
                }
            }
            impl FromValue for $ty {
                fn from_value(value: DslValue) -> Result<Self, ValueError> {
                    match value {
                        DslValue::Number(number) => number.as_i64().and_then(|n| <$ty>::try_from(n).ok())
                            .ok_or_else(|| ValueError::new(format!("expected an exact {} integer, found {number:?}", stringify!($ty)))),
                        other => Err(ValueError::new(format!("expected a number, found {other:?}"))),
                    }
                }
            }
        )+
    };
}
impl_int_codec!(i8, i16, i32, i64, isize);

/// 🔢️ Fractional scalars — always encode/decode through [`Number::Float`]; a whole float keeps its
/// explicit `.0` on the wire so it never collapses onto its integer twin.
macro_rules! impl_float_codec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ToValue for $ty {
                fn to_value(&self) -> DslValue {
                    DslValue::Number(Number::Float(*self as f64))
                }
            }
            impl FromValue for $ty {
                fn from_value(value: DslValue) -> Result<Self, ValueError> {
                    match value {
                        DslValue::Number(n) => Ok(n.as_f64() as $ty),
                        other => Err(ValueError::new(format!("expected a number, found {other:?}"))),
                    }
                }
            }
        )+
    };
}
impl_float_codec!(f64);

impl ToValue for f32 {
    fn to_value(&self) -> DslValue {
        let shortest = format!("{self:e}");
        let precision = shortest.split('e').next().unwrap().bytes().filter(u8::is_ascii_digit).count().saturating_sub(1);
        DslValue::Number(Number::Float(format!("{self:.precision$e}").parse().expect("f32 decimal text is a valid f64")))
    }
}

impl FromValue for f32 {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Number(number) => Ok(number.as_f64() as f32),
            other => Err(ValueError::new(format!("expected a number, found {other:?}"))),
        }
    }
}

impl ToValue for bool {
    fn to_value(&self) -> DslValue {
        DslValue::Bool(*self)
    }
}
impl FromValue for bool {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Bool(b) => Ok(b),
            other => Err(ValueError::new(format!("expected a bool, found {other:?}"))),
        }
    }
}

impl ToValue for String {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.clone())
    }
}
impl FromValue for String {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(s),
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🗂️ A path renders as its UTF-8 string form (lossy on the encode side — this is a local-only
/// config value, never a content-addressed hash input, so exact non-Unicode byte round-tripping
/// is not required the way `serde`'s own `Path`/`PathBuf` impl demands it).
impl ToValue for std::path::PathBuf {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string_lossy().into_owned())
    }
}
impl FromValue for std::path::PathBuf {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(std::path::PathBuf::from(s)),
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🌉️ No `FromValue` counterpart — decoding always needs owned data, `String`'s impl covers it.
impl ToValue for &str {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string())
    }
}

impl ToValue for () {
    fn to_value(&self) -> DslValue {
        DslValue::Null
    }
}
impl FromValue for () {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(()),
            other => Err(ValueError::new(format!("expected null, found {other:?}"))),
        }
    }
}
//#endregion 🔖️Scalars

//#region 🔖️Containers
impl<T: ToValue> ToValue for Option<T> {
    fn to_value(&self) -> DslValue {
        match self {
            Some(value) => value.to_value(),
            None => DslValue::Null,
        }
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        match (path.is_empty(), self) {
            (true, _) => Ok(self.to_value()),
            (false, Some(value)) => value.value_at_path(path),
            (false, None) => Err(ValueError::new(format!("null has no child `{}`", path[0]))),
        }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        match (path.is_empty(), self) {
            (true, None) => Ok(ValueShape::Null),
            (true, Some(value)) => value.value_shape_at_path(path),
            (false, Some(value)) => value.value_shape_at_path(path),
            (false, None) => Err(ValueError::new(format!("null has no child `{}`", path[0]))),
        }
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        self.as_ref().ok_or_else(|| ValueError::new("expected an object, found null"))?.value_key_at_path(path, index)
    }
}
impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(None),
            other => T::from_value(other).map(Some),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if path.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(value) if self.is_none() => {
                    *self = Some(T::from_value(value)?);
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert an already-present optional value")),
                ValueEdit::Remove if self.is_some() => {
                    *self = None;
                    Ok(())
                }
                ValueEdit::Remove => Err(ValueError::new("cannot remove an absent optional value")),
            };
        }
        self.as_mut().ok_or_else(|| ValueError::new(format!("null has no child `{}`", path[0])))?.edit_value_at_path(path, edit)
    }
}

impl<T: ToValue> ToValue for [T] {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let index = path_index(segment, self.len(), false)?;
        self[index].value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: self.len() }) };
        let index = path_index(segment, self.len(), false)?;
        self[index].value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new("expected an object, found an array"))?;
        let item_index = path_index(segment, self.len(), false)?;
        self[item_index].value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}
impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> DslValue {
        self.as_slice().to_value()
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        self.as_slice().value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        self.as_slice().value_shape_at_path(path)
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        self.as_slice().value_key_at_path(path, index)
    }
}
impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(format!("expected an array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert at the array root")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove the array root")),
            };
        };
        if !rest.is_empty() {
            let index = path_index(segment, self.len(), false)?;
            return self[index].edit_value_at_path(rest, edit).map_err(|error| error.under(segment));
        }
        match edit {
            ValueEdit::Set(value) => {
                let index = path_index(segment, self.len(), false)?;
                self[index] = T::from_value(value).map_err(|error| error.under(segment))?;
                Ok(())
            }
            ValueEdit::Insert(value) => {
                let index = path_index(segment, self.len(), true)?;
                let value = T::from_value(value).map_err(|error| error.under(segment))?;
                self.insert(index, value);
                Ok(())
            }
            ValueEdit::Remove => {
                let index = path_index(segment, self.len(), false)?;
                self.remove(index);
                Ok(())
            }
        }
    }
}

/// 📐️ Same plain-JSON-array wire shape as `Vec<T>` — a `VecDeque` field (e.g. `semio-framework-
/// actor`'s per-lane mailbox rings) round-trips identically to its `Vec` twin, just ring-backed in memory.
impl<T: ToValue> ToValue for std::collections::VecDeque<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let index = path_index(segment, self.len(), false)?;
        self[index].value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: self.len() }) };
        let index = path_index(segment, self.len(), false)?;
        self[index].value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new("expected an object, found an array"))?;
        let item_index = path_index(segment, self.len(), false)?;
        self[item_index].value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}
impl<T: FromValue> FromValue for std::collections::VecDeque<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(format!("expected an array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert at the array root")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove the array root")),
            };
        };
        if !rest.is_empty() {
            let index = path_index(segment, self.len(), false)?;
            return self[index].edit_value_at_path(rest, edit).map_err(|error| error.under(segment));
        }
        match edit {
            ValueEdit::Set(value) => {
                let index = path_index(segment, self.len(), false)?;
                self[index] = T::from_value(value).map_err(|error| error.under(segment))?;
            }
            ValueEdit::Insert(value) => {
                let index = path_index(segment, self.len(), true)?;
                let value = T::from_value(value).map_err(|error| error.under(segment))?;
                self.insert(index, value);
            }
            ValueEdit::Remove => {
                let index = path_index(segment, self.len(), false)?;
                self.remove(index);
            }
        }
        Ok(())
    }
}

/// 📐️ A fixed-size array encodes exactly like a `Vec<T>` (a plain JSON array) — the length is
/// carried by `N`, not the wire, so decode rejects any array whose length doesn't match `N`
/// (matches what a fixed-size `[T; N]` field means: this many, no more, no fewer).
impl<T: ToValue, const N: usize> ToValue for [T; N] {
    fn to_value(&self) -> DslValue {
        self.as_slice().to_value()
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        self.as_slice().value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        self.as_slice().value_shape_at_path(path)
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        self.as_slice().value_key_at_path(path, index)
    }
}
impl<T: FromValue, const N: usize> FromValue for [T; N] {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => {
                let found = items.len();
                let decoded: Vec<T> = items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect::<Result<_, _>>()?;
                decoded.try_into().map_err(|_| ValueError::new(format!("expected an array of length {N}, found {found}")))
            }
            other => Err(ValueError::new(format!("expected an array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert into a fixed array")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove a fixed array")),
            };
        };
        let index = path_index(segment, N, false)?;
        match (rest.is_empty(), edit) {
            (true, ValueEdit::Set(value)) => {
                self[index] = T::from_value(value).map_err(|error| error.under(segment))?;
                Ok(())
            }
            (true, ValueEdit::Insert(_)) => Err(ValueError::new("cannot insert into a fixed array")),
            (true, ValueEdit::Remove) => Err(ValueError::new("cannot remove a fixed array")),
            (false, edit) => self[index].edit_value_at_path(rest, edit).map_err(|error| error.under(segment)),
        }
    }
}

impl<T: ToValue> ToValue for Box<T> {
    fn to_value(&self) -> DslValue {
        (**self).to_value()
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        (**self).value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        (**self).value_shape_at_path(path)
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        (**self).value_key_at_path(path, index)
    }
}
impl<T: FromValue> FromValue for Box<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        T::from_value(value).map(Box::new)
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if path.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = Box::new(T::from_value(value)?);
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert at the boxed value root")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove the boxed value root")),
            };
        }
        (**self).edit_value_at_path(path, edit)
    }
}

/// 🌳️ A `BTreeSet<T>` encodes exactly like a `Vec<T>` (a plain JSON array), in the set's own sorted
/// iteration order — matches `serde`'s own `BTreeSet` representation.
impl<T: ToValue + Ord> ToValue for std::collections::BTreeSet<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let index = path_index(segment, self.len(), false)?;
        self.iter().nth(index).expect("validated set index").value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: self.len() }) };
        let index = path_index(segment, self.len(), false)?;
        self.iter().nth(index).expect("validated set index").value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new("expected an object, found an array"))?;
        let item_index = path_index(segment, self.len(), false)?;
        self.iter().nth(item_index).expect("validated set index").value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}
impl<T: FromValue + Ord> FromValue for std::collections::BTreeSet<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(format!("expected an array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert at the set root")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove the set root")),
            };
        };
        if !rest.is_empty() {
            return Err(ValueError::new("nested edits inside ordered set members require replacing the complete member"));
        }
        let mut values: Vec<T> = std::mem::take(self).into_iter().collect();
        let result = (|| -> Result<(), ValueError> {
            match edit {
                ValueEdit::Set(value) => {
                    let index = path_index(segment, values.len(), false)?;
                    let replacement = T::from_value(value).map_err(|error| error.under(segment))?;
                    let ordered = index.checked_sub(1).is_none_or(|previous| values[previous] < replacement) && values.get(index + 1).is_none_or(|next| replacement < *next);
                    if !ordered {
                        return Err(ValueError::new("ordered set replacement changes the member's wire index"));
                    }
                    values[index] = replacement;
                    Ok(())
                }
                ValueEdit::Insert(value) => {
                    let index = path_index(segment, values.len(), true)?;
                    let value = T::from_value(value).map_err(|error| error.under(segment))?;
                    let ordered = index.checked_sub(1).is_none_or(|previous| values[previous] < value) && values.get(index).is_none_or(|next| value < *next);
                    if !ordered {
                        return Err(ValueError::new("ordered set insertion index does not match the member's sort order"));
                    }
                    values.insert(index, value);
                    Ok(())
                }
                ValueEdit::Remove => {
                    let index = path_index(segment, values.len(), false)?;
                    values.remove(index);
                    Ok(())
                }
            }
        })();
        *self = values.into_iter().collect();
        result
    }
}

impl<T: ToValue> ToValue for std::collections::BTreeMap<String, T> {
    fn to_value(&self) -> DslValue {
        DslValue::object(self.iter().map(|(key, value)| (key.clone(), value.to_value())))
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((key, rest)) = path.split_first() else { return Ok(self.to_value()) };
        self.get(*key).ok_or_else(|| ValueError::new(format!("missing object key `{key}`")))?.value_at_path(rest).map_err(|error| error.under(key))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((key, rest)) = path.split_first() else { return Ok(ValueShape::Object { len: self.len() }) };
        self.get(*key).ok_or_else(|| ValueError::new(format!("missing object key `{key}`")))?.value_shape_at_path(rest).map_err(|error| error.under(key))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        if path.is_empty() {
            return self.keys().nth(index).cloned().ok_or_else(|| ValueError::new(format!("object key index {index} is out of range for length {}", self.len())));
        }
        let (key, rest) = path.split_first().expect("non-empty path checked above");
        self.get(*key).ok_or_else(|| ValueError::new(format!("missing object key `{key}`")))?.value_key_at_path(rest, index).map_err(|error| error.under(key))
    }
}
impl<T: FromValue> FromValue for std::collections::BTreeMap<String, T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Object(entries) => entries.into_iter().map(|(key, value)| T::from_value(value).map(|value| (key.clone(), value)).map_err(|error| error.under(key))).collect(),
            other => Err(ValueError::new(format!("expected an object, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((key, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert at the map root")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove the map root")),
            };
        };
        if !rest.is_empty() {
            return self.get_mut(*key).ok_or_else(|| ValueError::new(format!("missing object key `{key}`")))?.edit_value_at_path(rest, edit).map_err(|error| error.under(key));
        }
        match edit {
            ValueEdit::Set(value) => {
                let replacement = T::from_value(value).map_err(|error| error.under(key))?;
                let target = self.get_mut(*key).ok_or_else(|| ValueError::new(format!("missing object key `{key}`")))?;
                *target = replacement;
            }
            ValueEdit::Insert(value) => {
                if self.contains_key(*key) {
                    return Err(ValueError::new(format!("object key `{key}` already exists")));
                }
                self.insert((*key).to_owned(), T::from_value(value).map_err(|error| error.under(key))?);
            }
            ValueEdit::Remove => {
                self.remove(*key).ok_or_else(|| ValueError::new(format!("missing object key `{key}`")))?;
            }
        }
        Ok(())
    }
}

impl ToValue for DslValue {
    fn to_value(&self) -> DslValue {
        self.clone()
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.clone()) };
        match self {
            DslValue::Array(items) => {
                let index = path_index(segment, items.len(), false)?;
                items[index].value_at_path(rest).map_err(|error| error.under(segment))
            }
            DslValue::Object(entries) => {
                let index = object_entry_index(entries, segment)?.ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
                entries[index].1.value_at_path(rest).map_err(|error| error.under(segment))
            }
            _ => Err(ValueError::new(format!("value has no child `{segment}`"))),
        }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::of(self)) };
        match self {
            DslValue::Array(items) => {
                let index = path_index(segment, items.len(), false)?;
                items[index].value_shape_at_path(rest).map_err(|error| error.under(segment))
            }
            DslValue::Object(entries) => {
                let index = object_entry_index(entries, segment)?.ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
                entries[index].1.value_shape_at_path(rest).map_err(|error| error.under(segment))
            }
            _ => Err(ValueError::new(format!("value has no child `{segment}`"))),
        }
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        if path.is_empty() {
            let DslValue::Object(entries) = self else { return Err(ValueError::new(format!("expected an object, found {self:?}"))) };
            return entries.get(index).map(|(key, _)| key.clone()).ok_or_else(|| ValueError::new(format!("object key index {index} is out of range for length {}", entries.len())));
        }
        let (segment, rest) = path.split_first().expect("non-empty path checked above");
        match self {
            DslValue::Array(items) => {
                let item_index = path_index(segment, items.len(), false)?;
                items[item_index].value_key_at_path(rest, index).map_err(|error| error.under(segment))
            }
            DslValue::Object(entries) => {
                let entry_index = object_entry_index(entries, segment)?.ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
                entries[entry_index].1.value_key_at_path(rest, index).map_err(|error| error.under(segment))
            }
            _ => Err(ValueError::new(format!("value has no child `{segment}`"))),
        }
    }
}
impl FromValue for DslValue {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(value)
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = value;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert at the value root")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove the value root")),
            };
        };
        match self {
            DslValue::Array(items) => {
                if !rest.is_empty() {
                    let index = path_index(segment, items.len(), false)?;
                    return items[index].edit_value_at_path(rest, edit).map_err(|error| error.under(segment));
                }
                match edit {
                    ValueEdit::Set(value) => {
                        let index = path_index(segment, items.len(), false)?;
                        items[index] = value;
                    }
                    ValueEdit::Insert(value) => {
                        let index = path_index(segment, items.len(), true)?;
                        items.insert(index, value);
                    }
                    ValueEdit::Remove => {
                        let index = path_index(segment, items.len(), false)?;
                        items.remove(index);
                    }
                }
                Ok(())
            }
            DslValue::Object(entries) => {
                let index = object_entry_index(entries, segment)?;
                if !rest.is_empty() {
                    return entries[index.ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?]
                        .1
                        .edit_value_at_path(rest, edit)
                        .map_err(|error| error.under(segment));
                }
                match edit {
                    ValueEdit::Set(value) => entries[index.ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?].1 = value,
                    ValueEdit::Insert(value) => {
                        if index.is_some() {
                            return Err(ValueError::new(format!("object key `{segment}` already exists")));
                        }
                        entries.push(((*segment).to_owned(), value));
                    }
                    ValueEdit::Remove => {
                        entries.remove(index.ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?);
                    }
                }
                Ok(())
            }
            _ => Err(ValueError::new(format!("value has no child `{segment}`"))),
        }
    }
}

/// 👻️ `PhantomData<T>` is zero-sized and carries no data regardless of `T` — matches `serde`'s own
/// blanket impl (encodes as a unit value, decodes from anything). Unconditional on `T` (no `T:
/// ToValue`/`FromValue` bound) so a generic struct with a `PhantomData<SomeUnrelatedType>` marker
/// field never forces that unrelated type to implement these traits too.
impl<T: ?Sized> ToValue for std::marker::PhantomData<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Null
    }
}
impl<T: ?Sized> FromValue for std::marker::PhantomData<T> {
    fn from_value(_value: DslValue) -> Result<Self, ValueError> {
        Ok(std::marker::PhantomData)
    }
}

/// 🔗️ A 2-tuple encodes as a fixed-length array — the same shape `serde_json` gives a Rust tuple.
impl<A: ToValue, B: ToValue> ToValue for (A, B) {
    fn to_value(&self) -> DslValue {
        DslValue::Array(vec![self.0.to_value(), self.1.to_value()])
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        match path_index(segment, 2, false)? {
            0 => self.0.value_at_path(rest),
            1 => self.1.value_at_path(rest),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: 2 }) };
        match path_index(segment, 2, false)? {
            0 => self.0.value_shape_at_path(rest),
            1 => self.1.value_shape_at_path(rest),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new("expected an object, found an array"))?;
        match path_index(segment, 2, false)? {
            0 => self.0.value_key_at_path(rest, index),
            1 => self.1.value_key_at_path(rest, index),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }
}
impl<A: FromValue, B: FromValue> FromValue for (A, B) {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) if items.len() == 2 => {
                let mut iter = items.into_iter();
                let a = A::from_value(iter.next().expect("len == 2")).map_err(|error| error.under(0))?;
                let b = B::from_value(iter.next().expect("len == 2")).map_err(|error| error.under(1))?;
                Ok((a, b))
            }
            other => Err(ValueError::new(format!("expected a 2-element array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert into a fixed tuple")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove a fixed tuple")),
            };
        };
        if rest.is_empty() && !matches!(edit, ValueEdit::Set(_)) {
            return Err(ValueError::new("cannot change the length of a fixed tuple"));
        }
        match path_index(segment, 2, false)? {
            0 => self.0.edit_value_at_path(rest, edit),
            1 => self.1.edit_value_at_path(rest, edit),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }
}

/// 🔗️ A 3-tuple encodes as a fixed-length array — the same shape `serde_json` gives a Rust tuple.
impl<A: ToValue, B: ToValue, C: ToValue> ToValue for (A, B, C) {
    fn to_value(&self) -> DslValue {
        DslValue::Array(vec![self.0.to_value(), self.1.to_value(), self.2.to_value()])
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        match path_index(segment, 3, false)? {
            0 => self.0.value_at_path(rest),
            1 => self.1.value_at_path(rest),
            2 => self.2.value_at_path(rest),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: 3 }) };
        match path_index(segment, 3, false)? {
            0 => self.0.value_shape_at_path(rest),
            1 => self.1.value_shape_at_path(rest),
            2 => self.2.value_shape_at_path(rest),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new("expected an object, found an array"))?;
        match path_index(segment, 3, false)? {
            0 => self.0.value_key_at_path(rest, index),
            1 => self.1.value_key_at_path(rest, index),
            2 => self.2.value_key_at_path(rest, index),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }
}
impl<A: FromValue, B: FromValue, C: FromValue> FromValue for (A, B, C) {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) if items.len() == 3 => {
                let mut iter = items.into_iter();
                let a = A::from_value(iter.next().expect("len == 3")).map_err(|error| error.under(0))?;
                let b = B::from_value(iter.next().expect("len == 3")).map_err(|error| error.under(1))?;
                let c = C::from_value(iter.next().expect("len == 3")).map_err(|error| error.under(2))?;
                Ok((a, b, c))
            }
            other => Err(ValueError::new(format!("expected a 3-element array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert into a fixed tuple")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove a fixed tuple")),
            };
        };
        if rest.is_empty() && !matches!(edit, ValueEdit::Set(_)) {
            return Err(ValueError::new("cannot change the length of a fixed tuple"));
        }
        match path_index(segment, 3, false)? {
            0 => self.0.edit_value_at_path(rest, edit),
            1 => self.1.edit_value_at_path(rest, edit),
            2 => self.2.edit_value_at_path(rest, edit),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }
}

/// 🗺️ A `HashMap<K, V>` encodes as an object with `K` stringified into the key — matches
/// `serde_json`'s own behavior for non-`String` map keys (JSON objects only have string keys), so
/// `HashMap<u64, _>` etc. round-trip on the same wire shape a pre-conversion `serde_json::Value`
/// would have produced. Iteration order is unspecified, same as `serde_json` gives for a `HashMap`.
impl<K: ToString, V: ToValue> ToValue for std::collections::HashMap<K, V> {
    fn to_value(&self) -> DslValue {
        DslValue::object(self.iter().map(|(key, value)| (key.to_string(), value.to_value())))
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let mut matches = self.iter().filter(|(key, _)| key.to_string() == *segment);
        let (_, value) = matches.next().ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
        if matches.next().is_some() {
            return Err(ValueError::new(format!("multiple map keys encode as `{segment}`")));
        }
        value.value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Object { len: self.len() }) };
        let mut matches = self.iter().filter(|(key, _)| key.to_string() == *segment);
        let (_, value) = matches.next().ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
        if matches.next().is_some() {
            return Err(ValueError::new(format!("multiple map keys encode as `{segment}`")));
        }
        value.value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        if path.is_empty() {
            let key = self.keys().nth(index).ok_or_else(|| ValueError::new(format!("object key index {index} is out of range for length {}", self.len())))?.to_string();
            if self.keys().filter(|candidate| candidate.to_string() == key).count() != 1 {
                return Err(ValueError::new(format!("multiple map keys encode as `{key}`")));
            }
            return Ok(key);
        }
        let (segment, rest) = path.split_first().expect("non-empty path checked above");
        let mut matches = self.iter().filter(|(key, _)| key.to_string() == *segment);
        let (_, value) = matches.next().ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
        if matches.next().is_some() {
            return Err(ValueError::new(format!("multiple map keys encode as `{segment}`")));
        }
        value.value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}
impl<K: std::str::FromStr + std::hash::Hash + Eq, V: FromValue> FromValue for std::collections::HashMap<K, V>
where
    K::Err: std::fmt::Display,
{
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Object(entries) => entries
                .into_iter()
                .map(|(key, value)| {
                    let parsed_key = key.parse::<K>().map_err(|error| ValueError::new(format!("invalid map key {key:?}: {error}")))?;
                    V::from_value(value).map(|value| (parsed_key, value)).map_err(|error| error.under(key))
                })
                .collect(),
            other => Err(ValueError::new(format!("expected an object, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) => Err(ValueError::new("cannot insert at the map root")),
                ValueEdit::Remove => Err(ValueError::new("cannot remove the map root")),
            };
        };
        let key = segment.parse::<K>().map_err(|error| ValueError::new(format!("invalid map key `{segment}`: {error}")))?;
        if !rest.is_empty() {
            return self.get_mut(&key).ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?.edit_value_at_path(rest, edit).map_err(|error| error.under(segment));
        }
        match edit {
            ValueEdit::Set(value) => {
                let replacement = V::from_value(value).map_err(|error| error.under(segment))?;
                let target = self.get_mut(&key).ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
                *target = replacement;
            }
            ValueEdit::Insert(value) => {
                if self.contains_key(&key) {
                    return Err(ValueError::new(format!("object key `{segment}` already exists")));
                }
                self.insert(key, V::from_value(value).map_err(|error| error.under(segment))?);
            }
            ValueEdit::Remove => {
                self.remove(&key).ok_or_else(|| ValueError::new(format!("missing object key `{segment}`")))?;
            }
        }
        Ok(())
    }
}
//#endregion 🔖️Containers

//#region 🔖️ObjectHelpers
impl DslValue {
    /// 🗺️ Consumes an object value into its owned entries, or errors for any other shape — the
    /// entry point every derived struct `FromValue::from_value` starts from.
    pub fn into_object(self) -> Result<Vec<(String, DslValue)>, ValueError> {
        match self {
            DslValue::Object(entries) => Ok(entries),
            other => Err(ValueError::new(format!("expected an object, found {other:?}"))),
        }
    }
}
//#endregion 🔖️ObjectHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
