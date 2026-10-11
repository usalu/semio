//! 📥️ Actual OPC fields reconstruct through admitted identity and parent/ordinal indexes.
use super::super::super::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode, OpcRelationshipOwners};
use super::super::OpcSqliteTables;
use semio_framework_os_kernel::sqlite_snapshot::{
    artifact::{reconstruct_blob, reconstruct_text},
    transfer::reserve,
    SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase,
};
use semio_framework_value::{ValueError, ValueRefusalKind};
type Result<T> = std::result::Result<T, ValueError>;
fn invalid(message: &str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}
fn tick(control: &mut SqliteSnapshotControl<'_>, work: &mut usize) -> Result<()> {
    *work = work.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "OPC reconstruction work overflow"))?;
    if *work % 256 == 0 {
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, *work, 0)?;
    }
    Ok(())
}
fn sort<T>(values: &mut [T], control: &mut SqliteSnapshotControl<'_>, mut compare: impl FnMut(&T, &T) -> Result<std::cmp::Ordering>) -> Result<()> {
    fn sift<T>(values: &mut [T], mut root: usize, end: usize, work: &mut usize, control: &mut SqliteSnapshotControl<'_>, compare: &mut impl FnMut(&T, &T) -> Result<std::cmp::Ordering>) -> Result<()> {
        loop {
            let Some(left) = root.checked_mul(2).and_then(|value| value.checked_add(1)).filter(|value| *value < end) else { return Ok(()) };
            let right = left + 1;
            let next = if right < end && compare(&values[left], &values[right])?.is_lt() { right } else { left };
            tick(control, work)?;
            if !compare(&values[root], &values[next])?.is_lt() {
                return Ok(());
            }
            values.swap(root, next);
            root = next;
        }
    }
    let mut work = 0;
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, values.len())?;
    for root in (0..values.len() / 2).rev() {
        sift(values, root, values.len(), &mut work, control, &mut compare)?;
    }
    for end in (1..values.len()).rev() {
        values.swap(0, end);
        sift(values, 0, end, &mut work, control, &mut compare)?;
    }
    Ok(())
}
fn identities<'a>(database: &'a SqliteDatabase, name: &str, width: usize, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>> {
    let source = &database.table(name)?.rows;
    let mut rows = reserve(source.len(), control)?;
    let mut work = 0;
    for row in source {
        tick(control, &mut work)?;
        if row.rowid <= 0 || row.values.len() != width || row.integer(0)? != row.rowid {
            return Err(invalid("OPC identity or physical row width differs"));
        }
        rows.push(row);
    }
    sort(&mut rows, control, |a, b| Ok(a.rowid.cmp(&b.rowid)))?;
    for pair in rows.windows(2) {
        tick(control, &mut work)?;
        if pair[0].rowid == pair[1].rowid {
            return Err(invalid("OPC identity is duplicated"));
        }
    }
    Ok(rows)
}
fn ordered<'a>(mut rows: Vec<&'a SqliteRow>, package: i64, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>> {
    let mut work = 0;
    for row in &rows {
        tick(control, &mut work)?;
        if row.integer(1)? != package {
            return Err(invalid("OPC entity has another package owner"));
        }
    }
    sort(&mut rows, control, |a, b| Ok(a.integer(2)?.cmp(&b.integer(2)?)))?;
    for (index, row) in rows.iter().enumerate() {
        tick(control, &mut work)?;
        if row.integer(2)? != i64::try_from(index).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "OPC ordinal exceeds SQLite width"))? {
            return Err(invalid("OPC ordinals must be contiguous and unique"));
        }
    }
    Ok(rows)
}
pub(in super::super) fn reconstruct(database: &SqliteDatabase, tables: OpcSqliteTables, control: &mut SqliteSnapshotControl<'_>) -> Result<OpcPackage> {
    control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
    let package = identities(database, tables.package, 2, control)?;
    if package.len() != 1 || package[0].rowid != 1 {
        return Err(invalid("OPC requires one package identity"));
    }
    let id = package[0].rowid;
    let mut output = OpcPackage { comment: reconstruct_text(control, package[0].text(1)?)?, parts: Vec::new(), content_types: OpcContentTypes::default(), relationships: OpcRelationshipOwners::new() };
    let rows = ordered(identities(database, tables.part, 6, control)?, id, control)?;
    output.parts = reserve(rows.len(), control)?;
    for row in rows {
        output.parts.push(OpcPart { path: reconstruct_text(control, row.text(3)?)?, content_type: reconstruct_text(control, row.text(4)?)?, bytes: reconstruct_blob(control, row.blob(5)?)? });
    }
    for (name, target) in [(tables.default_type, &mut output.content_types.defaults), (tables.override_type, &mut output.content_types.overrides)] {
        let rows = ordered(identities(database, name, 5, control)?, id, control)?;
        *target = reserve(rows.len(), control)?;
        for row in rows {
            target.push((reconstruct_text(control, row.text(3)?)?, reconstruct_text(control, row.text(4)?)?));
        }
    }
    let mut owners = identities(database, tables.relationship_owner, 3, control)?;
    let mut relations = identities(database, tables.relationship, 7, control)?;
    let mut work = 0;
    for row in &owners {
        tick(control, &mut work)?;
        if row.integer(1)? != id {
            return Err(invalid("OPC relationship group has another package owner"));
        }
    }
    for row in &relations {
        tick(control, &mut work)?;
        let parent = row.integer(1)?;
        if owners.binary_search_by_key(&parent, |row| row.rowid).is_err() {
            return Err(invalid("OPC relationship has an unknown owner"));
        }
        if row.integer(2)? < 0 {
            return Err(invalid("OPC relationship ordinal must be nonnegative"));
        }
    }
    sort(&mut relations, control, |a, b| Ok(a.integer(1)?.cmp(&b.integer(1)?).then(a.integer(2)?.cmp(&b.integer(2)?))))?;
    let mut groups = reserve(owners.len(), control)?;
    for owner in owners {
        let mut first = 0;
        let mut last = relations.len();
        while first < last {
            tick(control, &mut work)?;
            let middle = first + (last - first) / 2;
            if relations[middle].integer(1)? < owner.rowid {
                first = middle + 1
            } else {
                last = middle
            }
        }
        let start = first;
        last = relations.len();
        while first < last {
            tick(control, &mut work)?;
            let middle = first + (last - first) / 2;
            if relations[middle].integer(1)? <= owner.rowid {
                first = middle + 1
            } else {
                last = middle
            }
        }
        let rows = &relations[start..first];
        let mut values = reserve(rows.len(), control)?;
        for (index, row) in rows.iter().enumerate() {
            tick(control, &mut work)?;
            if row.integer(2)? != i64::try_from(index).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "OPC relationship ordinal exceeds SQLite width"))? {
                return Err(invalid("OPC relationship ordinals must be contiguous and unique"));
            }
            values.push(OpcRelationship {
                id: reconstruct_text(control, row.text(3)?)?,
                rel_type: reconstruct_text(control, row.text(4)?)?,
                target: reconstruct_text(control, row.text(5)?)?,
                target_mode: match row.text(6)? {
                    "internal" => OpcTargetMode::Internal,
                    "external" => OpcTargetMode::External,
                    _ => return Err(invalid("OPC target mode differs")),
                },
            });
        }
        let key = reconstruct_text(control, owner.text(2)?)?;
        groups.push((key, values));
    }
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, work, work)?;
    output.relationships = control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot, |limits, progress, allocation| {
        let mut callback = |event: semio_framework_value::native_decoding::NativeDecodeProgress| progress(event.completed, event.total);
        let mut native_allocation = |request: semio_framework_value::native_decoding::NativeDecodeAllocation| allocation(request.bytes);
        let mut native = semio_framework_value::NativeDecodeControl::new_forwarded(limits, &mut callback, &mut native_allocation);
        let result = OpcRelationshipOwners::adopt_admitted(groups, &mut native);
        (result, native.owned_bytes())
    })??;
    Ok(output)
}
