//! 📥️ XLSX owns admitted parts and retains every XML document through final control.
use super::super::{Documents, Parts, XlsxSnapshot, XlsxXmlPart, OPC, XML};
use semio_framework_os_kernel::{
    sqlite_snapshot::{
        artifact::{ordered_row_refs, reconstruct_text},
        transfer::{heap_sort, reserve},
        validate_sqlite_database_schema_controlled, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind,
    },
    ArtifactSqliteSnapshot,
};
use semio_framework_value::DecodedValue;
use semio_s_artifact_stdio_xml::schema::snapshot::sqlite::reconstruct_xml_documents;
use semio_s_artifact_stdio_zip::opc::sqlite::reconstruct_opc_package;
const PHASE: SqliteSnapshotPhase = SqliteSnapshotPhase::ReconstructSnapshot;

fn invalid(message: &str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}
fn integer(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "XLSX ordinal exceeds SQLite integer width"))
}
fn retire(snapshot: XlsxSnapshot) {
    snapshot.retire_sqlite_snapshot();
}

pub(in super::super) fn reconstruct(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<XlsxSnapshot, ValueError> {
    control.check_database(database, PHASE)?;
    validate_sqlite_database_schema_controlled(database, XlsxSnapshot::SQLITE_SCHEMA, PHASE, control)?;
    let root = database.table("xlsx_document")?.single_row()?;
    if root.rowid != 1 || root.values.len() != 3 || root.integer(0)? != 1 || root.integer(2)? != 1 {
        return Err(invalid("XLSX document or package owner is invalid"));
    }
    let table = database.table("xlsx_xml_part")?;
    let mut identities = reserve(table.rows.len(), control)?;
    for (index, row) in table.rows.iter().enumerate() {
        if index % 256 == 0 {
            control.checkpoint(PHASE, index, table.rows.len())?;
        }
        if row.rowid <= 0 || row.values.len() != 5 || row.integer(0)? != row.rowid || row.integer(1)? != 1 {
            return Err(invalid("XLSX part identity or document owner is invalid"));
        }
        identities.push(row.rowid);
    }
    heap_sort(&mut identities, PHASE, control, |a, b, _| Ok(a.cmp(b)))?;
    for (index, pair) in identities.windows(2).enumerate() {
        if index % 256 == 0 {
            control.checkpoint(PHASE, index, identities.len())?;
        }
        if pair[0] == pair[1] {
            return Err(invalid("XLSX part identity is duplicated"));
        }
    }
    let rows = ordered_row_refs(table, 2, control)?;
    for (index, row) in rows.iter().enumerate() {
        if index % 256 == 0 {
            control.checkpoint(PHASE, index, rows.len())?;
        }
        let target = index.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XLSX XML identity overflow"))?;
        if row.integer(4)? != integer(target)? {
            return Err(invalid("XLSX part must own its ordered XML document"));
        }
    }
    let schema = reconstruct_text(control, root.text(1)?)?;
    let opc = reconstruct_opc_package(database, OPC, control)?;
    let mut documents = Documents(reconstruct_xml_documents(database, XlsxSnapshot::SQLITE_SCHEMA, XML, control)?);
    if documents.0.len() != rows.len() {
        return Err(invalid("XLSX part and XML document cardinality disagree"));
    }
    let mut parts = Parts(reserve(rows.len(), control)?);
    for (index, row) in rows.iter().enumerate() {
        if index % 256 == 0 {
            control.checkpoint(PHASE, index, rows.len())?;
        }
        let content_type = reconstruct_text(control, row.text(3)?)?;
        let document = &mut documents.0[index];
        parts.0.push(XlsxXmlPart { path: std::mem::take(&mut document.schema), content_type, document: std::mem::take(&mut document.doc) });
    }
    let snapshot = DecodedValue::new(XlsxSnapshot { schema, opc, xml_parts: std::mem::take(&mut parts.0) }, retire);
    control.checkpoint(PHASE, rows.len(), rows.len())?;
    Ok(snapshot.take())
}
