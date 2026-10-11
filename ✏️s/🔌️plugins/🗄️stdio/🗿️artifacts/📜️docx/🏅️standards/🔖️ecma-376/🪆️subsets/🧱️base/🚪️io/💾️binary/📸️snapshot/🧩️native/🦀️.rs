//! 📜️ Complete DOCX literal schema, ordered OPC metadata and typed XML fields.
use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,ValueError,ValueRefusalKind};
use semio_framework_value::DecodedValue;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{docx_xml_parts_from_iter_controlled,DocxSnapshot,DocxXmlPart};
use semio_framework_os_kernel::{sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,SqliteDatabaseLimits},io_schema::IoPayload};
use semio_s_artifact_stdio_zip::opc::native::{OpcNativeWriter,OpcNativeReader};
use semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::schema::snapshot::ownership::{XmlDocumentView,retire_xml_document};
#[path="💰️backing/🦀️.rs"] mod backing;
struct Parts(Vec<DocxXmlPart>);
struct Document(Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument>);
impl Drop for Document{fn drop(&mut self){if let Some(document)=self.0.take(){retire_xml_document(document);}}}
fn write(snapshot:&DocxSnapshot,writer:&mut OpcNativeWriter<'_, '_, '_>)->Result<(),ValueError>{
 writer.rows(1)?;
 if writer.encoding==SnapshotEncoding::Binary{let token=b"stdio.docx.pack v1";writer.raw(b"\x89SEM\r\n\x1a\n")?;writer.raw(&(token.len()as u32).to_le_bytes())?;writer.raw(token)?;writer.raw(&[1])?;}else{writer.raw(b"semio stdio.docx.dsl v1\n[")?;}
 writer.string(&snapshot.schema)?;writer.delimiter(b",")?;let package=snapshot.opc.materialize_package(writer.control)?;writer.package(&package)?;writer.delimiter(b",")?;
 writer.list_iter(snapshot.xml_parts.iter(),|writer,part|{writer.delimiter(b"[")?;writer.string(&part.path)?;writer.delimiter(b",")?;writer.string(&part.content_type)?;writer.delimiter(b",")?;let document=Document(Some(part.materialize_document(writer.control)?));writer.document(XmlDocumentView::from(document.0.as_ref().expect("owned DOCX native XML document")))?;writer.delimiter(b"]")})?;writer.delimiter(b"]")
}

pub(crate) fn preflight(snapshot:&DocxSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{backing::preflight(snapshot,encoding,control)}
fn read(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<DocxSnapshot,ValueError>{
 reader.rows(1)?;if reader.binary{if reader.take(1)?!=[1]{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DOCX native revision differs"));}}else{reader.delimiter(b'[')?;}
 let schema=reader.string()?;reader.delimiter(b',')?;let opc=RetainedOpcPackage::try_from_package(reader.package()?)?;reader.delimiter(b',')?;reader.delimiter(b'[')?;
 let count=if reader.binary{usize::try_from(reader.unsigned()?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"DOCX native part length exceeds platform width"))?}else{0};if reader.binary{reader.rows(count)?;}let mut parts=Parts(reader.control.allocate_vec::<DocxXmlPart>(count)?);
 while if reader.binary{parts.0.len()<count}else{reader.bytes.get(reader.position)!=Some(&b']')}{
  if !reader.binary{reader.rows(1)?;reader.reserve(&mut parts.0)?;}if !parts.0.is_empty(){reader.delimiter(b',')?;}reader.delimiter(b'[')?;let path=reader.string()?;reader.delimiter(b',')?;let content_type=reader.string()?;reader.delimiter(b',')?;
  let document=DecodedValue::new(reader.document()?,retire_xml_document);reader.delimiter(b']')?;parts.0.push(DocxXmlPart::try_from_document_controlled(path,content_type,document.take(),reader.control)?);
 }
 reader.delimiter(b']')?;reader.delimiter(b']')?;if reader.position!=reader.bytes.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DOCX native input has trailing fields"));}reader.control.checkpoint()?;let xml_parts=docx_xml_parts_from_iter_controlled(std::mem::take(&mut parts.0),reader.control)?;Ok(DocxSnapshot{schema,opc,xml_parts})
}

pub(crate) fn decode(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<DocxSnapshot,ValueError>{match payload{IoPayload::Binary(bytes)=>input(bytes,true,control),IoPayload::Text(text)=>input(text.as_bytes(),false,control)}}




pub(crate) fn encode(snapshot:&DocxSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<IoPayload,ValueError>{backing::encode(snapshot,encoding,control,native_owner)}
pub(crate) fn encode_standalone(snapshot:&DocxSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,ValueError>{let mut progress=|_|true;let mut native=semio_framework_value::NativeEncodeControl::new(control.limits().max_allocation_bytes,&mut progress);let mut owner=semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native,semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:u32::MAX as usize,maximum_copy_bytes:u32::MAX as usize,maximum_capacity_bytes:u32::MAX as usize,maximum_release_bytes:u32::MAX as usize,maximum_depth:1<<16});encode(snapshot,encoding,control,&mut owner)}
pub(crate) fn input(bytes:&[u8],binary:bool,control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,ValueError>{backing::input(bytes,binary,control)}
pub(crate) fn decode_binary(bytes:&[u8],control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,ValueError>{input(bytes,true,control)}
pub(crate) fn decode_text(text:&str,control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,ValueError>{input(text.as_bytes(),false,control)}
pub(crate) fn pack_limits(limits:&store::mounted_pack_rt::PackLimits)->SqliteDatabaseLimits{SqliteDatabaseLimits{max_file_bytes:usize::try_from(limits.max_file_len).unwrap_or(usize::MAX),max_value_bytes:usize::try_from(limits.max_total_alloc).unwrap_or(usize::MAX),max_rows:usize::try_from(limits.max_items).unwrap_or(usize::MAX),..SqliteDatabaseLimits::default()}}
