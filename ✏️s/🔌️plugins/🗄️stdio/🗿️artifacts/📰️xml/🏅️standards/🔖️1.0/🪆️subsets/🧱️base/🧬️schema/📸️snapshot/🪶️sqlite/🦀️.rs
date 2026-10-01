//! 📰️ Handcrafted XML document, node, attribute, declaration and entity relations.

use super::{validate_xml_document_boundaries, XmlAttr, XmlDeclaration, XmlDocument, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote, XmlSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};

/// 📰️ Explicit table names for the shared, typed XmlDocument fields.
#[derive(Clone, Copy)]
pub struct XmlSqliteTables {
    pub document: &'static str,
    pub node: &'static str,
    pub element: &'static str,
    pub text: &'static str,
    pub cdata: &'static str,
    pub comment: &'static str,
    pub processing_instruction: &'static str,
    pub attribute: &'static str,
    pub child: &'static str,
    pub document_misc: &'static str,
    pub declaration: &'static str,
    pub doctype: &'static str,
    pub entity: &'static str,
}

impl XmlSqliteTables {
    pub fn names(self) -> [&'static str; 13] { [self.document, self.node, self.element, self.text, self.cdata, self.comment, self.processing_instruction, self.attribute, self.child, self.document_misc, self.declaration, self.doctype, self.entity] }
}

/// 📰️ XML's handwritten native vocabulary.
pub const XML_SQLITE_TABLES: XmlSqliteTables = XmlSqliteTables {
    document: "xml_document",
    node: "xml_node",
    element: "xml_element",
    text: "xml_text",
    cdata: "xml_cdata",
    comment: "xml_comment",
    processing_instruction: "xml_processing_instruction",
    attribute: "xml_attribute",
    child: "xml_child",
    document_misc: "xml_document_misc",
    declaration: "xml_declaration",
    doctype: "xml_doctype",
    entity: "xml_entity",
};

enum Parent { Root, Misc(&'static str, usize), Child(i64, usize) }
fn integer(value: usize) -> Result<i64, String> { i64::try_from(value).map_err(|error| error.to_string()) }
fn index(value: i64) -> Result<usize, String> { usize::try_from(value).map_err(|_| "XML ordinal must be nonnegative".into()) }
fn add(value: &mut usize, amount: usize) -> Result<(), String> { *value = value.checked_add(amount).ok_or("XML relational size overflow")?; Ok(()) }
fn kind(node: &XmlNode) -> &'static str { match node { XmlNode::Element { .. } => "element", XmlNode::Text { .. } => "text", XmlNode::CData { .. } => "cdata", XmlNode::Comment { .. } => "comment", XmlNode::ProcessingInstruction { .. } => "processing_instruction" } }
fn push(database: &mut SqliteDatabase, table: &str, values: Vec<SqliteValue>) -> Result<i64, String> { let rows = &mut database.table_mut(table)?.rows; let id = integer(rows.len() + 1)?; let mut fields = vec![SqliteValue::Integer(id)]; fields.extend(values); rows.push(SqliteRow { rowid: id, values: fields }); Ok(id) }
fn component(database: &mut SqliteDatabase, table: &str, id: i64, values: Vec<SqliteValue>) -> Result<(), String> { let mut fields = vec![SqliteValue::Integer(id)]; fields.extend(values); database.table_mut(table)?.rows.push(SqliteRow { rowid: id, values: fields }); Ok(()) }
fn text(value: &str) -> SqliteValue { SqliteValue::Text(value.into()) }
fn optional(value: Option<&str>) -> SqliteValue { value.map(text).unwrap_or(SqliteValue::Null) }
fn boolean(row: &SqliteRow, column: usize) -> Result<bool, String> { match row.integer(column)? { 0 => Ok(false), 1 => Ok(true), _ => Err("XML boolean must be 0 or 1".into()) } }
fn identity(row: &SqliteRow, columns: usize) -> Result<(), String> { if row.values.len() != columns || row.integer(0)? != row.rowid { Err("XML row identity or column count is invalid".into()) } else { Ok(()) } }
fn tick(control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase, completed: &mut usize, total: usize) -> Result<(), String> { add(completed, 1)?; if *completed % 256 == 0 { control.checkpoint(phase, (*completed).min(total), total)?; } Ok(()) }

fn measure(schema: &str, doc: &XmlDocument, control: &mut SqliteSnapshotControl<'_>) -> Result<usize, String> {
    let mut rows = 1usize; let mut bytes = schema.len(); add(&mut bytes, if doc.root.is_some() { 16 } else { 8 })?;
    if let Some(declaration) = &doc.declaration { add(&mut rows, 1)?; add(&mut bytes, 16 + declaration.version.len() + declaration.encoding.as_ref().map_or(0, String::len) + if declaration.standalone.is_some() { 8 } else { 0 } + match declaration.quote { XmlQuote::Double => 6, XmlQuote::Single => 6 })?; }
    if let Some(doctype) = &doc.doctype {
        add(&mut rows, 1)?; add(&mut bytes, 24 + doctype.name.len())?;
        match &doctype.external_id { None => {}, Some(XmlExternalId::System { system_id }) => add(&mut bytes, 6 + system_id.len())?, Some(XmlExternalId::Public { public_id, system_id }) => add(&mut bytes, 6 + public_id.len() + system_id.len())? }
        for declaration in &doctype.declarations { let XmlDtdDeclaration::Entity { name, value, .. } = declaration; add(&mut rows, 1)?; add(&mut bytes, 32 + name.len() + value.len())?; control.check_rows(rows)?; control.check_value_bytes(bytes)?; if rows % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, rows)?; } }
    }
    let boundary_rows = doc.prolog.len().checked_add(doc.epilog.len()).ok_or("XML boundary size overflow")?; add(&mut rows, boundary_rows)?;
    add(&mut bytes, doc.prolog.len().checked_mul(38).ok_or("XML prolog size overflow")?)?; add(&mut bytes, doc.epilog.len().checked_mul(38).ok_or("XML epilog size overflow")?)?;
    control.check_rows(rows.checked_add(boundary_rows.checked_mul(2).ok_or("XML boundary size overflow")?).ok_or("XML boundary size overflow")?)?; control.check_value_bytes(bytes)?;
    let mut stack = Vec::new(); if let Some(root) = &doc.root { stack.push(root); } stack.extend(doc.prolog.iter()); stack.extend(doc.epilog.iter()); let mut visited = 0usize;
    while let Some(node) = stack.pop() {
        add(&mut rows, 2)?; add(&mut bytes, 16 + kind(node).len())?;
        match node {
            XmlNode::Element { name, attrs, children } => {
                add(&mut bytes, name.len())?; add(&mut rows, attrs.len())?; add(&mut rows, children.len())?;
                control.check_rows(rows.checked_add(children.len().checked_mul(2).ok_or("XML child size overflow")?).ok_or("XML child size overflow")?)?;
                add(&mut bytes, children.len().checked_mul(32).ok_or("XML child size overflow")?)?;
                for attr in attrs { add(&mut bytes, 24 + attr.name.len() + attr.value.len())?; control.check_value_bytes(bytes)?; visited += 1; if visited % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, rows)?; } }
                stack.extend(children.iter().rev());
            }
            XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => add(&mut bytes, text.len())?,
            XmlNode::ProcessingInstruction { target, data } => { add(&mut bytes, target.len())?; add(&mut bytes, data.len())?; }
        }
        control.check_rows(rows)?; control.check_value_bytes(bytes)?; visited += 1; if visited % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, rows)?; }
    }
    Ok(rows)
}

/// 📰️ Projects the shared native XML document fields into a caller's explicit typed relational vocabulary.
pub fn project_xml_document(schema: &str, doc: &XmlDocument, ddl: &str, tables: XmlSqliteTables, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?; validate_xml_document_boundaries(doc)?; let total = measure(schema, doc, control)?;
        let mut database = SqliteDatabase::from_schema(ddl).map_err(|error| error.to_string())?; let mut completed = 0usize;
        push(&mut database, tables.document, vec![text(schema), if doc.root.is_some() { SqliteValue::Integer(1) } else { SqliteValue::Null }])?;
        if let Some(declaration) = &doc.declaration { push(&mut database, tables.declaration, vec![SqliteValue::Integer(1), text(&declaration.version), optional(declaration.encoding.as_deref()), declaration.standalone.map(|value| SqliteValue::Integer(i64::from(value))).unwrap_or(SqliteValue::Null), text(match declaration.quote { XmlQuote::Double => "double", XmlQuote::Single => "single" })])?; }
        if let Some(doctype) = &doc.doctype {
            let (external_kind, public_id, system_id) = match &doctype.external_id { None => (None, None, None), Some(XmlExternalId::System { system_id }) => (Some("system"), None, Some(system_id.as_str())), Some(XmlExternalId::Public { public_id, system_id }) => (Some("public"), Some(public_id.as_str()), Some(system_id.as_str())) };
            push(&mut database, tables.doctype, vec![SqliteValue::Integer(1), SqliteValue::Integer(integer(doctype.prolog_position)?), text(&doctype.name), optional(external_kind), optional(public_id), optional(system_id)])?;
            for (ordinal, entity) in doctype.declarations.iter().enumerate() { let XmlDtdDeclaration::Entity { parameter, name, value } = entity; push(&mut database, tables.entity, vec![SqliteValue::Integer(1), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(i64::from(*parameter)), text(name), text(value)])?; tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut completed, total)?; }
        }
        let mut stack = Vec::new(); for (ordinal, node) in doc.epilog.iter().enumerate().rev() { stack.push((node, Parent::Misc("epilog", ordinal))); } for (ordinal, node) in doc.prolog.iter().enumerate().rev() { stack.push((node, Parent::Misc("prolog", ordinal))); } if let Some(root) = &doc.root { stack.push((root, Parent::Root)); }
        while let Some((node, parent)) = stack.pop() {
            if match node { XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => text.len() > 65536, XmlNode::ProcessingInstruction { target, data } => target.len().saturating_add(data.len()) > 65536, XmlNode::Element { name, .. } => name.len() > 65536 } { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, completed, total)?; }
            let id = push(&mut database, tables.node, vec![text(kind(node))])?;
            match node {
                XmlNode::Element { name, attrs, children } => {
                    component(&mut database, tables.element, id, vec![text(name)])?;
                    for (ordinal, attr) in attrs.iter().enumerate() { if attr.name.len().saturating_add(attr.value.len()) > 65536 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, completed, total)?; } push(&mut database, tables.attribute, vec![SqliteValue::Integer(id), SqliteValue::Integer(integer(ordinal)?), text(&attr.name), text(&attr.value)])?; tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut completed, total)?; }
                    for (ordinal, child) in children.iter().enumerate().rev() { stack.push((child, Parent::Child(id, ordinal))); }
                }
                XmlNode::Text { text: value } => component(&mut database, tables.text, id, vec![text(value)])?,
                XmlNode::CData { text: value } => component(&mut database, tables.cdata, id, vec![text(value)])?,
                XmlNode::Comment { text: value } => component(&mut database, tables.comment, id, vec![text(value)])?,
                XmlNode::ProcessingInstruction { target, data } => component(&mut database, tables.processing_instruction, id, vec![text(target), text(data)])?,
            }
            match parent { Parent::Root => {}, Parent::Misc(position, ordinal) => { push(&mut database, tables.document_misc, vec![SqliteValue::Integer(1), text(position), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(id)])?; }, Parent::Child(parent, ordinal) => { push(&mut database, tables.child, vec![SqliteValue::Integer(parent), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(id)])?; } }
            tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut completed, total)?;
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, total, total)?; Ok(database)
    }

/// 📰️ Reconstructs exactly the native XML document fields from a caller's declared typed vocabulary.
pub fn reconstruct_xml_document(database: &SqliteDatabase, ddl: &str, tables: XmlSqliteTables, control: &mut SqliteSnapshotControl<'_>) -> Result<(String, XmlDocument), String> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        validate_sqlite_database_schema(database, ddl, control.limits()).map_err(|error| error.to_string())?;
        if database.tables.len() != tables.names().len() { return Err("XML snapshot requires exactly its thirteen domain tables".into()); } for name in tables.names() { database.table(name)?; }
        let total = database.tables.iter().map(|table| table.rows.len()).sum(); let mut completed = 0usize;
        let document = database.table(tables.document)?.single_row()?; identity(document, 3)?; if document.rowid != 1 { return Err("XML document requires identifier 1".into()); }
        let root = match &document.values[2] { SqliteValue::Null => None, SqliteValue::Integer(id) => Some(*id), _ => return Err("XML root requires an integer reference or NULL".into()) };
        let mut nodes = BTreeMap::new();
        for row in &database.table(tables.node)?.rows { identity(row, 2)?; if !matches!(row.text(1)?, "element" | "text" | "cdata" | "comment" | "processing_instruction") || nodes.insert(row.rowid, row).is_some() { return Err("XML node kind or identity is invalid".into()); } tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
        let mut components = BTreeMap::new();
        for (table, expected) in [(tables.element, "element"), (tables.text, "text"), (tables.cdata, "cdata"), (tables.comment, "comment"), (tables.processing_instruction, "processing_instruction")] {
            for row in &database.table(table)?.rows { identity(row, if expected == "processing_instruction" { 3 } else { 2 })?; if nodes.get(&row.rowid).ok_or("XML component node is dangling")?.text(1)? != expected || components.insert(row.rowid, row).is_some() { return Err("XML component does not match its node kind or has multiple owners".into()); } row.text(1)?; if expected == "processing_instruction" { row.text(2)?; } tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
        }
        if nodes.len() != components.len() { return Err("XML node lacks its typed component".into()); }
        let mut owners = BTreeSet::new(); if let Some(root) = root { if !nodes.contains_key(&root) { return Err("XML root is dangling".into()); } owners.insert(root); }
        let mut boundaries = BTreeMap::<&str, Vec<(usize, i64)>>::new(); let mut ids = BTreeSet::new();
        for row in &database.table(tables.document_misc)?.rows {
            identity(row, 5)?; if row.integer(1)? != 1 || !ids.insert(row.rowid) { return Err("XML boundary identity is invalid".into()); } let position = row.text(2)?; let ordinal = index(row.integer(3)?)?; let id = row.integer(4)?;
            if !matches!(position, "prolog" | "epilog") || !nodes.contains_key(&id) || !owners.insert(id) { return Err("XML boundary position, node or ownership is invalid".into()); } boundaries.entry(position).or_default().push((ordinal, id)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?;
        }
        let mut children = BTreeMap::<i64, Vec<(usize, i64)>>::new(); ids.clear();
        for row in &database.table(tables.child)?.rows {
            identity(row, 4)?; let parent = row.integer(1)?; let ordinal = index(row.integer(2)?)?; let child = row.integer(3)?;
            if !ids.insert(row.rowid) || nodes.get(&parent).ok_or("XML child parent is dangling")?.text(1)? != "element" || !nodes.contains_key(&child) || !owners.insert(child) { return Err("XML child requires an element parent, a node and unique ownership".into()); } children.entry(parent).or_default().push((ordinal, child)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?;
        }
        let mut attributes = BTreeMap::<i64, Vec<(usize, &SqliteRow)>>::new(); ids.clear();
        for row in &database.table(tables.attribute)?.rows { identity(row, 5)?; let parent = row.integer(1)?; if !ids.insert(row.rowid) || nodes.get(&parent).ok_or("XML attribute parent is dangling")?.text(1)? != "element" { return Err("XML attribute requires an element parent and unique identity".into()); } row.text(3)?; row.text(4)?; attributes.entry(parent).or_default().push((index(row.integer(2)?)?, row)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
        for ordered in children.values_mut().chain(boundaries.values_mut()) { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed.min(total), total)?; ordered.sort_by_key(|(ordinal, _)| *ordinal); for (ordinal, (actual, _)) in ordered.iter().enumerate() { if ordinal != *actual { return Err("XML node ordinals must be contiguous and zero-based".into()); } } }
        for ordered in attributes.values_mut() { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed.min(total), total)?; ordered.sort_by_key(|(ordinal, _)| *ordinal); for (ordinal, (actual, _)) in ordered.iter().enumerate() { if ordinal != *actual { return Err("XML attribute ordinals must be contiguous and zero-based".into()); } } }
        if owners.len() != nodes.len() { return Err("XML node has no document or element owner".into()); }
        let mut roots = Vec::new(); if let Some(root) = root { roots.push(root); } for position in ["prolog", "epilog"] { if let Some(ordered) = boundaries.get(position) { roots.extend(ordered.iter().map(|(_, id)| *id)); } }
        let mut stack: Vec<_> = roots.iter().rev().map(|id| (*id, false)).collect(); let mut visited = BTreeSet::new(); let mut built = BTreeMap::new();
        while let Some((id, finish)) = stack.pop() {
            if !finish { if !visited.insert(id) { return Err("XML child graph contains a cycle".into()); } stack.push((id, true)); if let Some(ordered) = children.get(&id) { stack.extend(ordered.iter().rev().map(|(_, child)| (*child, false))); } continue; }
            let payload = components[&id]; if payload.values.iter().any(|value| matches!(value, SqliteValue::Text(text) if text.len() > 65536)) { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed.min(total), total)?; } let node = match nodes[&id].text(1)? {
                "element" => {
                    let mut attrs = Vec::new(); for (_, row) in attributes.remove(&id).unwrap_or_default() { attrs.push(XmlAttr { name: row.text(3)?.into(), value: row.text(4)?.into() }); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
                    let mut child_nodes = Vec::new(); for (_, child) in children.remove(&id).unwrap_or_default() { child_nodes.push(built.remove(&child).ok_or("XML child was not reconstructed")?); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
                    XmlNode::Element { name: payload.text(1)?.into(), attrs, children: child_nodes }
                }
                "text" => XmlNode::Text { text: payload.text(1)?.into() }, "cdata" => XmlNode::CData { text: payload.text(1)?.into() }, "comment" => XmlNode::Comment { text: payload.text(1)?.into() },
                "processing_instruction" => XmlNode::ProcessingInstruction { target: payload.text(1)?.into(), data: payload.text(2)?.into() }, _ => return Err("XML node kind is invalid".into()),
            }; built.insert(id, node); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?;
        }
        if visited.len() != nodes.len() || !children.is_empty() || !attributes.is_empty() { return Err("XML child graph is disconnected or cyclic".into()); }
        let mut doc = XmlDocument::default(); doc.root = root.map(|id| built.remove(&id).ok_or_else(|| "XML root was not reconstructed".to_string())).transpose()?;
        for (position, target) in [("prolog", &mut doc.prolog), ("epilog", &mut doc.epilog)] { for (_, id) in boundaries.remove(position).unwrap_or_default() { target.push(built.remove(&id).ok_or("XML boundary was not reconstructed")?); } }
        let declarations = &database.table(tables.declaration)?.rows; if declarations.len() > 1 { return Err("XML document has multiple declarations".into()); }
        if let Some(row) = declarations.first() { identity(row, 6)?; if row.rowid != 1 || row.integer(1)? != 1 { return Err("XML declaration document is invalid".into()); } doc.declaration = Some(XmlDeclaration { version: row.text(2)?.into(), encoding: row.optional_text(3)?.map(str::to_string), standalone: if row.values[4] == SqliteValue::Null { None } else { Some(boolean(row, 4)?) }, quote: match row.text(5)? { "double" => XmlQuote::Double, "single" => XmlQuote::Single, _ => return Err("XML declaration quote is invalid".into()) } }); }
        let doctypes = &database.table(tables.doctype)?.rows; if doctypes.len() > 1 { return Err("XML document has multiple doctypes".into()); }
        if let Some(row) = doctypes.first() {
            identity(row, 7)?; if row.rowid != 1 || row.integer(1)? != 1 { return Err("XML doctype document is invalid".into()); }
            let external_id = match (row.optional_text(4)?, row.optional_text(5)?, row.optional_text(6)?) { (None, None, None) => None, (Some("system"), None, Some(system_id)) => Some(XmlExternalId::System { system_id: system_id.into() }), (Some("public"), Some(public_id), Some(system_id)) => Some(XmlExternalId::Public { public_id: public_id.into(), system_id: system_id.into() }), _ => return Err("XML external identifier shape is invalid".into()) };
            let mut entities = Vec::new(); ids.clear(); for entity in database.table(tables.entity)?.ordered_rows(2)? { identity(entity, 6)?; if entity.integer(1)? != 1 || !ids.insert(entity.rowid) { return Err("XML entity doctype or identity is invalid".into()); } entities.push(XmlDtdDeclaration::Entity { parameter: boolean(entity, 3)?, name: entity.text(4)?.into(), value: entity.text(5)?.into() }); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
            doc.doctype = Some(XmlDoctype { prolog_position: index(row.integer(2)?)?, name: row.text(3)?.into(), external_id, declarations: entities });
        } else if !database.table(tables.entity)?.rows.is_empty() { return Err("XML entities lack their doctype".into()); }
        validate_xml_document_boundaries(&doc)?; control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?; Ok((document.text(1)?.into(), doc))
    }

/// 📏️ Bounds native XML state codecs by their explicitly shared XmlDocument fields.
pub fn preflight_xml_document(schema: &str, doc: &XmlDocument, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> {
    use semio_framework_os_kernel::sqlite_snapshot::{artifact::NativeEncodingBound, SnapshotEncoding};
    let mut bound = NativeEncodingBound::new(control)?; bound.add(1024)?;
    let multiplier = if encoding == SnapshotEncoding::Text { 16 } else { 4 };
    bound.repeated(schema.len(), multiplier)?;
    if let Some(declaration) = &doc.declaration {
        bound.add(128)?; bound.repeated(declaration.version.len(), multiplier)?;
        if let Some(encoding) = &declaration.encoding { bound.repeated(encoding.len(), multiplier)?; }
    }
    let mut rows = doc.prolog.len().checked_add(doc.epilog.len()).and_then(|value| value.checked_add(2)).ok_or("XML native row count overflow")?;
    if let Some(doctype) = &doc.doctype {
        rows = rows.checked_add(doctype.declarations.len()).ok_or("XML native DTD count overflow")?; bound.check_rows(rows)?;
        bound.add(128)?; bound.repeated(doctype.name.len(), multiplier)?;
        match &doctype.external_id {
            Some(XmlExternalId::System { system_id }) => bound.repeated(system_id.len(), multiplier)?,
            Some(XmlExternalId::Public { public_id, system_id }) => { bound.repeated(public_id.len(), multiplier)?; bound.repeated(system_id.len(), multiplier)?; },
            None => {},
        }
        for declaration in &doctype.declarations { match declaration { XmlDtdDeclaration::Entity { name, value, .. } => { bound.add(64)?; bound.repeated(name.len(), multiplier)?; bound.repeated(value.len(), multiplier)?; } } }
    }
    bound.check_rows(rows)?; bound.repeated(rows, 32)?;
    let mut stack = Vec::new(); stack.extend(doc.epilog.iter().rev()); if let Some(root) = &doc.root { stack.push(root); } stack.extend(doc.prolog.iter().rev());
    while let Some(node) = stack.pop() {
        bound.add(128)?;
        match node {
            XmlNode::Element { name, attrs, children } => {
                rows = rows.checked_add(attrs.len()).and_then(|value| value.checked_add(children.len())).ok_or("XML native entity count overflow")?;
                bound.check_rows(rows)?; bound.repeated(children.len(), 32)?; bound.repeated(name.len(), multiplier)?;
                for attr in attrs { bound.add(64)?; bound.repeated(attr.name.len(), multiplier)?; bound.repeated(attr.value.len(), multiplier)?; }
                stack.extend(children.iter().rev());
            },
            XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => bound.repeated(text.len(), multiplier)?,
            XmlNode::ProcessingInstruction { target, data } => { bound.repeated(target.len(), multiplier)?; bound.repeated(data.len(), multiplier)?; },
        }
    }
    bound.finish()
}

impl ArtifactSqliteSnapshot for XmlSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> { preflight_xml_document(&self.schema, &self.doc, encoding, control) }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.xml"||dialect.standard!="1.0"{return Err(String::from("owned xml snapshot dialect differs from its semantic standard").into());}
        let row=database.table("xml_document")?.single_row()?;if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("owned xml document identity differs from semantic projection").into());}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"valid"=>crate::standards::v1_0::subsets::valid::schema::check_valid_conformance(self),_=>return Err(String::from("named xml subset has no owned semantic validator").into())};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> { project_xml_document(&self.schema, &self.doc, Self::SQLITE_SCHEMA, XML_SQLITE_TABLES, control) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> { let (schema, doc) = reconstruct_xml_document(database, Self::SQLITE_SCHEMA, XML_SQLITE_TABLES, control)?; Ok(Self { schema, doc }) }
}
