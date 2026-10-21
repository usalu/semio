//! 🌐️ Canonical Value traits encode the actual downward immutable ordered producer.
use crate::ordered::*;
use crate::retained_clone::RetainedCloneGrant;
//#region 🔁️Value
/// 🌱️ First-party replacement for the `#[cfg(test)]`-only `Serialize`/`Deserialize` above — see
/// that region's doc. Written against `super::ToValue`/`FromValue` (a RELATIVE path) rather than
/// `#[derive(ToValue, FromValue)]`, because this component is mounted by BOTH `os-kernel` (where
/// the derive's hard-literal `::semio_framework_os_kernel::…` path would resolve) and
/// `replication` (where it would not — `replication` sits below `os-kernel` in the DAG); a
/// relative path resolves correctly under either mount point.
#[path = "../🚦️native/🦀️.rs"]
mod native_controlled;

impl<V: super::ToValue> super::ToValue for OrderedMap<V> {
    fn to_value_controlled(&self, control: &mut crate::NativeEncodeControl<'_>) -> Result<crate::DslValue, crate::ValueError> { native_controlled::encode(self, control) }
    fn to_value(&self) -> super::DslValue {
        super::DslValue::Object(self.iter().map(|(key, value)| (key.clone(), super::ToValue::to_value(value))).collect())
    }
}
impl<V: super::FromValue> super::FromValue for OrderedMap<V> {
    fn from_value_controlled(value: &crate::DslValue, control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, crate::ValueError> { native_controlled::decode(value, control) }
    fn default_value_controlled(control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, crate::ValueError> { native_controlled::empty(control) }
    fn retire_decoded(self) { native_controlled::retire(self) }
    fn from_value(value: super::DslValue) -> Result<Self, super::ValueError> {
        let super::DslValue::Object(fields) = value else {
            return Err(super::ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an ordered string-keyed object, found {value:?}")));
        };
        let mut map = OrderedMap::new();
        for (key, entry) in fields {
            let decoded = <V as super::FromValue>::from_value(entry).map_err(|error| error.under(key.clone()))?;
            map.insert(key, decoded);
        }
        Ok(map)
    }
}
//#endregion 🔁️Value
#[path="../🧺️set/🌐️wire/🦀️.rs"]
mod set_wire;
