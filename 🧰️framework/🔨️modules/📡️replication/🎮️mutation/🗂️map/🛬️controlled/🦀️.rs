//! 🛬️ Shared map codecs retain typed children under explicit allocation, progress and cancellation.
use super::*;
use semio_framework_value::{DecodedValue, NativeDecodeControl, NativeEncodeControl, ValueRefusalKind};

pub(super) fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
pub(super) fn presence(value: Option<&str>) -> Result<MapPresence, ValueError> {
    match value { Some("any") => Ok(MapPresence::Any), Some("present") => Ok(MapPresence::Present), Some("absent") => Ok(MapPresence::Absent), Some("never") => Ok(MapPresence::Never), _ => Err(invalid("unknown map presence requirement")) }
}
pub(super) fn compatible<V>(presence: MapPresence, operation: &MapEntryOperation<V>) -> Result<(), ValueError> {
    if (presence == MapPresence::Never) != matches!(operation, MapEntryOperation::Reject) { Err(invalid("unsatisfiable map changes must be explicit rejections")) } else { Ok(()) }
}
pub(super) fn charge_entry<V>(control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    let bytes = std::mem::size_of::<(String, MapEntryDelta<V>)>().checked_mul(11).and_then(|bytes| bytes.checked_add(256)).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "map entry ownership size overflow"))?;
    control.charge(bytes)
}
fn fields<'a>(value: &'a DslValue, expected: &[&str], control: &mut NativeDecodeControl<'_>) -> Result<&'a [(String, DslValue)], ValueError> {
    let DslValue::Object(fields) = value else { return Err(invalid("map delta value must be a record")); };
    if fields.len() != expected.len() { return Err(invalid("map delta record has incorrect fields")); }
    control.scoped_stage(|control| {
        control.begin_stage(fields.len())?;
        let mut seen = 0u8;
        for (key, _) in fields {
            let index = expected.iter().position(|field| *field == key).ok_or_else(|| invalid("unknown map delta record field"))?;
            if seen & (1 << index) != 0 { return Err(invalid("duplicate map delta record field")); }
            seen |= 1 << index;
            control.step()?;
        }
        Ok(fields.as_slice())
    })
}
fn get<'a>(fields: &'a [(String, DslValue)], key: &str) -> &'a DslValue { &fields.iter().find(|(name, _)| name == key).expect("validated map field").1 }

pub(super) fn decode<V: FromValue>(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<MapDelta<V>, ValueError> {
    control.scoped_depth(64, |control| {
        let root = fields(value, &["entries"], control)?;
        let DslValue::Object(entries) = get(root, "entries") else { return Err(invalid("map delta entries must be a keyed object")); };
        control.scoped_stage(|control| {
            control.begin_stage(entries.len())?;
            let mut output = MapDelta::<V>::default().guard_decoded();
            for (key, value) in entries {
                if output.get().entries.contains_key(key) { return Err(invalid("duplicate map delta key")); }
                charge_entry::<V>(control)?;
                let key = control.copy_text(key)?;
                let entry = fields(value, &["precondition", "operation"], control)?;
                let precondition = presence(get(entry, "precondition").as_str())?;
                let raw = get(entry, "operation");
                let kind = raw.as_object().and_then(|fields| fields.iter().find(|(key, _)| key == "kind")).and_then(|(_, value)| value.as_str());
                let operation_fields = fields(raw, if kind == Some("set") { &["kind", "value"] } else { &["kind"] }, control)?;
                let operation = match kind {
                    Some("set") if precondition != MapPresence::Never => MapEntryOperation::Set(control.scoped_stage(|control| V::from_value_controlled(get(operation_fields, "value"), control))?),
                    Some("remove") if precondition != MapPresence::Never => MapEntryOperation::Remove,
                    Some("reject") if precondition == MapPresence::Never => MapEntryOperation::Reject,
                    Some("set" | "remove" | "reject") => return Err(invalid("unsatisfiable map changes must be explicit rejections")),
                    _ => return Err(invalid("unknown map operation")),
                };
                output.get_mut().entries.insert(key, MapEntryDelta { precondition, operation });
                control.step()?;
            }
            Ok(output.take())
        })
    })
}

fn object(count: usize, control: &mut NativeEncodeControl<'_>) -> Result<DecodedValue<Vec<(String, DslValue)>>, ValueError> {
    Ok(DecodedValue::new(control.allocate_vec(count)?, |fields| { for (_, value) in fields { DslValue::retire_decoded(value); } }))
}
fn text(value: &str, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { Ok(DslValue::String(control.copy_text(value)?)) }

pub(super) fn encode<V: ToValue>(value: &MapDelta<V>, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    control.scoped_depth(64, |control| control.scoped_stage(|control| {
        control.begin_stage(value.entries.len())?;
        let mut root = object(1, control)?;
        let root_key = control.copy_text("entries")?;
        let mut entries = object(value.entries.len(), control)?;
        for (key, entry) in &value.entries {
            let key = control.copy_text(key)?;
            let mut fields = object(2, control)?;
            let operation_key = control.copy_text("operation")?;
            let mut operation = object(if matches!(entry.operation, MapEntryOperation::Set(_)) { 2 } else { 1 }, control)?;
            let kind_key = control.copy_text("kind")?;
            let kind = text(match entry.operation { MapEntryOperation::Set(_) => "set", MapEntryOperation::Remove => "remove", MapEntryOperation::Reject => "reject" }, control)?;
            operation.get_mut().push((kind_key, kind));
            if let MapEntryOperation::Set(value) = &entry.operation {
                let key = control.copy_text("value")?;
                let child = control.scoped_stage(|control| value.to_value_controlled(control))?;
                operation.get_mut().push((key, child));
            }
            fields.get_mut().push((operation_key, DslValue::Object(operation.take())));
            let precondition_key = control.copy_text("precondition")?;
            let precondition = text(entry.precondition.name(), control)?;
            fields.get_mut().push((precondition_key, precondition));
            entries.get_mut().push((key, DslValue::Object(fields.take())));
            control.step()?;
        }
        root.get_mut().push((root_key, DslValue::Object(entries.take())));
        Ok(DslValue::Object(root.take()))
    }))
}
