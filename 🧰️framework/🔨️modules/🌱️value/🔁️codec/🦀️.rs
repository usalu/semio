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
}

/// @emoji 🔁️ Hydrates `Self` from a [`DslValue`] tree. First-party analog of
/// `serde::de::DeserializeOwned`.
pub trait FromValue: Sized {
    fn from_value(value: DslValue) -> Result<Self, ValueError>;
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
                        DslValue::Number(Number::UInt(n)) => Ok(n as $ty),
                        DslValue::Number(Number::Int(n)) => Ok(n as $ty),
                        DslValue::Number(Number::Float(n)) => Ok(n as $ty),
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
                        DslValue::Number(Number::Int(n)) => Ok(n as $ty),
                        DslValue::Number(Number::UInt(n)) => Ok(n as $ty),
                        DslValue::Number(Number::Float(n)) => Ok(n as $ty),
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
impl_float_codec!(f64, f32);

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
}
impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(None),
            other => T::from_value(other).map(Some),
        }
    }
}

impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}
impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(format!("expected an array, found {other:?}"))),
        }
    }
}

/// 📐️ Same plain-JSON-array wire shape as `Vec<T>` — a `VecDeque` field (e.g. `semio-framework-
/// actor`'s per-lane mailbox rings) round-trips identically to its `Vec` twin, just ring-backed in memory.
impl<T: ToValue> ToValue for std::collections::VecDeque<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}
impl<T: FromValue> FromValue for std::collections::VecDeque<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(format!("expected an array, found {other:?}"))),
        }
    }
}

/// 📐️ A fixed-size array encodes exactly like a `Vec<T>` (a plain JSON array) — the length is
/// carried by `N`, not the wire, so decode rejects any array whose length doesn't match `N`
/// (matches what a fixed-size `[T; N]` field means: this many, no more, no fewer).
impl<T: ToValue, const N: usize> ToValue for [T; N] {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
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
}

impl<T: ToValue> ToValue for Box<T> {
    fn to_value(&self) -> DslValue {
        (**self).to_value()
    }
}
impl<T: FromValue> FromValue for Box<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        T::from_value(value).map(Box::new)
    }
}

/// 🌳️ A `BTreeSet<T>` encodes exactly like a `Vec<T>` (a plain JSON array), in the set's own sorted
/// iteration order — matches `serde`'s own `BTreeSet` representation.
impl<T: ToValue + Ord> ToValue for std::collections::BTreeSet<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}
impl<T: FromValue + Ord> FromValue for std::collections::BTreeSet<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(format!("expected an array, found {other:?}"))),
        }
    }
}

impl<T: ToValue> ToValue for std::collections::BTreeMap<String, T> {
    fn to_value(&self) -> DslValue {
        DslValue::object(self.iter().map(|(key, value)| (key.clone(), value.to_value())))
    }
}
impl<T: FromValue> FromValue for std::collections::BTreeMap<String, T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Object(entries) => entries.into_iter().map(|(key, value)| T::from_value(value).map(|value| (key.clone(), value)).map_err(|error| error.under(key))).collect(),
            other => Err(ValueError::new(format!("expected an object, found {other:?}"))),
        }
    }
}

impl ToValue for DslValue {
    fn to_value(&self) -> DslValue {
        self.clone()
    }
}
impl FromValue for DslValue {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(value)
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
}

/// 🔗️ A 3-tuple encodes as a fixed-length array — the same shape `serde_json` gives a Rust tuple.
impl<A: ToValue, B: ToValue, C: ToValue> ToValue for (A, B, C) {
    fn to_value(&self) -> DslValue {
        DslValue::Array(vec![self.0.to_value(), self.1.to_value(), self.2.to_value()])
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
}

/// 🗺️ A `HashMap<K, V>` encodes as an object with `K` stringified into the key — matches
/// `serde_json`'s own behavior for non-`String` map keys (JSON objects only have string keys), so
/// `HashMap<u64, _>` etc. round-trip on the same wire shape a pre-conversion `serde_json::Value`
/// would have produced. Iteration order is unspecified, same as `serde_json` gives for a `HashMap`.
impl<K: ToString, V: ToValue> ToValue for std::collections::HashMap<K, V> {
    fn to_value(&self) -> DslValue {
        DslValue::object(self.iter().map(|(key, value)| (key.to_string(), value.to_value())))
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
mod tests {
    use super::*;

    #[test]
    fn scalars_round_trip() {
        assert_eq!(42_i64.to_value(), DslValue::int(42));
        assert_eq!(i64::from_value(DslValue::int(42)), Ok(42_i64));
        assert_eq!(true.to_value(), DslValue::Bool(true));
        assert_eq!("hi".to_string().to_value(), DslValue::String("hi".to_string()));
    }

    #[test]
    fn option_collapses_nested_none_like_naive_serde() {
        let outer_none: Option<Option<String>> = None;
        let inner_none: Option<Option<String>> = Some(None);
        assert_eq!(outer_none.to_value(), DslValue::Null);
        assert_eq!(inner_none.to_value(), DslValue::Null);
    }

    #[test]
    fn vec_round_trips_and_reports_index_on_error() {
        let values = vec![1_i64, 2, 3];
        let encoded = values.to_value();
        assert_eq!(Vec::<i64>::from_value(encoded), Ok(values));
        let bad = DslValue::Array(vec![DslValue::int(1), DslValue::Bool(true)]);
        assert_eq!(Vec::<i64>::from_value(bad), Err(ValueError::new("1.expected a number, found Bool(true)")));
    }

    #[test]
    fn btreemap_round_trips_in_key_order() {
        let mut map = std::collections::BTreeMap::new();
        map.insert("b".to_string(), 2_i64);
        map.insert("a".to_string(), 1_i64);
        let encoded = map.to_value();
        assert_eq!(encoded, DslValue::object([("a".to_string(), DslValue::int(1)), ("b".to_string(), DslValue::int(2))]));
        assert_eq!(std::collections::BTreeMap::<String, i64>::from_value(encoded), Ok(map));
    }

    #[test]
    fn tuple_round_trips_as_two_element_array_like_serde_json() {
        let pair = ("a".to_string(), vec!["b".to_string(), "c".to_string()]);
        let encoded = pair.to_value();
        assert_eq!(encoded, DslValue::Array(vec![DslValue::String("a".to_string()), DslValue::Array(vec![DslValue::String("b".to_string()), DslValue::String("c".to_string())])]));
        assert_eq!(<(String, Vec<String>)>::from_value(encoded), Ok(pair));
        let bad = DslValue::Array(vec![DslValue::int(1)]);
        assert_eq!(<(i64, i64)>::from_value(bad), Err(ValueError::new("expected a 2-element array, found Array([Number(Int(1))])")));
    }

    #[test]
    fn fixed_size_array_round_trips_and_rejects_wrong_length() {
        let values: [f64; 3] = [1.0, 2.0, 3.0];
        let encoded = values.to_value();
        assert_eq!(encoded, DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0)]));
        assert_eq!(<[f64; 3]>::from_value(encoded), Ok(values));

        let too_short = DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0)]);
        assert_eq!(<[f64; 3]>::from_value(too_short), Err(ValueError::new("expected an array of length 3, found 2")));

        let too_long = DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0), DslValue::float(4.0)]);
        assert_eq!(<[f64; 3]>::from_value(too_long), Err(ValueError::new("expected an array of length 3, found 4")));
    }

    #[test]
    fn phantom_data_encodes_as_null_and_decodes_from_anything() {
        struct Marker;
        let phantom: std::marker::PhantomData<Marker> = std::marker::PhantomData;
        assert_eq!(phantom.to_value(), DslValue::Null);
        assert_eq!(std::marker::PhantomData::<Marker>::from_value(DslValue::Bool(true)), Ok(std::marker::PhantomData));
    }

    /// 🎯️ The regression this whole module exists for: a `u64` field round-trips as `Number::UInt`
    /// (bare `3600` on the wire, never `3600.0`), and a genuine `f64` field still round-trips as
    /// `Number::Float` (keeping its `.0`) — the two must stay distinguishable.
    #[test]
    fn u64_round_trips_as_uint_and_f64_round_trips_as_float() {
        let ttl_secs: u64 = 3600;
        let encoded = ttl_secs.to_value();
        assert_eq!(encoded, DslValue::uint(3600));
        assert!(matches!(encoded, DslValue::Number(Number::UInt(3600))));
        assert_eq!(u64::from_value(encoded), Ok(3600));

        let ratio: f64 = 3600.0;
        let encoded = ratio.to_value();
        assert_eq!(encoded, DslValue::float(3600.0));
        assert!(matches!(encoded, DslValue::Number(Number::Float(v)) if v == 3600.0));
        assert_eq!(f64::from_value(encoded), Ok(3600.0));
    }

    #[test]
    fn i64_min_and_max_round_trip_exactly() {
        for value in [i64::MIN, i64::MAX, 0_i64, -1_i64] {
            let encoded = value.to_value();
            assert!(matches!(encoded, DslValue::Number(Number::Int(v)) if v == value));
            assert_eq!(i64::from_value(encoded), Ok(value));
        }
    }

    #[test]
    fn u64_max_round_trips_exactly_beyond_f64_2_pow_53() {
        assert_eq!(u64::MAX as f64, (u64::MAX - 1) as f64, "test assumption: u64::MAX collides with u64::MAX - 1 once widened to f64");
        let encoded = u64::MAX.to_value();
        assert!(matches!(encoded, DslValue::Number(Number::UInt(v)) if v == u64::MAX));
        assert_eq!(u64::from_value(encoded), Ok(u64::MAX));

        let big: u64 = (1u64 << 53) + 1;
        assert!((big as f64) as u64 != big, "test assumption: 2^53+1 is not exactly representable in f64");
        let encoded = big.to_value();
        assert!(matches!(encoded, DslValue::Number(Number::UInt(v)) if v == big));
        assert_eq!(u64::from_value(encoded), Ok(big));
    }

    #[test]
    fn negative_zero_float_round_trips_and_stays_a_float() {
        let encoded = (-0.0_f64).to_value();
        assert!(matches!(encoded, DslValue::Number(Number::Float(v)) if v.is_sign_negative() && v == 0.0));
        assert_eq!(f64::from_value(encoded), Ok(-0.0));
    }

    #[test]
    fn whole_float_and_same_valued_integer_are_distinct_dsl_values() {
        let as_float = DslValue::float(3600.0);
        let as_uint = DslValue::uint(3600);
        assert_ne!(as_float, as_uint);
        assert!(matches!(as_float, DslValue::Number(Number::Float(_))));
        assert!(matches!(as_uint, DslValue::Number(Number::UInt(_))));
    }
}
//#endregion 🧪️Tests
