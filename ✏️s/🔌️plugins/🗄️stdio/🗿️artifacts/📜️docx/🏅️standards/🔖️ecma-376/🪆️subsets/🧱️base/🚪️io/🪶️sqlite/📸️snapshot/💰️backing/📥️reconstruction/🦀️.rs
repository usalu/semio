//! 📥️ DOCX owns admitted parts and retains every XML document through final control.
use crate::standards::v_ecma_376::subsets::base::io::sqlite::snapshot::{docx_xml_parts_from_iter_controlled, Documents, DocxSnapshot, DocxXmlPart, Parts, OPC, XML};
use semio_framework_os_kernel::{
    sqlite_snapshot::{
        artifact::{ordered_row_refs, reconstruct_text},
        transfer::{heap_sort, reserve},
        validate_sqlite_database_schema_controlled, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind,
    },
    ArtifactSqliteSnapshot,
};
use semio_framework_value::{native_decoding::NativeDecodeProgress, DecodedValue, NativeDecodeControl};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::reconstruct_xml_documents;
use semio_s_artifact_stdio_zip::opc::sqlite::reconstruct_opc_package;
use semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage;
const PHASE: SqliteSnapshotPhase = SqliteSnapshotPhase::ReconstructSnapshot;

fn invalid(message: &str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}
fn integer(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "DOCX ordinal exceeds SQLite integer width"))
}
fn retire(snapshot: DocxSnapshot) {
    snapshot.retire_sqlite_snapshot();
}

pub(crate) fn reconstruct(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<DocxSnapshot, ValueError> {
    control.check_database(database, PHASE)?;
    validate_sqlite_database_schema_controlled(database, DocxSnapshot::SQLITE_SCHEMA, PHASE, control)?;
    let root = database.table("docx_document")?.single_row()?;
    if root.rowid != 1 || root.values.len() != 3 || root.integer(0)? != 1 || root.integer(2)? != 1 {
        return Err(invalid("DOCX document or package owner is invalid"));
    }
    let table = database.table("docx_xml_part")?;
    let mut identities = reserve(table.rows.len(), control)?;
    for (index, row) in table.rows.iter().enumerate() {
        if index % 256 == 0 {
            control.checkpoint(PHASE, index, table.rows.len())?;
        }
        if row.rowid <= 0 || row.values.len() != 5 || row.integer(0)? != row.rowid || row.integer(1)? != 1 {
            return Err(invalid("DOCX part identity or document owner is invalid"));
        }
        identities.push(row.rowid);
    }
    heap_sort(&mut identities, PHASE, control, |a, b, _| Ok(a.cmp(b)))?;
    for (index, pair) in identities.windows(2).enumerate() {
        if index % 256 == 0 {
            control.checkpoint(PHASE, index, identities.len())?;
        }
        if pair[0] == pair[1] {
            return Err(invalid("DOCX part identity is duplicated"));
        }
    }
    let rows = ordered_row_refs(table, 2, control)?;
    for (index, row) in rows.iter().enumerate() {
        if index % 256 == 0 {
            control.checkpoint(PHASE, index, rows.len())?;
        }
        let target = index.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "DOCX XML identity overflow"))?;
        if row.integer(4)? != integer(target)? {
            return Err(invalid("DOCX part must own its ordered XML document"));
        }
    }
    let schema = reconstruct_text(control, root.text(1)?)?;
    let package = reconstruct_opc_package(database, OPC, control)?;
    let opc = control.allocation_stage(PHASE, |remaining, progress,allocation|{
        let mut callback = |event: semio_framework_value::native_decoding::NativeDecodeProgress| progress(event.completed, event.total);
        let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=semio_framework_value::NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);
        let result = RetainedOpcPackage::try_from_package_controlled(package, &mut native);
        (result, native.owned_bytes())
    })??;
    let mut documents = Documents(reconstruct_xml_documents(database, DocxSnapshot::SQLITE_SCHEMA, XML, control)?);
    if documents.0.len() != rows.len() {
        return Err(invalid("DOCX part and XML document cardinality disagree"));
    }
    let mut parts = Parts(reserve(rows.len(), control)?);
    for (index, row) in rows.iter().enumerate() {
        if index % 256 == 0 {
            control.checkpoint(PHASE, index, rows.len())?;
        }
        let content_type = reconstruct_text(control, row.text(3)?)?;
        let document = &mut documents.0[index];
        let path = std::mem::take(&mut document.schema);
        let source = std::mem::take(&mut document.doc);
        let part = control.allocation_stage(PHASE, |remaining, progress,allocation|{
            let mut callback = |event: NativeDecodeProgress| progress(event.completed, event.total);
            let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);
            let result = DocxXmlPart::try_from_document_controlled(path, content_type, source, &mut native);
            (result, native.owned_bytes())
        })??;
        parts.0.push(part);
    }
    let xml_parts = control.allocation_stage(PHASE, |remaining, progress,allocation|{
        let mut callback = |event: NativeDecodeProgress| progress(event.completed, event.total);
        let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);
        let result = docx_xml_parts_from_iter_controlled(std::mem::take(&mut parts.0), &mut native);
        (result, native.owned_bytes())
    })??;
    let snapshot = DecodedValue::new(DocxSnapshot { schema, opc, xml_parts }, retire);
    control.checkpoint(PHASE, rows.len(), rows.len())?;
    Ok(snapshot.take())
}
