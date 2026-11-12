//! 📕️ Complete DOCX OPC entities and typed multi-document XML graph.

use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::*;
use semio_os_kernel::{
    sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind},
    os_store::serialization::ArtifactSqliteSnapshotHost,
};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlSnapshot;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::XmlSqliteTables;
use semio_s_artifact_stdio_zip::opc::sqlite::OpcSqliteTables;

#[path = "💰️backing/🦀️.rs"]
mod backing;

const OPC: OpcSqliteTables =
    OpcSqliteTables { package: "docx_package", part: "docx_binary_part", default_type: "docx_default_content_type", override_type: "docx_override_content_type", relationship_owner: "docx_relationship_owner", relationship: "docx_relationship" };
const XML: XmlSqliteTables = XmlSqliteTables {
    document: "docx_xml_document",
    node: "docx_xml_node",
    element: "docx_xml_element",
    text: "docx_xml_text",
    cdata: "docx_xml_cdata",
    comment: "docx_xml_comment",
    processing_instruction: "docx_xml_processing_instruction",
    attribute: "docx_xml_attribute",
    child: "docx_xml_child",
    document_misc: "docx_xml_document_misc",
    declaration: "docx_xml_declaration",
    doctype: "docx_xml_doctype",
    entity: "docx_xml_entity",
};

fn add(total: &mut usize, value: usize) -> Result<(), ValueError> {
    *total = total.checked_add(value).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "DOCX relational size overflow"))?;
    Ok(())
}

fn add_bytes(total: &mut usize, value: usize) -> Result<(), ValueError> {
    *total = total.checked_add(value).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "DOCX relational size overflow"))?;
    Ok(())
}

struct Documents(Vec<XmlSnapshot>);

impl Drop for Documents {
    fn drop(&mut self) {
        for snapshot in self.0.drain(..) {
            snapshot.retire_sqlite_snapshot();
        }
    }
}

struct Parts(Vec<DocxXmlPart>);

impl ArtifactSqliteSnapshotHost for DocxSnapshot {
    fn encode_sqlite_snapshot_native(&self, encoding: semio_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome {
        let limits=control.limits();
        store::space_history::io::sqlite::snapshot::settlement::encode_typed(control,native_owner,output,|native|crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::encode(self,encoding,limits,native))
    }

    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>,native_owner: &mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>)->store::os_store::serialization::SnapshotReceivingOutcome {
        let limits=control.limits();
        store::space_history::io::sqlite::snapshot::settlement::decode_typed(control,native_owner,output,|native|crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::decode(payload,limits,native))
    }

    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::preflight(self, encoding, control)
    }

    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(store::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind != "s.stdio.docx" || dialect.standard != "ecma-376" {
            return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "DOCX owned dialect belongs to another snapshot owner")));
        }
        match dialect.subset.as_str() {
            "*" => Ok(store::io_schema::IoOutcome::clean(())),
            "strict" | "transitional" => subset::validate(self, &dialect.subset, control),
            _ => Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "DOCX owned subset has no declared semantic validator"))),
        }
    }

    

    fn retire_sqlite_snapshot(self) {
        drop(self);
    }

    

    
}
impl pack::snapshot::ArtifactSqliteSnapshot for DocxSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        backing::projection::project(self, control)
    }
fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        backing::reconstruction::reconstruct(database, control)
    }
}

#[path = "🛡️subset/🦀️.rs"]
pub(crate) mod subset;
