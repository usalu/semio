//! 📕️ Complete XLSX OPC entities and typed multi-document XML graph.

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
use crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native;
#[path = "🛡️subset/🦀️.rs"]
mod subset;

const OPC: OpcSqliteTables =
    OpcSqliteTables { package: "xlsx_package", part: "xlsx_binary_part", default_type: "xlsx_default_content_type", override_type: "xlsx_override_content_type", relationship_owner: "xlsx_relationship_owner", relationship: "xlsx_relationship" };
const XML: XmlSqliteTables = XmlSqliteTables {
    document: "xlsx_xml_document",
    node: "xlsx_xml_node",
    element: "xlsx_xml_element",
    text: "xlsx_xml_text",
    cdata: "xlsx_xml_cdata",
    comment: "xlsx_xml_comment",
    processing_instruction: "xlsx_xml_processing_instruction",
    attribute: "xlsx_xml_attribute",
    child: "xlsx_xml_child",
    document_misc: "xlsx_xml_document_misc",
    declaration: "xlsx_xml_declaration",
    doctype: "xlsx_xml_doctype",
    entity: "xlsx_xml_entity",
};

fn add(total: &mut usize, value: usize) -> Result<(), ValueError> {
    *total = total.checked_add(value).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XLSX relational size overflow"))?;
    Ok(())
}

fn add_bytes(total: &mut usize, value: usize) -> Result<(), ValueError> {
    *total = total.checked_add(value).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "XLSX relational size overflow"))?;
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

struct Parts(Vec<XlsxXmlPart>);

impl Drop for Parts {
    fn drop(&mut self) {
        for part in self.0.drain(..) {
            XmlSnapshot { schema: part.path, doc: part.document }.retire_sqlite_snapshot();
        }
    }
}

impl ArtifactSqliteSnapshot for XlsxSnapshot {
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(store::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind != "s.stdio.xlsx" || dialect.standard != "ecma-376" {
            return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "XLSX owned dialect belongs to another snapshot owner")));
        }
        match dialect.subset.as_str() {
            "*" => Ok(store::io_schema::IoOutcome::clean(())),
            "strict" | "transitional" => subset::validate(self, &dialect.subset, control),
            _ => Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "XLSX owned subset has no declared semantic validator"))),
        }
    }

    fn encode_sqlite_snapshot_native(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
        native::encode(self, encoding, control)
    }

    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        native::decode(payload, control)
    }

    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        native::preflight(self, encoding, control)
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

