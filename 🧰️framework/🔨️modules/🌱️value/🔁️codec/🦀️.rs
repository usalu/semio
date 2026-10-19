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

use super::{DslValue, Number, ValueError};
use crate::native_decoding::NativeDecodeControl;
use crate::native_encoding::NativeEncodeControl;
#[path="🛬️controlled/🦀️.rs"]
mod controlled;
pub use controlled::DecodedValue;
#[path="♻️retirement/🦀️.rs"]
mod retirement;
pub use retirement::IntrinsicRetirement;
#[path="🛫️controlled/🦀️.rs"]
mod encoding;

/// 🔑️ A hash-builder owner admits its own default state before controlled collection construction.
pub trait ControlledValueHasher: std::hash::BuildHasher + Default {
    fn from_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError> where Self:Sized;
}
impl ControlledValueHasher for std::collections::hash_map::RandomState {
    fn from_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;Ok(Self::new())}
}

//#region 🔖️Traits
/// 🔁️ Converts `self` into a [`DslValue`] tree. First-party analog of `serde::Serialize`.
pub trait ToValue {
    fn to_value(&self) -> DslValue;

    /// 🛫️ Constructs owned native output through an explicit cumulative allocation control.
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{control.checkpoint()?;Err(ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "value owner has no controlled native encoding implementation"))}

    /// 🔑️ Formats one owned map key without an uncontrolled `ToString` allocation.
    fn to_object_key_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{control.checkpoint()?;Err(ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "value key owner has no controlled native encoding implementation"))}

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        self.to_value().value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        Ok(ValueShape::of(&self.value_at_path(path)?))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        match self.value_at_path(path)? {
            DslValue::Object(entries) => entries.get(index).map(|(key, _)| key.clone()).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {}", entries.len()))),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an object, found {other:?}"))),
        }
    }
}

/// 🔁️ Hydrates `Self` from a [`DslValue`] tree. First-party analog of
/// `serde::de::DeserializeOwned`.
pub trait FromValue: Sized {
    fn from_value(value: DslValue) -> Result<Self, ValueError>;

    /// 🛬️ Constructs from borrowed native state under one cumulative ownership control.
    fn from_value_controlled(_value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.checkpoint()?;
        Err(ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "value owner has no controlled native construction implementation"))
    }

    /// 🔑️ Constructs a typed object key without an uncontrolled `FromStr` allocation.
    fn from_object_key_controlled(_key:&str,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        control.checkpoint()?;Err(ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "value key owner has no controlled construction implementation"))
    }

    /// 🌱️ Admits a missing field's default without invoking an uncontrolled allocator.
    fn default_value_controlled(control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.checkpoint()?;
        Err(ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "value owner has no controlled default construction implementation"))
    }

    /// 🛡️ Holds a completed child until its parent can publish a complete value.
    fn guard_decoded(self) -> DecodedValue<Self> { DecodedValue::new(self, Self::retire_decoded) }

    /// ♻️ Retires a completed child when a later construction step fails.
    fn retire_decoded(self) { drop(self); }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if !path.is_empty() {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("value has no child `{}`", path[0])));
        }
        match edit {
            ValueEdit::Set(value) => {
                *self = <Self as FromValue>::from_value(value)?;
                Ok(())
            }
            ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert at the value root")),
            ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove the value root")),
        }
    }

}

/// ✏️ One structural edit over already-decoded value path segments.
#[derive(Clone, Debug, PartialEq)]
pub enum ValueEdit {
    Set(DslValue),
    Insert(DslValue),
    /// 🗂️ Places an ordered object member; intrinsically ordered maps retain their key semantics.
    InsertAt { index: usize, value: DslValue },
    Remove,
}

/// 📐 Describes a value at a typed path without materializing its children.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueShape {
    Null,
    Bool,
    Number,
    String,
    Bytes { len: usize },
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
            DslValue::Bytes(bytes) => Self::Bytes{len:bytes.len()},
            DslValue::Array(items) => Self::Array { len: items.len() },
            DslValue::Object(entries) => Self::Object { len: entries.len() },
        }
    }
}

/// 🧭️ Applies `edit` at `path` of a value whose codec is hand-written: through the value tree the codec emits, decoded back
/// so every invariant its `from_value` checks holds for the edited value — the path edit a derived [`FromValue`] performs
/// field by field. On failure `target` is unchanged. A hand-written `FromValue` routes its `edit_value_at_path` here.
pub fn edit_through_value<T: ToValue + FromValue>(target: &mut T, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
    let mut value = target.to_value();
    value.edit_value_at_path(path, edit)?;
    *target = T::from_value(value)?;
    Ok(())
}

/// 🧭️ Resolves one path segment to a collection index for owners of other crates.
pub fn path_index(segment: &str, length: usize, insert: bool) -> Result<usize, ValueError> {
    if insert && segment == "-" {
        return Ok(length);
    }
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("invalid canonical array index `{segment}`")));
    }
    let index = segment.parse::<usize>().map_err(|_| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("array index `{segment}` is out of range")))?;
    let admitted = if insert { index <= length } else { index < length };
    admitted.then_some(index).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("array index `{segment}` is out of range for length {length}")))
}

fn object_entry_index(entries: &[(String, DslValue)], key: &str) -> Result<Option<usize>, ValueError> {
    let mut matches = entries.iter().enumerate().filter(|(_, (candidate, _))| candidate == key).map(|(index, _)| index);
    let first = matches.next();
    if matches.next().is_some() {
        return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("duplicate object key `{key}`")));
    }
    Ok(first)
}
//#endregion 🔖️Traits

//#region 🔖️Scalars
/// 🔢️ Unsigned integer scalars — round-trip through [`Number::UInt`] so a wire-visible
/// `u64` field (e.g. `ttl_secs`) encodes as bare `3600`, never `3600.0`. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️dslvalue-integer-fidelity.md`.
macro_rules! impl_uint_codec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ToValue for $ty {
                fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::scalar(control)?;Ok(DslValue::Number(Number::UInt(*self as u64)))}
                fn to_object_key_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{encoding::key(*self,control)}
                fn to_value(&self) -> DslValue {
                    DslValue::Number(Number::UInt(*self as u64))
                }
            }
            impl FromValue for $ty {
                fn from_object_key_controlled(key:&str,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;key.parse().map_err(|_|ValueError::new(crate::ValueRefusalKind::InvalidValue, "invalid numeric object key"))}

                fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
                    controlled::scalar(control)?;match value{DslValue::Number(number)=>number.as_u64().and_then(|n| <$ty>::try_from(n).ok()).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected exact unsigned integer")),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected number"))}
                }
                fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;Ok(0 as $ty)}

                fn from_value(value: DslValue) -> Result<Self, ValueError> {
                    match value {
                        DslValue::Number(number) => number.as_u64().and_then(|n| <$ty>::try_from(n).ok())
                            .ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an exact {} integer, found {number:?}", stringify!($ty)))),
                        other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a number, found {other:?}"))),
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
                fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::scalar(control)?;Ok(DslValue::Number(Number::Int(*self as i64)))}
                fn to_object_key_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{encoding::key(*self,control)}
                fn to_value(&self) -> DslValue {
                    DslValue::Number(Number::Int(*self as i64))
                }
            }
            impl FromValue for $ty {
                fn from_object_key_controlled(key:&str,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;key.parse().map_err(|_|ValueError::new(crate::ValueRefusalKind::InvalidValue, "invalid numeric object key"))}

                fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
                    controlled::scalar(control)?;match value{DslValue::Number(number)=>number.as_i64().and_then(|n| <$ty>::try_from(n).ok()).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected exact signed integer")),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected number"))}
                }
                fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;Ok(0 as $ty)}

                fn from_value(value: DslValue) -> Result<Self, ValueError> {
                    match value {
                        DslValue::Number(number) => number.as_i64().and_then(|n| <$ty>::try_from(n).ok())
                            .ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an exact {} integer, found {number:?}", stringify!($ty)))),
                        other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a number, found {other:?}"))),
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
                fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::scalar(control)?;Ok(DslValue::Number(Number::Float(*self as f64)))}
                fn to_object_key_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{encoding::key(*self,control)}
                fn to_value(&self) -> DslValue {
                    DslValue::Number(Number::Float(*self as f64))
                }
            }
            impl FromValue for $ty {
                fn from_object_key_controlled(key:&str,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;key.parse().map_err(|_|ValueError::new(crate::ValueRefusalKind::InvalidValue, "invalid numeric object key"))}

                fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
                    controlled::scalar(control)?;match value{DslValue::Number(number)=>Ok(number.as_f64() as $ty),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected number"))}
                }
                fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;Ok(0 as $ty)}

                fn from_value(value: DslValue) -> Result<Self, ValueError> {
                    match value {
                        DslValue::Number(n) => Ok(n.as_f64() as $ty),
                        other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a number, found {other:?}"))),
                    }
                }
            }
        )+
    };
}
impl_float_codec!(f64);

fn float32_from_number(number:Number)->f32{
    let value=number.as_f64();let bits=value.to_bits();let payload=bits&0x000f_ffff_ffff_ffff;
    if bits&0x7ff0_0000_0000_0000==0x7ff0_0000_0000_0000&&payload!=0{f32::from_bits(((bits>>32) as u32&0x8000_0000)|0x7f80_0000|((payload>>29) as u32).max(1))}else{value as f32}
}

impl ToValue for f32 {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::float32(*self,c)}
fn to_object_key_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{encoding::key(*self,c)}
    fn to_value(&self) -> DslValue {
        let bits=self.to_bits();let payload=bits&0x007f_ffff;
        if bits&0x7f80_0000==0x7f80_0000&&payload!=0{return DslValue::Number(Number::Float(f64::from_bits((u64::from(bits&0x8000_0000)<<32)|0x7ff0_0000_0000_0000|(u64::from(payload)<<29))))}
        let shortest = format!("{self:e}");
        let precision = shortest.split('e').next().unwrap().bytes().filter(u8::is_ascii_digit).count().saturating_sub(1);
        DslValue::Number(Number::Float(format!("{self:.precision$e}").parse().expect("f32 decimal text is a valid f64")))
    }
}

impl FromValue for f32 {
fn from_value_controlled(value:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;match value{DslValue::Number(n)=>Ok(float32_from_number(*n)),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected number"))}}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(0.0)}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Number(number) => Ok(float32_from_number(number)),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a number, found {other:?}"))),
        }
    }
}

impl ToValue for bool {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::scalar(c)?;Ok(DslValue::Bool(*self))}
fn to_object_key_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{encoding::key(*self,c)}
    fn to_value(&self) -> DslValue {
        DslValue::Bool(*self)
    }
}
impl FromValue for bool {
fn from_object_key_controlled(key:&str,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;key.parse().map_err(|_|ValueError::new(crate::ValueRefusalKind::InvalidValue, "invalid boolean object key"))}

fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;match v{DslValue::Bool(v)=>Ok(*v),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected bool"))}}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(false)}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Bool(b) => Ok(b),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a bool, found {other:?}"))),
        }
    }
}

impl ToValue for String {
fn value_shape_at_path(&self,path:&[&str])->Result<ValueShape,ValueError>{if path.is_empty(){Ok(ValueShape::String)}else{Err(ValueError::literal(crate::ValueRefusalKind::InvalidValue,"string has no child path"))}}
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{c.copy_text(self).map(DslValue::String)}
fn to_object_key_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{c.copy_text(self)}
    fn to_value(&self) -> DslValue {
        DslValue::String(self.clone())
    }
}
impl FromValue for String {
fn from_object_key_controlled(key:&str,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;control.copy_text(key)}

fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;match v{DslValue::String(v)=>c.copy_text(v),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected string"))}}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(String::new())}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(s),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🗂️ A path renders as its UTF-8 string form (lossy on the encode side — this is a local-only
/// config value, never a content-addressed hash input, so exact non-Unicode byte round-tripping
/// is not required the way `serde`'s own `Path`/`PathBuf` impl demands it).
impl ToValue for std::path::PathBuf {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::path(self,c)}
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string_lossy().into_owned())
    }
}
impl FromValue for std::path::PathBuf {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{String::from_value_controlled(v,c).map(Self::from)}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(Self::new())}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(std::path::PathBuf::from(s)),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🌉️ No `FromValue` counterpart — decoding always needs owned data, `String`'s impl covers it.
impl ToValue for &str {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{c.copy_text(self).map(DslValue::String)}
fn to_object_key_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{c.copy_text(self)}
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string())
    }
}

impl ToValue for () {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::scalar(c)?;Ok(DslValue::Null)}
    fn to_value(&self) -> DslValue {
        DslValue::Null
    }
}
impl FromValue for () {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;match v{DslValue::Null=>Ok(()),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected null"))}}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(()),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected null, found {other:?}"))),
        }
    }
}
//#endregion 🔖️Scalars

#[path = "📦️container-codecs/🦀️.rs"]
mod container_codecs;

//#region 🔖️ObjectHelpers
impl DslValue {
    /// 🗺️ Consumes an object value into its owned entries, or errors for any other shape — the
    /// entry point every derived struct `FromValue::from_value` starts from.
    pub fn into_object(self) -> Result<Vec<(String, DslValue)>, ValueError> {
        match self {
            DslValue::Object(entries) => Ok(entries),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an object, found {other:?}"))),
        }
    }
}
//#endregion 🔖️ObjectHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🛬️controlled/🦀️.rs"]
pub(crate) mod controlled_tests;

#[cfg(test)]
#[path="🧪️tests/🛫️controlled/🦀️.rs"]
mod controlled_encoding_tests;

/// 📦️ Encodes an exact-size sequence of values under the caller's native control.
pub fn controlled_sequence<'a,T:ToValue+'a>(items:impl ExactSizeIterator<Item=&'a T>,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::sequence(items,control)}
