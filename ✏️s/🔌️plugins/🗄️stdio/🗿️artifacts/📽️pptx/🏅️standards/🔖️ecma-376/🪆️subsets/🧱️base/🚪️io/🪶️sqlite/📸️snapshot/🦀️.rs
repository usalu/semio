//! 📕️ Complete PPTX OPC entities and typed multi-document XML graph.

use crate::standards::v_ecma_376::subsets::base::schema::snapshot::*;
use semio_framework_os_kernel::{
    sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind},
    ArtifactSqliteSnapshot,
};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlSnapshot;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::XmlSqliteTables;
use semio_s_artifact_stdio_zip::opc::sqlite::OpcSqliteTables;

#[path = "💰️backing/🦀️.rs"]
mod backing;

const OPC: OpcSqliteTables =
    OpcSqliteTables { package: "pptx_package", part: "pptx_binary_part", default_type: "pptx_default_content_type", override_type: "pptx_override_content_type", relationship_owner: "pptx_relationship_owner", relationship: "pptx_relationship" };
const XML: XmlSqliteTables = XmlSqliteTables {
    document: "pptx_xml_document",
    node: "pptx_xml_node",
    element: "pptx_xml_element",
    text: "pptx_xml_text",
    cdata: "pptx_xml_cdata",
    comment: "pptx_xml_comment",
    processing_instruction: "pptx_xml_processing_instruction",
    attribute: "pptx_xml_attribute",
    child: "pptx_xml_child",
    document_misc: "pptx_xml_document_misc",
    declaration: "pptx_xml_declaration",
    doctype: "pptx_xml_doctype",
    entity: "pptx_xml_entity",
};

fn add(total: &mut usize, value: usize) -> Result<(), ValueError> {
    *total = total.checked_add(value).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "PPTX relational size overflow"))?;
    Ok(())
}

fn add_bytes(total: &mut usize, value: usize) -> Result<(), ValueError> {
    *total = total.checked_add(value).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "PPTX relational size overflow"))?;
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

struct Parts(Vec<PptxXmlPart>);

impl Drop for Parts {
    fn drop(&mut self) {
        for part in self.0.drain(..) {
            XmlSnapshot { schema: part.path, doc: part.document }.retire_sqlite_snapshot();
        }
    }
}

impl ArtifactSqliteSnapshot for PptxSnapshot {
    fn encode_sqlite_snapshot_native(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, ValueError> {
        crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::encode(self, encoding, control,native_owner)
    }

    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::decode(payload, control,native_control)
    }

    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native::preflight(self, encoding, control)
    }

    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(store::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind != "s.stdio.pptx" || dialect.standard != "ecma-376" {
            return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "PPTX owned dialect belongs to another snapshot owner")));
        }
        match dialect.subset.as_str() {
            "*" => Ok(store::io_schema::IoOutcome::clean(())),
            "strict" | "transitional" => crate::standards::v_ecma_376::subsets::base::io::sqlite::snapshot::subset::validate(self, &dialect.subset, control),
            _ => Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "PPTX owned subset has no declared semantic validator"))),
        }
    }

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn retire_sqlite_snapshot(self) {
        drop(Parts(self.xml_parts));
    }

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        backing::projection::project(self, control)
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        backing::reconstruction::reconstruct(database, control)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path = "🛡️subset/🦀️.rs"]
pub(crate) mod subset;
