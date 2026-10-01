//! 🎨️ SVG's explicit relational vocabulary over its native typed XmlDocument fields.

use super::{SvgSnapshot, XmlDocument, XmlNode};
use semio_framework_os_kernel::{sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl}, ArtifactSqliteSnapshot};
use semio_s_artifact_stdio_xml::schema::snapshot::sqlite::{project_xml_document, reconstruct_xml_document, XmlSqliteTables};

const TABLES: XmlSqliteTables = XmlSqliteTables {
    document: "svg_document", node: "svg_node", element: "svg_element", text: "svg_text", cdata: "svg_cdata", comment: "svg_comment",
    processing_instruction: "svg_processing_instruction", attribute: "svg_attribute", child: "svg_child", document_misc: "svg_document_misc",
    declaration: "svg_declaration", doctype: "svg_doctype", entity: "svg_entity",
};

fn validate_root(doc: &XmlDocument) -> Result<(), String> { match &doc.root { Some(XmlNode::Element { name, .. }) if name == "svg" || name.ends_with(":svg") => Ok(()), _ => Err("SVG snapshot requires an svg root element".into()) } }

impl ArtifactSqliteSnapshot for SvgSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> { semio_s_artifact_stdio_xml::schema::snapshot::sqlite::preflight_xml_document(&self.schema, &self.doc, encoding, control) }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.stdio.svg"||dialect.standard!="1.1"{return Err(String::from("SVG owned snapshot dialect differs from SVG 1.1").into());}
        validate_root(&self.doc)?;let row=database.table("svg_document")?.single_row()?;if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("SVG owned document identity differs from semantic projection").into());}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"tiny"=>crate::standards::v1_1::subsets::tiny::schema::check_svg_tiny_conformance_controlled(self,control)?,"basic"=>crate::standards::v1_1::subsets::basic::schema::check_svg_basic_conformance_controlled(self,control)?,_=>return Err(String::from("SVG named subset has no owned semantic validator").into())};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> { validate_root(&self.doc)?; project_xml_document(&self.schema, &self.doc, Self::SQLITE_SCHEMA, TABLES, control) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> { let (schema, doc) = reconstruct_xml_document(database, Self::SQLITE_SCHEMA, TABLES, control)?; validate_root(&doc)?; Ok(Self { schema, doc }) }
}
