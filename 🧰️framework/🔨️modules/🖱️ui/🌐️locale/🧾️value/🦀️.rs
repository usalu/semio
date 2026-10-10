//! 🌱️ Direct declared-axis codecs under the canonical neutral Value interface.
use super::{Locale, Terminology};
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind, NativeDecodeControl, NativeEncodeControl};

macro_rules! axis_value {
    ($owner:ty) => {
        impl ToValue for $owner {
            fn to_value(&self) -> DslValue { DslValue::String(self.as_str().to_string()) }
            fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { control.copy_text(self.as_str()).map(DslValue::String) }
        }
        impl FromValue for $owner {
            fn from_value(value: DslValue) -> Result<Self, ValueError> {
                match value { DslValue::String(text) => Self::parse(&text).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "undeclared axis")), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected axis text")) }
            }
            fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
                control.scoped_stage(|control| { control.begin_stage(1)?; control.step() })?;
                match value { DslValue::String(text) => Self::parse(text).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "undeclared axis")), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected axis text")) }
            }
        }
    };
}
axis_value!(Locale);
axis_value!(Terminology);

#[cfg(test)]
#[path = "../🧪️tests/🌐️axis-value/🦀️.rs"]
mod locale_terminology_value_round_trip_tests;

impl Locale {
    /// 🌐️ Admits a declared language from an explicit well-formed language tag.
    pub fn from_language_tag(tag: &str) -> Result<Self, ValueError> {
        let mut parts = tag.split('-').peekable();
        let language = parts.next().filter(|part| (2..=3).contains(&part.len()) && part.bytes().all(|byte| byte.is_ascii_alphabetic())).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "malformed language tag"))?;
        let locale = Self::ALL.iter().copied().find(|locale| locale.as_str().eq_ignore_ascii_case(language)).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "undeclared locale"))?;
        if parts.peek().is_some_and(|part| part.len() == 4 && part.bytes().all(|byte| byte.is_ascii_alphabetic())) { parts.next(); }
        if parts.peek().is_some_and(|part| part.len() == 2 && part.bytes().all(|byte| byte.is_ascii_alphabetic()) || part.len() == 3 && part.bytes().all(|byte| byte.is_ascii_digit())) { parts.next(); }
        let mut variants = std::collections::HashSet::new();
        while parts.peek().is_some_and(|part| ((5..=8).contains(&part.len()) || part.len() == 4 && part.as_bytes()[0].is_ascii_digit()) && part.bytes().all(|byte| byte.is_ascii_alphanumeric())) {
            let variant = parts.next().expect("variant").to_ascii_lowercase();
            if !variants.insert(variant) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "duplicate language variant")); }
        }
        let mut extensions = std::collections::HashSet::new();
        while let Some(singleton) = parts.next() {
            if singleton.len() != 1 || !singleton.as_bytes()[0].is_ascii_alphanumeric() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "malformed language tag")); }
            let singleton = singleton.as_bytes()[0].to_ascii_lowercase();
            if !extensions.insert(singleton) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "duplicate language extension")); }
            let minimum = if singleton == b'x' { 1 } else { 2 };
            let mut count = 0;
            while parts.peek().is_some_and(|part| (minimum..=8).contains(&part.len()) && part.bytes().all(|byte| byte.is_ascii_alphanumeric())) { parts.next(); count += 1; }
            if count == 0 || singleton == b'x' && parts.peek().is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "malformed language tag")); }
        }
        Ok(locale)
    }
}

semio_framework_value::artifact_retire_leaf!(Locale, Terminology);
