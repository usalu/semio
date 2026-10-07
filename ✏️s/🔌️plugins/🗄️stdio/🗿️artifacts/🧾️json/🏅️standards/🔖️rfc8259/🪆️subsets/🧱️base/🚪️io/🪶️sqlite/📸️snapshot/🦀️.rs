//! 🧾️ Handcrafted RFC8259 value/member/element relations for JSON artifact snapshots only.

use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonMember, JsonSnapshot, JsonValue};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};

enum Parent<'a> { Root, Member(i64, usize, &'a str), Element(i64, usize) }

fn integer(value: usize) -> Result<i64, ValueError> { i64::try_from(value).map_err(|error| ValueError::new(ValueRefusalKind::WorkLimit, error.to_string())) }
fn add(value: &mut usize, amount: usize) -> Result<(), ValueError> { *value = value.checked_add(amount).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "JSON relational size overflow"))?; Ok(()) }
fn add_bytes(value: &mut usize, amount: usize) -> Result<(), ValueError> { *value = value.checked_add(amount).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "JSON relational size overflow"))?; Ok(()) }
fn kind(value: &JsonValue) -> &'static str { match value { JsonValue::Null => "null", JsonValue::Bool { .. } => "boolean", JsonValue::Number { .. } => "number", JsonValue::String { .. } => "string", JsonValue::Array { .. } => "array", JsonValue::Object { .. } => "object" } }

fn measure(snapshot: &JsonSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<usize, ValueError> {
    let mut rows = 1usize; let mut bytes = snapshot.schema.len().checked_add(16).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "JSON document size overflow"))?;
    control.check_value_bytes(bytes)?; let mut stack = vec![&snapshot.value]; let mut visited = 0usize;
    while let Some(value) = stack.pop() {
        add(&mut rows, 1)?; add_bytes(&mut bytes, 8 + kind(value).len())?;
        match value {
            JsonValue::Bool { .. } => add_bytes(&mut bytes, 8)?,
            JsonValue::Number { lexeme } => { add_bytes(&mut bytes, lexeme.len())?; add_bytes(&mut bytes,8)?; control.check_value_bytes(bytes)?; }
            JsonValue::String { value } => add_bytes(&mut bytes, value.len())?,
            JsonValue::Array { items } => { control.check_rows(rows.checked_add(items.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "JSON array size overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "JSON array size overflow"))?)?; add(&mut rows, items.len())?; add_bytes(&mut bytes, items.len().checked_mul(32).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "JSON array size overflow"))?)?; stack.extend(items.iter().rev()); }
            JsonValue::Object { members } => { control.check_rows(rows.checked_add(members.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "JSON object size overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "JSON object size overflow"))?)?; add(&mut rows, members.len())?; add_bytes(&mut bytes, members.len().checked_mul(32).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "JSON object size overflow"))?)?; for member in members.iter().rev() { add_bytes(&mut bytes, member.key.len())?; stack.push(&member.value); } }
            JsonValue::Null => {}
        }
        control.check_rows(rows)?; control.check_value_bytes(bytes)?; visited += 1;
        if visited % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, rows)?; }
    }
    Ok(rows)
}

use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};
impl ArtifactSqliteSnapshot for JsonSnapshot {
    fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::owned_pack::encode(self,encoding,control)}
    fn retire_sqlite_snapshot(self) { crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::owned_pack::retire(self); }
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::owned_pack::preflight(self, control)
    }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        use semio_framework_os_kernel::io_schema::IoError;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(IoError::from_value_error)?;
        if dialect.artifact_kind!="s.stdio.json"||dialect.standard!="rfc8259"{return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"JSON owned snapshot dialect differs from RFC8259")));}
        let row=database.table("json_document").map_err(IoError::from_value_error)?.single_row().map_err(IoError::from_value_error)?;if row.rowid!=1||row.text(1).map_err(IoError::from_value_error)?!=self.schema{return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"JSON owned document identity differs from semantic projection")));}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"i-json"=>crate::standards::v_rfc8259::subsets::i_json::io::check_i_json_conformance_controlled(self,control).map_err(IoError::from_value_error)?,"geojson"=>crate::standards::v_rfc8259::subsets::geojson::io::sqlite::snapshot::check_geojson_conformance_controlled(self,control).map_err(IoError::from_value_error)?,_=>return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"JSON named subset has no owned semantic validator")))};
        Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
        crate::standards::v_rfc8259::subsets::base::io::binary::snapshot::owned_pack::decode(payload,control)
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        (|| -> Result<_, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?; let total = measure(self, control)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("json_document")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Text(self.schema.clone()), SqliteValue::Integer(1)] });
        let mut values = Vec::new(); let mut members = Vec::new(); let mut elements = Vec::new(); let mut stack = vec![(&self.value, Parent::Root)];
        while let Some((value, parent)) = stack.pop() {
            if matches!(value, JsonValue::String { value } if value.len() > 65536) || matches!(value, JsonValue::Number { lexeme } if lexeme.len() > 65536) { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1 + values.len() + members.len() + elements.len(), total)?; }
            let id = integer(values.len() + 1)?; let mut boolean = SqliteValue::Null; let mut number = SqliteValue::Null; let mut string = SqliteValue::Null;let mut query_number=SqliteValue::Null;
            match value {
                JsonValue::Bool { value } => boolean = SqliteValue::Integer(i64::from(*value)),
                JsonValue::Number { lexeme } => {query_number=crate::standards::v_rfc8259::subsets::base::schema::snapshot::number::meaning(lexeme,&mut ||control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1+values.len()+members.len()+elements.len(),total))?.numeric.map(SqliteValue::Real).unwrap_or(SqliteValue::Null);number=SqliteValue::Text(lexeme.clone());},
                JsonValue::String { value } => string = SqliteValue::Text(value.clone()),
                JsonValue::Array { items } => for (ordinal, child) in items.iter().enumerate().rev() { stack.push((child, Parent::Element(id, ordinal))); },
                JsonValue::Object { members } => for (ordinal, member) in members.iter().enumerate().rev() { stack.push((&member.value, Parent::Member(id, ordinal, &member.key))); },
                JsonValue::Null => {}
            }
            values.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), SqliteValue::Text(kind(value).into()), boolean, number, string, query_number] });
            match parent {
                Parent::Root => {}
                Parent::Member(parent, ordinal, key) => { let member_id = integer(members.len() + 1)?; members.push(SqliteRow { rowid: member_id, values: vec![SqliteValue::Integer(member_id), SqliteValue::Integer(parent), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Text(key.into()), SqliteValue::Integer(id)] }); }
                Parent::Element(parent, ordinal) => { let element_id = integer(elements.len() + 1)?; elements.push(SqliteRow { rowid: element_id, values: vec![SqliteValue::Integer(element_id), SqliteValue::Integer(parent), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(id)] }); }
            }
            let completed = 1 + values.len() + members.len() + elements.len(); if values.len() % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, completed, total)?; }
        }
        database.table_mut("json_value")?.rows = values; database.table_mut("json_object_member")?.rows = members; database.table_mut("json_array_element")?.rows = elements;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, total, total)?; Ok(database)
    
        })()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        (|| -> Result<_, ValueError> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        if database.tables.len() != 4 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON snapshot requires exactly its four domain tables")); }
        let total = database.tables.iter().map(|table| table.rows.len()).sum(); let mut checked = 0usize;
        let document = database.table("json_document")?.single_row()?;
        if document.values.len() != 3 || document.rowid != 1 || document.integer(0)? != 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON document requires identifier 1 and three columns")); }
        let root = document.integer(2)?; let mut values = BTreeMap::new();
        for row in &database.table("json_value")?.rows {
            if row.values.len() != 6 || row.integer(0)? != row.rowid || values.insert(row.rowid, row).is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON value identity or column count is invalid")); }
            checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            let kind = row.text(1)?;
            let empty = |index| matches!(row.values.get(index), Some(SqliteValue::Null));
            match kind {
                "null" | "array" | "object" if empty(2) && empty(3) && empty(4) && empty(5) => {}
                "boolean" if empty(3) && empty(4) && empty(5) && matches!(row.integer(2)?, 0 | 1) => {}
                "number" if empty(2) && empty(4) => {let expected=crate::standards::v_rfc8259::subsets::base::schema::snapshot::number::meaning(row.text(3)?,&mut ||control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,total))?.numeric;if match expected{None=>!empty(5),Some(value)=>empty(5)||row.real(5)?!=value}{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON derived numeric value disagrees with its owned lexeme"))}},
                "string" if empty(2) && empty(3) && empty(5) => { row.text(4)?; }
                _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON primitive kind and payload columns disagree")),
            }
        }
        if !values.contains_key(&root) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON document root is dangling")); }
        let mut ownership = BTreeSet::from([root]); let mut links = BTreeMap::<i64, Vec<(i64, i64, Option<&str>)>>::new();
        for (name, required_kind) in [("json_object_member", "object"), ("json_array_element", "array")] {
            let mut ids = BTreeSet::new();
            for row in &database.table(name)?.rows {
                if row.values.len() != (if required_kind == "object" { 5 } else { 4 }) || row.integer(0)? != row.rowid || !ids.insert(row.rowid) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON relationship identity or column count is invalid")); }
                let parent = row.integer(1)?; let ordinal = row.integer(2)?; let key = if required_kind == "object" { Some(row.text(3)?) } else { None }; let child = row.integer(if required_kind == "object" { 4 } else { 3 })?;
                if values.get(&parent).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "JSON relationship parent is dangling"))?.text(1)? != required_kind || !values.contains_key(&child) || ordinal < 0 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON relationship parent, child or ordinal is invalid")); }
                if !ownership.insert(child) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON value has multiple owners or creates a cycle")); }
                links.entry(parent).or_default().push((ordinal, child, key)); checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            }
        }
        for ordered in links.values_mut() { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; ordered.sort_by_key(|(ordinal, _, _)| *ordinal); for (expected, (ordinal, _, _)) in ordered.iter().enumerate() { if *ordinal != integer(expected)? { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON child ordinals must be contiguous and zero-based")); } } }
        if ownership.len() != values.len() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON value has no document or container owner")); }
        let mut visited = BTreeSet::new(); let mut reconstructed = BTreeMap::new(); let mut stack = vec![(root, false)]; let mut completed = 1usize;
        while let Some((id, finish)) = stack.pop() {
            if !finish {
                if !visited.insert(id) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON relationships contain a cycle")); }
                stack.push((id, true)); if let Some(children) = links.get(&id) { stack.extend(children.iter().rev().map(|(_, child, _)| (*child, false))); }
                if visited.len() % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } continue;
            }
            let row = values[&id]; if row.values.iter().any(|value| matches!(value, SqliteValue::Text(text) if text.len() > 65536)) { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } let children = links.remove(&id).unwrap_or_default(); let child_count = children.len();
            let value = match row.text(1)? {
                "null" => JsonValue::Null,
                "boolean" => JsonValue::Bool { value: row.integer(2)? == 1 },
                "number" => JsonValue::Number { lexeme: row.text(3)?.into() },
                "string" => JsonValue::String { value: row.text(4)?.into() },
                "array" => { let mut items = Vec::new(); for (_, child, _) in children { items.push(reconstructed.remove(&child).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "JSON child was not reconstructed"))?); checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } } JsonValue::Array { items } },
                "object" => { let mut members = Vec::new(); for (_, child, key) in children { members.push(JsonMember { key: key.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "JSON object member lacks a key"))?.into(), value: reconstructed.remove(&child).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "JSON child was not reconstructed"))? }); checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } } JsonValue::Object { members } },
                _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON value kind is invalid")),
            };
            reconstructed.insert(id, value); add(&mut completed, 1 + child_count)?; if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; }
        }
        if visited.len() != values.len() || !links.is_empty() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JSON relationships are disconnected or cyclic")); }
        let value = reconstructed.remove(&root).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "JSON root was not reconstructed"))?;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?; Ok(Self { schema: document.text(1)?.into(), value })
    
        })()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

