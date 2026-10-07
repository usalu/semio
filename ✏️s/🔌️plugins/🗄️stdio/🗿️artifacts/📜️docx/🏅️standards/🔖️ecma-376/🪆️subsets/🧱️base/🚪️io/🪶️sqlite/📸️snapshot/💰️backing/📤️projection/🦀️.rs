//! 📚️ DOCX components share one cumulative owned projection and final handoff.
use crate::standards::v_ecma_376::subsets::base::io::sqlite::snapshot::{add, add_bytes, DocxSnapshot, OPC, XML};
use semio_framework_os_kernel::{
    sqlite_snapshot::{
        artifact::{Cell, Projection},
        transfer::reserve,
        SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind,
    },
    ArtifactSqliteSnapshot,
};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{append_xml_document_views,measure_xml_document_views};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::schema::snapshot::ownership::{XmlDocumentView};
use semio_s_artifact_stdio_zip::opc::sqlite::{append_opc_package, measure_opc_package};
use semio_framework_value::native_encoding::NativeEncodeProgress;
use semio_framework_value::NativeEncodeControl;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};

struct MaterializedDocument {
    document: Option<XmlDocument>,
    retirement: Vec<XmlNode>,
}

impl Drop for MaterializedDocument {
    fn drop(&mut self) {
        let Some(mut document) = self.document.take() else { return };
        self.retirement.extend(std::mem::take(&mut document.prolog));
        self.retirement.extend(std::mem::take(&mut document.epilog));
        self.retirement.extend(document.root.take());
        while let Some(node) = self.retirement.pop() {
            if let XmlNode::Element { children, .. } = node {
                self.retirement.extend(children);
            }
        }
    }
}

struct MaterializedDocuments(Vec<MaterializedDocument>);

fn integer(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "DOCX part identity exceeds SQLite integer width"))
}

fn materialize_package(snapshot: &DocxSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<semio_s_artifact_stdio_zip::opc::OpcPackage, ValueError> {
    control
        .allocation_stage(SqliteSnapshotPhase::ProjectSnapshot, |remaining, progress| {
            let mut callback = |event: NativeEncodeProgress| progress(event.completed, event.total);
            let mut native = NativeEncodeControl::new(remaining, &mut callback);
            let result = snapshot.opc.materialize_package(&mut native);
            let owned = native.owned_bytes();
            (result, owned)
        })?
}

pub(crate) fn project(snapshot: &DocxSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
    let opc = materialize_package(snapshot, control)?;
    let mut materialized = MaterializedDocuments(reserve(snapshot.xml_parts.len(), control)?);
    for (position, part) in snapshot.xml_parts.iter().enumerate() {
        if position % 256 == 0 {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, position, snapshot.xml_parts.len())?;
        }
        let document = control.allocation_stage(SqliteSnapshotPhase::ProjectSnapshot, |remaining, progress| {
            let mut callback = |event: NativeEncodeProgress| progress(event.completed, event.total);
            let mut native = NativeEncodeControl::new(remaining, &mut callback);
            let result = match native.allocate_vec::<XmlNode>(part.document.nodes.len()) {
                Ok(retirement) => part.materialize_document(&mut native).map(|document| MaterializedDocument { document: Some(document), retirement }),
                Err(error) => Err(error),
            };
            (result, native.owned_bytes())
        })??;
        materialized.0.push(document);
    }
    let (mut rows, mut bytes) = measure_opc_package(&opc, control)?;
    add(&mut rows, 1)?;
    add(&mut rows, snapshot.xml_parts.len())?;
    add_bytes(&mut bytes, 16)?;
    add_bytes(&mut bytes, snapshot.schema.len())?;
    control.check_rows(rows.checked_add(snapshot.xml_parts.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "DOCX document count overflow"))?)?;
    let mut documents = reserve(snapshot.xml_parts.len(), control)?;
    for (position, part) in snapshot.xml_parts.iter().enumerate() {
        if position % 256 == 0 {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, position, snapshot.xml_parts.len())?;
        }
        add_bytes(&mut bytes, 32)?;
        add_bytes(&mut bytes, part.content_type.len())?;
        control.check_value_bytes(bytes)?;
        documents.push((part.path.as_str(), XmlDocumentView::from(materialized.0[position].document.as_ref().expect("materialized DOCX XML owner"))));
    }
    let (xml_rows, xml_bytes) = measure_xml_document_views(&documents, control)?;
    add(&mut rows, xml_rows)?;
    add_bytes(&mut bytes, xml_bytes)?;
    control.check_rows(rows)?;
    control.check_value_bytes(bytes)?;
    let mut output = Projection::new(DocxSnapshot::SQLITE_SCHEMA, control)?;
    let package = append_opc_package(&opc, OPC, &mut output)?;
    append_xml_document_views(&documents, XML, &mut output)?;
    let document = output.insert("docx_document", &[Cell::Text(&snapshot.schema), Cell::Integer(package)])?;
    for (position, part) in snapshot.xml_parts.iter().enumerate() {
        let target = position.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "DOCX part identity overflow"))?;
        output.insert("docx_xml_part", &[Cell::Integer(document), Cell::Integer(integer(position)?), Cell::Text(&part.content_type), Cell::Integer(integer(target)?)])?;
        output.checkpoint_total(rows)?;
    }
    output.checkpoint_total(rows)?;
    output.finish()
}
