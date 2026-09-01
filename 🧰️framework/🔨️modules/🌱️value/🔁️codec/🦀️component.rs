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

use super::DslValue;

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
macro_rules! impl_number_codec {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ToValue for $ty {
                fn to_value(&self) -> DslValue {
                    DslValue::Number(*self as f64)
                }
            }
            impl FromValue for $ty {
                fn from_value(value: DslValue) -> Result<Self, ValueError> {
                    match value {
                        DslValue::Number(n) => Ok(n as $ty),
                        other => Err(ValueError::new(format!("expected a number, found {other:?}"))),
                    }
                }
            }
        )+
    };
}
impl_number_codec!(f64, f32, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

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
        assert_eq!(42_i64.to_value(), DslValue::Number(42.0));
        assert_eq!(i64::from_value(DslValue::Number(42.0)), Ok(42_i64));
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
        let bad = DslValue::Array(vec![DslValue::Number(1.0), DslValue::Bool(true)]);
        assert_eq!(Vec::<i64>::from_value(bad), Err(ValueError::new("1.expected a number, found Bool(true)")));
    }

    #[test]
    fn btreemap_round_trips_in_key_order() {
        let mut map = std::collections::BTreeMap::new();
        map.insert("b".to_string(), 2_i64);
        map.insert("a".to_string(), 1_i64);
        let encoded = map.to_value();
        assert_eq!(encoded, DslValue::object([("a".to_string(), DslValue::Number(1.0)), ("b".to_string(), DslValue::Number(2.0))]));
        assert_eq!(std::collections::BTreeMap::<String, i64>::from_value(encoded), Ok(map));
    }
}
//#endregion 🧪️Tests
