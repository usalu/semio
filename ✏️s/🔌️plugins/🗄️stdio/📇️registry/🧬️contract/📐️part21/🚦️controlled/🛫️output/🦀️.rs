//! 🛫️ Canonical Part21 intrinsic output admits keys, slots and each literal field independently.
use super::*;

pub(super) fn values(items: &[Part21Value], control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(items.len())?;
        let mut output = Vec::<DslValue>::guard_decoded(control.allocate_vec(items.len())?);
        for item in items {
            output.get_mut().push(encode_value(item, control)?);
            control.step()?;
        }
        Ok(DslValue::Array(output.take()))
    })
}

pub(crate) fn encode_value(value: &Part21Value, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    control.scoped_depth(64, |control| {
        control.scoped_stage(|control| {
            let (kind, count) = match value {
                Part21Value::Ref(_) => ("ref", 2),
                Part21Value::Str(_) => ("str", 2),
                Part21Value::Enum(_) => ("enum", 2),
                Part21Value::Int(_) => ("int", 2),
                Part21Value::Real(_) => ("real", 2),
                Part21Value::List(_) => ("list", 2),
                Part21Value::Typed { .. } => ("typed", 3),
                Part21Value::Unset => ("unset", 1),
                Part21Value::Derived => ("derived", 1),
            };
            control.begin_stage(count)?;
            let mut output = DslValue::object_encoding_controlled(count, control)?;
            let kind = DslValue::String(control.copy_text(kind)?);
            DslValue::push_encoding_controlled(output.get_mut(), "kind", kind, control)?;
            control.step()?;
            match value {
                Part21Value::Ref(value) => put(output.get_mut(), "value", value.to_value_controlled(control)?, control)?,
                Part21Value::Str(value) | Part21Value::Enum(value) => put(output.get_mut(), "value", value.to_value_controlled(control)?, control)?,
                Part21Value::Int(value) => put(output.get_mut(), "value", value.to_value_controlled(control)?, control)?,
                Part21Value::Real(value) => put(output.get_mut(), "value", value.to_value_controlled(control)?, control)?,
                Part21Value::List(items) => put(output.get_mut(), "values", values(items, control)?, control)?,
                Part21Value::Typed { name, items } => {
                    put(output.get_mut(), "typeName", name.to_value_controlled(control)?, control)?;
                    put(output.get_mut(), "values", values(items, control)?, control)?;
                }
                Part21Value::Unset | Part21Value::Derived => {}
            }
            Ok(DslValue::Object(output.take()))
        })
    })
}

fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue, control: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    DslValue::push_encoding_controlled(entries, key, value, control)?;
    control.step()
}

pub(crate) fn encode_instance(instance: &Part21Instance, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(2)?;
        let mut output = DslValue::object_encoding_controlled(2, control)?;
        put(output.get_mut(), "id", instance.id.to_value_controlled(control)?, control)?;
        let entities = control.scoped_stage(|control| {
            control.begin_stage(instance.entities.len())?;
            let mut entities = Vec::<DslValue>::guard_decoded(control.allocate_vec(instance.entities.len())?);
            for (name, arguments) in &instance.entities {
                let entity = control.scoped_stage(|control| {
                    control.begin_stage(2)?;
                    let mut entity = DslValue::object_encoding_controlled(2, control)?;
                    put(entity.get_mut(), "typeName", name.to_value_controlled(control)?, control)?;
                    put(entity.get_mut(), "arguments", values(arguments, control)?, control)?;
                    Ok::<_, ValueError>(DslValue::Object(entity.take()))
                })?;
                entities.get_mut().push(entity);
                control.step()?;
            }
            Ok::<_, ValueError>(DslValue::Array(entities.take()))
        })?;
        put(output.get_mut(), "entities", entities, control)?;
        Ok(DslValue::Object(output.take()))
    })
}
