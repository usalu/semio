//! 🔗️ Bounded ordinal and literal cell construction for explicitly named space entities.
use semio_framework_value::NativeDecodeControl;
use semio_framework_os_kernel::sqlite_snapshot::SqliteRow;
use semio_framework_os_kernel::sqlite_snapshot::SqliteTable;
use semio_framework_value::{ValueError, ValueRefusalKind};
pub(crate) fn invalid(message: impl Into<String>) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}
pub(crate) fn reconstruct<T>(control: &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, operation: impl FnOnce(&mut NativeDecodeControl<'_>) -> Result<T, ValueError>) -> Result<T, ValueError> {
    use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
    let maximum = control.reconstruction_remaining_bytes()?;
    let mut owned = 0;
    let result = control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot, |remaining,checkpoint,allocation| {
        let mut progress = |event: semio_framework_value::native_decoding::NativeDecodeProgress| checkpoint(event.completed,event.total);
        let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(maximum.min(remaining),&mut progress,&mut native_allocation);
        let result = operation(&mut native);
        owned = native.owned_bytes();
        (result,owned)
    });
    control.admit_reconstruction_bytes(owned)?;
    result?
}
pub(crate) fn ordinal(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "space ordinal exceeds SQLite INTEGER"))
}
pub(crate) fn keyed<'a>(table: &'a SqliteTable, width: usize, control: &mut NativeDecodeControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?;
        let mut rows = control.allocate_vec(table.rows.len())?;
        for row in &table.rows {
            if row.values.len() != width || row.integer(0)? != row.rowid {
                return Err(invalid("space body identity differs"));
            }
            rows.push(row);
            control.step()?;
        }
        fn sift(rows: &mut [&SqliteRow], mut root: usize, end: usize, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
            while root < end / 2 {
                control.step()?;
                let mut child = root * 2 + 1;
                if child + 1 < end && rows[child].rowid < rows[child + 1].rowid {
                    child += 1;
                }
                if rows[root].rowid >= rows[child].rowid {
                    break;
                }
                rows.swap(root, child);
                root = child;
            }
            Ok(())
        }
        let count = rows.len();
        for root in (0..count / 2).rev() {
            sift(&mut rows, root, count, control)?;
        }
        for end in (1..count).rev() {
            control.step()?;
            rows.swap(0, end);
            sift(&mut rows, 0, end, control)?;
        }
        for index in 1..count {
            if rows[index - 1].rowid == rows[index].rowid {
                return Err(invalid("space body identity is duplicated"));
            }
            control.step()?;
        }
        Ok(rows)
    })
}
pub(crate) fn ordered<'a>(table: &'a SqliteTable, parent: i64, width: usize, control: &mut NativeDecodeControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
    control.scoped_stage(|control| {
        let keyed = keyed(table, width, control)?;
        control.begin_stage(table.rows.len())?;
        let mut slots = control.allocate_vec::<Option<&SqliteRow>>(table.rows.len())?;
        slots.resize(table.rows.len(), None);
        for row in keyed {
            if row.integer(1)? != parent {
                return Err(invalid("space entity ownership differs"));
            }
            let index = usize::try_from(row.integer(2)?).map_err(|_| invalid("space ordinal must be nonnegative"))?;
            let slot = slots.get_mut(index).ok_or_else(|| invalid("space ordinal must be contiguous"))?;
            if slot.replace(row).is_some() {
                return Err(invalid("space ordinal is duplicated"));
            }
            control.step()?;
        }
        let mut rows = control.allocate_vec(slots.len())?;
        for row in slots {
            rows.push(row.ok_or_else(|| invalid("space ordinal must be contiguous"))?);
        }
        Ok(rows)
    })
}
pub(crate) fn text(row: &SqliteRow, index: usize, control: &mut NativeDecodeControl<'_>) -> Result<String, ValueError> {
    control.copy_text(row.text(index)?)
}
pub(crate) fn optional(row: &SqliteRow, index: usize, control: &mut NativeDecodeControl<'_>) -> Result<Option<String>, ValueError> {
    row.optional_text(index)?.map(|text| control.copy_text(text)).transpose()
}
pub(crate) fn list_count(record: &semio_framework_dsl_record::RecordValue, id: u16) -> Result<usize, ValueError> {
    match record.get(id) {
        Some(semio_framework_dsl_record::FieldValue::List(values)) => Ok(values.len()),
        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "space native field requires its declared list")),
    }
}
pub(crate) fn add(left: usize, right: usize) -> Result<usize, ValueError> {
    left.checked_add(right).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "space row count overflow"))
}
pub(crate) fn grouped<'a>(table: &'a SqliteTable, width: usize, parents: &[&SqliteRow], control: &mut NativeDecodeControl<'_>) -> Result<Vec<Vec<&'a SqliteRow>>, ValueError> {
    control.scoped_stage(|control| {
        let rows = keyed(table, width, control)?;
        let mut counts = control.allocate_vec::<usize>(parents.len())?;
        counts.resize(parents.len(), 0);
        control.begin_stage(rows.len())?;
        for row in &rows {
            let owner = parents.binary_search_by_key(&row.integer(1)?, |parent| parent.rowid).map_err(|_| invalid("space child owner is missing"))?;
            counts[owner] = counts[owner].checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "space row count overflow"))?;
            control.step()?;
        }
        let mut slots = control.allocate_vec::<Vec<Option<&SqliteRow>>>(parents.len())?;
        control.begin_stage(parents.len())?;
        for count in counts {
            let mut group = control.allocate_vec(count)?;
            group.resize(count, None);
            slots.push(group);
            control.step()?;
        }
        control.begin_stage(rows.len())?;
        for row in rows {
            let owner = parents.binary_search_by_key(&row.integer(1)?, |parent| parent.rowid).map_err(|_| invalid("space child owner is missing"))?;
            let index = usize::try_from(row.integer(2)?).map_err(|_| invalid("space child ordinal must be nonnegative"))?;
            let slot = slots[owner].get_mut(index).ok_or_else(|| invalid("space child ordinal must be contiguous"))?;
            if slot.replace(row).is_some() {
                return Err(invalid("space child ordinal is duplicated"));
            }
            control.step()?;
        }
        let mut groups = control.allocate_vec(parents.len())?;
        control.begin_stage(table.rows.len())?;
        for group in slots {
            let mut rows = control.allocate_vec(group.len())?;
            for row in group {
                rows.push(row.ok_or_else(|| invalid("space child ordinal must be contiguous"))?);
                control.step()?;
            }
            groups.push(rows);
        }
        Ok(groups)
    })
}
#[cfg(test)]
#[test]
fn sqlite_snapshot_space_fields_nested_controls_preserve_refusal_authority() {
    use semio_framework_os_kernel::sqlite_snapshot::SqliteValue;
    let corpus: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("🧫️fixtures/⚠️refusal/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let parent = corpus["parent"].as_i64().unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let owner = case["owner"].as_i64().unwrap();
        let table = SqliteTable {
            name: "space_field_child".into(),
            sql: include_str!("🧫️fixtures/⚠️refusal/🗄️.sql").into(),
            rows: vec![SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(owner), SqliteValue::Integer(0), SqliteValue::Text(corpus["label"].as_str().unwrap().into())] }],
        };
        let mut accepted = |_| !case["cancel"].as_bool().unwrap();
        let mut control = semio_framework_value::NativeDecodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize, &mut accepted);
        let result = ordered(&table, parent, 4, &mut control);
        match case["expectedKind"].as_str() {
            Some(kind) => {
                let error = result.unwrap_err();
                assert_eq!(error.kind.as_str(), kind);
                let decorated = error.under(case["id"].as_str().unwrap());
                assert_eq!(decorated.kind.as_str(), kind);
                assert!(decorated.message.starts_with(case["id"].as_str().unwrap()));
            }
            None => {
                let rows = result.unwrap();
                assert_eq!(text(rows[0], 3, &mut control).unwrap().as_bytes().len(), corpus["labelUtf8Bytes"].as_u64().unwrap() as usize);
            }
        }
    }
}
