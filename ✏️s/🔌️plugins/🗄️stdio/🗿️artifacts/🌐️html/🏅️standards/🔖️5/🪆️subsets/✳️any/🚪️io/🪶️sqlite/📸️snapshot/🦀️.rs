//! 🌐️ HTML's own element, text, comment, raw-text and optional attribute relations.

use crate::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode, HtmlSnapshot, RawTextKind};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_os_kernel::io_schema::IoError;

const TABLES: [&str; 8] = ["html_document", "html_node", "html_element", "html_text", "html_comment", "html_raw_text", "html_attribute", "html_child"];
struct OwnedNodeMap(BTreeMap<i64,HtmlNode>);
impl std::ops::Deref for OwnedNodeMap{type Target=BTreeMap<i64,HtmlNode>;fn deref(&self)->&Self::Target{&self.0}}
impl std::ops::DerefMut for OwnedNodeMap{fn deref_mut(&mut self)->&mut Self::Target{&mut self.0}}
impl Drop for OwnedNodeMap{fn drop(&mut self){for node in std::mem::take(&mut self.0).into_values(){crate::standards::v5::subsets::any::io::sqlite::snapshot::native::retire_node(node)}}}
struct OwnedChildren(Vec<HtmlNode>);
impl std::ops::Deref for OwnedChildren{type Target=Vec<HtmlNode>;fn deref(&self)->&Self::Target{&self.0}}
impl std::ops::DerefMut for OwnedChildren{fn deref_mut(&mut self)->&mut Self::Target{&mut self.0}}
impl Drop for OwnedChildren{fn drop(&mut self){while let Some(node)=self.0.pop(){crate::standards::v5::subsets::any::io::sqlite::snapshot::native::retire_node(node)}}}
fn integer(value: usize) -> Result<i64, ValueError> { i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"HTML ordinal exceeds SQLite integer range")) }
fn index(value: i64) -> Result<usize, ValueError> { usize::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"HTML ordinal must be nonnegative")) }
fn add(value: &mut usize, amount: usize) -> Result<(), ValueError> { *value = value.checked_add(amount).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"HTML relational size overflow"))?; Ok(()) }
fn text(value: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteValue,ValueError> { semio_framework_os_kernel::sqlite_snapshot::artifact::project_text(control,value).map(SqliteValue::Text) }
fn kind(node: &HtmlNode) -> &'static str { match node { HtmlNode::Element { .. } => "element", HtmlNode::Text { .. } => "text", HtmlNode::Comment { .. } => "comment", HtmlNode::RawText { .. } => "raw_text" } }
fn push(database: &mut SqliteDatabase, table: &str, values: Vec<SqliteValue>) -> Result<i64, ValueError> { let rows = &mut database.table_mut(table)?.rows; let id = integer(rows.len() + 1)?; let mut fields = vec![SqliteValue::Integer(id)]; fields.extend(values); rows.push(SqliteRow { rowid: id, values: fields }); Ok(id) }
fn component(database: &mut SqliteDatabase, table: &str, id: i64, values: Vec<SqliteValue>) -> Result<(), ValueError> { let mut fields = vec![SqliteValue::Integer(id)]; fields.extend(values); database.table_mut(table)?.rows.push(SqliteRow { rowid: id, values: fields }); Ok(()) }
fn identity(row: &SqliteRow, columns: usize) -> Result<(), ValueError> { if row.values.len() != columns || row.integer(0)? != row.rowid { Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML row identity or column count is invalid")) } else { Ok(()) } }
fn tick(control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase, checked: &mut usize, total: usize) -> Result<(), ValueError> { add(checked, 1)?; if *checked % 256 == 0 { control.checkpoint(phase, (*checked).min(total), total)?; } Ok(()) }

fn measure(snapshot: &HtmlSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<usize, ValueError> {
    let mut rows = 1usize; let mut bytes = snapshot.schema.len(); add(&mut bytes, 16)?; if let Some(doctype) = &snapshot.doctype { add(&mut bytes, doctype.len())?; } control.check_value_bytes(bytes)?;
    let mut stack = vec![&snapshot.root]; let mut checked = 0usize;
    while let Some(node) = stack.pop() {
        add(&mut rows, 2)?; add(&mut bytes, 16 + kind(node).len())?;
        match node {
            HtmlNode::Element { name, attributes, children } => {
                add(&mut bytes, name.len())?; add(&mut rows, attributes.len())?; add(&mut rows, children.len())?;
                control.check_rows(rows.checked_add(children.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"HTML child size overflow"))?).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"HTML child size overflow"))?)?;
                add(&mut bytes, children.len().checked_mul(32).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"HTML child size overflow"))?)?;
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
    fn decode_sqlite_snapshot_native(payload:&semio_framework_os_kernel::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v5::subsets::any::io::sqlite::snapshot::native::decode(payload,control)}
    fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<semio_framework_os_kernel::io_schema::IoPayload,ValueError>{crate::standards::v5::subsets::any::io::sqlite::snapshot::native::encode(self,encoding,control)}
    fn retire_sqlite_snapshot(self){crate::standards::v5::subsets::any::io::sqlite::snapshot::native::retire(self)}
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        use semio_framework_os_kernel::sqlite_snapshot::artifact::NativeEncodingBound;
        let mut bound = NativeEncodingBound::new(control)?; bound.add(1024)?;
        if let Some(doctype) = &self.doctype { bound.repeated(doctype.len(), 4)?; }
        let mut stack = vec![&self.root]; let mut rows = 1usize;
        while let Some(node) = stack.pop() {
            bound.add(128)?;
            match node {
                HtmlNode::Element { name, attributes, children } => {
                    rows = rows.checked_add(attributes.len()).and_then(|value| value.checked_add(children.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"HTML native entity count overflow"))?;
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

    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(IoError::from_value_error)?;
        if dialect.artifact_kind!="s.stdio.html"||dialect.standard!="5"{return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"owned html snapshot dialect differs from its semantic standard")));}
        let row=database.table("html_document").map_err(IoError::from_value_error)?.single_row().map_err(IoError::from_value_error)?;if row.rowid!=1||row.text(1).map_err(IoError::from_value_error)?!=self.schema{return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"owned html document identity differs from semantic projection")));}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),_=>return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"named html subset has no owned semantic validator")))};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(IoError::from_value_error)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?; let total = measure(self, control)?; let mut checked = 0usize;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        push(&mut database, "html_document", vec![text(&self.schema,control)?, self.doctype.as_deref().map(|v|text(v,control)).transpose()?.unwrap_or(SqliteValue::Null), SqliteValue::Integer(1)])?;
        let mut stack = vec![(&self.root, None::<(i64, usize)>)];
        while let Some((node, parent)) = stack.pop() {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, checked.min(total), total)?;
            let id = push(&mut database, "html_node", vec![text(kind(node),control)?])?;
            match node {
                HtmlNode::Element { name, attributes, children } => {
                    component(&mut database, "html_element", id, vec![text(name,control)?])?;
                    for (ordinal, attr) in attributes.iter().enumerate() { if attr.name.len().saturating_add(attr.value.as_ref().map_or(0, String::len)) > 65536 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, checked.min(total), total)?; } push(&mut database, "html_attribute", vec![SqliteValue::Integer(id), SqliteValue::Integer(integer(ordinal)?), text(&attr.name,control)?, attr.value.as_deref().map(|v|text(v,control)).transpose()?.unwrap_or(SqliteValue::Null)])?; tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut checked, total)?; }
                    for (ordinal, child) in children.iter().enumerate().rev() { stack.push((child, Some((id, ordinal)))); }
                }
                HtmlNode::Text { text: value } => component(&mut database, "html_text", id, vec![text(value,control)?])?,
                HtmlNode::Comment { text: value } => component(&mut database, "html_comment", id, vec![text(value,control)?])?,
                HtmlNode::RawText { parent_kind, text: value } => component(&mut database, "html_raw_text", id, vec![text(parent_kind.tag_name(),control)?, text(value,control)?])?,
            }
            if let Some((parent, ordinal)) = parent { push(&mut database, "html_child", vec![SqliteValue::Integer(parent), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(id)])?; }
            tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut checked, total)?;
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, total, total)?; Ok(database)
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        for name in TABLES { database.table(name)?; } let total = database.tables.iter().map(|table| table.rows.len()).sum(); let mut checked = 0usize;
        let document = database.table("html_document")?.single_row()?; identity(document, 4)?; if document.rowid != 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML document requires identifier 1")); } let root = document.integer(3)?;
        let mut nodes = BTreeMap::new();
        for row in &database.table("html_node")?.rows { identity(row, 2)?; if !matches!(row.text(1)?, "element" | "text" | "comment" | "raw_text") || nodes.insert(row.rowid, row).is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML node identity or kind is invalid")); } tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; }
        if !nodes.contains_key(&root) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML document root is dangling")); }
        let mut components = BTreeMap::new();
        for (table, expected) in [("html_element", "element"), ("html_text", "text"), ("html_comment", "comment"), ("html_raw_text", "raw_text")] { for row in &database.table(table)?.rows { identity(row, if expected == "raw_text" { 3 } else { 2 })?; if nodes.get(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"HTML component node is dangling"))?.text(1)? != expected || components.insert(row.rowid, row).is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML node requires exactly its typed component")); } row.text(1)?; if expected == "raw_text" { row.text(2)?; if !matches!(row.text(1)?, "script" | "style") { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML raw text parent kind is invalid")); } } tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; } }
        if components.len() != nodes.len() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML node lacks its typed component")); }
        let mut owners = BTreeSet::from([root]); let mut ids = BTreeSet::new(); let mut children = BTreeMap::<i64, Vec<(usize, i64)>>::new();
        for row in &database.table("html_child")?.rows { identity(row, 4)?; let parent = row.integer(1)?; let ordinal = index(row.integer(2)?)?; let child = row.integer(3)?; if !ids.insert(row.rowid) || nodes.get(&parent).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"HTML child parent is dangling"))?.text(1)? != "element" || !nodes.contains_key(&child) || !owners.insert(child) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML child requires an element parent and unique node ownership")); } children.entry(parent).or_default().push((ordinal, child)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; }
        let mut attributes = BTreeMap::<i64, Vec<(usize, &SqliteRow)>>::new(); ids.clear();
        for row in &database.table("html_attribute")?.rows { identity(row, 5)?; let parent = row.integer(1)?; if !ids.insert(row.rowid) || nodes.get(&parent).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"HTML attribute parent is dangling"))?.text(1)? != "element" { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML attribute requires an element parent and unique identity")); } row.text(3)?; row.optional_text(4)?; attributes.entry(parent).or_default().push((index(row.integer(2)?)?, row)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; }
        for ordered in children.values_mut() { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, checked.min(total), total)?; ordered.sort_by_key(|(ordinal, _)| *ordinal); for (expected, (actual, _)) in ordered.iter().enumerate() { if expected != *actual { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML child ordinals must be contiguous and zero-based")); } } }
        for ordered in attributes.values_mut() { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, checked.min(total), total)?; ordered.sort_by_key(|(ordinal, _)| *ordinal); for (expected, (actual, _)) in ordered.iter().enumerate() { if expected != *actual { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML attribute ordinals must be contiguous and zero-based")); } } }
        if owners.len() != nodes.len() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML node has no document or element owner")); }
        let mut stack = vec![(root, false)]; let mut visited = BTreeSet::new(); let mut built = OwnedNodeMap(BTreeMap::new());
        while let Some((id, finish)) = stack.pop() {
            if !finish { if !visited.insert(id) { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML node graph contains a cycle")); } stack.push((id, true)); if let Some(ordered) = children.get(&id) { stack.extend(ordered.iter().rev().map(|(_, child)| (*child, false))); } continue; }
            control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, checked.min(total), total)?; let payload = components[&id]; let node = match nodes[&id].text(1)? {
                "element" => { let mut attrs = Vec::new(); for (_, row) in attributes.remove(&id).unwrap_or_default() { attrs.push(HtmlAttr { name: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control,row.text(3)?)?, value: row.optional_text(4)?.map(|v|semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control,v)).transpose()? }); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; } let mut child_nodes = OwnedChildren(Vec::new()); for (_, child) in children.remove(&id).unwrap_or_default() { child_nodes.push(built.remove(&child).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"HTML child was not reconstructed"))?); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?; } HtmlNode::Element { name: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control,payload.text(1)?)?, attributes: attrs, children: std::mem::take(&mut child_nodes.0) } },
                "text" => HtmlNode::Text { text: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control,payload.text(1)?)? }, "comment" => HtmlNode::Comment { text: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control,payload.text(1)?)? },
                "raw_text" => HtmlNode::RawText { parent_kind: match payload.text(1)? { "script" => RawTextKind::Script, "style" => RawTextKind::Style, _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML raw text parent kind is invalid")) }, text: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control,payload.text(2)?)? }, _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML node kind is invalid")),
            }; built.insert(id, node); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut checked, total)?;
        }
        if visited.len() != nodes.len() || !children.is_empty() || !attributes.is_empty() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"HTML node graph is disconnected or cyclic")); }
        #[cfg(test)]let _late_scope=crate::standards::v5::subsets::any::io::sqlite::snapshot::lifecycle_tests::SqlLateCopyScope::enter();
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?; Ok(Self { schema: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control,document.text(1)?)?, doctype: document.optional_text(2)?.map(|v|semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control,v)).transpose()?, root: built.remove(&root).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"HTML root was not reconstructed"))? })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;

#[cfg(test)]
#[path = "🧪️tests/🧹️lifecycle/🦀️.rs"]
mod lifecycle_tests;
