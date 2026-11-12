//! 🌐️ Canonical Value traits encode the actual downward immutable ordered producer.
use crate::ordered::*;
/// 🌱️ First-party replacement for the `#[cfg(test)]`-only `Serialize`/`Deserialize` above — see [Design notes](README.md#source-notes-3).
#[path = "../🚦️native/🦀️.rs"]
mod native_controlled;

impl<V: super::ToValue> super::ToValue for OrderedMap<V> {
    fn to_value_controlled(&self, control: &mut crate::NativeEncodeControl<'_>) -> Result<crate::DslValue, crate::ValueError> { native_controlled::encode(self, control) }
    fn to_value(&self) -> super::DslValue {
        super::DslValue::Object(self.iter().map(|(key, value)| (key.clone(), super::ToValue::to_value(value))).collect())
    }
}
impl<V: super::FromValue> super::FromValue for OrderedMap<V> {
    fn from_value(value: super::DslValue) -> Result<Self, super::ValueError> {
        let super::DslValue::Object(fields) = value else {
            return Err(super::ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an ordered string-keyed object, found {value:?}")));
        };
        let mut map = <Self as super::FromValue>::guard_decoded(OrderedMap::<V>::new());
        for (key, entry) in fields {
            let decoded = <V as super::FromValue>::from_value(entry).map_err(|error| error.under(key.clone()))?;
            map.get_mut().insert(key, decoded);
        }
        Ok(map.take())
    }
    fn retire_decoded(self) { self.retire_cold(); }
}
//#endregion 🔁️Value
#[path="../🧺️set/🌐️wire/🦀️.rs"]
mod set_wire;
