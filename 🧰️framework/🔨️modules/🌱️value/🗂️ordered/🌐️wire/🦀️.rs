//! 🌐️ Canonical Value traits encode the actual downward immutable ordered producer.
use crate::ordered::*;
use semio_framework_value_native_ordered_engine::{Entry,Node};
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
    fn from_value_controlled(value:&super::DslValue,control:&mut crate::NativeDecodeControl<'_>)->Result<Self,super::ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{
        let entries=value.object_controlled(control)?;control.begin_stage(entries.len())?;
        let row=SharedOwner::<Entry<V>>::allocation_bytes()+SharedOwner::<Node<V>>::allocation_bytes()+SharedOwner::<String>::allocation_bytes()+SharedOwner::<V>::allocation_bytes();
        let mut map=<Self as super::FromValue>::guard_decoded(OrderedMap::<V>::new());
        for (key,entry) in entries{
            let key=<String as super::FromValue>::from_object_key_controlled(key,control)?;
            let child=<V as super::FromValue>::from_value_controlled(entry,control)?;
            if let Err(error)=control.charge(row){<String as super::FromValue>::retire_decoded(key);<V as super::FromValue>::retire_decoded(child);return Err(error)}
            if let Some(previous)=map.get_mut().insert(key,child){<V as super::FromValue>::retire_decoded(previous);return Err(super::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"duplicate ordered-map key"))}
            control.step()?;
        }
        Ok(map.take())
    }))}
    fn default_value_controlled(control:&mut crate::NativeDecodeControl<'_>)->Result<Self,super::ValueError>{control.checkpoint()?;Ok(Self::new())}
    fn retire_decoded(self) { self.retire_cold(); }
}
//#endregion 🔁️Value
#[path="../🧺️set/🌐️wire/🦀️.rs"]
mod set_wire;
