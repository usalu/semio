//! 📽️ The three canonical PPTX fields compose into one paid relational owner.
use crate::standards::v_ecma_376::subsets::base::io::sqlite::snapshot::{add, add_bytes, PptxSnapshot, OPC, XML};
use semio_framework_os_kernel::{
    sqlite_snapshot::{
        artifact::{Cell, Projection},
        transfer::reserve,
        SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind,
    },
    ArtifactSqliteSnapshot,
};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{append_xml_document_views, measure_xml_document_views, XmlDocumentView};
use semio_s_artifact_stdio_zip::opc::sqlite::{append_opc_package, measure_opc_package};

fn integer(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "PPTX part identity exceeds SQLite integer width"))
}

pub(in crate::standards::v_ecma_376::subsets::base::io::sqlite::snapshot::backing::super) fn project(snapshot: &PptxSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
    let (mut rows, mut bytes) = measure_opc_package(&snapshot.opc, control)?;
    add(&mut rows, 1)?;
    add(&mut rows, snapshot.xml_parts.len())?;
    add_bytes(&mut bytes, 16)?;
    add_bytes(&mut bytes, snapshot.schema.len())?;
    let document_count = snapshot.xml_parts.len();
    control.check_rows(rows.checked_add(document_count).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "PPTX document count overflow"))?)?;
    let mut documents = reserve(document_count, control)?;
    for (position, part) in snapshot.xml_parts.iter().enumerate() {
        if position % 256 == 0 {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, position, snapshot.xml_parts.len())?;
        }
        add_bytes(&mut bytes, 32)?;
        add_bytes(&mut bytes, part.content_type.len())?;
        control.check_value_bytes(bytes)?;
        documents.push((part.path.as_str(), XmlDocumentView::from(&part.document)));
    }
    let (xml_rows, xml_bytes) = measure_xml_document_views(&documents, control)?;
    add(&mut rows, xml_rows)?;
    add_bytes(&mut bytes, xml_bytes)?;
    control.check_rows(rows)?;
    control.check_value_bytes(bytes)?;
    let mut output = Projection::new(PptxSnapshot::SQLITE_SCHEMA, control)?;
    let package = append_opc_package(&snapshot.opc, OPC, &mut output)?;
    append_xml_document_views(&documents, XML, &mut output)?;
    let document = output.insert("pptx_document", &[Cell::Text(&snapshot.schema), Cell::Integer(package)])?;
    for (position, part) in snapshot.xml_parts.iter().enumerate() {
        let target = position.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "PPTX part identity overflow"))?;
        output.insert("pptx_xml_part", &[Cell::Integer(document), Cell::Integer(integer(position)?), Cell::Text(&part.content_type), Cell::Integer(integer(target)?)])?;
        output.checkpoint_total(rows)?;
    }
    output.checkpoint_total(rows)?;
    output.finish()
}
