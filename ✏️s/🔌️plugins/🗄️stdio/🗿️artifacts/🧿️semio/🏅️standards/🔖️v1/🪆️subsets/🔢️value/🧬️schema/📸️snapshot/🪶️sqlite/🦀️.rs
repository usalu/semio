//! 🔢️ Typed Semio value primitives, structural ownership and referential graph nodes.
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use super::{SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot, sqlite_snapshot::{artifact::{Cell, Projection, reconstruct_text, reconstruct_blob}, validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase}};
use std::collections::{BTreeMap, BTreeSet};

fn number(value: usize) -> Result<i64, String> { i64::try_from(value).map_err(|error| error.to_string()) }
fn identity(row: &SqliteRow, columns: usize) -> Result<(), String> { if row.rowid <= 0 || row.integer(0)? != row.rowid || row.values.len() != columns { Err("invalid Semio value identity or columns".into()) } else { Ok(()) } }
fn null(row: &SqliteRow, index: usize) -> Result<(), String> { if row.values.get(index) == Some(&SqliteValue::Null) { Ok(()) } else { Err("unexpected Semio value variant field".into()) } }
/// 🧩️ Explicit entity names for genuinely shared, typed SemioValue fields.
#[derive(Clone, Copy)]
pub struct ValueSqliteTables { pub value: &'static str, pub list_element: &'static str, pub map_entry: &'static str }
pub const VALUE_TABLES: ValueSqliteTables = ValueSqliteTables { value: "semio_value_value", list_element: "semio_value_list_element", map_entry: "semio_value_map_entry" };

enum Owner<'a> { Root, List(i64, usize), Map(i64, usize, &'a str) }

/// 🌱️ Projects exactly the owned SemioValue variants and their structural links.
pub fn project_value_tree(value: &SemioValue, tables: ValueSqliteTables, nodes: Option<&BTreeMap<&str, i64>>, projection: &mut Projection<'_, '_>) -> Result<i64, String> {
    let mut pending = vec![(value, Owner::Root)]; let mut root = 0;
    while let Some((value, owner)) = pending.pop() {
        projection.checkpoint()?; let mut cells = [Cell::Null; 7];
        cells[0] = Cell::Text(match value { SemioValue::Null => "null", SemioValue::Bool { .. } => "bool", SemioValue::Int { .. } => "int", SemioValue::Float { .. } => "float", SemioValue::Str { .. } => "str", SemioValue::Bytes { .. } => "bytes", SemioValue::List { .. } => "list", SemioValue::Map { .. } => "map", SemioValue::Ref { .. } => "ref" });
        match value { SemioValue::Bool { value } => cells[1] = Cell::Integer(i64::from(*value)), SemioValue::Int { lexeme } => cells[2] = Cell::Text(lexeme), SemioValue::Float { lexeme } => cells[3] = Cell::Text(lexeme), SemioValue::Str { value } => cells[4] = Cell::Text(value), SemioValue::Bytes { value } => cells[5] = Cell::Blob(value), SemioValue::Ref { id } => cells[6] = match nodes { Some(nodes) => Cell::Integer(*nodes.get(id.value.as_str()).ok_or("dangling Semio value reference")?), None => Cell::Text(&id.value) }, _ => {} }
        let id = projection.insert(tables.value, &cells)?;
        match owner { Owner::Root => root = id, Owner::List(parent, ordinal) => { projection.insert(tables.list_element, &[Cell::Integer(parent), Cell::Integer(number(ordinal)?), Cell::Integer(id)])?; }, Owner::Map(parent, ordinal, key) => { projection.insert(tables.map_entry, &[Cell::Integer(parent), Cell::Integer(number(ordinal)?), Cell::Text(key), Cell::Integer(id)])?; } }
        match value { SemioValue::List { items } => { projection.check_rows(pending.len().checked_add(items.len()).ok_or("Semio pending value count overflow")?)?; for (ordinal, item) in items.iter().enumerate().rev() { pending.push((item, Owner::List(id, ordinal))); } }, SemioValue::Map { entries } => { projection.check_rows(pending.len().checked_add(entries.len()).ok_or("Semio pending value count overflow")?)?; for (ordinal, entry) in entries.iter().enumerate().rev() { pending.push((&entry.value, Owner::Map(id, ordinal, &entry.key))); } }, _ => {} }
    }
    Ok(root)
}

/// 🌳️ Reconstructs only actual SemioValue primitives and their owned list/map relations.
pub fn reconstruct_value_forest(database: &SqliteDatabase, tables: ValueSqliteTables, roots: &[i64], names: Option<&BTreeMap<i64, &str>>, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<SemioValue>, String> {
        let mut values = BTreeMap::new(); let mut count = 0;
        for row in &database.table(tables.value)?.rows { identity(row, 8)?; if values.insert(row.rowid, row).is_some() { return Err("duplicate Semio value identity".into()); } count += 1; if count % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, count, 0)?; } }
        let mut children = BTreeMap::<i64, Vec<(i64, i64, Option<&str>)>>::new();
        for (table, columns, map) in [(tables.list_element, 4, false), (tables.map_entry, 5, true)] {
            let mut ids = BTreeSet::new(); for row in &database.table(table)?.rows { identity(row, columns)?; if !ids.insert(row.rowid) { return Err("duplicate Semio value relationship identifier".into()); } let parent = row.integer(1)?; let kind = values.get(&parent).ok_or("dangling Semio value parent")?.text(1)?; if kind != if map { "map" } else { "list" } { return Err("Semio value relationship has wrong parent kind".into()); } children.entry(parent).or_default().push((row.integer(2)?, row.integer(if map { 4 } else { 3 })?, if map { Some(row.text(3)?) } else { None })); count += 1; if count % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, count, 0)?; } }
        }
        for links in children.values_mut() { links.sort_by_key(|link| link.0); for (ordinal, link) in links.iter().enumerate() { if link.0 != number(ordinal)? { return Err("Semio value member ordinals must be contiguous".into()); } } }
        let mut seen = BTreeSet::new(); let mut complete = BTreeMap::<i64, SemioValue>::new(); let mut restored = Vec::new();
        for &root in roots {
            let mut pending = vec![(root, false)];
            while let Some((id, exit)) = pending.pop() {
                count += 1; if count % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, count, 0)?; } let row = *values.get(&id).ok_or("dangling Semio value ownership edge")?;
                if !exit { if !seen.insert(id) { return Err("cyclic or multiply owned Semio value".into()); } pending.push((id, true)); if let Some(links) = children.get(&id) { for link in links.iter().rev() { pending.push((link.1, false)); } } continue; }
                let active = match row.text(1)? { "null" | "list" | "map" => None, "bool" => Some(2), "int" => Some(3), "float" => Some(4), "str" => Some(5), "bytes" => Some(6), "ref" => Some(7), _ => return Err("unknown Semio value kind".into()) };
                for index in 2..8 { if Some(index) != active { null(row, index)?; } }
                let value = match row.text(1)? {
                    "null" => SemioValue::Null, "bool" => SemioValue::Bool { value: match row.integer(2)? { 0 => false, 1 => true, _ => return Err("invalid Semio boolean".into()) } },
                    "int" => SemioValue::Int { lexeme: reconstruct_text(control,row.text(3)?)? }, "float" => SemioValue::Float { lexeme: reconstruct_text(control,row.text(4)?)? }, "str" => SemioValue::Str { value: reconstruct_text(control,row.text(5)?)? }, "bytes" => SemioValue::Bytes { value: reconstruct_blob(control,row.blob(6)?)? }, "ref" => SemioValue::Ref { id: ValueId { value: match names { Some(names) => reconstruct_text(control,names.get(&row.integer(7)?).ok_or("dangling Semio value graph reference")?)?, None => reconstruct_text(control,row.text(7)?)? } } },
                    "list" => SemioValue::List { items: children.remove(&id).unwrap_or_default().into_iter().map(|link| complete.remove(&link.1).ok_or_else(|| "invalid Semio list ownership".into())).collect::<Result<_, String>>()? },
                    "map" => SemioValue::Map { entries: children.remove(&id).unwrap_or_default().into_iter().map(|link| Ok(SemioValueEntry { key: reconstruct_text(control,link.2.ok_or("missing Semio map member key")?)?, value: complete.remove(&link.1).ok_or("invalid Semio map ownership")? })).collect::<Result<_, String>>()? }, _ => unreachable!()
                }; complete.insert(id, value);
            }
            restored.push(complete.remove(&root).ok_or("missing Semio value root")?);
        }
        if seen.len() != values.len() || !children.is_empty() || !complete.is_empty() { return Err("unowned Semio value entity".into()); }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, count, count)?; Ok(restored)
}

impl ArtifactSqliteSnapshot for SemioValueSnapshot {
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="value"){return Err(String::from("Semio owned snapshot dialect differs from its dedicated semantic subset").into());}
let row=database.table("semio_value_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("Semio owned document identity differs from projected semantic fields").into());}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.check_rows(self.nodes.len().checked_add(2).ok_or("Semio value row overflow")?)?;
        let mut nodes = BTreeMap::new();
        for (ordinal, node) in self.nodes.iter().enumerate() { if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?; } if nodes.insert(node.id.value.as_str(), number(ordinal + 1)?).is_some() { return Err("duplicate Semio value node identifier".into()); } }
        let mut projection = Projection::new(Self::SQLITE_SCHEMA, control)?; let root = project_value_tree(&self.root, VALUE_TABLES, Some(&nodes), &mut projection)?;
        projection.insert_key("semio_value_document", 1, &[Cell::Text(&self.schema), Cell::Integer(root)])?;
        for (ordinal, node) in self.nodes.iter().enumerate() { let value = project_value_tree(&node.value, VALUE_TABLES, Some(&nodes), &mut projection)?; projection.insert_key("semio_value_node", number(ordinal + 1)?, &[Cell::Integer(1), Cell::Integer(number(ordinal)?), Cell::Text(&node.id.value), Cell::Integer(value)])?; }
        projection.finish()
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> { Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) }
}

impl SemioValueSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>, declared_schema: &str) -> Result<Self, String> {

        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?; validate_sqlite_database_schema(database, declared_schema, control.limits()).map_err(|error| error.to_string())?;
        let document = database.table("semio_value_document")?.single_row()?; identity(document, 3)?; if document.rowid != 1 { return Err("Semio value document requires identifier 1".into()); }
        let nodes = database.table("semio_value_node")?.ordered_rows(2)?; let mut names = BTreeMap::new(); let mut unique_names = BTreeSet::new(); let mut roots = vec![document.integer(2)?];
        for node in &nodes { identity(node, 5)?; if node.integer(1)? != 1 || !unique_names.insert(node.text(3)?) || names.insert(node.rowid, node.text(3)?).is_some() { return Err("invalid Semio value graph node".into()); } roots.push(node.integer(4)?); }
        let restored = reconstruct_value_forest(database, VALUE_TABLES, &roots, Some(&names), control)?;
        let mut restored = restored.into_iter(); let root = restored.next().ok_or("missing Semio value document root")?; let nodes = nodes.into_iter().zip(restored).map(|(node, value)| Ok(SemioValueNode { id: ValueId { value: reconstruct_text(control,node.text(3)?)? }, value })).collect::<Result<_, String>>()?;
        Ok(Self { schema: reconstruct_text(control,document.text(1)?)?, root, nodes })
    }
}

/// 🌱️ Bounds the actual typed SemioValue variants without resolving graph references.
pub fn native_values(roots: &[SemioValue], b: &mut Bound<'_, '_>) -> Result<(), String> {
    b.entities(roots.len())?; let mut pending: Vec<_> = roots.iter().rev().collect();
    while let Some(value) = pending.pop() {
        b.entities(1)?;
        match value {
            SemioValue::Null => {}, SemioValue::Bool { .. } => b.scalars(1)?,
            SemioValue::Int { lexeme } | SemioValue::Float { lexeme } => b.text(lexeme)?,
            SemioValue::Str { value } => b.text(value)?, SemioValue::Bytes { value } => b.bytes(value)?,
            SemioValue::Ref { id } => b.text(&id.value)?,
            SemioValue::List { items } => { b.entities(items.len())?; pending.extend(items.iter().rev()); },
            SemioValue::Map { entries } => { b.entities(entries.len())?; for entry in entries.iter().rev() { b.text(&entry.key)?; pending.push(&entry.value); } },
        }
    }
    Ok(())
}
impl SemioValueSnapshot {
    /// 📏️ Bounds the own root, graph identities and native values.
    pub fn native_fields(&self,b:&mut Bound<'_, '_>)->Result<(),String>{b.text(&self.schema)?;native_values(std::slice::from_ref(&self.root),b)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id.value)?;native_values(std::slice::from_ref(&node.value),b)?;}Ok(())}
}
