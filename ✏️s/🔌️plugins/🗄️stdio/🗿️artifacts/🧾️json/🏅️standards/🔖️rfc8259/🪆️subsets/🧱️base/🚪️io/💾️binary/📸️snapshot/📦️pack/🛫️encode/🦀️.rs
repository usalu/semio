//! 🛫️ Paid breadth-first projection of the authored JSON logical record.
use super::*;
use semio_framework_value::NativeEncodeControl;
use semio_framework_value::{ValueError,ValueRefusalKind};

fn push<T>(values: &mut Vec<T>, value: T, control: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    if values.len() == values.capacity() {
        let capacity = values.capacity().max(4).checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "JSON logical frontier capacity overflow"))?;
        let additional = capacity - values.capacity();
        let bytes = additional.checked_mul(std::mem::size_of::<T>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "JSON logical frontier byte size overflow"))?;
        control.charge(bytes)?;
        values.try_reserve_exact(additional).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "JSON logical frontier allocation failed"))?;
        control.checkpoint()?;
    }
    values.push(value);
    Ok(())
}

pub(super) fn project(source: &JsonSnapshot, control: &mut NativeEncodeControl<'_>, maximum_rows: usize) -> Result<Snapshot, ValueError> {
    control.begin_stage(0)?;
    if maximum_rows < 2 { return Err(ValueError::new(ValueRefusalKind::WorkLimit, "JSON logical rows exceed caller limit")); }
    let mut pending = control.allocate_vec::<&JsonValue>(1)?;
    pending.push(&source.value);
    let mut nodes = Vec::new();
    let mut rows = 2usize;
    let mut position = 0usize;
    while position < pending.len() {
        control.step()?;
        let value = pending[position];
        let children = match value { JsonValue::Array { items } => items.len(), JsonValue::Object { members } => members.len(), _ => 0 };
        rows = rows.checked_add(children.checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "JSON logical row count overflow"))?).filter(|rows| *rows <= maximum_rows).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "JSON logical rows exceed caller limit"))?;
        let mut node = Node { kind: Kind::Null, boolean: None, number_lexeme: None, string_value: None, items: Vec::new(), members: Vec::new() };
        match value {
            JsonValue::Null => {}
            JsonValue::Bool { value } => { node.kind = Kind::Boolean; node.boolean = Some(*value); }
            JsonValue::Number { lexeme } => { node.kind = Kind::Number; node.number_lexeme = Some(control.copy_text(lexeme)?); }
            JsonValue::String { value } => { node.kind = Kind::String; node.string_value = Some(control.copy_text(value)?); }
            JsonValue::Array { items } => {
                node.kind = Kind::Array;
                node.items = control.allocate_vec(items.len())?;
                for child in items {
                    control.step()?;
                    node.items.push(u64::try_from(pending.len()).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "JSON logical index overflow"))?);
                    push(&mut pending, child, control)?;
                }
            }
            JsonValue::Object { members } => {
                node.kind = Kind::Object;
                node.members = control.allocate_vec(members.len())?;
                for member in members {
                    control.step()?;
                    node.members.push(Member { key: control.copy_text(&member.key)?, value: u64::try_from(pending.len()).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "JSON logical index overflow"))? });
                    push(&mut pending, &member.value, control)?;
                }
            }
        }
        push(&mut nodes, node, control)?;
        position += 1;
    }
    let schema = control.copy_text(&source.schema)?;
    control.checkpoint()?;
    Ok(Snapshot { schema, nodes })
}

pub(super) fn encode(source: &JsonSnapshot, encoding: store::sqlite_snapshot::SnapshotEncoding, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, ValueError> {
    let maximum_rows = control.limits().max_rows;
    store::encode_sqlite_snapshot_record_native(encoding, "stdio.json", Snapshot::__dsl_spec_producer(), |native| {
        let projected = project(source, native, maximum_rows)?;
        projected.__dsl_to_record_controlled(native)
    }, control,native_owner)
}
