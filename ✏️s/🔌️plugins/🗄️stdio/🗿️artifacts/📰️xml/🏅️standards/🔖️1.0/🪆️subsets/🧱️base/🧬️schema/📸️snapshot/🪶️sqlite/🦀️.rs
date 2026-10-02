//! 📰️ Handcrafted XML document, node, attribute, declaration and entity relations.

use super::{XmlAttr, XmlDeclaration, XmlDocument, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote, XmlSnapshot};
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

/// 🪟️ Borrowed XML component roots for enclosing typed owners.
#[derive(Clone,Copy)]
pub struct XmlDocumentView<'a>{pub root:Option<&'a XmlNode>,pub doctype:Option<&'a XmlDoctype>,pub declaration:Option<&'a XmlDeclaration>,pub prolog:&'a[XmlNode],pub epilog:&'a[XmlNode]}
impl<'a> From<&'a XmlDocument> for XmlDocumentView<'a>{fn from(doc:&'a XmlDocument)->Self{Self{root:doc.root.as_ref(),doctype:doc.doctype.as_ref(),declaration:doc.declaration.as_ref(),prolog:&doc.prolog,epilog:&doc.epilog}}}
impl<'a> XmlDocumentView<'a>{pub fn node(node:&'a XmlNode)->Self{Self{root:Some(node),doctype:None,declaration:None,prolog:&[],epilog:&[]}}}

pub(super) fn retire_nodes(nodes: impl IntoIterator<Item = XmlNode>) {
    let mut pending: Vec<_> = nodes.into_iter().collect();
    while let Some(node) = pending.pop() { if let XmlNode::Element { children, .. } = node { pending.extend(children); } }
}
struct XmlForest(BTreeMap<i64, XmlNode>);
impl Drop for XmlForest { fn drop(&mut self) { retire_nodes(std::mem::take(&mut self.0).into_values()); } }
pub(super) struct XmlNodeList(pub(super) Vec<XmlNode>);
impl Drop for XmlNodeList { fn drop(&mut self) { retire_nodes(std::mem::take(&mut self.0)); } }
pub(super) struct XmlDocumentOwner(pub(super) Option<XmlDocument>);
impl Drop for XmlDocumentOwner { fn drop(&mut self) { if let Some(mut doc) = self.0.take() { let mut nodes = std::mem::take(&mut doc.prolog); nodes.extend(std::mem::take(&mut doc.epilog)); nodes.extend(doc.root.take()); retire_nodes(nodes); } } }
/// ♻️ Retires an enclosing owner's complete typed XML document iteratively.
pub fn retire_xml_document(doc:XmlDocument){drop(XmlDocumentOwner(Some(doc)));}
struct XmlSnapshots(Vec<XmlSnapshot>);
impl Drop for XmlSnapshots{fn drop(&mut self){for snapshot in self.0.drain(..){drop(XmlDocumentOwner(Some(snapshot.doc)));}}}
enum Parent { Root, Misc(&'static str, usize), Child(i64, usize) }
fn integer(value: usize) -> Result<i64, String> { i64::try_from(value).map_err(|error| error.to_string()) }
fn index(value: i64) -> Result<usize, String> { usize::try_from(value).map_err(|_| "XML ordinal must be nonnegative".into()) }
fn add(value: &mut usize, amount: usize) -> Result<(), String> { *value = value.checked_add(amount).ok_or("XML relational size overflow")?; Ok(()) }
fn kind(node: &XmlNode) -> &'static str { match node { XmlNode::Element { .. } => "element", XmlNode::Text { .. } => "text", XmlNode::CData { .. } => "cdata", XmlNode::Comment { .. } => "comment", XmlNode::ProcessingInstruction { .. } => "processing_instruction" } }
fn push(database: &mut SqliteDatabase, table: &str, values: Vec<SqliteValue>) -> Result<i64, String> { let rows = &mut database.table_mut(table)?.rows; let id = integer(rows.len() + 1)?; let mut fields = vec![SqliteValue::Integer(id)]; fields.extend(values); rows.push(SqliteRow { rowid: id, values: fields }); Ok(id) }
fn component(database: &mut SqliteDatabase, table: &str, id: i64, values: Vec<SqliteValue>) -> Result<(), String> { let mut fields = vec![SqliteValue::Integer(id)]; fields.extend(values); database.table_mut(table)?.rows.push(SqliteRow { rowid: id, values: fields }); Ok(()) }
fn text(value: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteValue, String> { semio_framework_os_kernel::sqlite_snapshot::artifact::project_text(control, value).map(SqliteValue::Text) }
fn optional(value: Option<&str>, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteValue, String> { value.map(|value| text(value, control)).transpose().map(|value| value.unwrap_or(SqliteValue::Null)) }
fn restored(value: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<String, String> { semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control, value) }
fn restored_optional(value: Option<&str>, control: &mut SqliteSnapshotControl<'_>) -> Result<Option<String>, String> { value.map(|value| restored(value, control)).transpose() }
fn boundaries(doc: XmlDocumentView<'_>, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), String> {
    let total = doc.prolog.len().checked_add(doc.epilog.len()).ok_or("XML boundary count overflow")?; control.check_rows(total.checked_add(1).ok_or("XML boundary count overflow")?)?;
    for ordinal in (0..total).step_by(256) { control.checkpoint(phase, ordinal, total)?; }
    Ok(())
}
fn boolean(row: &SqliteRow, column: usize) -> Result<bool, String> { match row.integer(column)? { 0 => Ok(false), 1 => Ok(true), _ => Err("XML boolean must be 0 or 1".into()) } }
fn identity(row: &SqliteRow, columns: usize) -> Result<(), String> { if row.values.len() != columns || row.integer(0)? != row.rowid { Err("XML row identity or column count is invalid".into()) } else { Ok(()) } }
fn tick(control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase, completed: &mut usize, total: usize) -> Result<(), String> { add(completed, 1)?; if *completed % 256 == 0 { control.checkpoint(phase, (*completed).min(total), total)?; } Ok(()) }

fn measure(schema: &str, doc: XmlDocumentView<'_>, control: &mut SqliteSnapshotControl<'_>) -> Result<(usize,usize), String> {
    let mut rows = 1usize; let mut bytes = schema.len(); add(&mut bytes, if doc.root.is_some() { 16 } else { 8 })?;
    if let Some(declaration) = &doc.declaration { add(&mut rows, 1)?; add(&mut bytes, 16 + declaration.version.len() + declaration.encoding.as_ref().map_or(0, String::len) + if declaration.standalone.is_some() { 8 } else { 0 } + match declaration.quote { XmlQuote::Double => 6, XmlQuote::Single => 6 })?; }
    if let Some(doctype) = &doc.doctype {
        add(&mut rows, 1)?; add(&mut bytes, 16 + super::position::digits(doctype.prolog_position) + doctype.name.len())?;
        match &doctype.external_id { None => {}, Some(XmlExternalId::System { system_id }) => add(&mut bytes, 6 + system_id.len())?, Some(XmlExternalId::Public { public_id, system_id }) => add(&mut bytes, 6 + public_id.len() + system_id.len())? }
        for declaration in &doctype.declarations { let XmlDtdDeclaration::Entity { name, value, .. } = declaration; add(&mut rows, 1)?; add(&mut bytes, 32 + name.len() + value.len())?; control.check_rows(rows)?; control.check_value_bytes(bytes)?; if rows % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, rows)?; } }
    }
    let boundary_rows = doc.prolog.len().checked_add(doc.epilog.len()).ok_or("XML boundary size overflow")?; add(&mut rows, boundary_rows)?;
    add(&mut bytes, doc.prolog.len().checked_mul(38).ok_or("XML prolog size overflow")?)?; add(&mut bytes, doc.epilog.len().checked_mul(38).ok_or("XML epilog size overflow")?)?;
    control.check_rows(rows.checked_add(boundary_rows.checked_mul(2).ok_or("XML boundary size overflow")?).ok_or("XML boundary size overflow")?)?; control.check_value_bytes(bytes)?;
    let mut stack = Vec::new(); if let Some(root) = doc.root { stack.push(root); } stack.extend(doc.prolog.iter()); stack.extend(doc.epilog.iter()); let mut visited = 0usize;
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
    Ok((rows,bytes))
}

/// 📰️ Projects the shared native XML document fields into a caller's explicit typed relational vocabulary.
pub fn project_xml_document(schema:&str,doc:&XmlDocument,ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project_xml_documents(&[(schema,doc)],ddl,tables,control)}

/// 📏️ Measures the literal XML component graph before a container owns any rows.
pub fn measure_xml_documents(documents:&[(&str,&XmlDocument)],control:&mut SqliteSnapshotControl<'_>)->Result<(usize,usize),String>{measure_views(documents.iter().map(|(schema,doc)|(*schema,XmlDocumentView::from(*doc))),control)}
/// 📏️ Measures borrowed XML node and document domains before row ownership.
pub fn measure_xml_document_views(documents:&[(&str,XmlDocumentView<'_>)],control:&mut SqliteSnapshotControl<'_>)->Result<(usize,usize),String>{measure_views(documents.iter().copied(),control)}
fn measure_views<'a>(documents:impl ExactSizeIterator<Item=(&'a str,XmlDocumentView<'a>)>,control:&mut SqliteSnapshotControl<'_>)->Result<(usize,usize),String>{
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;control.check_rows(documents.len())?;
    let count=documents.len();let(mut total,mut bytes)=(0,0);for(position,(schema,doc))in documents.enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,position,count)?;}boundaries(doc,control,SqliteSnapshotPhase::ProjectSnapshot)?;let(rows,size)=measure(schema,doc,control)?;add(&mut total,rows)?;add(&mut bytes,size)?;control.check_rows(total)?;control.check_value_bytes(bytes)?;}
    Ok((total,bytes))
}

/// 🕸️ Projects literal document roots directly into one declared typed XML graph.
pub fn project_xml_documents(documents:&[(&str,&XmlDocument)],ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project_views(documents.iter().map(|(schema,doc)|(*schema,XmlDocumentView::from(*doc))),ddl,tables,control)}
/// 🕸️ Projects borrowed typed XML nodes and documents without intermediate native clones.
pub fn project_xml_document_views(documents:&[(&str,XmlDocumentView<'_>)],ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project_views(documents.iter().copied(),ddl,tables,control)}
fn project_views<'a>(documents:impl ExactSizeIterator<Item=(&'a str,XmlDocumentView<'a>)>+Clone,ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
    let(total,_bytes)=measure_views(documents.clone(),control)?;let count=documents.len();
    let mut database=SqliteDatabase::from_schema(ddl).map_err(|error|error.to_string())?;let mut completed=0;
    for(position,(schema,doc))in documents.enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,position,count)?;}
        let root=integer(database.table(tables.node)?.rows.len().checked_add(1).ok_or("XML node count overflow")?)?;let document_id=push(&mut database, tables.document, vec![text(schema, control)?, if doc.root.is_some() { SqliteValue::Integer(root) } else { SqliteValue::Null }])?;
        if let Some(declaration) = &doc.declaration { push(&mut database, tables.declaration, vec![SqliteValue::Integer(document_id), text(&declaration.version, control)?, optional(declaration.encoding.as_deref(), control)?, declaration.standalone.map(|value| SqliteValue::Integer(i64::from(value))).unwrap_or(SqliteValue::Null), text(match declaration.quote { XmlQuote::Double => "double", XmlQuote::Single => "single" }, control)?])?; }
        if let Some(doctype) = &doc.doctype {
            let (external_kind, public_id, system_id) = match &doctype.external_id { None => (None, None, None), Some(XmlExternalId::System { system_id }) => (Some("system"), None, Some(system_id.as_str())), Some(XmlExternalId::Public { public_id, system_id }) => (Some("public"), Some(public_id.as_str()), Some(system_id.as_str())) };
            let doctype_id=push(&mut database, tables.doctype, vec![SqliteValue::Integer(document_id), text(&doctype.prolog_position.to_string(), control)?, text(&doctype.name, control)?, optional(external_kind, control)?, optional(public_id, control)?, optional(system_id, control)?])?;
            for (ordinal, entity) in doctype.declarations.iter().enumerate() { let XmlDtdDeclaration::Entity { parameter, name, value } = entity; push(&mut database, tables.entity, vec![SqliteValue::Integer(doctype_id), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(i64::from(*parameter)), text(name, control)?, text(value, control)?])?; tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut completed, total)?; }
        }
        let mut stack = Vec::new(); for (ordinal, node) in doc.epilog.iter().enumerate().rev() { stack.push((node, Parent::Misc("epilog", ordinal))); } for (ordinal, node) in doc.prolog.iter().enumerate().rev() { stack.push((node, Parent::Misc("prolog", ordinal))); } if let Some(root) = doc.root { stack.push((root, Parent::Root)); }
        while let Some((node, parent)) = stack.pop() {
            if match node { XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => text.len() > 65536, XmlNode::ProcessingInstruction { target, data } => target.len().saturating_add(data.len()) > 65536, XmlNode::Element { name, .. } => name.len() > 65536 } { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, completed, total)?; }
            let id = push(&mut database, tables.node, vec![text(kind(node), control)?])?;
            match node {
                XmlNode::Element { name, attrs, children } => {
                    component(&mut database, tables.element, id, vec![text(name, control)?])?;
                    for (ordinal, attr) in attrs.iter().enumerate() { if attr.name.len().saturating_add(attr.value.len()) > 65536 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, completed, total)?; } push(&mut database, tables.attribute, vec![SqliteValue::Integer(id), SqliteValue::Integer(integer(ordinal)?), text(&attr.name, control)?, text(&attr.value, control)?])?; tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut completed, total)?; }
                    for (ordinal, child) in children.iter().enumerate().rev() { stack.push((child, Parent::Child(id, ordinal))); }
                }
                XmlNode::Text { text: value } => component(&mut database, tables.text, id, vec![text(value, control)?])?,
                XmlNode::CData { text: value } => component(&mut database, tables.cdata, id, vec![text(value, control)?])?,
                XmlNode::Comment { text: value } => component(&mut database, tables.comment, id, vec![text(value, control)?])?,
                XmlNode::ProcessingInstruction { target, data } => component(&mut database, tables.processing_instruction, id, vec![text(target, control)?, text(data, control)?])?,
            }
            match parent { Parent::Root => {}, Parent::Misc(position, ordinal) => { push(&mut database, tables.document_misc, vec![SqliteValue::Integer(document_id), text(position, control)?, SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(id)])?; }, Parent::Child(parent, ordinal) => { push(&mut database, tables.child, vec![SqliteValue::Integer(parent), SqliteValue::Integer(integer(ordinal)?), SqliteValue::Integer(id)])?; } }
            tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut completed, total)?;
        }
    }
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,total,total)?;Ok(database)
}

/// 📰️ Reconstructs exactly the native XML document fields from a caller's declared typed vocabulary.
pub fn reconstruct_xml_document(database:&SqliteDatabase,ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<(String,XmlDocument),String>{
    if database.tables.len()!=tables.names().len()||database.table(tables.document)?.rows.len()!=1{return Err("single XML snapshot requires thirteen tables and one document".into());}
    let mut result=reconstruct_xml_documents(database,ddl,tables,control)?;let snapshot=result.pop().ok_or("XML graph has no document")?;Ok((snapshot.schema,snapshot.doc))
}

/// 🕸️ Reconstructs all declared document roots through one explicit typed component graph.
pub fn reconstruct_xml_documents(database: &SqliteDatabase, ddl: &str, tables: XmlSqliteTables, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<XmlSnapshot>, String> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        validate_sqlite_database_schema(database, ddl, control.limits()).map_err(|error| error.to_string())?;
        for name in tables.names() { database.table(name)?; }
        let total = database.tables.iter().map(|table| table.rows.len()).sum(); let mut completed = 0usize;
        let mut documents=BTreeMap::new();let mut document_roots=BTreeMap::new();
        for(position,row)in database.table(tables.document)?.rows.iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,database.table(tables.document)?.rows.len())?;}identity(row,3)?;if row.rowid!=integer(position+1)?||documents.insert(row.rowid,row).is_some(){return Err("XML document identities must be a dense ordered sequence".into());}let root=match row.values[2]{SqliteValue::Null=>None,SqliteValue::Integer(id)=>Some(id),_=>return Err("XML document root must be an integer reference or NULL".into())};document_roots.insert(row.rowid,root);}
        let mut nodes = BTreeMap::new();
        for row in &database.table(tables.node)?.rows { identity(row, 2)?; if !matches!(row.text(1)?, "element" | "text" | "cdata" | "comment" | "processing_instruction") || nodes.insert(row.rowid, row).is_some() { return Err("XML node kind or identity is invalid".into()); } tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
        let mut components = BTreeMap::new();
        for (table, expected) in [(tables.element, "element"), (tables.text, "text"), (tables.cdata, "cdata"), (tables.comment, "comment"), (tables.processing_instruction, "processing_instruction")] {
            for row in &database.table(table)?.rows { identity(row, if expected == "processing_instruction" { 3 } else { 2 })?; if nodes.get(&row.rowid).ok_or("XML component node is dangling")?.text(1)? != expected || components.insert(row.rowid, row).is_some() { return Err("XML component does not match its node kind or has multiple owners".into()); } row.text(1)?; if expected == "processing_instruction" { row.text(2)?; } tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
        }
        if nodes.len() != components.len() { return Err("XML node lacks its typed component".into()); }
        let mut owners=BTreeSet::new();for root in document_roots.values().flatten(){if !nodes.contains_key(root)||!owners.insert(*root){return Err("XML root is dangling or has multiple document owners".into());}}
        let mut boundary_nodes=BTreeMap::<(i64,&str),Vec<(usize,i64)>>::new();let mut ids=BTreeSet::new();
        for row in &database.table(tables.document_misc)?.rows{
            identity(row,5)?;let document=row.integer(1)?;if !documents.contains_key(&document)||!ids.insert(row.rowid){return Err("XML boundary document or identity is invalid".into());}let position=row.text(2)?;let ordinal=index(row.integer(3)?)?;let node=row.integer(4)?;
            if !matches!(position,"prolog"|"epilog")||!nodes.contains_key(&node)||!owners.insert(node){return Err("XML boundary position, node or ownership is invalid".into());}boundary_nodes.entry((document,position)).or_default().push((ordinal,node));tick(control,SqliteSnapshotPhase::ReconstructSnapshot,&mut completed,total)?;
        }
        let mut children = BTreeMap::<i64, Vec<(usize, i64)>>::new(); ids.clear();
        for row in &database.table(tables.child)?.rows {
            identity(row, 4)?; let parent = row.integer(1)?; let ordinal = index(row.integer(2)?)?; let child = row.integer(3)?;
            if !ids.insert(row.rowid) || nodes.get(&parent).ok_or("XML child parent is dangling")?.text(1)? != "element" || !nodes.contains_key(&child) || !owners.insert(child) { return Err("XML child requires an element parent, a node and unique ownership".into()); } children.entry(parent).or_default().push((ordinal, child)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?;
        }
        let mut attributes = BTreeMap::<i64, Vec<(usize, &SqliteRow)>>::new(); ids.clear();
        for row in &database.table(tables.attribute)?.rows { identity(row, 5)?; let parent = row.integer(1)?; if !ids.insert(row.rowid) || nodes.get(&parent).ok_or("XML attribute parent is dangling")?.text(1)? != "element" { return Err("XML attribute requires an element parent and unique identity".into()); } row.text(3)?; row.text(4)?; attributes.entry(parent).or_default().push((index(row.integer(2)?)?, row)); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
        for ordered in children.values_mut().chain(boundary_nodes.values_mut()) { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed.min(total), total)?; ordered.sort_by_key(|(ordinal, _)| *ordinal); for (ordinal, (actual, _)) in ordered.iter().enumerate() { if ordinal != *actual { return Err("XML node ordinals must be contiguous and zero-based".into()); } } }
        for ordered in attributes.values_mut() { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed.min(total), total)?; ordered.sort_by_key(|(ordinal, _)| *ordinal); for (ordinal, (actual, _)) in ordered.iter().enumerate() { if ordinal != *actual { return Err("XML attribute ordinals must be contiguous and zero-based".into()); } } }
        if owners.len() != nodes.len() { return Err("XML node has no document or element owner".into()); }
        let mut roots=Vec::new();roots.extend(document_roots.values().flatten().copied());for ordered in boundary_nodes.values(){roots.extend(ordered.iter().map(|(_,id)|*id));}
        let mut stack: Vec<_> = roots.iter().rev().map(|id| (*id, false)).collect(); let mut visited = BTreeSet::new(); let mut built = XmlForest(BTreeMap::new());
        while let Some((id, finish)) = stack.pop() {
            if !finish { if !visited.insert(id) { return Err("XML child graph contains a cycle".into()); } stack.push((id, true)); if let Some(ordered) = children.get(&id) { stack.extend(ordered.iter().rev().map(|(_, child)| (*child, false))); } continue; }
            let payload = components[&id]; if payload.values.iter().any(|value| matches!(value, SqliteValue::Text(text) if text.len() > 65536)) { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed.min(total), total)?; } let node = match nodes[&id].text(1)? {
                "element" => {
                    let mut attrs = Vec::new(); for (_, row) in attributes.remove(&id).unwrap_or_default() { attrs.push(XmlAttr { name: restored(row.text(3)?, control)?, value: restored(row.text(4)?, control)? }); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
                    let mut child_nodes = XmlNodeList(Vec::new()); for (_, child) in children.remove(&id).unwrap_or_default() { child_nodes.0.push(built.0.remove(&child).ok_or("XML child was not reconstructed")?); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?; }
                    XmlNode::Element { name: restored(payload.text(1)?, control)?, attrs, children: std::mem::take(&mut child_nodes.0) }
                }
                "text" => XmlNode::Text { text: restored(payload.text(1)?, control)? }, "cdata" => XmlNode::CData { text: restored(payload.text(1)?, control)? }, "comment" => XmlNode::Comment { text: restored(payload.text(1)?, control)? },
                "processing_instruction" => XmlNode::ProcessingInstruction { target: restored(payload.text(1)?, control)?, data: restored(payload.text(2)?, control)? }, _ => return Err("XML node kind is invalid".into()),
            }; built.0.insert(id, node); tick(control, SqliteSnapshotPhase::ReconstructSnapshot, &mut completed, total)?;
        }
        if visited.len() != nodes.len() || !children.is_empty() || !attributes.is_empty() { return Err("XML child graph is disconnected or cyclic".into()); }
        let mut declarations=BTreeMap::new();ids.clear();
        for row in &database.table(tables.declaration)?.rows{identity(row,6)?;let document=row.integer(1)?;if row.rowid<=0||!documents.contains_key(&document)||!ids.insert(row.rowid)||declarations.insert(document,row).is_some(){return Err("XML declaration has an unknown or duplicate document owner".into());}}
        let mut doctypes=BTreeMap::new();let mut doctype_ids=BTreeSet::new();
        for row in &database.table(tables.doctype)?.rows{identity(row,7)?;let document=row.integer(1)?;if row.rowid<=0||!documents.contains_key(&document)||!doctype_ids.insert(row.rowid)||doctypes.insert(document,row).is_some(){return Err("XML doctype has an unknown or duplicate document owner".into());}}
        let mut entity_rows=BTreeMap::<i64,Vec<&SqliteRow>>::new();ids.clear();
        for row in &database.table(tables.entity)?.rows{identity(row,6)?;let owner=row.integer(1)?;if row.rowid<=0||!doctype_ids.contains(&owner)||!ids.insert(row.rowid){return Err("XML entity doctype or identity is invalid".into());}entity_rows.entry(owner).or_default().push(row);}
        for rows in entity_rows.values_mut(){rows.sort_by_key(|row|row.integer(2).unwrap_or(-1));for(ordinal,row)in rows.iter().enumerate(){if row.integer(2)?!=integer(ordinal)?{return Err("XML entity ordinals must be dense".into());}}}
        let mut result=XmlSnapshots(Vec::new());
        let document_count=documents.len();for(position,(id,document))in documents.into_iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,document_count)?;}
            let mut owner=XmlDocumentOwner(Some(XmlDocument::default()));let doc=owner.0.as_mut().ok_or("XML document owner is empty")?;
            doc.root=document_roots[&id].map(|node|built.0.remove(&node).ok_or_else(||"XML root was not reconstructed".to_string())).transpose()?;
            for(position,target)in[("prolog",&mut doc.prolog),("epilog",&mut doc.epilog)]{for(_,node)in boundary_nodes.remove(&(id,position)).unwrap_or_default(){target.push(built.0.remove(&node).ok_or("XML boundary was not reconstructed")?);}}
            if let Some(row)=declarations.remove(&id){doc.declaration=Some(XmlDeclaration{version:restored(row.text(2)?,control)?,encoding:restored_optional(row.optional_text(3)?,control)?,standalone:if row.values[4]==SqliteValue::Null{None}else{Some(boolean(row,4)?)},quote:match row.text(5)?{"double"=>XmlQuote::Double,"single"=>XmlQuote::Single,_=>return Err("XML declaration quote is invalid".into())}});}
            if let Some(row)=doctypes.remove(&id){
                let external_id=match(row.optional_text(4)?,row.optional_text(5)?,row.optional_text(6)?){(None,None,None)=>None,(Some("system"),None,Some(system_id))=>Some(XmlExternalId::System{system_id:restored(system_id,control)?}),(Some("public"),Some(public_id),Some(system_id))=>Some(XmlExternalId::Public{public_id:restored(public_id,control)?,system_id:restored(system_id,control)?}),_=>return Err("XML external identifier shape is invalid".into())};
                let mut entities=Vec::new();for entity in entity_rows.remove(&row.rowid).unwrap_or_default(){entities.push(XmlDtdDeclaration::Entity{parameter:boolean(entity,3)?,name:restored(entity.text(4)?,control)?,value:restored(entity.text(5)?,control)?});tick(control,SqliteSnapshotPhase::ReconstructSnapshot,&mut completed,total)?;}
                doc.doctype=Some(XmlDoctype{prolog_position:super::position::parse(row.text(2)?)?,name:restored(row.text(3)?,control)?,external_id,declarations:entities});
            }
            boundaries(XmlDocumentView::from(&*doc),control,SqliteSnapshotPhase::ReconstructSnapshot)?;let schema=restored(document.text(1)?,control)?;result.0.push(XmlSnapshot{schema,doc:owner.0.take().ok_or("XML document owner is empty")?});
        }
        if !built.0.is_empty()||!boundary_nodes.is_empty()||!entity_rows.is_empty(){return Err("XML graph has unclaimed owned components".into());}
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,total,total)?;Ok(std::mem::take(&mut result.0))
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
    fn decode_sqlite_snapshot_native(payload: &semio_framework_os_kernel::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> { super::native_decoding::decode(payload, control) }
    fn retire_sqlite_snapshot(self) { drop(XmlDocumentOwner(Some(self.doc))); }
    fn encode_sqlite_snapshot_native(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<semio_framework_os_kernel::io_schema::IoPayload, String> { super::native_encoding::encode(self, encoding, control) }
    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> { preflight_xml_document(&self.schema, &self.doc, encoding, control) }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.xml"||dialect.standard!="1.0"{return Err(String::from("owned xml snapshot dialect differs from its semantic standard").into());}
        let row=database.table("xml_document")?.single_row()?;if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("owned xml document identity differs from semantic projection").into());}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"valid"=>crate::standards::v1_0::subsets::valid::schema::check_valid_conformance_controlled(self,control)?,_=>return Err(String::from("named xml subset has no owned semantic validator").into())};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> { project_xml_document(&self.schema, &self.doc, Self::SQLITE_SCHEMA, XML_SQLITE_TABLES, control) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> { let (schema, doc) = reconstruct_xml_document(database, Self::SQLITE_SCHEMA, XML_SQLITE_TABLES, control)?; Ok(Self { schema, doc }) }
}
