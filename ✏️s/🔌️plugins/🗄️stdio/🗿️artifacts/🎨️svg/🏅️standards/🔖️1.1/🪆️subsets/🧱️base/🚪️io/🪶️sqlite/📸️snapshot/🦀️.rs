//! 🎨️ SVG's explicit relational vocabulary over its native typed XmlDocument fields.

use crate::standards::v1_1::subsets::base::schema::snapshot::{SvgSnapshot, XmlDocument, XmlNode};
use semio_framework_os_kernel::{sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl}, ArtifactSqliteSnapshot};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{project_xml_document, reconstruct_xml_document, XmlSqliteTables};

const TABLES: XmlSqliteTables = XmlSqliteTables {
    document: "svg_document", node: "svg_node", element: "svg_element", text: "svg_text", cdata: "svg_cdata", comment: "svg_comment",
    processing_instruction: "svg_processing_instruction", attribute: "svg_attribute", child: "svg_child", document_misc: "svg_document_misc",
    declaration: "svg_declaration", doctype: "svg_doctype", entity: "svg_entity",
};

fn validate_root(doc: &XmlDocument) -> Result<(), ValueError> { match &doc.root { Some(XmlNode::Element { name, .. }) if name == "svg" || name.ends_with(":svg") => Ok(()), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"SVG snapshot requires an svg root element")) } }

impl ArtifactSqliteSnapshot for SvgSnapshot {
    fn decode_sqlite_snapshot_native(payload:&semio_framework_os_kernel::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let mut value=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(crate::standards::v1_1::subsets::base::schema::snapshot::native::decode(payload,control)?,|value:Self|semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::retire_xml_document(value.doc));validate_root(&value.as_mut().doc)?;Ok(value.take())}
    fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<semio_framework_os_kernel::io_schema::IoPayload,ValueError>{validate_root(&self.doc)?;crate::standards::v1_1::subsets::base::schema::snapshot::native::encode(self,encoding,control)}
    fn retire_sqlite_snapshot(self){semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::retire_xml_document(self.doc)}
    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { crate::standards::v1_1::subsets::base::schema::snapshot::native::preflight(self,encoding,control) }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind!="s.stdio.svg"||dialect.standard!="1.1"{return Err(semio_framework_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"SVG owned snapshot dialect differs from SVG 1.1")));}
        validate_root(&self.doc).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;let row=database.table("svg_document").map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?.single_row().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;if row.rowid!=1||row.text(1).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?!=self.schema{return Err(semio_framework_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"SVG owned document identity differs from semantic projection")));}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"tiny"=>crate::standards::v1_1::subsets::tiny::schema::check_svg_tiny_conformance_controlled(self,control).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?,"basic"=>crate::standards::v1_1::subsets::basic::schema::check_svg_basic_conformance_controlled(self,control).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?,_=>return Err(semio_framework_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"SVG named subset has no owned semantic validator")))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {
         validate_root(&self.doc)?; project_xml_document(&self.schema, &self.doc, Self::SQLITE_SCHEMA, TABLES, control) 
        
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> {
         let (schema, doc) = reconstruct_xml_document(database, Self::SQLITE_SCHEMA, TABLES, control)?; let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self{schema,doc},|value:Self|semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::retire_xml_document(value.doc));validate_root(&owner.as_mut().doc)?;Ok(owner.take())
        
    }
}

use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
