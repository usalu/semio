//! 🛬️ Borrowed Part21 records admit copies and explicit collection slots before construction.
use super::*;

pub(super) fn values(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Vec<Part21Value>, ValueError> {
    let DslValue::Array(items) = value else {
        return Err(invalid("Part21 arguments require an array"));
    };
    control.scoped_stage(|control| {
        control.begin_stage(items.len())?;
        let mut output = DecodedValue::new(control.allocate_vec(items.len())?, retire_values);
        for item in items {
            output.get_mut().push(decode_value(item, control)?);
            control.step()?;
        }
        Ok(output.take())
    })
}

pub(crate) fn decode_value(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Part21Value, ValueError> {
    control.scoped_depth(64, |control| {
        control.scoped_stage(|control| {
            let entries = value.object_controlled(control)?;
            let kind = required(entries, "kind", control)?.as_str().ok_or_else(|| invalid("Part21 kind requires Text"))?;
            let allowed: &[&str] = match kind {
                "ref" | "str" | "enum" | "int" | "real" => &["kind", "value"],
                "list" => &["kind", "values"],
                "typed" => &["kind", "typeName", "values"],
                "unset" | "derived" => &["kind"],
                _ => return Err(invalid("unknown Part21 value kind")),
            };
            DslValue::deny_fields_controlled(entries, allowed, control)?;
            control.begin_stage(1)?;
            let result = match kind {
                "ref" => Part21Value::Ref(u64::from_value_controlled(required(entries, "value", control)?, control)?),
                "str" => Part21Value::Str(String::from_value_controlled(required(entries, "value", control)?, control)?),
                "enum" => Part21Value::Enum(String::from_value_controlled(required(entries, "value", control)?, control)?),
                "int" => Part21Value::Int(i64::from_value_controlled(required(entries, "value", control)?, control)?),
                "real" => Part21Value::Real(Part21Decimal::from_value_controlled(required(entries, "value", control)?, control)?),
                "list" => Part21Value::List(values(required(entries, "values", control)?, control)?),
                "typed" => {
                    let name = DecodedValue::new(String::from_value_controlled(required(entries, "typeName", control)?, control)?, retire_name);
                    let items = values(required(entries, "values", control)?, control)?;
                    Part21Value::Typed { name: name.take(), items }
                }
                "unset" => Part21Value::Unset,
                "derived" => Part21Value::Derived,
                _ => unreachable!(),
            };
            let result = DecodedValue::new(result, retire_value);
            control.step()?;
            Ok(result.take())
        })
    })
}

pub(crate) fn decode_instance(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Part21Instance, ValueError> {
    control.scoped_stage(|control| {
        let entries = value.object_controlled(control)?;
        DslValue::deny_fields_controlled(entries, &["id", "entities"], control)?;
        let id = u64::from_value_controlled(required(entries, "id", control)?, control)?;
        let DslValue::Array(records) = required(entries, "entities", control)? else {
            return Err(invalid("Part21 entities require an array"));
        };
        control.begin_stage(records.len())?;
        let mut entities = DecodedValue::new(control.allocate_vec(records.len())?, retire_entities);
        for record in records {
            let fields = record.object_controlled(control)?;
            DslValue::deny_fields_controlled(fields, &["typeName", "arguments"], control)?;
            let name = DecodedValue::new(String::from_value_controlled(required(fields, "typeName", control)?, control)?, retire_name);
            let arguments = values(required(fields, "arguments", control)?, control)?;
            entities.get_mut().push((name.take(), arguments));
            control.step()?;
        }
        Ok(Part21Instance { id, entities: entities.take() })
    })
}
