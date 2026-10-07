//! 📰️ Handcrafted XML document, node, attribute, declaration and entity relations.

use crate::standards::v1_0::subsets::base::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDocument, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote, XmlSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
#[path="💰️backing/🦀️.rs"]
mod backing;

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

use crate::standards::v1_0::subsets::base::schema::snapshot::ownership::{XmlDocumentView,retire_xml_document,retire_xml_document_with_frontier,retire_nodes,XmlNodeList,XmlDocumentOwner};
struct XmlSnapshots(Vec<XmlSnapshot>);
impl Drop for XmlSnapshots{fn drop(&mut self){for snapshot in self.0.drain(..){drop(XmlDocumentOwner(Some(snapshot.doc)));}}}
enum Parent { Root, Misc(&'static str, usize), Child(i64, usize) }
fn integer(value: usize) -> Result<i64, ValueError> { i64::try_from(value).map_err(|error| ValueError::new(ValueRefusalKind::WorkLimit, error.to_string())) }
fn index(value: i64) -> Result<usize, ValueError> { usize::try_from(value).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "XML ordinal must be nonnegative")) }
fn add(value: &mut usize, amount: usize) -> Result<(), ValueError> { *value = value.checked_add(amount).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML relational size overflow"))?; Ok(()) }
fn add_bytes(value: &mut usize, amount: usize) -> Result<(), ValueError> { *value = value.checked_add(amount).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "XML relational size overflow"))?; Ok(()) }
fn kind(node: &XmlNode) -> &'static str { match node { XmlNode::Element { .. } => "element", XmlNode::Text { .. } => "text", XmlNode::CData { .. } => "cdata", XmlNode::Comment { .. } => "comment", XmlNode::ProcessingInstruction { .. } => "processing_instruction" } }
fn restored(value: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<String, ValueError> { semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control, value) }
fn restored_optional(value: Option<&str>, control: &mut SqliteSnapshotControl<'_>) -> Result<Option<String>, ValueError> { value.map(|value| restored(value, control)).transpose() }
fn boundaries(doc: XmlDocumentView<'_>, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {
    let total = doc.prolog.len().checked_add(doc.epilog.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML boundary count overflow"))?; control.check_rows(total.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML boundary count overflow"))?)?;
    for ordinal in (0..total).step_by(256) { control.checkpoint(phase, ordinal, total)?; }
    Ok(())
}
fn boolean(row: &SqliteRow, column: usize) -> Result<bool, ValueError> { match row.integer(column)? { 0 => Ok(false), 1 => Ok(true), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "XML boolean must be 0 or 1")) } }
fn identity(row: &SqliteRow, columns: usize) -> Result<(), ValueError> { if row.values.len() != columns || row.integer(0)? != row.rowid { Err(ValueError::new(ValueRefusalKind::InvalidValue, "XML row identity or column count is invalid")) } else { Ok(()) } }
fn tick(control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase, completed: &mut usize, total: usize) -> Result<(), ValueError> { add(completed, 1)?; if *completed % 256 == 0 { control.checkpoint(phase, (*completed).min(total), total)?; } Ok(()) }

/// 📰️ Projects the shared native XML document fields into a caller's explicit typed relational vocabulary.
pub fn project_xml_document(schema:&str,doc:&XmlDocument,ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase, ValueError>{project_xml_documents(&[(schema,doc)],ddl,tables,control)}

/// 📏️ Measures the literal XML component graph before a container owns any rows.
pub fn measure_xml_documents(documents:&[(&str,&XmlDocument)],control:&mut SqliteSnapshotControl<'_>)->Result<(usize,usize), ValueError>{measure_views(documents.iter().map(|(schema,doc)|(*schema,XmlDocumentView::from(*doc))),control)}
/// 📏️ Measures borrowed XML node and document domains before row ownership.
pub fn measure_xml_document_views(documents:&[(&str,XmlDocumentView<'_>)],control:&mut SqliteSnapshotControl<'_>)->Result<(usize,usize), ValueError>{measure_views(documents.iter().copied(),control)}
fn measure_views<'a>(documents:impl ExactSizeIterator<Item=(&'a str,XmlDocumentView<'a>)>,control:&mut SqliteSnapshotControl<'_>)->Result<(usize,usize),ValueError>{backing::measure(documents,control)}

/// 🧩️ Appends borrowed XML component rows into an enclosing owner's single controlled projection.
pub fn append_xml_document_views(documents:&[(&str,XmlDocumentView<'_>)],tables:XmlSqliteTables,output:&mut semio_framework_os_kernel::sqlite_snapshot::artifact::Projection<'_, '_>)->Result<(),ValueError>{backing::append(documents.iter().copied(),tables,output)}

/// 🕸️ Projects literal document roots directly into one declared typed XML graph.
pub fn project_xml_documents(documents:&[(&str,&XmlDocument)],ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase, ValueError>{project_views(documents.iter().map(|(schema,doc)|(*schema,XmlDocumentView::from(*doc))),ddl,tables,control)}
/// 🕸️ Projects borrowed typed XML nodes and documents without intermediate native clones.
pub fn project_xml_document_views(documents:&[(&str,XmlDocumentView<'_>)],ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase, ValueError>{project_views(documents.iter().copied(),ddl,tables,control)}
fn project_views<'a>(documents:impl ExactSizeIterator<Item=(&'a str,XmlDocumentView<'a>)>+Clone,ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(documents,ddl,tables,control)}

/// 📰️ Reconstructs exactly the native XML document fields from a caller's declared typed vocabulary.
pub fn reconstruct_xml_document(database:&SqliteDatabase,ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<(String,XmlDocument), ValueError>{
    if database.tables.len()!=tables.names().len()||database.table(tables.document)?.rows.len()!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "single XML snapshot requires thirteen tables and one document"));}
    let mut result=reconstruct_xml_documents(database,ddl,tables,control)?;let snapshot=result.pop().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "XML graph has no document"))?;Ok((snapshot.schema,snapshot.doc))
}

/// 🕸️ Reconstructs all declared document roots through one explicit typed component graph.
pub fn reconstruct_xml_documents(database:&SqliteDatabase,ddl:&str,tables:XmlSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<XmlSnapshot>,ValueError>{backing::reconstruct(database,ddl,tables,control)}

/// 📏️ Bounds native XML state codecs by their explicitly shared XmlDocument fields.
pub fn preflight_xml_document(schema: &str, doc: &XmlDocument, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    use semio_framework_os_kernel::sqlite_snapshot::{artifact::NativeEncodingBound, SnapshotEncoding};
    let mut bound = NativeEncodingBound::new(control)?; bound.add(1024)?;
    let multiplier = if encoding == SnapshotEncoding::Text { 16 } else { 4 };
    bound.repeated(schema.len(), multiplier)?;
    if let Some(declaration) = &doc.declaration {
        bound.add(128)?; bound.repeated(declaration.version.len(), multiplier)?;
        if let Some(encoding) = &declaration.encoding { bound.repeated(encoding.len(), multiplier)?; }
    }
    let mut rows = doc.prolog.len().checked_add(doc.epilog.len()).and_then(|value| value.checked_add(2)).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML native row count overflow"))?;
    if let Some(doctype) = &doc.doctype {
        rows = rows.checked_add(doctype.declarations.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML native DTD count overflow"))?; bound.check_rows(rows)?;
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
                rows = rows.checked_add(attrs.len()).and_then(|value| value.checked_add(children.len())).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML native entity count overflow"))?;
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

use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};
impl ArtifactSqliteSnapshot for XmlSnapshot {
    fn decode_sqlite_snapshot_native(payload: &semio_framework_os_kernel::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> { crate::standards::v1_0::subsets::base::io::sqlite::snapshot::native_decoding::decode(payload, control) }
    fn retire_sqlite_snapshot(self) { drop(XmlDocumentOwner(Some(self.doc))); }
    fn encode_sqlite_snapshot_native(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<semio_framework_os_kernel::io_schema::IoPayload,ValueError> { crate::standards::v1_0::subsets::base::io::sqlite::snapshot::native_encoding::encode(self, encoding, control) }
    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { preflight_xml_document(&self.schema, &self.doc, encoding, control) }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind!="s.stdio.xml"||dialect.standard!="1.0"{return Err(semio_framework_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"owned xml snapshot dialect differs from its semantic standard")));}
        let row=database.table("xml_document").map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?.single_row().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;if row.rowid!=1||row.text(1).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?!=self.schema{return Err(semio_framework_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"owned xml document identity differs from semantic projection")));}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"valid"=>crate::standards::v1_0::subsets::valid::io::check_valid_conformance_controlled(self,control).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?,_=>return Err(semio_framework_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"named xml subset has no owned semantic validator")))};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> { project_xml_document(&self.schema, &self.doc, Self::SQLITE_SCHEMA, XML_SQLITE_TABLES, control) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> { let (schema, doc) = reconstruct_xml_document(database, Self::SQLITE_SCHEMA, XML_SQLITE_TABLES, control)?; Ok(Self { schema, doc }) }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;

/// 🧩️ Typed XML components for enclosing owned native documents.
pub use native_encoding::{XmlNativeEmission,emit_xml_native_document,emit_xml_native_node,emit_xml_native_snapshot_fields};
pub use native_decoding::{XmlNativeInput,read_xml_native_document,read_xml_native_node,read_xml_native_snapshot_fields};
