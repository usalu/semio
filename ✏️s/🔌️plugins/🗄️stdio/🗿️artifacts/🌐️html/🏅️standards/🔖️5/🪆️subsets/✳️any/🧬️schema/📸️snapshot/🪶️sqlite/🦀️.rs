//! 🌐️ HTML's own element, text, comment, raw-text and optional attribute relations.

use super::{HtmlAttr, HtmlNode, HtmlSnapshot, RawTextKind};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};

const TABLES: [&str; 8] = ["html_document", "html_node", "html_element", "html_text", "html_comment", "html_raw_text", "html_attribute", "html_child"];
fn integer(value: usize) -> Result<i64, String> { i64::try_from(value).map_err(|error| error.to_string()) }
fn index(value: i64) -> Result<usize, String> { usize::try_from(value).map_err(|_| "HTML ordinal must be nonnegative".into()) }
fn add(value: &mut usize, amount: usize) -> Result<(), String> { *value = value.checked_add(amount).ok_or("HTML relational size overflow")?; Ok(()) }
fn text(value: &str) -> SqliteValue { SqliteValue::Text(value.into()) }
fn kind(node: &HtmlNode) -> &'static str { match node { HtmlNode::Element { .. } => "element", HtmlNode::Text { .. } => "text", HtmlNode::Comment { .. } => "comment", HtmlNode::RawText { .. } => "raw_text" } }
fn push(database: &mut SqliteDatabase, table: &str, values: Vec<SqliteValue>) -> Result<i64, String> { let rows = &mut database.table_mut(table)?.rows; let id = integer(rows.len() + 1)?; let mut fields = vec![SqliteValue::Integer(id)]; fields.extend(values); rows.push(SqliteRow { rowid: id, values: fields }); Ok(id) }
fn component(database: &mut SqliteDatabase, table: &str, id: i64, values: Vec<SqliteValue>) -> Result<(), String> { let mut fields = vec![SqliteValue::Integer(id)]; fields.extend(values); database.table_mut(table)?.rows.push(SqliteRow { rowid: id, values: fields }); Ok(()) }
fn identity(row: &SqliteRow, columns: usize) -> Result<(), String> { if row.values.len() != columns || row.integer(0)? != row.rowid { Err("HTML row identity or column count is invalid".into()) } else { Ok(()) } }
fn tick(control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase, checked: &mut usize, total: usize) -> Result<(), String> { add(checked, 1)?; if *checked % 256 == 0 { control.checkpoint(phase, (*checked).min(total), total)?; } Ok(()) }

fn measure(snapshot: &HtmlSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<usize, String> {
    let mut rows = 1usize; let mut bytes = snapshot.schema.len(); add(&mut bytes, 16)?; if let Some(doctype) = &snapshot.doctype { add(&mut bytes, doctype.len())?; } control.check_value_bytes(bytes)?;
    let mut stack = vec![&snapshot.root]; let mut checked = 0usize;
    while let Some(node) = stack.pop() {
        add(&mut rows, 2)?; add(&mut bytes, 16 + kind(node).len())?;
        match node {
            HtmlNode::Element { name, attributes, children } => {
                add(&mut bytes, name.len())?; add(&mut rows, attributes.len())?; add(&mut rows, children.len())?;
                control.check_rows(rows.checked_add(children.len().checked_mul(2).ok_or("HTML child size overflow")?).ok_or("HTML child size overflow")?)?;
                add(&mut bytes, children.len().checked_mul(32).ok_or("HTML child size overflow")?)?;
                for attr in attributes { add(&mut bytes, 24 + attr.name.len())?; if let Some(value) = &attr.value { add(&mut bytes, value.len())?; } control.check_value_bytes(bytes)?; tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut checked, rows)?; }
                stack.extend(children.iter().rev());
            }
            HtmlNode::Text { text } | HtmlNode::Comment { text } => add(&mut bytes, text.len())?,
            HtmlNode::RawText { parent_kind, text } => { add(&mut bytes, parent_kind.tag_name().len())?; add(&mut bytes, text.len())?; }
        }
        control.check_rows(rows)?; control.check_value_bytes(bytes)?; tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut checked, rows)?;
    }
    Ok(rows)
}

impl ArtifactSqliteSnapshot for HtmlSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> {
        use semio_framework_os_kernel::sqlite_snapshot::artifact::NativeEncodingBound;
        let mut bound = NativeEncodingBound::new(control)?; bound.add(1024)?;
        if let Some(doctype) = &self.doctype { bound.repeated(doctype.len(), 4)?; }
        let mut stack = vec![&self.root]; let mut rows = 1usize;
        while let Some(node) = stack.pop() {
            bound.add(128)?;
            match node {
                HtmlNode::Element { name, attributes, children } => {
                    rows = rows.checked_add(attributes.len()).and_then(|value| value.checked_add(children.len())).ok_or("HTML native entity count overflow")?;
                    bound.check_rows(rows)?; bound.repeated(children.len(), 32)?; bound.repeated(name.len(), 8)?;
                    for attr in attributes { bound.add(64)?; bound.repeated(attr.name.len(), 4)?; if let Some(value) = &attr.value { bound.repeated(value.len(), 24)?; } }
                    stack.extend(children.iter().rev());
                },
                HtmlNode::Text { text } => bound.repeated(text.len(), 24)?,
                HtmlNode::Comment { text } | HtmlNode::RawText { text, .. } => bound.repeated(text.len(), 4)?,
            }
        }
        bound.finish()
    }

    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.html"||dialect.standard!="5"{return Err(String::from("owned html snapshot dialect differs from its semantic standard").into());}
        let row=database.table("html_document")?.single_row()?;if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("owned html document identity differs from semantic projection").into());}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),_=>return Err(String::from("named html subset has no owned semantic validator").into())};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?; let total = measure(self, control)?; let mut checked = 0usize;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        push(&mut database, "html_document", vec![text(&self.schema), self.doctype.as_deref().map(text).unwrap_or(SqliteValue::Null), SqliteValue::Integer(1)])?;
        let mut stack = vec![(&self.root, None::<(i64, usize)>)];
        while let Some((node, parent)) = stack.pop() {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, checked.min(total), total)?;
            let id = push(&mut database, "html_node", vec![text(kind(node))])?;
            match node {
                HtmlNode::Element { name, attributes, children } => {
                    component(&mut database, "html_element", id, vec![text(name)])?;
                    for (ordinal, attr) in attributes.iter().enumerate() { if attr.name.len().saturating_add(attr.value.as_ref().map_or(0, String::len)) > 65536 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, checked.min(total), total)?; } push(&mut database, "html_attribute", vec![SqliteValue::Integer(id), SqliteValue::Integer(integer(ordinal)?), text(&attr.name), attr.value.as_deref().map(text).unwrap_or(SqliteValue::Null)])?; tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut checked, total)?; }
                    for (ordinal, child) in children.iter().enumerate().rev() { stack.push((child, Some((id, ordinal)))); }
                }
                HtmlNode::Text { text: value } => component(&mut database, "html_text", id, vec![text(value)])?,
                HtmlNode::Comment { text: value } => component(&mut database, "html_comment", id, vec![text(value)])?,
                HtmlNode::RawText { parent_kind, text: value } => component(&mut database, "html_raw_text", id, vec![text(parent_kind.tag_name()), text(value)])?,
            }
            if let Some((parent, ordinal)) = parent { push(&mut database, "html_child", vec![SqliteValue::Integer(parent), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(id)])?; }
            tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut checked, total)?;
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, total, total)?; Ok(database)
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        for name in TABLES { database.table(name)?; } let total = database.tables.iter().map(|table| table.rows.len()).sum(); let mut checked = 0usize;
        let document = database.table("html_document")?.single_row()?; identity(document, 4)?; if document.rowid != 1 { return Err("HTML document requires identifier 1".into()); } let root = document.integer(3)?;
        let mut nodes = BTreeMap::new();
        for row in &database.table("html_node")?.rows { identity(row, 2)?; if !matches!(row.text(1)?, "element" | "text" | "comment" | "raw_text") || nodes.insert(row.rowid, row).is_some() { return Err("HTML node identity or kind is invalid".into()); } tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; }
        if !nodes.contains_key(&root) { return Err("HTML document root is dangling".into()); }
        let mut components = BTreeMap::new();
        for (table, expected) in [("html_element", "element"), ("html_text", "text"), ("html_comment", "comment"), ("html_raw_text", "raw_text")] { for row in &database.table(table)?.rows { identity(row, if expected == "raw_text" { 3 } else { 2 })?; if nodes.get(&row.rowid).ok_or("HTML component node is dangling")?.text(1)? != expected || components.insert(row.rowid, row).is_some() { return Err("HTML node requires exactly its typed component".into()); } row.text(1)?; if expected == "raw_text" { row.text(2)?; if !matches!(row.text(1)?, "script" | "style") { return Err("HTML raw text parent kind is invalid".into()); } } tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; } }
        if components.len() != nodes.len() { return Err("HTML node lacks its typed component".into()); }
        let mut owners = BTreeSet::from([root]); let mut ids = BTreeSet::new(); let mut children = BTreeMap::<i64, Vec<(usize, i64)>>::new();
        for row in &database.table("html_child")?.rows { identity(row, 4)?; let parent = row.integer(1)?; let ordinal = index(row.integer(2)?)?; let child = row.integer(3)?; if !ids.insert(row.rowid) || nodes.get(&parent).ok_or("HTML child parent is dangling")?.text(1)? != "element" || !nodes.contains_key(&child) || !owners.insert(child) { return Err("HTML child requires an element parent and unique node ownership".into()); } children.entry(parent).or_default().push((ordinal, child)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; }
        let mut attributes = BTreeMap::<i64, Vec<(usize, &SqliteRow)>>::new(); ids.clear();
        for row in &database.table("html_attribute")?.rows { identity(row, 5)?; let parent = row.integer(1)?; if !ids.insert(row.rowid) || nodes.get(&parent).ok_or("HTML attribute parent is dangling")?.text(1)? != "element" { return Err("HTML attribute requires an element parent and unique identity".into()); } row.text(3)?; row.optional_text(4)?; attributes.entry(parent).or_default().push((index(row.integer(2)?)?, row)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; }
        for ordered in children.values_mut() { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, checked.min(total), total)?; ordered.sort_by_key(|(ordinal, _)| *ordinal); for (expected, (actual, _)) in ordered.iter().enumerate() { if expected != *actual { return Err("HTML child ordinals must be contiguous and zero-based".into()); } } }
        for ordered in attributes.values_mut() { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, checked.min(total), total)?; ordered.sort_by_key(|(ordinal, _)| *ordinal); for (expected, (actual, _)) in ordered.iter().enumerate() { if expected != *actual { return Err("HTML attribute ordinals must be contiguous and zero-based".into()); } } }
        if owners.len() != nodes.len() { return Err("HTML node has no document or element owner".into()); }
        let mut stack = vec![(root, false)]; let mut visited = BTreeSet::new(); let mut built = BTreeMap::new();
        while let Some((id, finish)) = stack.pop() {
            if !finish { if !visited.insert(id) { return Err("HTML node graph contains a cycle".into()); } stack.push((id, true)); if let Some(ordered) = children.get(&id) { stack.extend(ordered.iter().rev().map(|(_, child)| (*child, false))); } continue; }
            control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, checked.min(total), total)?; let payload = components[&id]; let node = match nodes[&id].text(1)? {
                "element" => { let mut attrs = Vec::new(); for (_, row) in attributes.remove(&id).unwrap_or_default() { attrs.push(HtmlAttr { name: row.text(3)?.into(), value: row.optional_text(4)?.map(str::to_string) }); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; } let mut child_nodes = Vec::new(); for (_, child) in children.remove(&id).unwrap_or_default() { child_nodes.push(built.remove(&child).ok_or("HTML child was not reconstructed")?); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; } HtmlNode::Element { name: payload.text(1)?.into(), attributes: attrs, children: child_nodes } },
                "text" => HtmlNode::Text { text: payload.text(1)?.into() }, "comment" => HtmlNode::Comment { text: payload.text(1)?.into() },
                "raw_text" => HtmlNode::RawText { parent_kind: match payload.text(1)? { "script" => RawTextKind::Script, "style" => RawTextKind::Style, _ => return Err("HTML raw text parent kind is invalid".into()) }, text: payload.text(2)?.into() }, _ => return Err("HTML node kind is invalid".into()),
            }; built.insert(id, node); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?;
        }
        if visited.len() != nodes.len() || !children.is_empty() || !attributes.is_empty() { return Err("HTML node graph is disconnected or cyclic".into()); }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?; Ok(Self { schema: document.text(1)?.into(), doctype: document.optional_text(2)?.map(str::to_string), root: built.remove(&root).ok_or("HTML root was not reconstructed")? })
    }
}
