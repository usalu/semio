//! 🏷️ Explicit controlled Graph property type conversion and canonical owner retirement.
use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, ValueError, ValueType};
pub(super) fn decode(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<ValueType, ValueError> {
    c.scoped_stage(|c| {
        c.begin_stage(0)?;
        match value {
            DslValue::String(value) => {
                c.charge(size_of::<ValueType>())?;
                c.step()?;
                Ok(match value.as_str() {
                    "boolean" | "bool" => ValueType::Boolean,
                    "integer" | "int" => ValueType::Integer,
                    "number" | "decimal" | "float" => ValueType::Decimal,
                    "text" | "string" => ValueType::Text,
                    "object" | "any" => ValueType::Any,
                    _ => ValueType::Schema(c.copy_text(value)?),
                })
            }
            DslValue::Object(fields) if fields.len() == 1 && fields[0].0 == "schema" => {
                let DslValue::String(schema) = &fields[0].1 else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"Graph type schema must be text")) };
                c.charge(size_of::<ValueType>())?;
                c.step()?;
                Ok(ValueType::Schema(c.copy_text(schema)?))
            }
            _ => ValueType::from_value_controlled(value, c),
        }
    })
}
pub(super) fn retire(value: ValueType) {
    ValueType::retire_decoded(value)
}
