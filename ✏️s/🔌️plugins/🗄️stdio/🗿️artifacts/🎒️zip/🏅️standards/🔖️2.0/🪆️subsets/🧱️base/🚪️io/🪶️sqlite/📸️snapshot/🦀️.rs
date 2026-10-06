use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};

use semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding;
use crate::standards::v2_0::subsets::base::schema::snapshot::*;
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{Cell, Projection, reconstruct_text}, validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase}, ArtifactSqliteSnapshot};

#[path="🚦️native/🦀️.rs"]
mod native;

fn ordinal(index: usize) -> Result<i64,ValueError> { i64::try_from(index).map_err(|error| ValueError::new(ValueRefusalKind::WorkLimit, error.to_string())) }
fn word(row: &SqliteRow, index: usize) -> Result<u16,ValueError> { u16::try_from(row.integer(index)?).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string())) }
fn boolean(row: &SqliteRow, index: usize) -> Result<bool,ValueError> { match row.integer(index)? { 0 => Ok(false), 1 => Ok(true), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "ZIP boolean must be zero or one")) } }
fn identifiers<'a>(rows:impl IntoIterator<Item=&'a SqliteRow>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeSet<i64>,ValueError>{let mut keys=BTreeSet::new();for(index,row)in rows.into_iter().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,0)?;}keys.insert(row.rowid);}Ok(keys)}
fn emit_bytes(out: &mut Projection<'_, '_>, name: &str, parent: i64, bytes: &[u8]) -> Result<(),ValueError> { for (index, value) in bytes.iter().enumerate() { out.insert(name, &[Cell::Integer(parent), Cell::Integer(ordinal(index)?), Cell::Integer(i64::from(*value))])?; } Ok(()) }
fn emit_fields(out: &mut Projection<'_, '_>, name: &str, byte_name: &str, parent: i64, fields: &[ZipExtraField]) -> Result<(),ValueError> { for (index, field) in fields.iter().enumerate() { let id = out.insert(name, &[Cell::Integer(parent), Cell::Integer(ordinal(index)?), Cell::Integer(i64::from(field.id))])?; emit_bytes(out, byte_name, id, &field.data)?; } Ok(()) }

fn children<'a>(database: &'a SqliteDatabase, name: &str, parents: &BTreeSet<i64>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<&'a SqliteRow>>,ValueError> {
    let mut grouped = BTreeMap::<i64, Vec<&SqliteRow>>::new();
    let table = database.table(name)?;
    let mut identities = BTreeSet::new();
    for (index, row) in table.rows.iter().enumerate() { if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, table.rows.len())?; } if row.rowid <= 0 || row.integer(0)? != row.rowid || !identities.insert(row.rowid) || !parents.contains(&row.integer(1)?) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{name} has an invalid identity or parent"))); } grouped.entry(row.integer(1)?).or_default().push(row); }
    for rows in grouped.values_mut() {
        let mut slots=vec![None;rows.len()];
        for(index,row)in rows.iter().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,rows.len())?;}let index=usize::try_from(row.integer(2)?).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string()))?;let slot=slots.get_mut(index).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("{name} requires contiguous child ordinals")))?;if slot.replace(*row).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{name} requires unique child ordinals")));}}
        let total=slots.len();rows.clear();for(index,row)in slots.into_iter().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,total)?;}rows.push(row.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("{name} requires contiguous child ordinals")))?);}
    }
    Ok(grouped)
}

fn read_bytes(database: &SqliteDatabase, name: &str, parents: &BTreeSet<i64>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<u8>>,ValueError> {
    let mut result = BTreeMap::new();
    for (parent, rows) in children(database, name, parents, control)? { let mut bytes = Vec::new(); for (index, row) in rows.iter().enumerate() { if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, rows.len())?; } bytes.push(u8::try_from(row.integer(3)?).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string()))?); } result.insert(parent, bytes); }
    Ok(result)
}

fn read_fields(database: &SqliteDatabase, name: &str, byte_name: &str, parents: &BTreeSet<i64>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<ZipExtraField>>,ValueError> {
    let grouped = children(database, name, parents, control)?;
    let keys = identifiers(grouped.values().flatten().copied(),control)?;
    let mut bytes = read_bytes(database, byte_name, &keys, control)?;
    let mut result = BTreeMap::new();
    for (parent, rows) in grouped { let mut fields = Vec::new(); for (index, row) in rows.iter().enumerate() { if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, rows.len())?; } fields.push(ZipExtraField { id: word(row, 3)?, data: bytes.remove(&row.rowid).unwrap_or_default() }); } result.insert(parent, fields); }
    Ok(result)
}

fn headers<'a>(database: &'a SqliteDatabase, name: &str, parents: &BTreeSet<i64>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, &'a SqliteRow>,ValueError> {
    let mut rows = BTreeMap::new();
    let table = database.table(name)?;
    for (index, row) in table.rows.iter().enumerate() { if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, table.rows.len())?; } if row.integer(0)? != row.rowid || !parents.contains(&row.rowid) || rows.insert(row.rowid, row).is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{name} has an invalid member identity"))); } }
    if rows.len() != parents.len() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{name} must contain exactly one header per member"))); }
    Ok(rows)
}

fn optional_bytes(present: bool, bytes: &mut BTreeMap<i64, Vec<u8>>, id: i64) -> Result<Option<Vec<u8>>,ValueError> { let bytes = bytes.remove(&id).unwrap_or_default(); if present { Ok(Some(bytes)) } else if bytes.is_empty() { Ok(None) } else { Err(ValueError::new(ValueRefusalKind::InvalidValue, "ZIP absent legacy text cannot own bytes")) } }

impl ArtifactSqliteSnapshot for ZipSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{native::preflight(self,encoding,control)}
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{native::decode(payload,control)}
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{native::encode(self,encoding,control)}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()> {
        (|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.zip"||dialect.standard!="2.0"||!matches!(dialect.subset.as_str(),"*"|"iso21320"){return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("ZIP does not own semantic subset {}",dialect.to_coordinate())));}
        let archive=database.table("zip_archive")?.single_row()?;if archive.rowid!=1||archive.integer(0)?!=1||archive.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "ZIP semantic subset document identity disagrees with its snapshot"));}
        let entries=&database.table("zip_entry")?.rows;for(index,row)in entries.iter().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index,entries.len())?;}word(row,4)?;}
        let diagnostics=if dialect.subset=="iso21320"{crate::standards::v2_0::subsets::iso21320::schema::check_iso21320_conformance_controlled(self,control)?}else{Vec::new()};
        Ok(store::io_schema::IoOutcome{value:(),diagnostics})
    
        })().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)
    }

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        let mut out = Projection::new(Self::SQLITE_SCHEMA, control)?;
        out.insert("zip_archive", &[Cell::Text(&self.schema), Cell::Text(&self.comment), Cell::Integer(i64::from(self.comment_utf8))])?;
        for (index, entry) in self.entries.iter().enumerate() {
            let id = out.insert("zip_entry", &[Cell::Integer(1), Cell::Integer(ordinal(index)?), Cell::Text(&entry.name), Cell::Integer(i64::from(entry.metadata.compression_method)), Cell::Integer(i64::from(entry.metadata.data_descriptor_signature))])?;
            emit_bytes(&mut out, "zip_entry_byte", id, &entry.data)?;
            let local = &entry.metadata.local;
            out.insert_key("zip_local_header", id, &[Cell::Integer(i64::from(local.version_needed)), Cell::Integer(i64::from(local.flags)), Cell::Integer(i64::from(local.modified_time)), Cell::Integer(i64::from(local.modified_date)), Cell::Integer(i64::from(local.unicode_path_legacy_name.is_some()))])?;
            if let Some(bytes) = &local.unicode_path_legacy_name { emit_bytes(&mut out, "zip_local_legacy_name_byte", id, bytes)?; }
            emit_fields(&mut out, "zip_local_extra_field", "zip_local_extra_field_byte", id, &local.extra_fields)?;
            let central = &entry.metadata.central;
            out.insert_key("zip_central_header", id, &[Cell::Integer(i64::from(central.version_made_by)), Cell::Integer(i64::from(central.version_needed)), Cell::Integer(i64::from(central.flags)), Cell::Integer(i64::from(central.modified_time)), Cell::Integer(i64::from(central.modified_date)), Cell::Integer(i64::from(central.unicode_path_legacy_name.is_some())), Cell::Text(&central.comment), Cell::Integer(i64::from(central.unicode_comment_legacy.is_some())), Cell::Integer(i64::from(central.internal_attributes)), Cell::Integer(i64::from(central.external_attributes))])?;
            if let Some(bytes) = &central.unicode_path_legacy_name { emit_bytes(&mut out, "zip_central_legacy_name_byte", id, bytes)?; }
            if let Some(bytes) = &central.unicode_comment_legacy { emit_bytes(&mut out, "zip_central_legacy_comment_byte", id, bytes)?; }
            emit_fields(&mut out, "zip_central_extra_field", "zip_central_extra_field_byte", id, &central.extra_fields)?;
        }
        out.finish()
    
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let archive = database.table("zip_archive")?.single_row()?;
        if archive.integer(0)? != 1 || archive.rowid != 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "ZIP archive identifier must be 1")); }
        let mut entries = children(database, "zip_entry", &BTreeSet::from([1]), control)?;
        let entries = entries.remove(&1).unwrap_or_default();
        let keys = identifiers(entries.iter().copied(),control)?;
        let mut data = read_bytes(database, "zip_entry_byte", &keys, control)?;
        let local_headers = headers(database, "zip_local_header", &keys, control)?;
        let central_headers = headers(database, "zip_central_header", &keys, control)?;
        let mut local_names = read_bytes(database, "zip_local_legacy_name_byte", &keys, control)?;
        let mut central_names = read_bytes(database, "zip_central_legacy_name_byte", &keys, control)?;
        let mut central_comments = read_bytes(database, "zip_central_legacy_comment_byte", &keys, control)?;
        let mut local_fields = read_fields(database, "zip_local_extra_field", "zip_local_extra_field_byte", &keys, control)?;
        let mut central_fields = read_fields(database, "zip_central_extra_field", "zip_central_extra_field_byte", &keys, control)?;
        let mut result = Self { schema: reconstruct_text(control,archive.text(1)?)?, comment: reconstruct_text(control,archive.text(2)?)?, comment_utf8: boolean(archive, 3)?, entries: Vec::new() };
        for (index, row) in entries.iter().enumerate() {
            if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, entries.len())?; }
            let id = row.rowid;
            let local = local_headers[&id];
            let central = central_headers[&id];
            result.entries.push(ZipEntry { name: reconstruct_text(control,row.text(3)?)?, data: data.remove(&id).unwrap_or_default(), metadata: ZipEntryMetadata {
                compression_method: word(row, 4)?, data_descriptor_signature: boolean(row, 5)?,
                local: ZipLocalHeaderMetadata { version_needed: word(local, 1)?, flags: word(local, 2)?, modified_time: word(local, 3)?, modified_date: word(local, 4)?, extra_fields: local_fields.remove(&id).unwrap_or_default(), unicode_path_legacy_name: optional_bytes(boolean(local, 5)?, &mut local_names, id)? },
                central: ZipCentralHeaderMetadata { version_made_by: word(central, 1)?, version_needed: word(central, 2)?, flags: word(central, 3)?, modified_time: word(central, 4)?, modified_date: word(central, 5)?, extra_fields: central_fields.remove(&id).unwrap_or_default(), unicode_path_legacy_name: optional_bytes(boolean(central, 6)?, &mut central_names, id)?, comment: reconstruct_text(control,central.text(7)?)?, unicode_comment_legacy: optional_bytes(boolean(central, 8)?, &mut central_comments, id)?, internal_attributes: word(central, 9)?, external_attributes: u32::try_from(central.integer(10)?).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string()))? },
            } });
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, entries.len(), entries.len())?;
        Ok(result)
    
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

