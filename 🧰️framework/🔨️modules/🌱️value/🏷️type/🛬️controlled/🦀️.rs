//! 🏷️ Iterative exact kind/of construction with cumulative native ownership admission.
use super::ValueType;
use crate::{DecodedValue, DslValue, FromValue, NativeDecodeControl, NativeEncodeControl, ToValue, ValueError};
pub(super) fn retire(mut value: ValueType) {
    while let ValueType::List(inner) = value {
        value = *inner
    }
}
pub(super) fn decode(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<ValueType, ValueError> {
    c.scoped_stage(|c| {
        c.begin_stage(0)?;
        let mut cursor = value;
        let mut depth = 0usize;
        let leaf = loop {
            let fields = cursor.object_controlled(c)?;
            DslValue::deny_fields_controlled(fields, &["kind", "of"], c)?;
            let kind = DslValue::field_controlled(fields, "kind", c)?.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "missing type kind"))?;
            let DslValue::String(kind) = kind else { return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "type kind must be text")) };
            let of = DslValue::field_controlled(fields, "of", c)?;
            c.charge(size_of::<ValueType>())?;
            c.step()?;
            if kind == "list" {
                cursor = of.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "missing list child"))?;
                depth = depth.checked_add(1).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::DepthLimit, "type depth overflow"))?;
                continue;
            }
            let value = match kind.as_str() {
                "boolean" => ValueType::Boolean,
                "integer" => ValueType::Integer,
                "decimal" => ValueType::Decimal,
                "text" => ValueType::Text,
                "any" => ValueType::Any,
                "schema" => ValueType::Schema(String::from_value_controlled(of.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "missing type schema"))?, c)?),
                _ => return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "unknown type kind")),
            };
            if kind != "schema" && of.is_some() {
                return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "scalar type has of field"));
            }
            break value;
        };
        let mut output = DecodedValue::new(leaf, retire);
        for _ in 0..depth {
            output = DecodedValue::new(ValueType::List(Box::new(output.take())), retire);
            c.step()?;
        }
        c.checkpoint()?;
        Ok(output.take())
    })
}
pub(super) fn encode(value: &ValueType, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    c.scoped_stage(|c| {
        c.begin_stage(0)?;
        let mut cursor = value;
        let mut depth = 0usize;
        while let ValueType::List(inner) = cursor {
            depth = depth.checked_add(1).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::DepthLimit, "type depth overflow"))?;
            cursor = inner;
            c.step()?;
        }
        let kind = match cursor {
            ValueType::Boolean => "boolean",
            ValueType::Integer => "integer",
            ValueType::Decimal => "decimal",
            ValueType::Text => "text",
            ValueType::Any => "any",
            ValueType::Schema(_) => "schema",
            ValueType::List(_) => unreachable!(),
        };
        let mut fields = DslValue::object_encoding_controlled(if matches!(cursor, ValueType::Schema(_)) { 2 } else { 1 }, c)?;
        let tag = kind.to_value_controlled(c)?;
        DslValue::push_encoding_controlled(fields.get_mut(), "kind", tag, c)?;
        if let ValueType::Schema(schema) = cursor {
            let schema = schema.to_value_controlled(c)?;
            DslValue::push_encoding_controlled(fields.get_mut(), "of", schema, c)?;
        }
        let mut output = DslValue::Object(fields.take()).guard_encoded();
        for _ in 0..depth {
            let mut fields = DslValue::object_encoding_controlled(2, c)?;
            let tag = "list".to_value_controlled(c)?;
            DslValue::push_encoding_controlled(fields.get_mut(), "kind", tag, c)?;
            DslValue::push_encoding_controlled(fields.get_mut(), "of", output.take(), c)?;
            output = DslValue::Object(fields.take()).guard_encoded();
            c.step()?;
        }
        c.checkpoint()?;
        Ok(output.take())
    })
}
