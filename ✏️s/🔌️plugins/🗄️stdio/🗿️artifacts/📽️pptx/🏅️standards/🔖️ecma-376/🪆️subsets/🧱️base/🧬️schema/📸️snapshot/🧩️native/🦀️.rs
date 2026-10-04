//! 🎞️ Literal PPTX OPC and authoritative XML parts under one native control.
use super::{PptxSnapshot, PptxXmlPart};
use semio_framework_os_kernel::{
    io_schema::IoPayload,
    sqlite_snapshot::{SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotControl},
};
use semio_framework_value::{DecodedValue, ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_xml::schema::snapshot::sqlite::{retire_xml_document, XmlDocumentView};
use semio_s_artifact_stdio_zip::opc::native::{OpcNativeReader, OpcNativeWriter};
#[path = "💰️backing/🦀️.rs"]
mod backing;
struct Parts(Vec<PptxXmlPart>);
impl Drop for Parts {
    fn drop(&mut self) {
        for part in self.0.drain(..) {
            retire_xml_document(part.document);
        }
    }
}
fn write(snapshot: &PptxSnapshot, writer: &mut OpcNativeWriter<'_, '_, '_>) -> Result<(), ValueError> {
    writer.rows(1)?;
    if writer.encoding == SnapshotEncoding::Binary {
        let token = b"stdio.pptx.pack v1";
        writer.raw(b"\x89SEM\r\n\x1a\n")?;
        writer.raw(&(token.len() as u32).to_le_bytes())?;
        writer.raw(token)?;
        writer.raw(&[1])?;
    } else {
        writer.raw(b"semio stdio.pptx.dsl v1\n[")?;
    }
    writer.string(&snapshot.schema)?;
    writer.delimiter(b",")?;
    writer.package(&snapshot.opc)?;
    writer.delimiter(b",")?;
    writer.list(&snapshot.xml_parts, |writer, part| {
        writer.delimiter(b"[")?;
        writer.string(&part.path)?;
        writer.delimiter(b",")?;
        writer.string(&part.content_type)?;
        writer.delimiter(b",")?;
        writer.document(XmlDocumentView::from(&part.document))?;
        writer.delimiter(b"]")
    })?;
    writer.delimiter(b"]")
}
pub(super) fn encode(snapshot: &PptxSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<IoPayload, ValueError> {
    backing::encode(snapshot, encoding, control)
}
pub(super) fn preflight(snapshot: &PptxSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    backing::preflight(snapshot, encoding, control)
}
fn length(reader: &mut OpcNativeReader<'_, '_, '_>) -> Result<usize, ValueError> {
    reader.delimiter(b'[')?;
    let count = if reader.binary { usize::try_from(reader.unsigned()?).map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "PPTX collection exceeds platform width"))? } else { 0 };
    if reader.binary {
        reader.rows(count)?;
    }
    Ok(count)
}
fn more(reader: &OpcNativeReader<'_, '_, '_>, position: usize, count: usize) -> bool {
    if reader.binary {
        position < count
    } else {
        reader.bytes.get(reader.position) != Some(&b']')
    }
}
fn read(reader: &mut OpcNativeReader<'_, '_, '_>) -> Result<PptxSnapshot, ValueError> {
    reader.rows(1)?;
    if reader.binary {
        if reader.take(1)? != [1] {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "PPTX native revision differs"));
        }
    } else {
        reader.delimiter(b'[')?;
    }
    let schema = reader.string()?;
    reader.delimiter(b',')?;
    let opc = reader.package()?;
    reader.delimiter(b',')?;
    let count = length(reader)?;
    let mut parts = Parts(reader.control.allocate_vec::<PptxXmlPart>(count)?);
    while more(reader, parts.0.len(), count) {
        if !reader.binary {
            reader.rows(1)?;
            reader.reserve(&mut parts.0)?;
        }
        if !parts.0.is_empty() {
            reader.delimiter(b',')?;
        }
        reader.delimiter(b'[')?;
        let path = reader.string()?;
        reader.delimiter(b',')?;
        let content_type = reader.string()?;
        reader.delimiter(b',')?;
        let document = DecodedValue::new(reader.document()?, retire_xml_document);
        reader.delimiter(b']')?;
        parts.0.push(PptxXmlPart { path, content_type, document: document.take() });
    }
    reader.delimiter(b']')?;
    reader.delimiter(b']')?;
    if reader.position != reader.bytes.len() {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "PPTX native input has trailing fields"));
    }
    reader.control.checkpoint()?;
    Ok(PptxSnapshot { schema, opc, xml_parts: std::mem::take(&mut parts.0) })
}
fn input(bytes: &[u8], binary: bool, control: &mut SqliteSnapshotControl<'_>) -> Result<PptxSnapshot, ValueError> {
    backing::input(bytes, binary, control)
}
pub(super) fn decode(payload: &IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<PptxSnapshot, ValueError> {
    match payload {
        IoPayload::Binary(bytes) => input(bytes, true, control),
        IoPayload::Text(text) => input(text.as_bytes(), false, control),
    }
}
pub(super) fn decode_text(text: &str, control: &mut SqliteSnapshotControl<'_>) -> Result<PptxSnapshot, ValueError> {
    input(text.as_bytes(), false, control)
}
pub(super) fn decode_binary(bytes: &[u8], control: &mut SqliteSnapshotControl<'_>) -> Result<PptxSnapshot, ValueError> {
    input(bytes, true, control)
}
pub(super) fn pack_limits(limits: &store::mounted_pack_rt::PackLimits) -> SqliteDatabaseLimits {
    SqliteDatabaseLimits {
        max_file_bytes: usize::try_from(limits.max_file_len).unwrap_or(usize::MAX),
        max_value_bytes: usize::try_from(limits.max_total_alloc).unwrap_or(usize::MAX),
        max_rows: usize::try_from(limits.max_items).unwrap_or(usize::MAX),
        ..SqliteDatabaseLimits::default()
    }
}
