//! 🏷️ Canonical seven-variant value types bind through their actual owned intrinsic tree.
use semio_framework_value::{FromValue, ToValue, ValueType};
use crate::{DslField, FieldValue, Shape, NativeSchemaControl, NativeEncodeControl, NativeDecodeControl, ValueError};
impl DslField for ValueType {
    fn shape() -> Shape { Shape::Value }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Value)}
    fn to_value(&self)->FieldValue{FieldValue::Value(ToValue::to_value(self))}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{Ok(FieldValue::Value(ToValue::to_value_controlled(self,control)?))}
    fn from_value(value:&FieldValue)->Result<Self,String>{match value{FieldValue::Value(value)=><Self as FromValue>::from_value(value.clone()).map_err(ValueError::into_message),_=>Err("expected a typed value declaration".into())}}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{FieldValue::Value(value)=><Self as FromValue>::from_value_controlled(value,control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"expected a typed value declaration"))}}
    fn retire_decoded(self){<Self as FromValue>::retire_decoded(self)}
}
