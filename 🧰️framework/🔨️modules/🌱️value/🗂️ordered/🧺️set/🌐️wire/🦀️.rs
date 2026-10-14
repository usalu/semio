//! 🌐️ Canonical membership Value traits consume the actual immutable set owner.
use semio_framework_value_ordered::ordered::{OrderedMap,OrderedSet};
use crate::{DslValue,FromValue,ToValue,ValueError};
//#region 🔁️ValueCodec
/// 🔁️ Mirrors the hand-written `Serialize`/`Deserialize` below: a plain string array.
#[path = "../🚦️native/🦀️.rs"]
mod native_controlled;

impl ToValue for OrderedSet {
    fn to_value_controlled(&self, control: &mut crate::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { native_controlled::encode(self, control) }
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(|value| DslValue::String(value.clone())).collect())
    }
}

impl FromValue for OrderedSet {
    fn from_value_controlled(value: &DslValue, control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, ValueError> { native_controlled::decode(value, control) }
    fn default_value_controlled(control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, ValueError> { control.checkpoint()?; Ok(Self::new()) }
    fn retire_decoded(self) { self.retire_cold() }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Array(items) = value else { return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected an array for OrderedSet")) };
        items.into_iter().map(String::from_value).collect()
    }
}
//#endregion 🔁️ValueCodec