//! 🧾️ Handcrafted RFC8259 value/member/element relations for JSON artifact snapshots only.

use super::{JsonMember, JsonSnapshot, JsonValue};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};

enum Parent<'a> { Root, Member(i64, usize, &'a str), Element(i64, usize) }

fn integer(value: usize) -> Result<i64, String> { i64::try_from(value).map_err(|error| error.to_string()) }
fn add(value: &mut usize, amount: usize) -> Result<(), String> { *value = value.checked_add(amount).ok_or("JSON relational size overflow")?; Ok(()) }
fn kind(value: &JsonValue) -> &'static str { match value { JsonValue::Null => "null", JsonValue::Bool { .. } => "boolean", JsonValue::Number { .. } => "number", JsonValue::String { .. } => "string", JsonValue::Array { .. } => "array", JsonValue::Object { .. } => "object" } }

fn digits(bytes:&[u8],cursor:&mut usize,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,completed:usize,total:usize)->Result<bool,String>{let start=*cursor;while bytes.get(*cursor).is_some_and(u8::is_ascii_digit){*cursor+=1;if *cursor%65536==0{control.checkpoint(phase,completed,total)?;}}Ok(*cursor>start)}
fn numeric(lexeme:&str,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,completed:usize,total:usize)->Result<Option<f64>,String>{
 let bytes=lexeme.as_bytes();let mut cursor=0;if bytes.len()>65536{control.checkpoint(phase,completed,total)?;}
 if bytes.get(cursor)==Some(&b'-'){cursor+=1;}
 if bytes.get(cursor)==Some(&b'0'){cursor+=1;}else if bytes.get(cursor).is_some_and(|byte|(b'1'..=b'9').contains(byte)){if !digits(bytes,&mut cursor,control,phase,completed,total)?{return Ok(None)}}else{return Ok(None)}
 if bytes.get(cursor)==Some(&b'.'){cursor+=1;if !digits(bytes,&mut cursor,control,phase,completed,total)?{return Ok(None)}}
 if bytes.get(cursor).is_some_and(|byte|*byte==b'e'||*byte==b'E'){cursor+=1;if bytes.get(cursor).is_some_and(|byte|*byte==b'+'||*byte==b'-'){cursor+=1;}if !digits(bytes,&mut cursor,control,phase,completed,total)?{return Ok(None)}}
 if cursor!=bytes.len(){return Ok(None)}Ok(lexeme.parse::<f64>().ok().filter(|value|value.is_finite()).map(|value|if value==0.0{0.0}else{value}))
}

fn measure(snapshot: &JsonSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<usize, String> {
    let mut rows = 1usize; let mut bytes = snapshot.schema.len().checked_add(16).ok_or("JSON document size overflow")?;
    control.check_value_bytes(bytes)?; let mut stack = vec![&snapshot.value]; let mut visited = 0usize;
    while let Some(value) = stack.pop() {
        add(&mut rows, 1)?; add(&mut bytes, 8 + kind(value).len())?;
        match value {
            JsonValue::Bool { .. } => add(&mut bytes, 8)?,
            JsonValue::Number { lexeme } => { add(&mut bytes, lexeme.len())?; add(&mut bytes,8)?; control.check_value_bytes(bytes)?; }
            JsonValue::String { value } => add(&mut bytes, value.len())?,
            JsonValue::Array { items } => { control.check_rows(rows.checked_add(items.len().checked_mul(2).ok_or("JSON array size overflow")?).ok_or("JSON array size overflow")?)?; add(&mut rows, items.len())?; add(&mut bytes, items.len().checked_mul(32).ok_or("JSON array size overflow")?)?; stack.extend(items.iter().rev()); }
            JsonValue::Object { members } => { control.check_rows(rows.checked_add(members.len().checked_mul(2).ok_or("JSON object size overflow")?).ok_or("JSON object size overflow")?)?; add(&mut rows, members.len())?; add(&mut bytes, members.len().checked_mul(32).ok_or("JSON object size overflow")?)?; for member in members.iter().rev() { add(&mut bytes, member.key.len())?; stack.push(&member.value); } }
            JsonValue::Null => {}
        }
        control.check_rows(rows)?; control.check_value_bytes(bytes)?; visited += 1;
        if visited % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, rows)?; }
    }
    Ok(rows)
}

impl ArtifactSqliteSnapshot for JsonSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> {
        use semio_framework_os_kernel::sqlite_snapshot::{artifact::NativeEncodingBound, SnapshotEncoding};
        let mut bound = NativeEncodingBound::new(control)?; bound.add(1024)?;
        let mut stack = vec![(&self.value, 0usize)]; let mut rows = 1usize;
        while let Some((value, depth)) = stack.pop() {
            let indentation = if encoding == SnapshotEncoding::Text { depth.checked_mul(16).ok_or("JSON native indentation overflow")? } else { 0 };
            bound.add(indentation.checked_add(64).ok_or("JSON native node size overflow")?)?;
            match value {
                JsonValue::Null | JsonValue::Bool { .. } => {},
                JsonValue::Number { lexeme } => bound.repeated(lexeme.len(), 4)?,
                JsonValue::String { value } => bound.repeated(value.len(), 24)?,
                JsonValue::Array { items } => {
                    rows = rows.checked_add(items.len()).ok_or("JSON native row count overflow")?; bound.check_rows(rows)?;
                    bound.repeated(items.len(), 32)?; let next = depth.checked_add(1).ok_or("JSON native depth overflow")?;
                    stack.extend(items.iter().rev().map(|child| (child, next)));
                },
                JsonValue::Object { members } => {
                    rows = rows.checked_add(members.len()).ok_or("JSON native row count overflow")?; bound.check_rows(rows)?;
                    bound.repeated(members.len(), 32)?; let next = depth.checked_add(1).ok_or("JSON native depth overflow")?;
                    for member in members.iter().rev() { bound.repeated(member.key.len(), 24)?; stack.push((&member.value, next)); }
                },
            }
        }
        bound.finish()
    }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.json"||dialect.standard!="rfc8259"{return Err(String::from("JSON owned snapshot dialect differs from RFC8259").into());}
        let row=database.table("json_document")?.single_row()?;if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("JSON owned document identity differs from semantic projection").into());}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"i-json"=>crate::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance_controlled(self,control)?,"geojson"=>crate::standards::v_rfc8259::subsets::geojson::schema::check_geojson_conformance_controlled(self,control)?,_=>return Err(String::from("JSON named subset has no owned semantic validator").into())};
        Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?; let total = measure(self, control)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        database.table_mut("json_document")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Text(self.schema.clone()), SqliteValue::Integer(1)] });
        let mut values = Vec::new(); let mut members = Vec::new(); let mut elements = Vec::new(); let mut stack = vec![(&self.value, Parent::Root)];
        while let Some((value, parent)) = stack.pop() {
            if matches!(value, JsonValue::String { value } if value.len() > 65536) || matches!(value, JsonValue::Number { lexeme } if lexeme.len() > 65536) { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1 + values.len() + members.len() + elements.len(), total)?; }
            let id = integer(values.len() + 1)?; let mut boolean = SqliteValue::Null; let mut number = SqliteValue::Null; let mut string = SqliteValue::Null;let mut query_number=SqliteValue::Null;
            match value {
                JsonValue::Bool { value } => boolean = SqliteValue::Integer(i64::from(*value)),
                JsonValue::Number { lexeme } => {query_number=numeric(lexeme,control,SqliteSnapshotPhase::ProjectSnapshot,1+values.len()+members.len()+elements.len(),total)?.map(SqliteValue::Real).unwrap_or(SqliteValue::Null);number=SqliteValue::Text(lexeme.clone());},
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
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        if database.tables.len() != 4 { return Err("JSON snapshot requires exactly its four domain tables".into()); }
        let total = database.tables.iter().map(|table| table.rows.len()).sum(); let mut checked = 0usize;
        let document = database.table("json_document")?.single_row()?;
        if document.values.len() != 3 || document.rowid != 1 || document.integer(0)? != 1 { return Err("JSON document requires identifier 1 and three columns".into()); }
        let root = document.integer(2)?; let mut values = BTreeMap::new();
        for row in &database.table("json_value")?.rows {
            if row.values.len() != 6 || row.integer(0)? != row.rowid || values.insert(row.rowid, row).is_some() { return Err("JSON value identity or column count is invalid".into()); }
            checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            let kind = row.text(1)?;
            let empty = |index| matches!(row.values.get(index), Some(SqliteValue::Null));
            match kind {
                "null" | "array" | "object" if empty(2) && empty(3) && empty(4) && empty(5) => {}
                "boolean" if empty(3) && empty(4) && empty(5) && matches!(row.integer(2)?, 0 | 1) => {}
                "number" if empty(2) && empty(4) => {let expected=numeric(row.text(3)?,control,SqliteSnapshotPhase::ReconstructSnapshot,0,total)?;if match expected{None=>!empty(5),Some(value)=>empty(5)||row.real(5)?!=value}{return Err("JSON derived numeric value disagrees with its owned lexeme".into())}},
                "string" if empty(2) && empty(3) && empty(5) => { row.text(4)?; }
                _ => return Err("JSON primitive kind and payload columns disagree".into()),
            }
        }
        if !values.contains_key(&root) { return Err("JSON document root is dangling".into()); }
        let mut ownership = BTreeSet::from([root]); let mut links = BTreeMap::<i64, Vec<(i64, i64, Option<&str>)>>::new();
        for (name, required_kind) in [("json_object_member", "object"), ("json_array_element", "array")] {
            let mut ids = BTreeSet::new();
            for row in &database.table(name)?.rows {
                if row.values.len() != (if required_kind == "object" { 5 } else { 4 }) || row.integer(0)? != row.rowid || !ids.insert(row.rowid) { return Err("JSON relationship identity or column count is invalid".into()); }
                let parent = row.integer(1)?; let ordinal = row.integer(2)?; let key = if required_kind == "object" { Some(row.text(3)?) } else { None }; let child = row.integer(if required_kind == "object" { 4 } else { 3 })?;
                if values.get(&parent).ok_or("JSON relationship parent is dangling")?.text(1)? != required_kind || !values.contains_key(&child) || ordinal < 0 { return Err("JSON relationship parent, child or ordinal is invalid".into()); }
                if !ownership.insert(child) { return Err("JSON value has multiple owners or creates a cycle".into()); }
                links.entry(parent).or_default().push((ordinal, child, key)); checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            }
        }
        for ordered in links.values_mut() { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; ordered.sort_by_key(|(ordinal, _, _)| *ordinal); for (expected, (ordinal, _, _)) in ordered.iter().enumerate() { if *ordinal != integer(expected)? { return Err("JSON child ordinals must be contiguous and zero-based".into()); } } }
        if ownership.len() != values.len() { return Err("JSON value has no document or container owner".into()); }
        let mut visited = BTreeSet::new(); let mut reconstructed = BTreeMap::new(); let mut stack = vec![(root, false)]; let mut completed = 1usize;
        while let Some((id, finish)) = stack.pop() {
            if !finish {
                if !visited.insert(id) { return Err("JSON relationships contain a cycle".into()); }
                stack.push((id, true)); if let Some(children) = links.get(&id) { stack.extend(children.iter().rev().map(|(_, child, _)| (*child, false))); }
                if visited.len() % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } continue;
            }
            let row = values[&id]; if row.values.iter().any(|value| matches!(value, SqliteValue::Text(text) if text.len() > 65536)) { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } let children = links.remove(&id).unwrap_or_default(); let child_count = children.len();
            let value = match row.text(1)? {
                "null" => JsonValue::Null,
                "boolean" => JsonValue::Bool { value: row.integer(2)? == 1 },
                "number" => JsonValue::Number { lexeme: row.text(3)?.into() },
                "string" => JsonValue::String { value: row.text(4)?.into() },
                "array" => { let mut items = Vec::new(); for (_, child, _) in children { items.push(reconstructed.remove(&child).ok_or("JSON child was not reconstructed")?); checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } } JsonValue::Array { items } },
                "object" => { let mut members = Vec::new(); for (_, child, key) in children { members.push(JsonMember { key: key.ok_or("JSON object member lacks a key")?.into(), value: reconstructed.remove(&child).ok_or("JSON child was not reconstructed")? }); checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } } JsonValue::Object { members } },
                _ => return Err("JSON value kind is invalid".into()),
            };
            reconstructed.insert(id, value); add(&mut completed, 1 + child_count)?; if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; }
        }
        if visited.len() != values.len() || !links.is_empty() { return Err("JSON relationships are disconnected or cyclic".into()); }
        let value = reconstructed.remove(&root).ok_or("JSON root was not reconstructed")?;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?; Ok(Self { schema: document.text(1)?.into(), value })
    }
}
